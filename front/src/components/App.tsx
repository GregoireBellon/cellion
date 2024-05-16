import AppBar from "./AppBar/AppBar";
import { Box } from "@mui/material";
import { FC, useEffect } from "react";
import { Outlet } from "react-router-dom";
import { Settings } from "luxon";

const App: FC = () => {
  useEffect(() => {
    Settings.defaultWeekSettings = {
      firstDay: 1,
      minimalDays: 1,
      weekend: [6, 7],
    };
  }, []);

  return (
    <>
      <AppBar />
      <Box sx={{ mt: 6, paddingX: 2 }}>
        <Outlet />
      </Box>
    </>
  );
};

export default App;
