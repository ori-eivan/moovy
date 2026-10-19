# Moovy: Character Prompt Kit (for ChatGPT)

How to produce the 11 illustrations for one Moovy character so they look like the same character every time.

**Who uses it:** Ori, in ChatGPT, with image generation.
**What comes out:** 11 transparent PNG files, named exactly as listed in step 4.

---

## How it works (read once)

1. **One chat per character.** Do every image for a character in the same ChatGPT conversation. Never start a new chat in the middle.
2. **Attach the master image every time.** Each request includes the approved master image (step 2) as the reference.
3. **Paste the Style Lock first** (step 3), then the image prompt (step 4).
4. **One image per request.** Never ask for several poses in one image.
5. **Check before you keep it.** Run the checklist (step 5). If it fails, say which point failed and ask again. Don't start over.
6. **Send the files to Claude** when all 11 pass. Claude makes a contact sheet and flags any that drift.

Fill in the `[brackets]` once, in step 1, and reuse the same words everywhere.

---

## Step 1: Character card (fill once)

```text
Name: [e.g. Pip]
What it is: [e.g. a small round radish]
Body: round main body with a CLEAR NECK, two arms WITH ELBOWS and hands, two legs with feet
Face: [e.g. big oval eyes, small smile, rosy cheeks]
Signature detail: [e.g. two leaves on top of the head]
Palette (hex):
  Body       [#______]
  Shade      [#______]
  Accent 1   [#______]
  Accent 2   [#______]
  Blush      [#______]
  Outline    #2B2420
Personality in one line: [e.g. cheerful, a bit clumsy, always encouraging]
```

**Why the neck, elbows and legs are required:** four of the six movements are neck, shoulder and upper-back moves. Without a neck and elbows, Neck Flexion and Wall Angels can't be read in the picture.

---

## Step 2: Master image prompt (first request in the chat)

Attach your pilot character image if you have one, then paste:

```text
Create the master reference image for an original mascot character for a desktop app called Moovy.

Character: [Name], [What it is].
Body: round main body with a clear visible neck, two arms with visible elbows and hands, two short legs with feet.
Face: [Face].
Signature detail: [Signature detail].
Palette: body [hex], shade [hex], accent [hex], accent [hex], blush [hex], outline #2B2420.

Pose: standing, facing the viewer, arms relaxed at the sides, neutral friendly smile, full body visible.
Style: flat 2D vector illustration, bold clean dark outline of even thickness, flat colours with one simple shade tone, no gradients, no texture, no drop shadow, a thin white sticker border around the whole character.
Canvas: square, 1024x1024, character centred and filling about 80% of the height, TRANSPARENT background.
Do not add text, logos, props or background elements.
```

Keep asking for tweaks until you love it. **This image is now the master.** Save it as `master.png`.

---

## Step 3: Style Lock (paste at the top of EVERY request after the master)

```text
STYLE LOCK. Use the attached master image as the exact reference for [Name].
Keep identical: proportions, head size, neck, arm and leg length, face, signature detail, palette hex values, outline colour and thickness, white sticker border, flat 2D vector style.
Canvas: square 1024x1024, transparent background, full body visible, character centred, same scale as the master (about 80% of the height).
No text, no background, no floor shadow, no extra characters.
Only change what the next instruction asks for.
```

---

## Step 4: The 11 images

Paste the Style Lock, then one of these. Save each result with the exact file name.

### States

**`idle.png`**
```text
Pose: standing relaxed, one hand giving a small friendly wave, gentle smile, eyes open. Calm, welcoming.
```

**`break.png`**
```text
Pose: energetic "time to move" pose. One arm raised high pointing up, the other on the hip, big open smile, eyebrows up. Encouraging, not urgent.
```

**`celebration.png`**
```text
Pose: celebrating. Small jump with both feet off the ground, both arms up in a V, eyes happily closed (curved lines), big smile. Two or three tiny sparkle shapes near the hands in the accent colour.
```

**`water.png`**
```text
Pose: holding a clear glass of fresh water with both hands at chest height, a few light-blue water highlights in the glass, refreshed happy expression, slight head tilt. The glass is simple, flat style, same outline.
```

**`waiting.png`**
```text
Pose: patient waiting. Sitting cross-legged on nothing (no floor), one hand under the chin, relaxed half-smile, eyes looking up and to the side as if thinking "see you soon". Calm, not sad, not bored-looking.
```

### Movement poses

**`wall-angels.png`**
```text
Pose: Wall Angels exercise, front view. Standing tall with back straight. Both arms raised in a W shape: upper arms out to the sides at shoulder height, elbows bent 90 degrees, forearms pointing straight up, palms facing forward. A simple vertical light-grey rectangle behind the character suggests a wall. Focused, calm expression.
```

**`spine-twist.png`**
```text
Pose: Standing Spine Twist, front view. Feet hip-width apart, hips facing forward, arms crossed over the chest, upper body clearly rotated to the character's right, head following the turn. Small curved motion arrow beside the torso in the accent colour.
```

**`hip-flexor.png`**
```text
Pose: standing Hip Flexor Stretch, side view facing left. Split stance: front foot forward with a slightly bent front knee, back leg extended behind with heel lifted, back knee nearly straight, hips tucked, torso upright, hands resting on the hips. Calm expression.
```

### Stretch poses

> **These three are the ones image models get wrong.** On full character sheets they were the
> poses that came back missing (Thomas: Neck Flexion and Doorway) or wrong (a hand pulling the
> head instead of resting). So: always ask for them **one at a time**, never inside a sheet; name
> every limb's position; and say what must **not** be in the picture. Check them against the
> checklist below before keeping them.

**`neck-flexion.png`**
```text
Pose: Neck Flexion stretch, FRONT view, standing upright, feet hip-width apart.
- BOTH hands (paws) rest lightly on the BACK of the head, fingers interlaced or touching.
- Both elbows point FORWARD and slightly down, close together in front of the face.
- The head is bent straight FORWARD: chin down toward the chest, so the top of the head faces the viewer and the back of the neck is long.
- Eyes closed, calm relaxed face.
Must NOT show: the head tilted to the side, elbows pointing out to the sides, hands pulling, a hand on the side of the head.
```

**`upper-trap.png`**
```text
Pose: Upper Trapezius stretch, FRONT view, standing upright, feet hip-width apart.
- RIGHT hand (paw) rests flat on the lower back, behind the body, mostly hidden.
- LEFT arm hangs relaxed at the side. The left hand does NOT touch the head.
- The head tilts gently toward the LEFT shoulder, ear toward the shoulder, so the right side of the neck is long and visible.
- Eyes closed, calm face.
Must NOT show: any hand on the head, the head bent forward, arms raised.
```

**`doorway-chest.png`**
```text
Pose: Doorway Chest stretch, SIDE view, the character faces RIGHT.
- A simple light-grey vertical door frame (a plain upright rectangle, same outline style) stands on the LEFT side of the image, taller than the character.
- The character's arm nearest the frame is raised to shoulder height, elbow bent 90 degrees, the whole forearm and paw resting FLAT against the frame.
- The body steps gently forward, away from the frame: front foot a small step ahead, chest open and pushed slightly forward, shoulders back.
- The other arm relaxed at the side or on the hip. Calm, open expression.
Must NOT show: both arms on the frame, the character facing the viewer, the character leaning on the frame with its back.
```

**Before you finish:** count your files against the list at the top of step 4. Sheets with
many poses tend to silently drop the hard ones, so a "complete" sheet is not proof that all 11
are there.

---

## Step 5: Checklist (every image)

Hold the new image next to `master.png`:

- [ ] Same head size and body proportions
- [ ] Same face shapes (eyes, mouth, cheeks)
- [ ] Signature detail present and identical
- [ ] Colours match the hex palette, no new colours except the allowed props (water, wall, door frame, sparkles)
- [ ] Same outline colour and thickness, white sticker border present
- [ ] Transparent background (no white or grey box)
- [ ] Full body visible, nothing cropped
- [ ] The movement is obvious at small size: shrink it to about 120 px wide, and you should still tell what the pose is
- [ ] No text anywhere in the image

**If one point fails,** reply in the same chat: "Keep everything, fix only: [the failed point]."

**If the background isn't transparent,** don't fight it. Keep the image and send it to Claude, who removes the background with a script.

---

## Step 6: Hand-off

Put all 11 files plus `master.png` in one folder and tell Claude where it is. Claude will:

1. Build a contact sheet of all 12 side by side at full size and at 120 px.
2. Flag any drift by file name.
3. Remove any leftover background and trim the images to the same canvas.
4. Fill in `character.json`.
