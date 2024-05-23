import {
  Toolbar,
  AppBar as MuiAppBar,
  Box,
  ButtonBase,
  Typography,
  IconButton,
  Tooltip,
} from "@mui/material";
import { FC, useCallback, useContext, useEffect, useState } from "react";
import Logo from "./Logo";
import { Link } from "react-router-dom";
import ColorModeContext from "../ColorModeContext";
import { DarkModeOutlined, LightModeOutlined } from "@mui/icons-material";
import { useIsDarkMode } from "../../utils/colors/useIsDarkMode";
import CalendarSearchButton from "../CalendarPage/CalendarSearch/CalendarSearchButton";
import CalendarSearchDialog from "../CalendarPage/CalendarSearch/CalendarSearchDialog";

const AppBar: FC = () => {
  const isDarkMode = useIsDarkMode();
  const colorMode = useContext(ColorModeContext);

  const [searchDialogOpen, setSearchDialogOpen] = useState<boolean>(false);

  const handleSearchDialogClose = useCallback(() => {
    setSearchDialogOpen(false);
  }, []);

  const handleSearchButtonClick = useCallback(() => {
    setSearchDialogOpen(true);
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.key === "k") {
        event.stopPropagation();
        event.preventDefault();
        setSearchDialogOpen(true);
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return (
    <>
      <MuiAppBar color="primary" component="nav">
        <Toolbar sx={{ height: 42, minHeight: "0px !important" }}>
          <Box
            display="flex"
            flexDirection="row"
            justifyContent="space-between"
            flexGrow={1}
          >
            <ButtonBase LinkComponent={Link} href="/">
              <Box display="flex" flexDirection="row" gap={1}>
                <Logo />
                <Typography
                  lineHeight={1.4}
                  sx={{ verticalAlign: "middle" }}
                  variant="h6"
                >
                  Cellion
                </Typography>
              </Box>
            </ButtonBase>
            <Box display="flex" flexDirection="row" alignItems="center" gap={1}>
              <CalendarSearchButton
                fullWidth
                onClick={handleSearchButtonClick}
                size="small"
                variant="outlined"
                color={isDarkMode ? "primary" : "inherit"}
                sx={{ fontWeight: 400, fontSize: 16, height: 30 }}
              />
              <Tooltip
                title={`Passer en mode ${isDarkMode ? "clair" : "sombre"}`}
              >
                <IconButton
                  onClick={colorMode.toggleColorMode}
                  color={isDarkMode ? "primary" : "inherit"}
                >
                  {isDarkMode ? <LightModeOutlined /> : <DarkModeOutlined />}
                </IconButton>
              </Tooltip>
            </Box>
          </Box>
        </Toolbar>
      </MuiAppBar>
      <CalendarSearchDialog
        open={searchDialogOpen}
        onClose={handleSearchDialogClose}
      />
    </>
  );
};

export default AppBar;
