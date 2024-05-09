import { FC } from "react";

import { Box, Button, styled } from "@mui/material";
import { CalendarFiltersInfo } from "../../types/api";
import CalendarFilters from "./CalendarFilters";
import CalendarDisplay from "./CalendarDisplay";
import { Search } from "@mui/icons-material";

const KBM = styled("span", {
  shouldForwardProp: (propName) => propName !== "size",
})<{ size?: "small" | "medium" }>(({ theme, size }) => ({
  background: `linear-gradient(180deg, ${theme.palette.grey[100]} 0%, ${theme.palette.grey[50]} 75%)`,
  borderRadius: theme.shape.borderRadius,
  boxShadow: `inset 0 1px 2px 1px white, 0 1px 0 0 ${theme.palette.grey[600]}`,
  color: theme.palette.text.secondary,
  display: "inline-flex",
  alignItems: "center",
  justifyContent: "center",
  padding: size === "small" ? "0 2px" : "0 0.25em",
  height: size === "small" ? "20px" : "1.5em",
  minWidth: size === "small" ? "10px" : "1.5em",
  fontSize: size === "small" ? "13px !important" : "1em",
}));
interface Props {
  calendarFilters: CalendarFiltersInfo;
}

const CalendarDrawer: FC<Props> = ({ calendarFilters }) => {
  return (
    <Box>
      <Button
        variant="outlined"
        startIcon={<Search />}
        endIcon={<KBM size="small">Ctrl+k</KBM>}
        color="secondary"
        sx={{ textTransform: "none", borderRadius: "10px", mb: 2 }}
        fullWidth
      >
        Chercher un fichier...
      </Button>
      <CalendarDisplay sx={{ p: 1 }} />
      <CalendarFilters
        calendarFilters={calendarFilters}
        sx={{ p: 1, width: 250 }}
      />
    </Box>
  );
};

export default CalendarDrawer;
