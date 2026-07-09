import type {
  FlowSurfaceSummary,
  ObservationSurfaceSummary,
  WebPerformanceSummary,
} from "../package/types/diagnostics.js";
import type {
  ResourceLineAwaitSettlementResult,
  ResourceLineFreshness,
  ResourceLineStatus,
} from "../package/types/resource/resource_lifecycle.js";
import type { ResourceMutationResponsePlan } from "../package/types/resource/resource_mutation_response.js";
import type {
  ResourceLineDiagnosticsSummary,
  ResourceLineSummary,
} from "../package/types/resource/resource_line_summary.js";

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
  readonly canUndo: boolean;
  readonly canRedo: boolean;
}

export interface BrowserHistoryStoryReactLike<
  TEntry = unknown,
  TEvent = unknown,
  TBreadcrumbTrail = unknown,
  TBackProvenance = unknown,
> {
  subscribe(listener: () => void): () => void;
  record(report: unknown): unknown;
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

export interface ResourceLineFamilyReactLike<
  TParams = unknown,
  TLine extends ResourceLineReactLike<any, TParams> = ResourceLineReactLike<any, TParams>,
> {
  line(params: TParams): TLine;
  optionalLine?(selection: ResourceLineSelection<TParams>): TLine | null;
}

export interface DisabledResourceLineSelection {
  readonly enabled: false;
}

export type ResourceLineSelection<TParams> =
  | TParams
  | null
  | undefined
  | DisabledResourceLineSelection;

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
  TSignals extends SignalsLike = SignalsLike,
  TCatalog = unknown,
> {
  readonly id: string;
  build(signals: TSignals): TCatalog;
}

export type {
  FormActionDebugReactLike,
  FormActionPlanReactLike,
  FormBoundInputReactLike,
  FormControllerReactLike,
  FormFieldHandleReactLike,
  FormInteractionFieldReactLike,
  FormVisibleMessageReactLike,
  RuntimeFormController,
  RuntimeFormDeclaration,
  RuntimeFormFieldHandleFor,
  SignalsFormActionBinding,
  SignalsFormBinding,
  SignalsFormCheckboxBinding,
  SignalsFormFieldBinding,
  SignalsFormFieldState,
  SignalsFormMultiSelectBinding,
  SignalsFormOption,
  SignalsFormSelectBinding,
  SignalsWithFormLike,
} from "./form_model.js";
export type {
  ManagedResourceWriteExecution,
  ManagedResourceWriteFeedback,
  ManagedResourceWriteFeedbackMessages,
  ManagedResourceRecoveryLineLike,
  ManagedResourceWriteFailureResult,
  ManagedResourceWriteHookOptions,
  ManagedResourceWriteLineLike,
  ManagedResourceWriteOptions,
  ManagedResourceWriteRecoveryDeclaration,
  ManagedResourceWriteRecoveryExecution,
  ManagedResourceWriteRecoveryKind,
  ManagedResourceWriteRecoveryPolicy,
  ManagedResourceWriteRecoverySummary,
  ManagedResourceWriteRecoverySurface,
  ManagedResourceWriteResult,
  ManagedResourceWriteState,
  ManagedResourceWriteSuccessResult,
} from "./resource_write_model.js";
