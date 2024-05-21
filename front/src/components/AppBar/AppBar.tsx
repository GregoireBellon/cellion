import {
  Toolbar,
  AppBar as MuiAppBar,
  Box,
  Button,
  ButtonBase,
  Typography,
} from "@mui/material";
import { FC } from "react";
import Logo from "./Logo";
import { Link } from "react-router-dom";

//ADD DARKMODE ?
const AppBar: FC = () => {
  return (
    <Box sx={{ flexGrow: 1 }}>
      <MuiAppBar
        position="sticky"
        sx={{ h: 1, zIndex: (theme) => theme.zIndex.drawer + 1 }}
        component="nav"
      >
        <Toolbar sx={{ gap: 6 }}>
          <ButtonBase LinkComponent={Link} href="/">
            <Box display="flex" flexDirection="row" gap={1}>
              <Logo />
              <Typography
                lineHeight={1.4}
                sx={{ verticalAlign: "middle" }}
                variant="h4"
              >
                Cellion
              </Typography>
            </Box>
          </ButtonBase>
          <Box display="flex" flexDirection="row" sx={{ gap: 2 }}>
            <Button
              component={Link}
              to="/calendar"
              sx={{ color: (theme) => theme.palette.secondary.main }}
            >
              Calendrier
            </Button>
            <Button
              component={Link}
              to="/files"
              sx={{ color: (theme) => theme.palette.secondary.main }}
            >
              Fichiers
            </Button>
          </Box>
        </Toolbar>
      </MuiAppBar>
    </Box>
  );
};

export default AppBar;
