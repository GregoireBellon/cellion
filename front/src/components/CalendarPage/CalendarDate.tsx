import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionSummary,
  Typography,
  AccordionDetails,
} from "@mui/material";
import { FC, useCallback } from "react";
import { LocalizationProvider, DateCalendar } from "@mui/x-date-pickers";
import { AdapterLuxon } from "@mui/x-date-pickers/AdapterLuxon";
import { DateTime } from "luxon";

interface Props {
  value: DateTime | null;
  onChange: (newDate: DateTime | null) => void;
}

const CalendarDate: FC<Props> = ({ value, onChange }) => {
  const handleChange = useCallback(
    (newDate: DateTime | null) => {
      onChange(newDate);
    },
    [onChange]
  );

  return (
    <Accordion sx={{ p: 1 }} defaultExpanded>
      <AccordionSummary expandIcon={<ExpandMore />}>
        <Typography variant="h4">Date</Typography>
      </AccordionSummary>
      <AccordionDetails>
        <LocalizationProvider dateAdapter={AdapterLuxon} adapterLocale="fr">
          <DateCalendar value={value} onChange={handleChange} />
        </LocalizationProvider>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarDate;
