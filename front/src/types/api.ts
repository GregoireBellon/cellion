import { ShortSessionInfo } from "./core";

export interface SolutionFiltersInfo {
  courses: string[];
  parts: string[];
  teachers: string[];
  rooms: string[];
  groups: string[];
}

export interface ReadSolutionBody {
  from: string;
  to: string;
  courses: string[];
  parts: string[];
  teachers: string[];
  rooms: string[];
  groups: string[];
}

export interface ShortSolutionInfo {
  id: string;
  fileName: string;
  calendarStart: Date;
  createdAt: Date;
}

export interface SolutionInfo extends ShortSolutionInfo {
  sessions: ShortSessionInfo[];
}

export interface ImportSolutionResponse {
  id: string;
  rowsInserted: number;
}
