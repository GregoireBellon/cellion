/* eslint-disable @typescript-eslint/no-unused-vars */
import { ISDK } from ".";
import {
  SolutionFiltersInfo,
  ReadSolutionBody,
  ShortSolutionInfo,
  ImportSolutionResponse,
} from "../../types/api";
import { ShortSessionInfo } from "../../types/core";

export class SDKMock implements ISDK {
  public async getFilters(_id: string): Promise<SolutionFiltersInfo> {
    return {
      courses: ["Maths", "Français", "Histoire"],
      parts: ["CM"],
      groups: ["A", "B"],
      rooms: ["L203", "L205"],
      teachers: ["Einstein", "Nash"],
    };
  }

  public async querySolution(
    _id: string,
    _body: ReadSolutionBody
  ): Promise<ShortSessionInfo[]> {
    return [
      {
        from: new Date("05/06/2024 09:00"),
        to: new Date("05/06/2024 11:00"),
        id: "1",
        course: { id: "Maths" },
        groups: [{ id: "M1" }],
        part: { id: "CM" },
        rooms: [{ id: "L001", capacity: 30, label: "Bat, G" }],
        teachers: [{ id: "Nash" }],
      },
      {
        from: new Date("05/06/2024 11:00"),
        to: new Date("05/06/2024 12:00"),
        id: "2",
        course: { id: "Maths" },
        groups: [{ id: "M2" }],
        part: { id: "CM" },
        rooms: [{ id: "L205", capacity: 30, label: "Bat, G" }],
        teachers: [{ id: "Nash" }],
      },

      {
        from: new Date("05/06/2024 14:00"),
        to: new Date("05/06/2024 16:00"),
        id: "3",
        course: { id: "Francais" },
        groups: [{ id: "L1" }],
        part: { id: "CM" },
        rooms: [{ id: "L203", capacity: 30, label: "Bat, G" }],
        teachers: [{ id: "Einstein" }],
      },
    ];
  }

  public async listSolutions(): Promise<ShortSolutionInfo[]> {
    const solutions: ShortSolutionInfo[] = [];
    for (let i = 0; i < 100; ++i) {
      solutions.push({
        id: i.toString(),
        createdAt: new Date(),
        fileName: `TEST${i}`,
        calendarStart: new Date(),
      });
    }
    return solutions;
  }

  public async importSolution(_file: File): Promise<ImportSolutionResponse> {
    return {
      id: "35",
      rowsInserted: 2,
    };
  }

  public async getSolution(_solutionId: string): Promise<ShortSolutionInfo> {
    return {
      createdAt: new Date(),
      fileName: "test",
      id: "2",
      calendarStart: new Date(),
    };
  }
}
