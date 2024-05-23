import chroma from "chroma-js";
import baseColors from "./baseColors";

const darkModeColors = baseColors.map((c) => chroma(c).luminance(0.05).css());

export default darkModeColors;
