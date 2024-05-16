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
  createdAt: Date;
}

export interface SolutionInfo extends ShortSolutionInfo {
  sessions: ShortSessionInfo[];
}
