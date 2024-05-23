import { useTheme } from "@mui/material";

export function useIsDarkMode() {
  const theme = useTheme();
  return theme.palette.mode === "dark";
}
