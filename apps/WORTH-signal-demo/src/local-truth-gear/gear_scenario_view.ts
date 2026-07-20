import type { GearTruth } from "./gear_truth.ts";

export type GearBranchRole = "main" | "design";
export type GearConflictChoice = GearBranchRole;

export interface GearConflictView {
  id: string;
  aspectId: string;
  mainValue: unknown;
  designValue: unknown;
}

export type GearHistoryLane = GearBranchRole;

export interface GearHistoryNode {
  id: string;
  branchId: string;
  lane: GearHistoryLane;
  parentIds: readonly string[];
  kind: "genesis" | "mutation" | "merge" | "checkpoint";
  title: string;
  detail: string;
  headLabels: readonly string[];
  isLiveHead: boolean;
}

export interface GearHistorySelection {
  commitId: string;
  branchId: string;
  lane: GearHistoryLane;
  snapshotId: string;
  values: GearTruth;
  visitedCommits: number;
}

export interface GearSignalProjectionView {
  branchName: string;
  signalBranchId: number;
  basisDigest: string;
}

export interface GearScenarioView {
  phase: "ready" | "editing" | "review" | "merged";
  headline: string;
  main: GearTruth;
  design: GearTruth;
  designBranchName: string;
  activeDesignBranchId: string | null;
  conflicts: readonly GearConflictView[];
  history: readonly GearHistoryNode[];
  historySelection: GearHistorySelection | null;
  signalProjection: GearSignalProjectionView | null;
}
