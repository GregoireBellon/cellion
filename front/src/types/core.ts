// SOLUTION

export interface SolutionInfo {
  id: string;
  fileName: string;
  createdAt: Date;
}

// COURSE

export interface ShortCourseInfo {
  id: string;
}

export interface CourseInfo extends ShortCourseInfo {
  name: string;
}

// PART

export interface ShortPartInfo {
  id: string;
}

export interface PartInfo extends ShortPartInfo {
  label: string;
  sessionTeachers: number;
  sessionRooms: string;
  sessionLength: number;
  maxHeadCount: number;
  nbSessions: number;
}

// TEACHER

export interface ShortTeacherInfo {
  id: string; // id doesnt exists in backend, the backend must send name as id
}

export interface TeacherInfo extends ShortTeacherInfo {
  name: string;
  department: string;
}

// ROOM

export interface ShortRoomInfo {
  id: string;
}

export interface RoomInfo extends ShortRoomInfo {
  label: string;
  capacity: number;
}

// STUDENT

export interface ShortStudentInfo {
  id: string;
}

export interface StudentInfo extends ShortStudentInfo {
  label: string;
}

// GROUP

export interface ShortGroupInfo {
  id: string;
}

export interface GroupInfo extends ShortGroupInfo {
  name: string;
}

// SESSION

export interface ShortSessionInfo {
  id: string;
  from: Date;
  to: Date;
  course: ShortCourseInfo;
  part: ShortPartInfo;
  rooms: RoomInfo[];
  groups: ShortGroupInfo[];
  teachers: ShortTeacherInfo[];
}

export interface SessionInfo {
  id: string;
  from: Date;
  to: Date;
  course: CourseInfo;
  part: PartInfo;
  rooms: RoomInfo[];
  groups: GroupInfo[];
  teachers: TeacherInfo[];

  // TODO
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedSlots: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedRooms: any;
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  allowedTeacher: any;
}
