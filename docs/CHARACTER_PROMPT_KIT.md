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

Example, filled in for Corni:

```text
Name: Corni
What it is: a small cheerful cornichon pickle
Body: chibi; big rounded pickle-top head, CLEAR NECK, smaller pickle body, short arms WITH ELBOWS, stubby legs with round feet
Face: large shiny oval eyes with two white highlights, small happy open smile, pink blush
Signature detail: a small sprig of fresh dill on top of the head
Palette (hex):
  Body       #7FB83A
  Shade      #4F8A2B
  Accent 1   #A6D45E   (bumps)
  Accent 2   #3E7A2E   (dill)
  Blush      #FF9AA2
  Outline    #2B2420
Style line: Chibi game (step 2)
Personality in one line: cheerful, a little silly, always ready to stretch
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
Style: [Style line, see below].
Canvas: square, 1024x1024, character centred and filling about 80% of the height, TRANSPARENT background.
Do not add text, logos, props or background elements. Original character design, not based on any existing character.
```

**Style line: pick one and use the same one for all 11 images.**

| Style | Used for | Style line |
|---|---|---|
| Flat sticker | Thomas | `flat 2D vector illustration, bold clean dark outline of even thickness, flat colours with one simple shade tone, no gradients, no texture, no drop shadow, a thin white sticker border around the whole character` |
| Chibi game | Corni | `cute 2D chibi game character style: big head about half the height, small body, soft cel shading with one shade tone and small glossy highlights, clean dark outline of even thickness, a thin white sticker border around the whole character` |

Never name a game, film or brand in the style line. Describe the look in words, so the result is your own character and not a copy.

Keep asking for tweaks until you love it. **This image is now the master.** Save it as `master.png`.

---

## Step 3: Style Lock (paste at the top of EVERY request after the master)

```text
STYLE LOCK. Use the attached master image as the exact reference for [Name], [What it is].
Keep identical: proportions, head size, neck, arm and leg length, face, signature detail, palette hex values, outline colour and thickness, white sticker border, and the style: [Style line].
Canvas: square 1024x1024, transparent background, full body visible, character centred, same scale as the master (about 80% of the height).
No text, no background, no floor shadow, no extra characters. One single image of one pose.
Only change what the next instruction asks for.
```

If you already have a character sheet instead of a single master (like Corni), attach the sheet and write "the attached sheet" instead of "the attached master image".

---

## Step 4: The 11 images

Paste the Style Lock, then one of these. Save each result with the exact file name. Every prompt
below is general: it works for any character, with no edits.

| # | File | What the card uses it for |
|---|---|---|
| 1 | `idle.png` | Settings window, character picker |
| 2 | `break.png` | "It's time to move!" intro |
| 3 | `celebration.png` | End of a timed movement |
| 4 | `water.png` | Water card |
| 5 | `waiting.png` | After Snooze |
| 6 | `wall-angels.png` | Movement |
| 7 | `spine-twist.png` | Movement |
| 8 | `hip-flexor.png` | Movement |
| 9 | `neck-flexion.png` | Stretch |
| 10 | `upper-trap.png` | Stretch |
| 11 | `doorway-chest.png` | Stretch |

### States

**`idle.png`**
```text
Pose: idle, FRONT view, standing relaxed, feet hip-width apart.
- One hand raised beside the head giving a small friendly wave, palm toward the viewer.
- The other arm relaxed at the side.
- Eyes open, gentle smile. Calm and welcoming.
Must NOT show: props, motion lines, both hands raised.
```

**`break.png`**
```text
Pose: "time to move!", FRONT view, standing, feet hip-width apart.
- One arm raised high, index finger pointing straight up.
- The other hand resting on the hip, elbow out.
- Big open smile, eyebrows up. Encouraging, not urgent.
- Two or three small exclamation dashes near the raised hand, in a warm accent colour.
Must NOT show: a worried or stern face, both arms up, text.
```

**`celebration.png`**
```text
Pose: celebrating, FRONT view, a small happy jump with both feet off the ground.
- Both arms up in a V, hands in small fists.
- Eyes happily closed (curved lines), big smile.
- Two or three tiny four-point sparkles near the hands, in a warm accent colour.
Must NOT show: confetti, text, a floor or shadow under the feet.
```

**`water.png`**
```text
Pose: drinking water, FRONT view, standing, feet hip-width apart.
- Both hands hold a clear glass of fresh water at chest height.
- The glass is simple and see-through, with a few light-blue highlights, same outline style.
- Refreshed happy expression, slight head tilt.
Must NOT show: a bottle, a cup with a handle, a straw, spilled water.
```

**`waiting.png`**
```text
Pose: patient waiting, FRONT view, sitting cross-legged on nothing (no floor).
- One elbow resting on a knee, that hand under the chin.
- The other hand resting in the lap.
- Relaxed half-smile, eyes looking up and to the side, as if thinking "see you soon".
Must NOT show: a sad, bored or sleepy face, a chair, a floor.
```

### Movement poses

**`wall-angels.png`**
```text
Pose: Wall Angels exercise, FRONT view, standing tall, back straight, feet hip-width apart.
- A simple light-grey vertical rectangle behind the character suggests a wall, taller and a bit wider than the body.
- Both arms raised in a W: upper arms out to the sides at shoulder height, elbows bent 90 degrees, forearms pointing straight up, palms facing forward.
- Focused, calm expression.
Must NOT show: arms straight up in a Y, hands on the head, the character turned to the side.
```

**`spine-twist.png`**
```text
Pose: Standing Spine Twist, FRONT view, feet hip-width apart, hips and feet facing forward.
- Arms crossed over the chest, each hand on the opposite shoulder.
- The upper body clearly rotated toward the character's right; the head follows the turn.
- One curved double-headed motion arrow around the waist, in a warm accent colour.
Must NOT show: hips turned with the chest, arms out to the sides, a side bend.
```

**`hip-flexor.png`**
```text
Pose: standing Hip Flexor Stretch, SIDE view, the character faces LEFT.
- Split stance: front foot forward with the front knee slightly bent, back leg extended far behind with the heel lifted and the back knee nearly straight.
- Hips tucked under, torso upright, both hands resting on the hips (or on the front thigh).
- Calm expression.
- Two small accent dashes near the front of the back hip, where the stretch is felt.
Must NOT show: the back knee on the ground, the torso leaning far forward, a front view.
```

### Stretch poses

> **These three are the ones image models get wrong.** On full character sheets they were the
> poses that came back missing (Thomas and Corni: Neck Flexion and Doorway) or wrong (a hand
> pulling the head instead of resting, on both characters' Upper Trap). So: always ask for them **one at a time**, never inside a sheet; name
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

### Shortcut: one sheet for the easy eight

A full sheet is faster, and it's fine for poses 1 to 8. Ask for it after the master, then get the
three stretches one by one with their own prompts above. Claude cuts the sheet into files.

```text
STYLE LOCK. Use the attached master image as the exact reference for [Name], [What it is].
Keep identical: proportions, face, signature detail, colours, outline and white sticker border, and the style: [Style line].

Make ONE character sheet on a transparent background with exactly 8 poses in two rows of four, same size, with clear space between them and no labels:
1. Idle: standing, one hand waving, gentle smile.
2. Break: one arm pointing straight up, other hand on the hip, big smile, small exclamation dashes.
3. Celebration: small jump, both arms up in a V, eyes happily closed, a few sparkles.
4. Water: holding a clear glass of water with both hands at chest height.
5. Waiting: sitting cross-legged, hand under the chin, looking up and to the side.
6. Wall Angels: front view, arms in a W against a light-grey wall rectangle.
7. Spine Twist: arms crossed on the chest, upper body turned, curved arrow around the waist.
8. Hip Flexor: side view, deep split stance, back heel lifted, hands on hips.
No text, no numbers, no floor shadows.
```

### When a pose comes back wrong

Reply in the same chat, attach the wrong image, and paste:

```text
Keep everything about this image, fix only this: [what is wrong, e.g. "no hand on the head: the right hand rests on the lower back and the left arm hangs at the side"]. Same style, same canvas, transparent background.
```

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
