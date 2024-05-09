import {
  ShortCourseInfo,
  ShortPartInfo,
  ShortTeacherInfo,
  ShortRoomInfo,
  ShortGroupInfo,
  ShortSessionInfo,
} from "./core";

export interface CalendarFiltersInfo {
  courses: ShortCourseInfo[];
  parts: ShortPartInfo[];
  teachers: ShortTeacherInfo[];
  rooms: ShortRoomInfo[];
  groups: ShortGroupInfo[];
}

export interface ReadCalendarBody {
  courses: string[];
  parts: string[];
  teachers: string[];
  rooms: string[];
  groups: string[];
}

export interface CalendarInfo {
  sessions: ShortSessionInfo[];
}
