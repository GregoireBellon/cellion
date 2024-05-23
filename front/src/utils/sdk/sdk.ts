import axios, { Axios } from "axios";
import {
  SolutionFiltersInfo,
  ReadSolutionBody,
  ShortSolutionInfo,
  ImportSolutionResponse,
} from "../../types/api";
import { ISDK } from ".";
import { ShortSessionInfo } from "../../types/core";

export class SDK implements ISDK {
  private client: Axios;

  public constructor() {
    this.client = axios.create({ baseURL: "/api" });
  }

  public async getFilters(id: string): Promise<SolutionFiltersInfo> {
    const { data } = await this.client.get<SolutionFiltersInfo>(
      `/solutions/${id}/filters`
    );
    return data;
  }

  public async listSolutions(): Promise<ShortSolutionInfo[]> {
    const { data } = await this.client.get<
      {
        id: number;
        filename: string;
        created_at: string;
        calendar_start: string;
      }[]
    >(`/solutions`);
    return data.map((d) => ({
      id: d.id.toString(),
      createdAt: new Date(d.created_at),
      fileName: d.filename,
      calendarStart: new Date(d.calendar_start),
    }));
  }

  public async querySolution(
    id: string,
    body: ReadSolutionBody
  ): Promise<ShortSessionInfo[]> {
    const { data } = await this.client.post<ShortSessionInfo[]>(
      `/solutions/${id}/query`,
      body
    );
    return data.map((d) => ({
      ...d,
      from: new Date(d.from),
      to: new Date(d.to),
    }));
  }

  public async importSolution(file: File): Promise<ImportSolutionResponse> {
    const { data } = await this.client.postForm<{
      id: number;
      row_inserted: number;
    }>("/solutions", { solution: file });
    return { id: data.id.toString(), rowsInserted: data.row_inserted };
  }

  public async getSolution(solutionId: string): Promise<ShortSolutionInfo> {
    const { data } = await this.client.get<{
      id: number;
      filename: string;
      created_at: string;
      calendar_start: string;
    }>(`/solutions/${solutionId}`);
    return {
      id: data.id.toString(),
      createdAt: new Date(data.created_at),
      fileName: data.filename,
      calendarStart: new Date(data.calendar_start),
    };
  }
}
