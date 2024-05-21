import { FC, useCallback } from "react";

import { Box } from "@mui/material";
import CalendarDrawerDisplay from "./CalendarDisplay";

import { DateTime } from "luxon";
import CalendarFilters from "./CalendarFilters";
import { SolutionFiltersInfo } from "../../../types/api";
import { CalendarDisplaySettings } from "../../../types/calendar";
import CalendarSearch from "./CalendarSearch";

interface Props {
  date: DateTime | null;
  onDateChange: (newDate: DateTime | null) => void;
  filtersOptions: SolutionFiltersInfo;
  filters: SolutionFiltersInfo;
  onFiltersChange: (newFilters: SolutionFiltersInfo) => void;
  display: CalendarDisplaySettings;
  onDisplayChange: (newDisplay: CalendarDisplaySettings) => void;
}

const CalendarDrawer: FC<Props> = ({
  // date,
  // onDateChange,
  filtersOptions,
  filters,
  onFiltersChange,
  display,
  onDisplayChange,
}) => {
  const handleFiltersChange = useCallback(
    (newFilters: SolutionFiltersInfo) => {
      onFiltersChange(newFilters);
    },
    [onFiltersChange]
  );

  const handleDisplayChange = useCallback(
    (newDisplay: CalendarDisplaySettings) => {
      onDisplayChange(newDisplay);
    },
    [onDisplayChange]
  );

  return (
    <Box display="flex" flexDirection="column" flexGrow={1} minWidth={360}>
      <CalendarSearch />
      {/* <CalendarDate value={date} onChange={handleDateChange} /> */}
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
