import { PaletteOptions, createTheme } from "@mui/material";

const lightPalette: PaletteOptions = {
  primary: {
    main: "#00696f",
    contrastText: "#FFFFFF",
  },
  secondary: {
    main: "#4a6365",
    contrastText: "#FFFFFF",
  },
  error: {
    main: "#ba1a1a",
    contrastText: "#FFFFFF",
  },
  background: {
    default: "#F4FAFB",
    paper: "#E9EFEF",
  },
  text: {
    primary: "#171D1E",
    secondary: "#3F484B",
  },
  divider: "#6F797B",
};

const darkPalette: PaletteOptions = {
  primary: {
    main: "#80d4db",
    contrastText: "#00363a",
  },
  secondary: {
    main: "#b1cbce",
    contrastText: "#1b3436",
  },
  error: {
    main: "#ffb4ab",
    contrastText: "#690005",
  },
  background: {
    default: "#0e1415",
    paper: "#1a2121",
  },
  text: {
    primary: "#dde4e4",
    secondary: "#bec8c9",
  },
  divider: "#899393",
};

const lightTheme = createTheme({
  palette: {
    mode: "light",
    ...lightPalette,
  },

  typography: {
    fontFamily: ["Roboto", "Helvetica", "Arial", "sans-serif"].join(","),
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 700,
  },
  breakpoints: {
    values: {
      xs: 0,
      sm: 600,
      md: 960,
      lg: 1280,
      xl: 1920,
    },
  },
  spacing: 8,
});

const darkTheme = createTheme({
  typography: {
    fontFamily: ["Roboto", "Helvetica", "Arial", "sans-serif"].join(","),
    fontWeightLight: 300,
    fontWeightRegular: 400,
    fontWeightMedium: 500,
    fontWeightBold: 700,
  },
  breakpoints: {
    values: {
      xs: 0,
      sm: 600,
      md: 960,
      lg: 1280,
      xl: 1920,
    },
  },
  spacing: 8,
  palette: { mode: "dark", ...darkPalette },
});

function getDefaultColorMode(): "light" | "dark" {
  const localStorageValue = localStorage.getItem("colorMode");
  if (localStorageValue === "light") {
    return "light";
  }
  if (localStorageValue === "dark") {
    return "dark";
  }
  if (
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme:dark)").matches
  ) {
    return "dark";
  } else {
    return "light";
  }
}

export { lightTheme, darkTheme, getDefaultColorMode };
