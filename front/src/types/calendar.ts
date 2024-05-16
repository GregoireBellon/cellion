export enum ViewMode {
  DEFAULT = "DEFAULT",
  BY_ROOM = "BY_ROOM",
}

export enum ViewLevel {
  MONTH = "MONTH",
  WEEK = "WEEK",
  DAY = "DAY",
}

export enum ColorMode {
  BY_PART = "BY_PART",
  BY_COURSE = "BY_COURSE",
  BY_TEACHER = "BY_TEACHER",
  BY_ROOM = "BY_ROOM",
}

export enum CalendarSearchParams {
  FROM = "from",
  TO = "to",
  VIEW_MODE = "viewMode",
  COLOR_MODE = "colorMode",
  VIEW_LEVEL = "viewLevel",
  COURSE = "course",
  PART = "part",
  ROOM = "room",
  GROUP = "group",
  TEACHER = "teacher",
}

export type CalendarFilterSearchParams =
  | CalendarSearchParams.COURSE
  | CalendarSearchParams.PART
  | CalendarSearchParams.ROOM
  | CalendarSearchParams.TEACHER
  | CalendarSearchParams.GROUP;

export interface CalendarDisplay {
  viewMode: ViewMode;
  colorMode: ColorMode;
  viewLevel: ViewLevel;
}
