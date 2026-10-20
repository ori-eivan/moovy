//! What Windows can tell us about the user, read once a second. Nothing here is stored or sent.

/// A snapshot of the user's situation.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Signals {
    /// Milliseconds since the last keyboard or mouse input.
    pub idle_ms: u64,
    /// Screen locked or the screen saver is on.
    pub locked: bool,
    /// A presentation, a full-screen game or app, or quiet time.
    pub busy: bool,
    /// Some app is using the microphone right now (a call).
    pub mic_in_use: bool,
    /// The browser is sharing the screen (a call or a presentation, maybe with the mic off).
    pub sharing_screen: bool,
}

#[cfg(windows)]
pub fn read() -> Signals {
    let (locked, busy) = notification_state();
    Signals { idle_ms: idle_ms(), locked, busy, mic_in_use: mic_in_use(), sharing_screen: sharing_screen() }
}

#[cfg(target_os = "macos")]
pub fn read() -> Signals {
    Signals {
        idle_ms: mac::idle_ms(),
        locked: mac::screen_locked(),
        // macOS has no "presentation mode" signal, and recognising a screen share needs the
        // Screen Recording permission, which Moovy does not ask for. The card is left out of
        // captures anyway, and a call already holds cards through the microphone.
        busy: false,
        mic_in_use: mac::mic_in_use(),
        sharing_screen: false,
    }
}

#[cfg(not(any(windows, target_os = "macos")))]
pub fn read() -> Signals {
    Signals::default()
}

/// The macOS side of the signals, through the system frameworks directly. None of these ask
/// for a permission: idle time and the lock state come from the window server, and the
/// microphone check asks Core Audio whether an input device is running, without opening it.
#[cfg(target_os = "macos")]
pub mod mac {
    use std::ffi::{c_char, c_void};

    type CFTypeRef = *const c_void;

    #[link(name = "CoreGraphics", kind = "framework")]
    unsafe extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(state: i32, event_type: u32) -> f64;
        fn CGEventSourceButtonState(state: i32, button: u32) -> bool;
        fn CGSessionCopyCurrentDictionary() -> CFTypeRef;
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    unsafe extern "C" {
        fn CFDictionaryGetValue(dict: CFTypeRef, key: CFTypeRef) -> CFTypeRef;
        fn CFStringCreateWithCString(alloc: CFTypeRef, s: *const c_char, encoding: u32) -> CFTypeRef;
        fn CFBooleanGetValue(b: CFTypeRef) -> u8;
        fn CFGetTypeID(cf: CFTypeRef) -> usize;
        fn CFBooleanGetTypeID() -> usize;
        fn CFRelease(cf: CFTypeRef);
    }

    #[repr(C)]
    struct PropertyAddress {
        selector: u32,
        scope: u32,
        element: u32,
    }

    #[link(name = "CoreAudio", kind = "framework")]
    unsafe extern "C" {
        fn AudioObjectGetPropertyDataSize(
            object: u32,
            address: *const PropertyAddress,
            qualifier_size: u32,
            qualifier: *const c_void,
            size: *mut u32,
        ) -> i32;
        fn AudioObjectGetPropertyData(
            object: u32,
            address: *const PropertyAddress,
            qualifier_size: u32,
            qualifier: *const c_void,
            size: *mut u32,
            data: *mut c_void,
        ) -> i32;
    }

    /// The input as the hardware sees it, for every app in the session.
    const HID_SYSTEM_STATE: i32 = 1;
    const ANY_INPUT_EVENT: u32 = u32::MAX;
    const LEFT_BUTTON: u32 = 0;
    const UTF8: u32 = 0x0800_0100;

    const fn fourcc(code: &[u8; 4]) -> u32 {
        ((code[0] as u32) << 24) | ((code[1] as u32) << 16) | ((code[2] as u32) << 8) | code[3] as u32
    }
    const SYSTEM_OBJECT: u32 = 1;
    const DEVICES: u32 = fourcc(b"dev#");
    const STREAMS: u32 = fourcc(b"stm#");
    const RUNNING_SOMEWHERE: u32 = fourcc(b"gone");
    const SCOPE_GLOBAL: u32 = fourcc(b"glob");
    const SCOPE_INPUT: u32 = fourcc(b"inpt");
    const ELEMENT_MAIN: u32 = 0;

    pub fn idle_ms() -> u64 {
        let seconds = unsafe { CGEventSourceSecondsSinceLastEventType(HID_SYSTEM_STATE, ANY_INPUT_EVENT) };
        if seconds.is_finite() && seconds > 0.0 { (seconds * 1000.0) as u64 } else { 0 }
    }

    pub fn left_button_down() -> bool {
        unsafe { CGEventSourceButtonState(HID_SYSTEM_STATE, LEFT_BUTTON) }
    }

    /// Locked, or switched to another user (the session is no longer on the screen).
    pub fn screen_locked() -> bool {
        unsafe {
            let session = CGSessionCopyCurrentDictionary();
            if session.is_null() {
                return true;
            }
            let locked = bool_value(session, c"CGSSessionScreenIsLocked".as_ptr()).unwrap_or(false);
            let on_console = bool_value(session, c"kCGSSessionOnConsoleKey".as_ptr()).unwrap_or(true);
            CFRelease(session);
            locked || !on_console
        }
    }

    unsafe fn bool_value(dict: CFTypeRef, key: *const c_char) -> Option<bool> {
        unsafe {
            let key = CFStringCreateWithCString(std::ptr::null(), key, UTF8);
            if key.is_null() {
                return None;
            }
            let value = CFDictionaryGetValue(dict, key);
            CFRelease(key);
            if value.is_null() || CFGetTypeID(value) != CFBooleanGetTypeID() {
                return None;
            }
            Some(CFBooleanGetValue(value) != 0)
        }
    }

    /// Is any app recording from a microphone? True when any device that has input streams is
    /// running for some process. Output-only devices (speakers playing music) don't count.
    pub fn mic_in_use() -> bool {
        unsafe {
            let all = PropertyAddress { selector: DEVICES, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
            let mut size = 0u32;
            if AudioObjectGetPropertyDataSize(SYSTEM_OBJECT, &all, 0, std::ptr::null(), &mut size) != 0 {
                return false;
            }
            let mut devices = vec![0u32; size as usize / std::mem::size_of::<u32>()];
            if AudioObjectGetPropertyData(SYSTEM_OBJECT, &all, 0, std::ptr::null(), &mut size, devices.as_mut_ptr().cast()) != 0 {
                return false;
            }
            devices.iter().any(|&device| {
                let streams = PropertyAddress { selector: STREAMS, scope: SCOPE_INPUT, element: ELEMENT_MAIN };
                let mut streams_size = 0u32;
                let has_input = AudioObjectGetPropertyDataSize(device, &streams, 0, std::ptr::null(), &mut streams_size) == 0
                    && streams_size > 0;
                if !has_input {
                    return false;
                }
                let running = PropertyAddress { selector: RUNNING_SOMEWHERE, scope: SCOPE_GLOBAL, element: ELEMENT_MAIN };
                let mut value = 0u32;
                let mut value_size = std::mem::size_of::<u32>() as u32;
                AudioObjectGetPropertyData(device, &running, 0, std::ptr::null(), &mut value_size, (&mut value as *mut u32).cast()) == 0
                    && value != 0
            })
        }
    }
}

#[cfg(windows)]
fn idle_ms() -> u64 {
    use windows::Win32::System::SystemInformation::GetTickCount;
    use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
    let mut info = LASTINPUTINFO { cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32, dwTime: 0 };
    unsafe {
        if !GetLastInputInfo(&mut info).as_bool() {
            return 0;
        }
        // Both are 32-bit tick counts, so subtract with wrap-around.
        GetTickCount().wrapping_sub(info.dwTime) as u64
    }
}

/// Windows' own answer to "is now a good time for a notification?".
#[cfg(windows)]
fn notification_state() -> (bool, bool) {
    use windows::Win32::UI::Shell::{
        SHQueryUserNotificationState, QUNS_ACCEPTS_NOTIFICATIONS, QUNS_NOT_PRESENT,
    };
    match unsafe { SHQueryUserNotificationState() } {
        Ok(state) if state == QUNS_NOT_PRESENT => (true, false),
        Ok(state) if state == QUNS_ACCEPTS_NOTIFICATIONS => (false, false),
        // Busy, full-screen game, presentation, quiet time, full-screen app.
        Ok(_) => (false, true),
        Err(_) => (false, false),
    }
}

/// Is any app recording from a microphone right now (a call, a voice note)?
///
/// Asked the audio system directly: is any capture session active. Moovy never opens the
/// microphone or reads audio. The privacy registry is checked too, but on some Windows builds it
/// stops updating, so it can't be the only source (seen on 26200 in October 2026).
#[cfg(windows)]
fn mic_in_use() -> bool {
    capture_session_active() || mic_in_use_registry()
}

#[cfg(windows)]
fn capture_session_active() -> bool {
    use windows::Win32::Media::Audio::{
        eCapture, AudioSessionStateActive, IAudioSessionManager2, IMMDeviceEnumerator,
        MMDeviceEnumerator, DEVICE_STATE_ACTIVE,
    };
    use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};

    unsafe {
        // Once per thread; calling it again is harmless.
        let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
        let Ok(enumerator) = CoCreateInstance::<_, IMMDeviceEnumerator>(&MMDeviceEnumerator, None, CLSCTX_ALL) else {
            return false;
        };
        let Ok(devices) = enumerator.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE) else {
            return false;
        };
        let count = devices.GetCount().unwrap_or(0);
        for i in 0..count {
            let Ok(device) = devices.Item(i) else { continue };
            let Ok(manager) = device.Activate::<IAudioSessionManager2>(CLSCTX_ALL, None) else { continue };
            let Ok(sessions) = manager.GetSessionEnumerator() else { continue };
            for j in 0..sessions.GetCount().unwrap_or(0) {
                if let Ok(session) = sessions.GetSession(j)
                    && session.GetState().is_ok_and(|state| state == AudioSessionStateActive)
                {
                    return true;
                }
            }
        }
        false
    }
}

/// The titles Chrome and Edge give the "<site> is sharing ..." bar, for the screen, a window or a
/// tab, in English and Hebrew (from Chromium's own translations).
const SHARE_BAR_TITLES: &[&str] = &[
    "is sharing your screen",
    "is sharing a window",
    "is sharing a Chrome tab",
    "משתף את המסך",
    "משתף חלון",
    "משתפת חלון",
    "משתף כרטיסיית",
    "משתף כרטיסייה",
];

fn is_share_bar(title: &str, class: &str) -> bool {
    // A browser window whose page title merely mentions sharing is not the bar.
    let browser_window = title.contains(" - Google Chrome") || title.contains(" - Microsoft");
    (!browser_window && SHARE_BAR_TITLES.iter().any(|t| title.contains(t))) || class == "ZPFloatToolbarClass"
}

#[cfg(test)]
mod tests {
    use super::is_share_bar;

    #[test]
    fn recognises_share_bars() {
        assert!(is_share_bar("meet.google.com is sharing your screen.", "Chrome_WidgetWin_1"));
        assert!(is_share_bar("zoom.us is sharing a window.", "Chrome_WidgetWin_1"));
        assert!(is_share_bar("\u{202b}meet.google.com משתף את המסך שלך.\u{202c}", "Chrome_WidgetWin_1"));
        assert!(is_share_bar("\u{200f}teams.microsoft.com משתף כרטיסיית Chrome.", "Chrome_WidgetWin_1"));
        assert!(is_share_bar("Screen sharing meeting controls", "ZPFloatToolbarClass"));
    }

    #[test]
    fn ignores_ordinary_windows() {
        assert!(!is_share_bar("Meet - abc-defg-hij - Google Chrome", "Chrome_WidgetWin_1"));
        assert!(!is_share_bar("Why Chrome is sharing your screen - Google Chrome", "Chrome_WidgetWin_1"));
        assert!(!is_share_bar("Zoom Meeting", "ConfMultiTabContentWndClass"));
    }
}

/// Is the screen being shared? Recognised by the small bar window each app shows while sharing:
/// - Chrome and Edge: see SHARE_BAR_TITLES (English and Hebrew UI)
/// - Zoom app: the "Screen sharing meeting controls" bar, matched by its class name, which does
///   not change with the language.
///
/// Anything else is not matched, and the card is left out of the capture anyway.
#[cfg(windows)]
fn sharing_screen() -> bool {
    use windows::core::BOOL;
    use windows::Win32::Foundation::{HWND, LPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetClassNameW, GetWindowTextW, IsWindowVisible};

    unsafe extern "system" fn each(hwnd: HWND, found: LPARAM) -> BOOL {
        unsafe {
            if IsWindowVisible(hwnd).as_bool() {
                let mut title = [0u16; 256];
                let n = GetWindowTextW(hwnd, &mut title);
                let mut class = [0u16; 256];
                let c = GetClassNameW(hwnd, &mut class);
                if is_share_bar(&String::from_utf16_lossy(&title[..n as usize]), &String::from_utf16_lossy(&class[..c as usize])) {
                    *(found.0 as *mut bool) = true;
                    return false.into(); // stop looking
                }
            }
            true.into()
        }
    }

    let mut found = false;
    unsafe {
        let _ = EnumWindows(Some(each), LPARAM(&mut found as *mut bool as isize));
    }
    found
}

/// Windows' privacy record of which apps use the microphone: an app is "in use" when it has a
/// start time and no stop time yet.
#[cfg(windows)]
fn mic_in_use_registry() -> bool {
    use winreg::enums::HKEY_CURRENT_USER;
    use winreg::RegKey;

    const STORE: &str = r"Software\Microsoft\Windows\CurrentVersion\CapabilityAccessManager\ConsentStore\microphone";

    fn any_in_use(key: &RegKey) -> bool {
        key.enum_keys().flatten().any(|name| {
            if name == "NonPackaged" {
                return key.open_subkey(&name).map(|k| any_in_use(&k)).unwrap_or(false);
            }
            key.open_subkey(&name)
                .map(|app| {
                    let start: u64 = app.get_value("LastUsedTimeStart").unwrap_or(0);
                    let stop: u64 = app.get_value("LastUsedTimeStop").unwrap_or(1);
                    start > 0 && stop == 0
                })
                .unwrap_or(false)
        })
    }

    RegKey::predef(HKEY_CURRENT_USER)
        .open_subkey(STORE)
        .map(|k| any_in_use(&k))
        .unwrap_or(false)
}

#[cfg(all(test, windows))]
mod probe {
    /// Prints every capture session and its state. Run by hand: cargo test probe -- --ignored --nocapture
    #[test]
    #[ignore]
    fn probe_capture_sessions() {
        use windows::core::Interface;
        use windows::Win32::Media::Audio::{eCapture, IAudioSessionControl2, IAudioSessionManager2, IMMDeviceEnumerator, MMDeviceEnumerator, DEVICE_STATE_ACTIVE};
        use windows::Win32::System::Com::{CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_MULTITHREADED};
        unsafe {
            let _ = CoInitializeEx(None, COINIT_MULTITHREADED);
            let en: IMMDeviceEnumerator = CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL).unwrap();
            let devs = en.EnumAudioEndpoints(eCapture, DEVICE_STATE_ACTIVE).unwrap();
            for i in 0..devs.GetCount().unwrap() {
                let d = devs.Item(i).unwrap();
                let m: IAudioSessionManager2 = d.Activate(CLSCTX_ALL, None).unwrap();
                let s = m.GetSessionEnumerator().unwrap();
                for j in 0..s.GetCount().unwrap() {
                    let c = s.GetSession(j).unwrap();
                    let pid = c.cast::<IAudioSessionControl2>().ok().and_then(|c2| c2.GetProcessId().ok());
                    println!("device {i} session {j}: state={:?} pid={pid:?}", c.GetState());
                }
            }
            println!("capture_session_active={}", super::capture_session_active());
        }
    }
    /// Lists visible top-level windows with their class and process. Run by hand while sharing
    /// and while not, then compare: cargo test probe_windows -- --ignored --nocapture
    #[test]
    #[ignore]
    fn probe_windows() {
        use windows::core::BOOL;
        use windows::Win32::Foundation::{CloseHandle, HWND, LPARAM, MAX_PATH};
        use windows::Win32::System::Threading::{OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION};
        use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetClassNameW, GetWindowTextW, GetWindowThreadProcessId, IsWindowVisible};
        unsafe extern "system" fn each(hwnd: HWND, _: LPARAM) -> BOOL {
            unsafe {
                if !IsWindowVisible(hwnd).as_bool() {
                    return true.into();
                }
                let mut title = [0u16; 256];
                let n = GetWindowTextW(hwnd, &mut title);
                let mut class = [0u16; 256];
                let c = GetClassNameW(hwnd, &mut class);
                let mut pid = 0u32;
                GetWindowThreadProcessId(hwnd, Some(&mut pid));
                let mut exe = String::new();
                if let Ok(h) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
                    let mut buf = [0u16; MAX_PATH as usize];
                    let mut len = buf.len() as u32;
                    if QueryFullProcessImageNameW(h, PROCESS_NAME_WIN32, windows::core::PWSTR(buf.as_mut_ptr()), &mut len).is_ok() {
                        exe = String::from_utf16_lossy(&buf[..len as usize]).rsplit(std::path::MAIN_SEPARATOR).next().unwrap_or("").to_string();
                    }
                    let _ = CloseHandle(h);
                }
                println!("window: exe={exe} class={} title={}", String::from_utf16_lossy(&class[..c as usize]), String::from_utf16_lossy(&title[..n as usize]));
                true.into()
            }
        }
        unsafe {
            let _ = EnumWindows(Some(each), LPARAM(0));
        }
    }
}
