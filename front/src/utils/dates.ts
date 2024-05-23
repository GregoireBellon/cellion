import { DateTime } from "luxon";
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

export function timestampStrToDateTime(timestamp: string | null) {
  if (timestamp === null) {
    return DateTime.invalid("empty");
  }
  return DateTime.fromMillis(Number.parseInt(timestamp));
}
