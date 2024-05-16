import { FC, useCallback, useEffect, useState } from "react";

import { Box, Button, styled } from "@mui/material";
import CalendarDrawerDisplay from "./CalendarDisplay";
import { Search } from "@mui/icons-material";
import CalendarSearchDialog from "./CalendarSearchDialog";
import CalendarDate from "./CalendarDate";
import { DateTime } from "luxon";
import CalendarFilters from "./CalendarFilters";
import { SolutionFiltersInfo } from "../../types/api";
import { CalendarDisplay } from "../../types/calendar";

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
  date: DateTime | null;
  onDateChange: (newDate: DateTime | null) => void;
  filtersOptions: SolutionFiltersInfo;
  filters: SolutionFiltersInfo;
  onFiltersChange: (newFilters: SolutionFiltersInfo) => void;
  display: CalendarDisplay;
  onDisplayChange: (newDisplay: CalendarDisplay) => void;
}

const CalendarDrawer: FC<Props> = ({
  date,
  onDateChange,
  filtersOptions,
  filters,
  onFiltersChange,
  display,
  onDisplayChange,
}) => {
  const [searchDialogOpen, setSeachDialogOpen] = useState<boolean>(false);

  const handleDateChange = useCallback(
    (newDate: DateTime | null) => {
      onDateChange(newDate);
    },
    [onDateChange]
  );

  const handleFiltersChange = useCallback(
    (newFilters: SolutionFiltersInfo) => {
      onFiltersChange(newFilters);
    },
    [onFiltersChange]
  );

  const handleDisplayChange = useCallback(
    (newDisplay: CalendarDisplay) => {
      onDisplayChange(newDisplay);
    },
    [onDisplayChange]
  );

  const handleSearchButtonClick = useCallback(() => {
    setSeachDialogOpen(true);
  }, []);

  const handleSearchDialogClose = useCallback(() => {
    setSeachDialogOpen(false);
  }, []);

  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (event.ctrlKey && event.key === "k") {
        event.preventDefault();
        setSeachDialogOpen(true);
      }
    };
    document.addEventListener("keydown", handleKeyDown);
    return () => {
      document.removeEventListener("keydown", handleKeyDown);
    };
  }, []);

  return (
    <Box>
      <CalendarSearchDialog
        open={searchDialogOpen}
        onClose={handleSearchDialogClose}
      />
      <Button
        startIcon={<Search />}
        endIcon={<KBM size="small">Ctrl+k</KBM>}
        sx={{ textTransform: "none", borderRadius: "10px", mb: 2 }}
        fullWidth
        onClick={handleSearchButtonClick}
        disableFocusRipple
      >
        Rechercher une instance...
      </Button>
      <CalendarDate value={date} onChange={handleDateChange} />
      <CalendarDrawerDisplay value={display} onChange={handleDisplayChange} />
      <CalendarFilters
        options={filtersOptions}
        value={filters}
        onChange={handleFiltersChange}
      />
    </Box>
  );
};

export default CalendarDrawer;
