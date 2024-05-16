import { DateView } from "@mui/x-date-pickers";
import { ViewLevel, ViewMode } from "../types/calendar";

export type FullCalendarViewName =
  | "dayGridMonth"
  | "timeGridWeek"
  | "timeGridDay"
  | "resourceTimelineMonth"
  | "resourceTimelineWeek"
  | "resourceTimelineDay";

const fullCalendarViewDict: Record<
  ViewMode,
  Record<ViewLevel, FullCalendarViewName>
> = {
  [ViewMode.DEFAULT]: {
    [ViewLevel.DAY]: "timeGridDay",
    [ViewLevel.WEEK]: "timeGridWeek",
    [ViewLevel.MONTH]: "dayGridMonth",
  },
  [ViewMode.BY_ROOM]: {
    [ViewLevel.DAY]: "resourceTimelineDay",
    [ViewLevel.WEEK]: "resourceTimelineWeek",
    [ViewLevel.MONTH]: "resourceTimelineMonth",
  },
};

export function getFullCalendarViewName(
  viewMode: ViewMode,
  viewLevel: ViewLevel
): FullCalendarViewName {
  return fullCalendarViewDict[viewMode][viewLevel];
}

const datePickerViewDict: Record<ViewLevel, DateView[]> = {
  [ViewLevel.DAY]: ["day", "month", "year"],
  [ViewLevel.WEEK]: ["day", "month", "year"],
  [ViewLevel.MONTH]: ["month", "year"],
};

export function getDatePickerViews(viewLevel: ViewLevel) {
  return datePickerViewDict[viewLevel];
}
