import chroma from "chroma-js";
import baseColors from "./baseColors";

const lightModeColors = baseColors.map((c) => chroma(c).luminance(0.4).css());

export default lightModeColors;
