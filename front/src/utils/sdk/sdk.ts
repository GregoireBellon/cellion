import axios, { Axios } from "axios";
import {
  SolutionFiltersInfo,
  SolutionInfo,
  ReadSolutionBody,
  ShortSolutionInfo,
} from "../../types/api";
import { ISDK } from ".";

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
    const { data } = await this.client.get<ShortSolutionInfo[]>(`/solutions`);
    return data;
  }

  public async getSolution(
    id: string,
    body: ReadSolutionBody
  ): Promise<SolutionInfo> {
    const { data } = await this.client.post<SolutionInfo>(
      `/solutions/${id}/query`,
      body
    );
    return data;
  }

  public async importSolution(file: File): Promise<ShortSolutionInfo> {
    const { data } = await this.client.postForm<ShortSolutionInfo>(
      "/solutions",
      { file }
    );
    return data;
  }
}
