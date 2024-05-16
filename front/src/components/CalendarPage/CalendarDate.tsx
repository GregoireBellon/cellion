import { ExpandMore } from "@mui/icons-material";
import {
  Accordion,
  AccordionSummary,
  Typography,
  AccordionDetails,
  styled,
} from "@mui/material";
import { FC, useCallback, useMemo, useState } from "react";
import {
  LocalizationProvider,
  DateCalendar,
  PickersDay,
  PickersDayProps,
} from "@mui/x-date-pickers";
import { AdapterLuxon } from "@mui/x-date-pickers/AdapterLuxon";
import { DateTime } from "luxon";
import { ViewLevel } from "../../types/calendar";

interface CustomPickerDayProps extends PickersDayProps<DateTime> {
  isSelected: boolean;
  isHovered: boolean;
}

const CustomPickersDay = styled(PickersDay, {
  shouldForwardProp: (prop) => prop !== "isSelected" && prop !== "isHovered",
})<CustomPickerDayProps>(({ theme, isSelected, isHovered, day }) => ({
  borderRadius: 0,
  ...(isSelected && {
    backgroundColor: theme.palette.primary.main,
    color: theme.palette.primary.contrastText,
    "&:hover, &:focus": {
      backgroundColor: theme.palette.primary.main,
    },
  }),
  ...(isHovered && {
    backgroundColor: theme.palette.primary[theme.palette.mode],
    "&:hover, &:focus": {
      backgroundColor: theme.palette.primary[theme.palette.mode],
    },
  }),
  ...(day.weekday === 1 && {
    borderTopLeftRadius: "50%",
    borderBottomLeftRadius: "50%",
  }),
  ...(day.weekday === 7 && {
    borderTopRightRadius: "50%",
    borderBottomRightRadius: "50%",
  }),
}));

const CustomDay: FC<
  PickersDayProps<DateTime> & {
    selectedDay?: DateTime | null;
    hoveredDay?: DateTime | null;
    viewLevel: ViewLevel;
  }
> = ({ day, selectedDay, hoveredDay, viewLevel, ...other }) => {
  const isInSame = useCallback(
    (
      dayA: DateTime | undefined | null,
      dayB: DateTime | undefined | null,
      viewLevel: ViewLevel
    ) => {
      if (!dayA || !dayB) {
        return false;
      }
      if (viewLevel === ViewLevel.DAY) {
        return dayA.hasSame(dayB, "day");
      }
      if (viewLevel === ViewLevel.WEEK) {
        return dayA.hasSame(dayB, "week");
      }
      if (viewLevel === ViewLevel.MONTH) {
        return dayA.hasSame(dayB, "month");
      }
      return false;
    },
    []
  );

  const isSelected = useMemo(
    () => isInSame(day, selectedDay, viewLevel),
    [day, isInSame, selectedDay, viewLevel]
  );

  const isHovered = useMemo(
    () => isInSame(day, hoveredDay, viewLevel),
    [day, hoveredDay, isInSame, viewLevel]
  );

  return (
    <CustomPickersDay
      {...other}
      day={day}
      disableMargin
      selected={false}
      isSelected={isSelected}
      isHovered={isHovered}
    />
  );
};

const DayByDay: FC<
  PickersDayProps<DateTime> & {
    selectedDay?: DateTime | null;
    hoveredDay?: DateTime | null;
  }
> = (props) => <CustomDay {...props} viewLevel={ViewLevel.DAY} />;

const DayByWeek: FC<
  PickersDayProps<DateTime> & {
    selectedDay?: DateTime | null;
    hoveredDay?: DateTime | null;
  }
> = (props) => <CustomDay {...props} viewLevel={ViewLevel.WEEK} />;

const DayByMonth: FC<
  PickersDayProps<DateTime> & {
    selectedDay?: DateTime | null;
    hoveredDay?: DateTime | null;
  }
> = (props) => <CustomDay {...props} viewLevel={ViewLevel.MONTH} />;

interface Props {
  value: DateTime | null;
  onChange: (newDate: DateTime | null) => void;
  viewLevel: ViewLevel;
}

const CalendarDate: FC<Props> = ({ value, onChange, viewLevel }) => {
  const [hoveredDay, setHoveredDay] = useState<DateTime | null>(null);

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
          <DateCalendar
            value={value}
            onChange={handleChange}
            slots={{
              day: DayByMonth,
            }}
            slotProps={{
              day: (ownerState) => ({
                selectedDay: value,
                hoveredDay,
                onPointerEnter: () => setHoveredDay(ownerState.day),
                onPointerLeave: () => setHoveredDay(null),
              }),
            }}
          />
        </LocalizationProvider>
      </AccordionDetails>
    </Accordion>
  );
};

export default CalendarDate;
