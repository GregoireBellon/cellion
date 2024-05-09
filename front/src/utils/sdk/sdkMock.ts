/* eslint-disable @typescript-eslint/no-unused-vars */
import axios, { Axios } from "axios";
import {
  CalendarFiltersInfo,
  CalendarInfo,
  ReadCalendarBody,
} from "../../types/api";

export class SDKMock {
  public client: Axios;
  public constructor() {
    // TODO setup proxy
    this.client = axios.create({ baseURL: "http://localhost" });
  }

  public async getFilters(_id: string): Promise<CalendarFiltersInfo> {
    return {
      courses: [
        { id: "1", name: "Maths" },
        { id: "2", name: "Français" },
        { id: "3", name: "Histoire" },
      ],
      parts: [{ id: "CM", label: "CM" }],
      groups: [
        { id: "A", name: "A" },
        { id: "B", name: "B" },
      ],
      rooms: [
        { id: "1", name: "L203" },
        { id: "2", name: "L205" },
      ],
      teachers: [
        { name: "Einstein", id: "aa" },
        { name: "Nash", id: "bb" },
      ],
    };
  }

  public async getCalendar(
    _id: string,
    _body: ReadCalendarBody
  ): Promise<CalendarInfo> {
    return {
      sessions: [
        {
          from: new Date("05/06/2024 09:00"),
          to: new Date("05/06/2024 11:00"),
          id: "1",
          course: { id: "a", name: "Maths" },
          groups: [{ id: "M1", name: "M1" }],
          part: { id: "a", label: "CM" },
          room: { id: "a", name: "L203" },
          teacher: { id: "a", name: "Nash" },
        },
      ],
    };
  }
}
