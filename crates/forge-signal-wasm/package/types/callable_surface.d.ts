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
  HostCapabilityDiagnosticsReport,
  HostCapabilityTransportReport,
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
declare const forgeSignalHostCapabilityPlanBrand: unique symbol;
declare const forgeSignalViewportCapabilityRegistrationBrand: unique symbol;
declare const forgeSignalViewportCapabilityHandleBrand: unique symbol;
declare const forgeSignalVisibilityCapabilityRegistrationBrand: unique symbol;
declare const forgeSignalVisibilityCapabilityHandleBrand: unique symbol;
declare const forgeSignalOnlineCapabilityRegistrationBrand: unique symbol;
declare const forgeSignalOnlineCapabilityHandleBrand: unique symbol;
declare const forgeSignalClockCapabilityRegistrationBrand: unique symbol;
declare const forgeSignalClockCapabilityHandleBrand: unique symbol;
declare const forgeSignalPersistenceCapabilityRegistrationBrand: unique symbol;
declare const forgeSignalPersistenceCapabilityValueBrand: unique symbol;
declare const forgeSignalPersistenceCapabilityHandleBrand: unique symbol;

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

export type HostCapabilityCompatibility =
  | "LiveOnly"
  | "Reattachable"
  | "SnapshotPortable"
  | "ImportDenied";

export type HostCapabilitySubscription =
  | void
  | (() => void)
  | { dispose(): void }
  | { free(): void };

export interface ViewportCapabilityState {
  width: number;
  height: number;
}

export interface ViewportCapabilitySource {
  current(): ViewportCapabilityState;
  subscribe(listener: () => void): HostCapabilitySubscription;
}

export interface ViewportCapabilityOptions {
  source: ViewportCapabilitySource;
  compatibility?: HostCapabilityCompatibility;
}

export interface ViewportCapabilityRegistration {
  readonly family: "viewport";
  readonly compatibility: HostCapabilityCompatibility;
  readonly [forgeSignalViewportCapabilityRegistrationBrand]: "viewportCapabilityRegistration";
}

export interface VisibilityCapabilitySource {
  current(): boolean | "visible" | "hidden";
  subscribe(listener: () => void): HostCapabilitySubscription;
}

export interface VisibilityCapabilityOptions {
  source: VisibilityCapabilitySource;
  compatibility?: HostCapabilityCompatibility;
}

export interface VisibilityCapabilityRegistration {
  readonly family: "visibility";
  readonly compatibility: HostCapabilityCompatibility;
  readonly [forgeSignalVisibilityCapabilityRegistrationBrand]: "visibilityCapabilityRegistration";
}

export interface OnlineCapabilitySource {
  current(): boolean | "online" | "offline";
  subscribe(listener: () => void): HostCapabilitySubscription;
}

export interface OnlineCapabilityOptions {
  source: OnlineCapabilitySource;
  compatibility?: HostCapabilityCompatibility;
}

export interface OnlineCapabilityRegistration {
  readonly family: "online";
  readonly compatibility: HostCapabilityCompatibility;
  readonly [forgeSignalOnlineCapabilityRegistrationBrand]: "onlineCapabilityRegistration";
}

export interface ClockCapabilitySource {
  current(): number;
}

export interface ClockCapabilityOptions {
  source: ClockCapabilitySource;
  pollMs?: number;
  compatibility?: HostCapabilityCompatibility;
}

export interface ClockCapabilityRegistration {
  readonly family: "clock";
  readonly compatibility: HostCapabilityCompatibility;
  readonly pollMs: number;
  readonly [forgeSignalClockCapabilityRegistrationBrand]: "clockCapabilityRegistration";
}

export interface PersistenceCapabilitySource<T = SignalValue> {
  current(): T;
}

export interface PersistenceCapabilityOptions<T = SignalValue> {
  source: PersistenceCapabilitySource<T>;
  compatibility?: HostCapabilityCompatibility;
}

export interface PersistenceCapabilityRegistration<T = SignalValue> {
  readonly family: "persistence";
  readonly compatibility: HostCapabilityCompatibility;
  readonly [forgeSignalPersistenceCapabilityRegistrationBrand]: "persistenceCapabilityRegistration";
  readonly [forgeSignalPersistenceCapabilityValueBrand]?: T;
}

export interface HostCapabilityPlanInput<TPersistence = SignalValue> {
  viewport?: ViewportCapabilityRegistration;
  visibility?: VisibilityCapabilityRegistration;
  online?: OnlineCapabilityRegistration;
  clock?: ClockCapabilityRegistration;
  persistence?: PersistenceCapabilityRegistration<TPersistence>;
}

export interface HostCapabilityPlan<TPersistence = SignalValue> {
  readonly viewport?: ViewportCapabilityRegistration;
  readonly visibility?: VisibilityCapabilityRegistration;
  readonly online?: OnlineCapabilityRegistration;
  readonly clock?: ClockCapabilityRegistration;
  readonly persistence?: PersistenceCapabilityRegistration<TPersistence>;
  readonly [forgeSignalHostCapabilityPlanBrand]: "hostCapabilityPlan";
}

export interface CreateSignalsOptions<TPersistence = SignalValue> {
  hostCapabilities?: HostCapabilityPlan<TPersistence>;
}

export interface ViewportCapabilityDescriptor {
  family: "viewport";
  compatibility: HostCapabilityCompatibility;
  registrationId: "viewport";
}

export interface HostViewportCapability {
  size(): ViewportCapabilityState;
  width(): number;
  height(): number;
  descriptor(): ViewportCapabilityDescriptor;
  readonly [forgeSignalViewportCapabilityHandleBrand]: "viewportCapabilityHandle";
}

export interface VisibilityCapabilityDescriptor {
  family: "visibility";
  compatibility: HostCapabilityCompatibility;
  registrationId: "visibility";
}

export interface HostVisibilityCapability {
  state(): "visible" | "hidden";
  isVisible(): boolean;
  descriptor(): VisibilityCapabilityDescriptor;
  readonly [forgeSignalVisibilityCapabilityHandleBrand]: "visibilityCapabilityHandle";
}

export interface OnlineCapabilityDescriptor {
  family: "online";
  compatibility: HostCapabilityCompatibility;
  registrationId: "online";
}

export interface HostOnlineCapability {
  state(): "online" | "offline";
  isOnline(): boolean;
  descriptor(): OnlineCapabilityDescriptor;
  readonly [forgeSignalOnlineCapabilityHandleBrand]: "onlineCapabilityHandle";
}

export interface ClockCapabilityDescriptor {
  family: "clock";
  compatibility: HostCapabilityCompatibility;
  registrationId: "clock";
}

export interface HostClockCapability {
  now(): number;
  descriptor(): ClockCapabilityDescriptor;
  readonly [forgeSignalClockCapabilityHandleBrand]: "clockCapabilityHandle";
}

export interface PersistenceCapabilityDescriptor {
  family: "persistence";
  compatibility: HostCapabilityCompatibility;
  registrationId: "persistence";
}

export interface HostPersistenceCapability<T = SignalValue> {
  value(): T;
  commit(): RunSummary;
  descriptor(): PersistenceCapabilityDescriptor;
  readonly [forgeSignalPersistenceCapabilityHandleBrand]: "persistenceCapabilityHandle";
}

export interface CallableSignalsHost<TPersistence = SignalValue> {
  readonly viewport?: HostViewportCapability;
  readonly visibility?: HostVisibilityCapability;
  readonly online?: HostOnlineCapability;
  readonly clock?: HostClockCapability;
  readonly persistence?: HostPersistenceCapability<TPersistence>;
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
  replaceRuntimeEnvelope(envelope: RuntimeEnvelope): void;
  restoreExactRuntimeEnvelope(envelope: RuntimeEnvelopeArtifact): void;
  runtimeProofReport(): RuntimeProofReport;
  hostCapabilityTransportReport(envelope?: RuntimeEnvelope): HostCapabilityTransportReport;
  free(): void;
  [Symbol.dispose](): void;
}

export type CallableSignalDiagnostics =
  SignalDiagnostics & {
    hostCapabilityReport(): HostCapabilityDiagnosticsReport;
  };

export type CallableBranchId = number | bigint;

export interface RuntimeEnvelopeArtifact extends RuntimeEnvelope {
  runtimeEnvelopeRestoreToken: string;
  runtimeEnvelopeRestoreMode: "SameRuntimeExact";
  runtimeEnvelopePortableWire: string;
}

export interface RuntimeSnapshotEnvelopeArtifact extends RuntimeSnapshotEnvelope {
  snapshotEnvelopeRestoreToken: string;
  snapshotEnvelopeRestoreMode: "SameRuntimeExact";
  snapshotEnvelopePortableWire: string;
}

export interface RuntimeSnapshotArtifactWithWire extends RuntimeSnapshotArtifact {
  snapshotRestoreToken: string;
  snapshotRestoreMode: "SameRuntimeExact";
  snapshotPortableWire: string;
}

export interface CallableSignalHistory {
  replay_for(id: string): ReplaySummary;
  lineage_for(id: string): LineageSummary;
  snapshot(): RuntimeSnapshotEnvelopeArtifact;
  restore_snapshot(snapshot: RuntimeSnapshotEnvelope): void;
  restore_exact_snapshot(snapshot: RuntimeSnapshotEnvelopeArtifact): void;
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
    snapshot: RuntimeSnapshotArtifact,
  ): void;
  restore_exact_branch_snapshot(
    branchId: CallableBranchId,
    snapshot: RuntimeSnapshotArtifactWithWire,
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

export interface CallableSignals<TPersistence = SignalValue> {
  readonly host: CallableSignalsHost<TPersistence>;
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
  diagnostics(): CallableSignalDiagnostics;
  history(): CallableSignalHistory;
  specialist(): CallableSignalSpecialist;
  adapters(): CallableSignalAdapters;
  compatibilityApp(): SignalApp;
  compatibilityRuntime(): SignalRuntime;
  free(): void;
  [Symbol.dispose](): void;
}

export function viewportCapability(options: ViewportCapabilityOptions): ViewportCapabilityRegistration;
export function visibilityCapability(options: VisibilityCapabilityOptions): VisibilityCapabilityRegistration;
export function onlineCapability(options: OnlineCapabilityOptions): OnlineCapabilityRegistration;
export function clockCapability(options: ClockCapabilityOptions): ClockCapabilityRegistration;
export function persistenceCapability<T = SignalValue>(options: PersistenceCapabilityOptions<T>): PersistenceCapabilityRegistration<T>;
export function hostCapabilityPlan<TPersistence = SignalValue>(input: HostCapabilityPlanInput<TPersistence>): HostCapabilityPlan<TPersistence>;
export function createSignals<TPersistence = SignalValue>(options?: CreateSignalsOptions<TPersistence>): CallableSignals<TPersistence>;
export function createCallableSignals<TPersistence = SignalValue>(options?: CreateSignalsOptions<TPersistence>): CallableSignals<TPersistence>;
export function wrapSignals<TPersistence = SignalValue>(signals: Signals, options?: CreateSignalsOptions<TPersistence>): CallableSignals<TPersistence>;

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
