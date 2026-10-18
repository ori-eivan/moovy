// The six movements from PRD section 8. Card copy says what to do and how it feels,
// never what it fixes.

export type Category = "movement" | "stretch";

export interface Movement {
  id: string;
  category: Category;
  name: string;
  cardLine: string;
  dose: string;
  howTo: string;
  alternative: string;
  seconds: number;
  /** Shows "Switch sides" at the halfway point of the timer. */
  twoSided: boolean;
  image: string;
}

export const MOVEMENTS: Record<string, Movement> = {
  "wall-angels": {
    id: "wall-angels",
    category: "movement",
    name: "Wall Angels",
    cardLine: "Back to the wall, slide your arms up and down.",
    dose: "10 slow reps",
    howTo: "Back, head and arms on the wall, elbows in a W. Slide your arms up, then down.",
    alternative: "No wall? Do it standing, arms in a W.",
    seconds: 60,
    twoSided: false,
    image: "wall-angels.png",
  },
  "spine-twist": {
    id: "spine-twist",
    category: "movement",
    name: "Standing Spine Twist",
    cardLine: "Stand tall and turn gently side to side.",
    dose: "10 per side",
    howTo: "Feet hip-width, arms crossed. Turn your upper body side to side, hips facing forward.",
    alternative: "Seated version works too.",
    seconds: 60,
    twoSided: true,
    image: "spine-twist.png",
  },
  "hip-flexor": {
    id: "hip-flexor",
    category: "movement",
    name: "Hip Flexor Stretch",
    cardLine: "Step back and feel the front of your hip open.",
    dose: "30 sec per side",
    howTo: "Split stance, one foot back. Tuck your hips under and lean forward gently.",
    alternative: "Hold a chair or desk for balance.",
    seconds: 60,
    twoSided: true,
    image: "hip-flexor.png",
  },
  "neck-flexion": {
    id: "neck-flexion",
    category: "stretch",
    name: "Neck Flexion Stretch",
    cardLine: "Hands rest behind your head, chin to chest.",
    dose: "30 sec",
    howTo: "Hands rest lightly behind your head. Let your chin drop to your chest. Don't pull.",
    alternative: "Without hands is fine.",
    seconds: 30,
    twoSided: false,
    image: "neck-flexion.png",
  },
  "upper-trap": {
    id: "upper-trap",
    category: "stretch",
    name: "Upper Trap Stretch",
    cardLine: "One hand behind your back, tilt your head away.",
    dose: "30 sec per side",
    howTo: "One hand behind your back. Tilt your head gently to the other shoulder. Don't pull.",
    alternative: "Or hold the seat of your chair.",
    seconds: 60,
    twoSided: true,
    image: "upper-trap.png",
  },
  "doorway-chest": {
    id: "doorway-chest",
    category: "stretch",
    name: "Doorway Stretch",
    cardLine: "Forearm on a door frame, lean forward a little.",
    dose: "30 sec per side",
    howTo: "Forearm on a door frame, elbow at shoulder height. Step through gently.",
    alternative: "No doorway? Clasp your hands behind your back.",
    seconds: 60,
    twoSided: true,
    image: "doorway-chest.png",
  },
};
