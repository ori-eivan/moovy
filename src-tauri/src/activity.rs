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
}

#[cfg(windows)]
pub fn read() -> Signals {
    let (locked, busy) = notification_state();
    Signals { idle_ms: idle_ms(), locked, busy, mic_in_use: mic_in_use() }
}

#[cfg(not(windows))]
pub fn read() -> Signals {
    Signals::default()
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

/// Reads Windows' privacy record of which apps use the microphone. Moovy never opens the
/// microphone itself: an app is "in use" when it has a start time and no stop time yet.
#[cfg(windows)]
fn mic_in_use() -> bool {
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
