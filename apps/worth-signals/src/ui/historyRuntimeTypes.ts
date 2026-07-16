import {
  initialGearParams,
  normalizeGearParams,
  type GearParamKey,
  type GearParams,
  type RuntimeTimelineBookmark,
} from "./demos/demoSixTypes";

export interface RuntimeBranch {
  id: number;
  name: string;
  parent_branch_id: number | null;
  head_snapshot_id: number | null;
}

export interface RawRuntimeBranch {
  id: number | bigint;
  name: string;
  parent_branch_id: number | bigint | null;
  head_snapshot_id: number | bigint | null;
}

export interface SignalInputHandle {
  value(): number;
  set(value: number): unknown;
}

export interface SignalsTransaction {
  set(handle: SignalInputHandle, value: number): void;
}

export interface MergePlanView {
  merge_kind: string;
  node_plan: readonly unknown[];
  resolution_plan?: unknown | null;
}

export interface MergeResultProofView {
  result?: { merge_kind: string };
  merge_kind?: string;
  proof?: { resultDigest?: string };
}

export interface SignalHistoryHandle {
  current_branch(): RawRuntimeBranch;
  branches(): readonly RawRuntimeBranch[];
  create_branch(name: string): RawRuntimeBranch | Promise<RawRuntimeBranch>;
  switch_branch(branchId: number): void | Promise<void>;
  branch_snapshot(branchId: number): unknown;
  branch_snapshot_id(branchId: number): number | bigint;
  restore_exact_branch_snapshot(branchId: number, snapshot: unknown): void | Promise<void>;
  plan_merge_branches(sourceBranchId: number, targetBranchId: number): MergePlanView | Promise<MergePlanView>;
  plan_merge_policy_preview?(request: {
    source_branch_id: number;
    target_branch_id: number;
    conflict_policy_name?: string | null;
  }): MergePlanView | Promise<MergePlanView>;
  merge_branches_with_proof(sourceBranchId: number, targetBranchId: number): MergeResultProofView | Promise<MergeResultProofView>;
  branch_state_proof(branchId: number): { snapshotId: number | null; stateDigest: string };
}

export interface SignalsFacade {
  input(initial: number, options?: { debugName?: string }): SignalInputHandle;
  batch(callback: (tx: SignalsTransaction) => void): unknown;
  form(declaration: unknown): unknown;
  history(): SignalHistoryHandle;
}

export interface GearRuntime {
  signals: SignalsFacade;
  history: SignalHistoryHandle;
  inputs: Record<GearParamKey, SignalInputHandle>;
}

export function numberId(value: number | bigint | null | undefined): number | null {
  return value == null ? null : Number(value);
}

export function readBranchFromRaw(branch: RawRuntimeBranch): RuntimeBranch {
  return {
    id: Number(branch.id),
    name: branch.name,
    parent_branch_id: numberId(branch.parent_branch_id),
    head_snapshot_id: numberId(branch.head_snapshot_id),
  };
}

export function readBranch(history: SignalHistoryHandle): RuntimeBranch {
  return readBranchFromRaw(history.current_branch());
}

export function readBranches(history: SignalHistoryHandle): RuntimeBranch[] {
  return history.branches().map(readBranchFromRaw);
}

export function readParams(inputs: GearRuntime["inputs"]): GearParams {
  return normalizeGearParams({
    innerRadius: inputs.innerRadius.value(),
    outerRadius: inputs.outerRadius.value(),
    thickness: inputs.thickness.value(),
    teeth: inputs.teeth.value(),
  });
}

export function formatNumber(value: number) {
  return Number.isInteger(value) ? String(value) : value.toFixed(2);
}

export function createBookmark(
  history: SignalHistoryHandle,
  label: string,
  params: GearParams = initialGearParams,
  parentIds: readonly string[] = [],
): RuntimeTimelineBookmark {
  const branch = readBranch(history);
  const snapshotId = numberId(history.branch_snapshot_id(branch.id));
  return {
    id: `${branch.id}:${snapshotId ?? "none"}:${label}`,
    parentIds,
    branchId: branch.id,
    branchName: branch.name,
    snapshotId,
    snapshot: history.branch_snapshot(branch.id),
    label,
    params,
  };
}
