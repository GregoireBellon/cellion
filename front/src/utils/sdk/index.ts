import {
  ReadSolutionBody,
  ShortSolutionInfo,
  SolutionFiltersInfo,
} from "../../types/api";
import { ShortSessionInfo } from "../../types/core";
import { SDK } from "./sdk";
// import { SDKMock } from "./sdkMock";

export interface ISDK {
  getFilters(solutionId: string): Promise<SolutionFiltersInfo>;
  listSolutions(): Promise<ShortSolutionInfo[]>;
  getSolution(
    solutionId: string,
    body: ReadSolutionBody
  ): Promise<ShortSessionInfo[]>;
  importSolution(file: File): Promise<ShortSolutionInfo>;
}

// const sdk: ISDK = new SDKMock();
const sdk: ISDK = new SDK();

export default sdk;
