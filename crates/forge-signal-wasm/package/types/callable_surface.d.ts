import type {
  AspectId,
  VersionSummary,
  InputOptions,
  ComputedSpec,
  OutputSpec,
  RunSummary,
  SignalValue,
  WebObservationNotice,
} from "./model.js";
import type {
  BranchStateProofReport,
  LineageSummary,
  MergePlanArtifact,
  MergePlanProofEnvelope,
  MergePolicyPreviewRequest,
  MergeResultArtifact,
  MergeResultProofEnvelope,
  ReplayArtifactProofInput,
  ReplayArtifactProofReport,
  ReplayParityProofReport,
  ReplaySummary,
  RuntimeBranchHandle,
  RuntimeDefinitionEnvelope,
  RuntimeEnvelope,
  RuntimeProofReport,
  RuntimeSnapshotArtifact,
  RuntimeSnapshotEnvelope,
  GraphSummary,
} from "./diagnostics.js";
import type {
  ComputedSignal,
  DisposableHandle,
  InputSignal,
  OutputSignal,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
  Signals,
} from "./raw_surface.js";

declare const forgeSignalBrand: unique symbol;
declare const forgeSignalInputBrand: unique symbol;
declare const forgeSignalComputedBrand: unique symbol;
declare const forgeSignalOutputBrand: unique symbol;

export interface Signal<T = SignalValue> {
  (): T;
  get(): T;
  free(): void;
  [Symbol.dispose](): void;
  readonly id: string;
  readonly [forgeSignalBrand]: "signal";
}

export interface InputSignalHandle<T = SignalValue> extends Signal<T> {
  set(value: T): RunSummary;
  readonly [forgeSignalInputBrand]: "input";
}

export interface ComputedSignalHandle<T = SignalValue> extends Signal<T> {
  readonly [forgeSignalComputedBrand]: "computed";
}

export interface OutputSignalHandle<T = SignalValue> extends Signal<T> {
  readonly [forgeSignalOutputBrand]: "output";
}

export type CallableSignalTarget =
  | string
  | InputSignalHandle
  | ComputedSignalHandle
  | OutputSignalHandle;

export interface CallableSignalsTransaction {
  set(input: InputSignalHandle, value: SignalValue): void;
  setWithAspects(input: InputSignalHandle, value: SignalValue, aspects: ReadonlyArray<AspectId>): void;
  setWithRegions(input: InputSignalHandle, value: SignalValue, changedRegions: unknown): void;
  setWithRegionsAndAspects(
    input: InputSignalHandle,
    value: SignalValue,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): void;
  free(): void;
  [Symbol.dispose](): void;
}

export interface CallableSignalAdapters {
  exportDefinitions(): RuntimeDefinitionEnvelope;
  exportRuntimeEnvelope(): RuntimeEnvelopeArtifact;
  replaceRuntimeEnvelope(envelope: RuntimeEnvelope | RuntimeEnvelopeArtifact): void;
  runtimeProofReport(): RuntimeProofReport;
  free(): void;
  [Symbol.dispose](): void;
}

export type CallableBranchId = number | bigint;

export interface RuntimeEnvelopeArtifact extends RuntimeEnvelope {
  runtimeEnvelopeRestoreToken: string;
}

export interface RuntimeSnapshotEnvelopeArtifact extends RuntimeSnapshotEnvelope {
  snapshotEnvelopeRestoreToken: string;
}

export interface RuntimeSnapshotArtifactWithWire extends RuntimeSnapshotArtifact {
  snapshotRestoreToken: string;
}

export interface CallableSignalHistory {
  replay_for(id: string): ReplaySummary;
  lineage_for(id: string): LineageSummary;
  snapshot(): RuntimeSnapshotEnvelopeArtifact;
  restore_snapshot(snapshot: RuntimeSnapshotEnvelope | RuntimeSnapshotEnvelopeArtifact): void;
  current_branch(): RuntimeBranchHandle;
  branches(): ReadonlyArray<RuntimeBranchHandle>;
  create_branch(name: string): RuntimeBranchHandle;
  switch_branch(branchId: CallableBranchId): void;
  replay_for_branch(branchId: CallableBranchId): ReplaySummary;
  branch_snapshot(branchId: CallableBranchId): RuntimeSnapshotArtifactWithWire;
  branch_snapshot_id(branchId: CallableBranchId): bigint;
  branch_snapshot_envelope(branchId: CallableBranchId): RuntimeSnapshotEnvelopeArtifact;
  restore_branch_snapshot(
    branchId: CallableBranchId,
    snapshot: RuntimeSnapshotArtifact | RuntimeSnapshotArtifactWithWire,
  ): void;
  restore_branch_snapshot_by_id(branchId: CallableBranchId, snapshotId: number | bigint): void;
  merge_branches(sourceBranchId: CallableBranchId, targetBranchId: CallableBranchId): MergeResultArtifact;
  merge_branches_with_proof(
    sourceBranchId: CallableBranchId,
    targetBranchId: CallableBranchId,
  ): MergeResultProofEnvelope;
  plan_merge_branches(sourceBranchId: CallableBranchId, targetBranchId: CallableBranchId): MergePlanArtifact;
  plan_merge_branches_with_proof(
    sourceBranchId: CallableBranchId,
    targetBranchId: CallableBranchId,
  ): MergePlanProofEnvelope;
  plan_merge_policy_preview(request: MergePolicyPreviewRequest): MergePlanArtifact;
  plan_merge_policy_preview_with_proof(request: MergePolicyPreviewRequest): MergePlanProofEnvelope;
  merge_branches_policy_preview(request: MergePolicyPreviewRequest): MergeResultArtifact;
  merge_branches_policy_preview_with_proof(request: MergePolicyPreviewRequest): MergeResultProofEnvelope;
  branch_state_proof(branchId: CallableBranchId): BranchStateProofReport;
  replay_parity_proof(
    expectedBranchId: CallableBranchId,
    replayedBranchId: CallableBranchId,
  ): ReplayParityProofReport;
  replay_artifact_proof(
    expected: ReplayArtifactProofInput,
    replayedBranchId: CallableBranchId,
  ): ReplayArtifactProofReport;
  free(): void;
  [Symbol.dispose](): void;
}

export interface CallableSignalSpecialist {
  evaluateDirty(): RunSummary;
  evaluate_dirty(): RunSummary;
  graphSummary(): GraphSummary;
  graph_summary(): GraphSummary;
  readVersions(ids: ReadonlyArray<string>): ReadonlyArray<VersionSummary>;
  read_versions(ids: ReadonlyArray<string>): ReadonlyArray<VersionSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface CallableSignals {
  input<T = SignalValue>(id: string, initial: T, options?: InputOptions): InputSignalHandle<T>;
  input<T = SignalValue>(initial: T, options: InputOptions & { id: string }): InputSignalHandle<T>;
  computedSpec<T = SignalValue>(id: string, spec: ComputedSpec): ComputedSignalHandle<T>;
  computed<T = SignalValue>(id: string, spec: ComputedSpec): ComputedSignalHandle<T>;
  computed<T = SignalValue>(spec: ComputedSpec, options: { id: string }): ComputedSignalHandle<T>;
  computed<T = SignalValue>(id: string, compute: () => T): ComputedSignalHandle<T>;
  computed<T = SignalValue>(compute: () => T, options?: { id?: string }): ComputedSignalHandle<T>;
  outputSpec<T = SignalValue>(id: string, spec: OutputSpec): OutputSignalHandle<T>;
  output<T = SignalValue>(id: string, spec: OutputSpec): OutputSignalHandle<T>;
  output<T = SignalValue>(spec: OutputSpec, options: { id: string }): OutputSignalHandle<T>;
  output<T = SignalValue>(id: string, compute: () => T): OutputSignalHandle<T>;
  output<T = SignalValue>(compute: () => T, options?: { id?: string }): OutputSignalHandle<T>;
  outputCallback<T = SignalValue>(id: string, compute: () => T): OutputSignalHandle<T>;
  read<T = SignalValue>(target: CallableSignalTarget): T;
  transaction(callback: (tx: CallableSignalsTransaction) => void): RunSummary;
  batch(callback: (tx: CallableSignalsTransaction) => void): RunSummary;
  watch(target: CallableSignalTarget, callback: (notice: WebObservationNotice) => void): DisposableHandle;
  effect(target: CallableSignalTarget, callback: () => void): DisposableHandle;
  nuke(handle: DisposableHandle): boolean;
  diagnostics(): SignalDiagnostics;
  history(): CallableSignalHistory;
  specialist(): CallableSignalSpecialist;
  adapters(): CallableSignalAdapters;
  compatibilityApp(): SignalApp;
  compatibilityRuntime(): SignalRuntime;
  free(): void;
  [Symbol.dispose](): void;
}

export function createSignals(): CallableSignals;
export function createCallableSignals(): CallableSignals;
export function wrapSignals(signals: Signals): CallableSignals;

export {
  Signals as RawSignals,
  InputSignal as RawInputSignal,
  ComputedSignal as RawComputedSignal,
  OutputSignal as RawOutputSignal,
  DisposableHandle as RawDisposableHandle,
  SignalAdapters,
  SignalApp,
  SignalDiagnostics,
  SignalHistory,
  SignalRuntime,
  SignalSpecialist,
};
