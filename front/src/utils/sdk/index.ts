import {
  ReadSolutionBody,
  ShortSolutionInfo,
  SolutionFiltersInfo,
  SolutionInfo,
} from "../../types/api";
import { SDKMock } from "./sdkMock";

export interface ISDK {
  getFilters(solutionId: string): Promise<SolutionFiltersInfo>;
  listSolutions(): Promise<ShortSolutionInfo[]>;
  getSolution(
    solutionId: string,
    body: ReadSolutionBody
  ): Promise<SolutionInfo>;
  importSolution(file: File): Promise<ShortSolutionInfo>;
}

const sdk: ISDK = new SDKMock();
// const sdk: ISDK = new SDK();

export default sdk;
