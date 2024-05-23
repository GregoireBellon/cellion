import AppBar from "./AppBar/AppBar";
import { Box, CssBaseline, ThemeProvider } from "@mui/material";
import { FC, useEffect, useMemo, useState } from "react";
import { Outlet } from "react-router-dom";
import { Settings } from "luxon";
import { ToastContainer } from "react-toastify";

import "react-toastify/dist/ReactToastify.css";
import { darkTheme, getDefaultColorMode, lightTheme } from "./theme";
import ColorModeContext from "./ColorModeContext";

const App: FC = () => {
  const [mode, setMode] = useState<"light" | "dark">(getDefaultColorMode);

  const colorMode = useMemo(
    () => ({
      toggleColorMode: () => {
        setMode((prevMode) => {
          const newMode = prevMode === "light" ? "dark" : "light";
          localStorage.setItem("colorMode", newMode);
          return newMode;
        });
      },
    }),
    []
  );

  const theme = useMemo(
    () => (mode === "light" ? lightTheme : darkTheme),
    [mode]
  );

  useEffect(() => {
    Settings.defaultWeekSettings = {
      firstDay: 1,
      minimalDays: 1,
      weekend: [6, 7],
    };
  }, []);

  return (
    <ColorModeContext.Provider value={colorMode}>
      <ThemeProvider theme={theme}>
        <CssBaseline enableColorScheme />
        <AppBar />
        <Box sx={{ mt: 6, paddingX: 2 }}>
          <Outlet />
        </Box>
        <ToastContainer
          theme="colored"
          position="top-right"
          closeOnClick
          newestOnTop
          limit={5}
        />
      </ThemeProvider>
    </ColorModeContext.Provider>
  );
};

export default App;
