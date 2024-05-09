// SOLUTION

export interface SolutionInfo {
  id: string;
  fileName: string;
  createdAt: Date;
}

// COURSE

export interface ShortCourseInfo {
  id: string;
  name: string;
}

export interface CourseInfo extends ShortCourseInfo {}

// PART

export interface ShortPartInfo {
  id: string;
  label: string;
}

export interface PartInfo extends ShortPartInfo {
  sessionTeachers: number;
  sessionRooms: string;
  sessionLength: number;
  maxHeadCount: number;
  nbSessions: number;
}

// TEACHER

export interface ShortTeacherInfo {
  id: string; // id doesnt exists in backend, the backend must send name as id
  name: string;
}

export interface TeacherInfo extends ShortTeacherInfo {
  department: string;
}

// ROOM

export interface ShortRoomInfo {
  id: string;
  name: string;
}

export interface RoomInfo extends ShortRoomInfo {
  capacity: number;
}

// STUDENT

export interface ShortStudentInfo {
  id: string;
  label: string;
}

export interface StudentInfo extends ShortStudentInfo {}

// GROUP

export interface ShortGroupInfo {
  id: string;
  name: string;
}

export interface GroupInfo extends ShortGroupInfo {}

// SESSION

export interface ShortSessionInfo {
  id: string;
  from: Date;
  to: Date;
  course: ShortCourseInfo;
  part: ShortPartInfo;
  room: ShortRoomInfo;
  groups: ShortGroupInfo[];
  teacher: ShortTeacherInfo;
}

export interface SessionInfo {
  id: string;
  from: Date;
  to: Date;
  course: CourseInfo;
  part: PartInfo;
  room: RoomInfo;
  groups: GroupInfo[];
  teacher: TeacherInfo;

  // TODO
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedSlots: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedRooms: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedTeacher: any;
}
