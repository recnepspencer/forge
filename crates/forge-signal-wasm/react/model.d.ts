import type {
  FlowSurfaceSummary,
  ObservationSurfaceSummary,
  WebPerformanceSummary,
} from "../package/types/diagnostics.js";
import type {
  ResourceLine,
  ResourceLineFreshness,
  ResourceLineStatus,
} from "../package/types/resource/resource_lifecycle.js";
import type {
  ResourceLineDiagnosticsSummary,
  ResourceLineSummary,
} from "../package/types/resource/resource_line_summary.js";
import type { ResourceMutationResponsePlan } from "../package/types/resource/resource_mutation_response.js";

export interface SignalsDiagnosticsSnapshot {
  latestObservation: ObservationSurfaceSummary | null;
  latestFlow: FlowSurfaceSummary | null;
  performanceSummary: WebPerformanceSummary;
}

export interface SignalsHistoryReactLike<TBranch = unknown> {
  subscribe(listener: () => void): () => void;
  current_branch(): TBranch;
  branches(): readonly TBranch[];
}

export interface SignalsHistoryView<TBranch = unknown> {
  readonly currentBranch: TBranch;
  readonly branches: readonly TBranch[];
}

export interface BrowserHistoryStoryReactLike<
  TEntry = unknown,
  TEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
> {
  subscribe(listener: () => void): () => void;
  current(): TEntry | null;
  admittedEntries(): readonly TEntry[];
  breadcrumbTrail(): TBreadcrumbTrail;
  backProvenance(): TBackProvenance;
  events(): readonly TEvent[];
  latestBoundaryEvent(): TEvent | null;
}

export interface BrowserHistoryStoryView<
  TEntry = unknown,
  TEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
> {
  readonly current: TEntry | null;
  readonly entries: readonly TEntry[];
  readonly breadcrumbTrail: TBreadcrumbTrail;
  readonly backProvenance: TBackProvenance;
  readonly events: readonly TEvent[];
}

export interface ReactPerformanceSummary {
  activeSignalSubscriptionCount: number;
  activeReactSubscriberCount: number;
  activeRuntimeWatchHandleCount: number;
  diagnosticsSubscriberCount: number;
  sharedFanoutRatio: number;
}

export interface WebObservationNotice {
  triggerMatched: boolean;
  meaningfulChange: boolean;
}

export interface DisposableHandleLike {
  free(): void;
}

export interface SignalHandleLike {
  id: string;
  get(): unknown;
}

export interface OptionalSignalValueInactiveResult<TInactive = undefined> {
  readonly kind: "inactive";
  readonly reason: "authorInactive";
  readonly value: TInactive;
}

export interface OptionalSignalValueActiveResult<TValue> {
  readonly kind: "active";
  readonly value: TValue;
}

export type OptionalSignalValueResult<TValue, TInactive = undefined> =
  | OptionalSignalValueInactiveResult<TInactive>
  | OptionalSignalValueActiveResult<TValue>;

export interface ResourceLineReactLike<TValue = unknown, TParams = unknown> {
  signal(): SignalHandleLike;
  summarySignal(): SignalHandleLike;
  summary(): ResourceLineSummary<TParams>;
  status(): ResourceLineStatus;
  freshness(): ResourceLineFreshness;
  diagnosticsSummary(): ResourceLineDiagnosticsSummary;
  mutationResponse(): ResourceMutationResponsePlan | null;
}

export interface ResourceOperationExecutionReactLike<
  TValue = unknown,
  TParams = unknown,
  TLine extends ResourceLineReactLike<TValue, TParams> = ResourceLineReactLike<TValue, TParams>,
> {
  readonly line: TLine;
}

export type ResourceOperationResultKind =
  | "pending"
  | "fulfilled"
  | "partial"
  | "rejected"
  | "timedOut";

export interface ResourceOperationView<
  TLine extends ResourceLineReactLike<TValue, TParams>,
  TValue = unknown,
  TParams = unknown,
> {
  readonly line: TLine;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly status: ReturnType<TLine["status"]>;
  readonly freshness: ReturnType<TLine["freshness"]>;
  readonly diagnosticsSummary: ReturnType<TLine["diagnosticsSummary"]>;
  readonly mutationResponse: ReturnType<TLine["mutationResponse"]>;
  readonly confirmationKind: ResourceMutationResponsePlan["confirmation"]["kind"] | null;
  readonly resultKind: ResourceOperationResultKind;
  readonly pending: boolean;
  readonly settled: boolean;
  readonly message: string | null;
}

export interface OptionalResourceLineInactiveResult<TInactive = undefined> {
  readonly kind: "inactive";
  readonly reason: "authorInactive";
  readonly line: null;
  readonly value: TInactive;
  readonly summary: null;
  readonly status: null;
  readonly freshness: null;
  readonly diagnosticsSummary: null;
}

export interface OptionalResourceLineActiveResult<
  TLine extends ResourceLineReactLike<TValue>,
  TValue,
> {
  readonly kind: "active";
  readonly line: TLine;
  readonly value: TValue;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly status: ReturnType<TLine["status"]>;
  readonly freshness: ReturnType<TLine["freshness"]>;
  readonly diagnosticsSummary: ReturnType<TLine["diagnosticsSummary"]>;
}

export type OptionalResourceLineResult<
  TLine extends ResourceLineReactLike<TValue>,
  TValue,
  TInactive = undefined,
> =
  | OptionalResourceLineInactiveResult<TInactive>
  | OptionalResourceLineActiveResult<TLine, TValue>;

export type ResourceViewContentState =
  | "loading"
  | "refreshing"
  | "ready"
  | "empty"
  | "error";

export interface ResourceViewInactiveResult<TInactive = undefined> {
  readonly kind: "inactive";
  readonly reason: "authorInactive";
  readonly contentState: null;
  readonly line: null;
  readonly value: TInactive;
  readonly summary: null;
  readonly status: null;
  readonly freshness: null;
  readonly diagnosticsSummary: null;
  readonly message: null;
  readonly hasVisibleValue: false;
  readonly isRefreshing: false;
  readonly isEmpty: false;
}

export interface ResourceViewActiveResult<
  TLine extends ResourceLineReactLike<TValue>,
  TValue,
> {
  readonly kind: "active";
  readonly contentState: ResourceViewContentState;
  readonly line: TLine;
  readonly value: TValue;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly status: ReturnType<TLine["status"]>;
  readonly freshness: ReturnType<TLine["freshness"]>;
  readonly diagnosticsSummary: ReturnType<TLine["diagnosticsSummary"]>;
  readonly message: string | null;
  readonly hasVisibleValue: boolean;
  readonly isRefreshing: boolean;
  readonly isEmpty: boolean;
}

export type ResourceViewResult<
  TLine extends ResourceLineReactLike<TValue>,
  TValue,
  TInactive = undefined,
> =
  | ResourceViewInactiveResult<TInactive>
  | ResourceViewActiveResult<TLine, TValue>;

export interface SignalsTransactionLike {
  set(input: SignalHandleLike, value: unknown): void;
}

export interface SignalDiagnosticsLike {
  latestObservation(): ObservationSurfaceSummary | null;
  latestFlow(): FlowSurfaceSummary | null;
  performanceSummary(): WebPerformanceSummary;
  subscribe(listener: () => void): DisposableHandleLike;
}

export interface CompatibilityAppLike {
  read(id: string): unknown;
}

export interface SignalsLike {
  read(target: SignalHandleLike | string): unknown;
  watch(
    target: SignalHandleLike | string,
    callback: (notice: WebObservationNotice) => void,
  ): DisposableHandleLike;
  nuke(handle: DisposableHandleLike): boolean;
  diagnostics(): SignalDiagnosticsLike;
  compatibilityApp(): CompatibilityAppLike;
  transaction(callback: (tx: SignalsTransactionLike) => void): unknown;
  batch(callback: (tx: SignalsTransactionLike) => void): unknown;
}

export interface ReactSignalsStore<TSignals extends SignalsLike = SignalsLike> {
  readonly signals: TSignals;
  subscribeSignal(signal: SignalHandleLike | string, listener: () => void): () => void;
  getSignalSnapshot(signal: SignalHandleLike | string): unknown;
  subscribeDiagnostics(listener: () => void): () => void;
  getDiagnosticsSnapshot(): SignalsDiagnosticsSnapshot;
  transaction(callback: Parameters<TSignals["transaction"]>[0]): unknown;
  batch(callback: Parameters<TSignals["batch"]>[0]): unknown;
  refreshDiagnostics(): SignalsDiagnosticsSnapshot;
  performanceSummary(): ReactPerformanceSummary;
  dispose(): void;
}

export interface ResourceCatalogDefinition<
  TSignals extends object = object,
  TCatalog = unknown,
> {
  readonly id: string;
  build(signals: TSignals): TCatalog;
}

export interface FormBoundInputReactLike<TValue = unknown, TRaw = TValue> {
  input(rawValue: TRaw, options?: { readonly commit?: boolean; readonly source?: string }): void;
  focus(): void;
  blur(): void;
  touch(): void;
  visit(): void;
  set(value: TValue): void;
  clearDraft(): void;
}

export interface FormFieldHandleReactLike<TValue = unknown, TRaw = TValue> {
  id: string;
  path: string;
  value(): TValue;
  dirty(): unknown;
  diagnostics(): unknown;
}

export interface FormActionPlanReactLike {
  readonly status: string;
  readonly readiness: {
    readonly canRun: boolean;
    readonly blockers: readonly unknown[];
  };
}

export interface FormActionDebugReactLike {
  readonly pending: boolean;
  readonly latestExecution: unknown;
}

export interface FormVisibleMessageReactLike {
  readonly target?: string;
  readonly visibility?: string;
}

export interface FormInteractionFieldReactLike {
  readonly field: string;
}

export interface FormControllerReactLike {
  bindInput<TValue = unknown, TRaw = TValue>(
    fieldId: string,
    options?: unknown,
  ): FormBoundInputReactLike<TValue, TRaw>;
  field<TValue = unknown, TRaw = TValue>(
    fieldId: string,
  ): FormFieldHandleReactLike<TValue, TRaw>;
  visibleMessages(): readonly FormVisibleMessageReactLike[];
  interaction(): {
    readonly fields: readonly FormInteractionFieldReactLike[];
  };
  fieldWritePosture(fieldId: string, capability?: "edit" | "patch"): unknown;
  actionPlan(actionId: string): FormActionPlanReactLike;
  debugAction(actionId: string): FormActionDebugReactLike;
  executeAction(actionId: string): unknown;
}

export interface ManagedResourceWriteLineLike<
  TParams = unknown,
  TValue = unknown,
> extends ResourceLine<TParams, TValue> {
  mutationResponse(): ResourceMutationResponsePlan | null;
  diagnosticsSummary(): ResourceLineDiagnosticsSummary;
}

export interface ManagedResourceRecoveryLineLike {
  refresh(): ResourceLineStatus;
  revalidate(): ResourceLineStatus;
}

export type ManagedResourceWriteRecoveryKind =
  | "refreshResourceLine"
  | "revalidateResourceLine";

interface ManagedResourceWriteRecoveryBase<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly reason?: string;
  readonly line: TLine | (() => TLine);
}

export interface ManagedResourceWriteRefreshRecovery<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> extends ManagedResourceWriteRecoveryBase<TLine> {
  readonly kind: "refreshResourceLine";
}

export interface ManagedResourceWriteRevalidateRecovery<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> extends ManagedResourceWriteRecoveryBase<TLine> {
  readonly kind: "revalidateResourceLine";
}

export type ManagedResourceWriteRecoveryDeclaration<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> =
  | ManagedResourceWriteRefreshRecovery<TLine>
  | ManagedResourceWriteRevalidateRecovery<TLine>;

export interface ManagedResourceWriteRecoveryExecution<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly kind: ManagedResourceWriteRecoveryKind;
  readonly line: TLine;
  readonly reason: string | null;
  readonly status: ResourceLineStatus;
}

export interface ManagedResourceWriteRecoveryPolicy<
  TLine extends ManagedResourceRecoveryLineLike = ManagedResourceRecoveryLineLike,
> {
  readonly partial?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
  readonly rejected?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
  readonly timedOut?: readonly ManagedResourceWriteRecoveryDeclaration<TLine>[];
}

export interface ManagedResourceWriteSuccessResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly resultKind: "fulfilled" | "partial";
  readonly status: Extract<ResourceLineStatus, { kind: "fulfilled" }>;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly freshness: ResourceLineFreshness;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
  readonly mutationResponse: ResourceMutationResponsePlan | null;
  readonly confirmationKind: ResourceMutationResponsePlan["confirmation"]["kind"] | null;
  readonly value: ReturnType<TLine["value"]>;
  readonly recovery: readonly ManagedResourceWriteRecoveryExecution[];
}

export interface ManagedResourceWriteFailureResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly resultKind: "rejected" | "timedOut";
  readonly status: Extract<ResourceLineStatus, { kind: "rejected" | "timedOut" }>;
  readonly summary: ReturnType<TLine["summary"]>;
  readonly freshness: ResourceLineFreshness;
  readonly diagnosticsSummary: ResourceLineDiagnosticsSummary;
  readonly mutationResponse: ResourceMutationResponsePlan | null;
  readonly confirmationKind: null;
  readonly recovery: readonly ManagedResourceWriteRecoveryExecution[];
}

export type ManagedResourceWriteResult<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> =
  | ManagedResourceWriteSuccessResult<TLine>
  | ManagedResourceWriteFailureResult<TLine>;

type MaybePromise<TValue> = TValue | Promise<TValue>;

export interface ManagedResourceWriteOptions<
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  timeoutMs?: number;
  freeOnSettle?: boolean;
  recovery?: ManagedResourceWriteRecoveryPolicy;
  onPendingChange?(pending: boolean): MaybePromise<void>;
  onFulfilled?(result: ManagedResourceWriteSuccessResult<TLine>): MaybePromise<void>;
  onPartial?(result: Extract<ManagedResourceWriteResult<TLine>, { resultKind: "partial" }>): MaybePromise<void>;
  onRejected?(result: ManagedResourceWriteFailureResult<TLine>): MaybePromise<void>;
  onSettled?(result: ManagedResourceWriteResult<TLine>): MaybePromise<void>;
}

export interface ManagedResourceWriteState<
  TArgs,
  TLine extends ManagedResourceWriteLineLike = ManagedResourceWriteLineLike,
> {
  readonly pending: boolean;
  readonly lastResult: ManagedResourceWriteResult<TLine> | null;
  readonly lastError: unknown;
  execute(args: TArgs): Promise<ManagedResourceWriteResult<TLine>>;
  reset(): void;
}
