import { createContext } from "react";

interface IColorModeContext {
  toggleColorMode: () => void;
}

const ColorModeContext = createContext<IColorModeContext>({
  toggleColorMode: () => {},
});

export default ColorModeContext;
