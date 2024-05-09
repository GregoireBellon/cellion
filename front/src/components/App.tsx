import AppBar from "./AppBar/AppBar";
import { Box } from "@mui/material";
import { FC } from "react";
import { Outlet } from "react-router-dom";

const App: FC = () => {
  return (
    <>
      <AppBar />
      <Box sx={{ mt: 6, paddingX: 6 }}>
        <Outlet />
      </Box>
    </>
  );
};

export default App;
