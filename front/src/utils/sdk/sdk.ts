import axios, { Axios } from "axios";
import {
  SolutionFiltersInfo,
  ReadSolutionBody,
  ShortSolutionInfo,
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
      { id: number; filename: string; created_at: string }[]
    >(`/solutions`);
    return data.map((d) => ({
      id: d.id.toString(),
      createdAt: new Date(d.created_at),
      fileName: d.filename,
    }));
  }

  public async getSolution(
    id: string,
    body: ReadSolutionBody
  ): Promise<ShortSessionInfo[]> {
    const { data } = await this.client.post<ShortSessionInfo[]>(
      `/solutions/${id}/query`,
      body
    );
    return data;
  }

  public async importSolution(file: File): Promise<ShortSolutionInfo> {
    const { data } = await this.client.postForm<ShortSolutionInfo>(
      "/solutions",
      { solution: file }
    );
    return data;
  }
}
