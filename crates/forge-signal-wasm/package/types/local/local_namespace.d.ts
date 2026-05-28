import type { AspectId, RunSummary, SignalValue } from "../model.js";
import type {
  ComputedSignalHandle,
  InputSignalHandle,
  ScopedSignalNamespace,
} from "../callable_surface.js";
import type { FormSourceDeclaration } from "../forms/sources.js";
import type {
  FormCollaborationComment,
  FormCollaborationMode,
  FormCollaborationPresence,
  FormCollaborationReport,
} from "../forms/collaboration.js";
import type { FormController } from "../forms/controller.js";

export interface LocalDialogStateShape<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  readonly isOpen: boolean;
  readonly mode: TMode | null;
  readonly data: TData | null;
  readonly context: TContext | null;
  readonly loading: boolean;
}

export interface LocalDialogCollaborationDeclaration {
  readonly mode: FormCollaborationMode;
  readonly actorId?: string;
  readonly supportsPresence?: boolean;
  readonly supportsComments?: boolean;
}

export interface LocalDialogCollaborationReportInput<
  TMode extends string = string,
> {
  readonly posture: "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly lockOwnerId?: string | null;
  readonly leasedModes?: readonly (TMode | null)[] | null;
  readonly branchId?: string | number | null;
  readonly readOnly?: boolean;
  readonly remoteUpdateDigest?: string | null;
  readonly presence?: readonly FormCollaborationPresence[];
  readonly comments?: readonly FormCollaborationComment[];
}

export interface LocalDialogCollaborationArtifact<
  TMode extends string = string,
> extends LocalDialogCollaborationReportInput<TMode> {
  readonly kind: "dialogCollaboration";
  readonly artifactId: number;
  readonly source: "report" | "clear";
  readonly mode: FormCollaborationMode | null;
  readonly actorId: string | null;
  readonly digest: string;
}

export interface LocalDialogCollaborationEvent<
  TMode extends string = string,
> {
  readonly kind:
    | "postureChange"
    | "lockChange"
    | "leaseChange"
    | "branchChange"
    | "readOnlyChange"
    | "remoteUpdateChange"
    | "presenceChange"
    | "commentChange";
  readonly source: "report" | "clear";
  readonly artifactId: number;
  readonly previousArtifactId: number | null;
  readonly mode: FormCollaborationMode | null;
  readonly posture: "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly lockOwnerId: string | null;
  readonly leasedModes: readonly (TMode | null)[];
  readonly branchId: string | number | null;
  readonly readOnly: boolean;
  readonly remoteUpdateDigest: string | null;
  readonly presence: readonly FormCollaborationPresence[];
  readonly comments: readonly FormCollaborationComment[];
  readonly digest: string;
}

export interface LocalDialogCollaborationConflict {
  readonly kind: "modeConflict";
  readonly reason: string;
  readonly nativeMode: FormCollaborationMode;
  readonly boundFormMode: FormCollaborationMode;
}

export interface LocalDialogCollaborationReport<
  TMode extends string = string,
> {
  readonly declared: boolean;
  readonly mode: FormCollaborationMode | "notDeclared";
  readonly actorId: string | null;
  readonly posture: "notDeclared" | "active" | "blocked" | "settling" | "unavailable";
  readonly reason: string;
  readonly lockOwnerId: string | null;
  readonly leasedModes: readonly (TMode | null)[];
  readonly branchId: string | number | null;
  readonly readOnly: boolean;
  readonly remoteUpdateDigest: string | null;
  readonly presence: readonly FormCollaborationPresence[];
  readonly comments: readonly FormCollaborationComment[];
  readonly history: readonly LocalDialogCollaborationArtifact<TMode>[];
  readonly events: readonly LocalDialogCollaborationEvent<TMode>[];
  readonly conflicts: readonly LocalDialogCollaborationConflict[];
  readonly sources: {
    readonly native: LocalDialogCollaborationArtifact<TMode> | null;
    readonly boundForm: FormCollaborationReport | null;
  };
  readonly digest: string;
}

export interface LocalDialogReadinessBlocker {
  readonly kind: string;
  readonly source: "dialog" | "form" | "collaboration";
  readonly action?: string;
  readonly reason: string;
}

export interface LocalDialogReadinessActionReport {
  readonly actionId: string;
  readonly status: "accepted" | "blocked";
  readonly readiness: {
    readonly canRun: boolean;
    readonly blockers: readonly LocalDialogReadinessBlocker[];
  };
}

export interface LocalDialogReadinessReport {
  readonly dirty: boolean;
  readonly blockers: readonly LocalDialogReadinessBlocker[];
  readonly actions: Readonly<Record<string, LocalDialogReadinessActionReport>>;
  readonly currentStepId: string | null;
  readonly stepProgress: "none" | "started";
}

export interface LocalDialogMessage {
  readonly code: string;
  readonly source: "dialog" | "form" | "collaboration";
  readonly target: string | null;
  readonly severity: "info" | "warning" | "error";
  readonly visibility: "visible" | "summary" | "blocked";
  readonly text: string;
}

export interface LocalDialogPatchPlan<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  readonly changed: boolean;
  readonly changedKeys: readonly (keyof LocalDialogStateShape<TMode, TData, TContext>)[];
}

export interface LocalDialogStateHistoryArtifact<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  readonly kind: "dialogState";
  readonly action:
    | "open"
    | "close"
    | "toggle"
    | "patch"
    | "reset"
    | "setLoading"
    | "requestClose";
  readonly reason: string | null;
  readonly previous: LocalDialogStateShape<TMode, TData, TContext>;
  readonly next: LocalDialogStateShape<TMode, TData, TContext>;
  readonly timestampMs: number;
}

export interface LocalDialogActionExecutionArtifact {
  readonly actionId: string;
  readonly source: "dialog" | "form";
  readonly resultKind: "accepted" | "blocked" | "pending" | "fulfilled" | "rejected";
  readonly reason: string | null;
  readonly startedAtMs: number;
  readonly finishedAtMs: number | null;
  readonly error: unknown;
  readonly delegatedResultKind?: string | null;
}

export type LocalDialogBindableForm = Pick<
  FormController,
  | "summarySignal"
  | "dirty"
  | "visibleMessages"
  | "steps"
  | "navigation"
  | "actionPlan"
  | "executeAction"
  | "collaboration"
  | "readiness"
  | "reset"
>;

export interface LocalDialogFormBindingOptions {
  readonly confirmActionId?: string;
  readonly closeOnSuccess?: boolean;
  readonly resetOnClose?: boolean;
  readonly stayOpenOnError?: boolean;
  readonly blockCloseWhenDirty?: boolean;
}

export interface LocalDialogActionContext<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  readonly dialog: LocalDialogState<TMode, TData, TContext, Record<string, LocalDialogActionBinding>>;
  readonly state: LocalDialogStateShape<TMode, TData, TContext>;
  readonly collaboration: LocalDialogCollaborationReport<TMode>;
  readonly form: LocalDialogBindableForm | null;
}

export interface LocalDialogCustomActionDefinition<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  readonly writes?: boolean;
  readonly closeOnSuccess?: boolean;
  readonly readiness?: (
    context: LocalDialogActionContext<TMode, TData, TContext>,
  ) => boolean | {
    readonly canRun: boolean;
    readonly reason?: string;
    readonly blockers?: readonly LocalDialogReadinessBlocker[];
  };
  readonly execute: (
    context: LocalDialogActionContext<TMode, TData, TContext>,
  ) => unknown;
}

export interface LocalDialogActionBuilder<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
> {
  custom(
    definition: LocalDialogCustomActionDefinition<TMode, TData, TContext>,
  ): LocalDialogCustomActionDefinition<TMode, TData, TContext>;
}

export interface LocalDialogActionBinding {
  readonly plan: LocalDialogReadinessActionReport;
  readonly disabled: boolean;
  readonly pending: boolean;
  readonly latestExecution: LocalDialogActionExecutionArtifact | null;
  readonly resultKind: LocalDialogActionExecutionArtifact["resultKind"] | null;
  execute(): unknown;
}

export interface LocalDialogStateOptions<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
  TCustomActions extends Record<string, LocalDialogCustomActionDefinition<TMode, TData, TContext>> = Record<
    string,
    LocalDialogCustomActionDefinition<TMode, TData, TContext>
  >,
> {
  readonly identity: string;
  readonly modes?: readonly TMode[];
  readonly initial?: Partial<LocalDialogStateShape<TMode, TData, TContext>>;
  readonly debugName?: string;
  readonly collaboration?: LocalDialogCollaborationDeclaration;
  readonly actions?: (
    factory: LocalDialogActionBuilder<TMode, TData, TContext>,
  ) => TCustomActions;
}

export interface LocalDialogState<
  TMode extends string = string,
  TData = SignalValue,
  TContext = SignalValue,
  TActions extends Record<string, LocalDialogActionBinding> = Record<string, LocalDialogActionBinding>,
> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly isOpen: InputSignalHandle<boolean>;
  readonly mode: InputSignalHandle<TMode | null>;
  readonly data: InputSignalHandle<TData | null>;
  readonly context: InputSignalHandle<TContext | null>;
  readonly loading: InputSignalHandle<boolean>;
  source(): LocalDialogStateShape<TMode, TData, TContext>;
  draft(): LocalDialogStateShape<TMode, TData, TContext>;
  effective(): LocalDialogStateShape<TMode, TData, TContext>;
  dirty(): boolean;
  patchPlan(): LocalDialogPatchPlan<TMode, TData, TContext>;
  readiness(): LocalDialogReadinessReport;
  visibleMessages(): readonly LocalDialogMessage[];
  summarySignal(): ComputedSignalHandle<unknown>;
  stateHistory(): readonly LocalDialogStateHistoryArtifact<TMode, TData, TContext>[];
  actionHistory(): readonly LocalDialogActionExecutionArtifact[];
  diagnostics(): {
    readonly state: LocalDialogStateShape<TMode, TData, TContext>;
    readonly source: LocalDialogStateShape<TMode, TData, TContext>;
    readonly readiness: LocalDialogReadinessReport;
    readonly collaboration: LocalDialogCollaborationReport<TMode>;
    readonly actions: TActions;
  };
  open(
    mode: TMode | null,
    options?: Partial<Omit<LocalDialogStateShape<TMode, TData, TContext>, "mode" | "isOpen">> & {
      readonly reason?: string;
    },
  ): RunSummary | Promise<RunSummary>;
  close(options?: { readonly reason?: string; readonly clear?: boolean }): RunSummary | Promise<RunSummary>;
  toggle(options?: { readonly reason?: string }): RunSummary | Promise<RunSummary>;
  patch(next: Partial<LocalDialogStateShape<TMode, TData, TContext>>): RunSummary | Promise<RunSummary>;
  setLoading(next: boolean, options?: { readonly reason?: string }): RunSummary | Promise<RunSummary>;
  reset(options?: { readonly reason?: string }): RunSummary | Promise<RunSummary>;
  requestClose(options?: { readonly reason?: string }): Promise<{
    readonly status: "accepted" | "blocked";
    readonly blockers: readonly LocalDialogReadinessBlocker[];
    readonly closed: boolean;
  }>;
  action<TActionId extends keyof TActions & string>(actionId: TActionId): TActions[TActionId];
  actions(): TActions;
  bindForm(form: LocalDialogBindableForm, options?: LocalDialogFormBindingOptions): void;
  collaboration(): LocalDialogCollaborationReport<TMode>;
  reportCollaboration(
    artifact: LocalDialogCollaborationReportInput<TMode>,
  ): LocalDialogCollaborationArtifact<TMode>;
  clearCollaboration(options?: { readonly reason?: string }): LocalDialogCollaborationArtifact<TMode>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalListStateOptions<TItem = SignalValue> {
  readonly identity: string;
  readonly initial: readonly TItem[] | TItem[];
  readonly aspects?: ReadonlyArray<AspectId>;
  readonly debugName?: string;
}

export interface LocalListState<TItem = SignalValue> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly items: InputSignalHandle<readonly TItem[] | TItem[]>;
  reset(): RunSummary | Promise<RunSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalFormSourceStateOptions<TValue = SignalValue> {
  readonly identity: string;
  readonly initial: TValue;
  readonly debugName?: string;
  readonly sourceId?: string;
  readonly contract?: string;
}

export interface LocalFormSourceState<TValue = SignalValue> {
  readonly scope: ScopedSignalNamespace;
  readonly scopeId: string;
  readonly signal: InputSignalHandle<TValue>;
  readonly source: FormSourceDeclaration<TValue>;
  reset(): RunSummary | Promise<RunSummary>;
  free(): void;
  [Symbol.dispose](): void;
}

export interface LocalNamespace {
  dialogState<
    TMode extends string = string,
    TData = SignalValue,
    TContext = SignalValue,
    TCustomActions extends Record<string, LocalDialogCustomActionDefinition<TMode, TData, TContext>> = Record<
      string,
      LocalDialogCustomActionDefinition<TMode, TData, TContext>
    >,
  >(
    options: LocalDialogStateOptions<TMode, TData, TContext, TCustomActions>,
  ): LocalDialogState<
    TMode,
    TData,
    TContext,
    Readonly<Record<"close" | "confirm" | "discard" | keyof TCustomActions, LocalDialogActionBinding>>
  >;
  listState<TItem = SignalValue>(options: LocalListStateOptions<TItem>): LocalListState<TItem>;
  formSource<TValue = SignalValue>(
    options: LocalFormSourceStateOptions<TValue>,
  ): LocalFormSourceState<TValue>;
}
