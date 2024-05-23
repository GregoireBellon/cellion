import { FC, useCallback } from "react";

import { Box } from "@mui/material";
import CalendarDrawerDisplay from "./CalendarDisplay";

import CalendarFilters from "./CalendarFilters";
import { SolutionFiltersInfo } from "../../../types/api";
import { CalendarDisplaySettings } from "../../../types/calendar";
import CalendarDates from "./CalendarDates";
import { DateTime } from "luxon";

interface Props {
  from: DateTime;
  to: DateTime;
  onPrevDate: () => void;
  onNextDate: () => void;
  filtersOptions: SolutionFiltersInfo;
  filters: SolutionFiltersInfo;
  onFiltersChange: (newFilters: SolutionFiltersInfo) => void;
  display: CalendarDisplaySettings;
  onDisplayChange: (newDisplay: CalendarDisplaySettings) => void;
}

const CalendarDrawer: FC<Props> = ({
  from,
  to,
  onPrevDate,
  onNextDate,
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

  const handlePrevDate = useCallback(() => {
    onPrevDate();
  }, [onPrevDate]);

  const handleNextDate = useCallback(() => {
    onNextDate();
  }, [onNextDate]);

  return (
    <Box
      display="flex"
      flexDirection="column"
      flexGrow={1}
      minWidth={360}
      gap={2}
      sx={{ position: "sticky", top: 54 }}
    >
      <CalendarDates
        from={from}
        to={to}
        onNext={handleNextDate}
        onPrev={handlePrevDate}
      />
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
