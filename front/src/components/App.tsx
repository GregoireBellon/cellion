import AppBar from "./AppBar/AppBar";
import { Box } from "@mui/material";
import { FC, useEffect } from "react";
import { Outlet } from "react-router-dom";
import { Settings } from "luxon";
import { ToastContainer } from "react-toastify";

import "react-toastify/dist/ReactToastify.css";

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
      <ToastContainer
        theme="colored"
        position="top-right"
        closeOnClick
        newestOnTop
        limit={5}
      />
    </>
  );
};

export default App;
