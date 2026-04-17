export type SignalValue =
  | null
  | boolean
  | number
  | string
  | SignalValue[]
  | { [key: string]: SignalValue };

export type RecipeReadSpec =
  | string
  | {
      id: string;
      scope?: unknown;
    };

export type Expr =
  | { kind: "value"; value: SignalValue }
  | { kind: "read"; id: string }
  | { kind: "get"; target: Expr; field: string }
  | { kind: "at"; target: Expr; index: Expr }
  | { kind: "first"; target: Expr }
  | { kind: "last"; target: Expr }
  | { kind: "slice"; target: Expr; start: Expr; end?: Expr }
  | { kind: "join"; target: Expr; separator: Expr }
  | { kind: "flatten"; target: Expr }
  | { kind: "object"; fields: ReadonlyArray<readonly [string, Expr]> }
  | { kind: "array"; items: ReadonlyArray<Expr> }
  | { kind: "sum"; args: ReadonlyArray<Expr> }
  | { kind: "multiply"; args: ReadonlyArray<Expr> }
  | { kind: "concat"; args: ReadonlyArray<Expr> }
  | { kind: "coalesce"; args: ReadonlyArray<Expr> }
  | { kind: "length"; target: Expr }
  | { kind: "contains"; target: Expr; value: Expr }
  | { kind: "mergeObjects"; args: ReadonlyArray<Expr> }
  | { kind: "keys"; target: Expr }
  | { kind: "values"; target: Expr }
  | { kind: "hasField"; target: Expr; field: string }
  | { kind: "pick"; target: Expr; fields: ReadonlyArray<string> }
  | { kind: "omit"; target: Expr; fields: ReadonlyArray<string> }
  | { kind: "append"; target: Expr; value: Expr }
  | { kind: "abs"; target: Expr }
  | { kind: "min"; args: ReadonlyArray<Expr> }
  | { kind: "max"; args: ReadonlyArray<Expr> }
  | { kind: "sqrt"; target: Expr }
  | { kind: "sin"; target: Expr }
  | { kind: "cos"; target: Expr }
  | { kind: "floor"; target: Expr }
  | { kind: "mod"; left: Expr; right: Expr }
  | { kind: "clamp"; value: Expr; min: Expr; max: Expr }
  | { kind: "atan2"; y: Expr; x: Expr }
  | { kind: "subtract"; left: Expr; right: Expr }
  | { kind: "divide"; left: Expr; right: Expr }
  | { kind: "eq"; left: Expr; right: Expr }
  | { kind: "neq"; left: Expr; right: Expr }
  | { kind: "gt"; left: Expr; right: Expr }
  | { kind: "gte"; left: Expr; right: Expr }
  | { kind: "lt"; left: Expr; right: Expr }
  | { kind: "lte"; left: Expr; right: Expr }
  | { kind: "and"; args: ReadonlyArray<Expr> }
  | { kind: "or"; args: ReadonlyArray<Expr> }
  | { kind: "not"; arg: Expr }
  | { kind: "if"; condition: Expr; thenExpr: Expr; elseExpr: Expr };

export interface ConditionSpec {
  expr: Expr;
}

export type IdentitySpec =
  | { kind: "exact" }
  | { kind: "expr"; expr: Expr };

export interface ComputedSpec {
  reads?: ReadonlyArray<RecipeReadSpec>;
  expr: Expr;
  when?: ConditionSpec;
  identity?: IdentitySpec;
}

export interface OutputSpec {
  reads?: ReadonlyArray<RecipeReadSpec>;
  expr: Expr;
  when?: ConditionSpec;
  identity?: IdentitySpec;
}

export interface RunSummary {
  touchedNodes: number;
  nodesEvaluated: number;
  nodesRecomputed: number;
  nodesSuppressed: number;
  plansBuilt: number;
  stagesExecuted: number;
  totalNanos: string;
  evaluationNanos: string;
  commitNanos: string;
}

export interface HealthSummary {
  activeNodeCount: number;
  cleanNodeCount: number;
  maybeStaleNodeCount: number;
  dirtyNodeCount: number;
  dependencyEdgeCount: number;
  subscriberEdgeCount: number;
}

export interface WhySummary {
  id: string;
  node: string;
  state: string;
  upstream: ReadonlyArray<string>;
  changedRegions: ReadonlyArray<string>;
  propagationSuppressed: boolean;
  outputChange: string | null;
  outputIdentity: string | null;
}

export interface WebObservationNotice {
  observerId: number;
  handleId: number;
  signalId: string;
  branchId: number;
  policy: unknown;
  touched: boolean;
  recomputed: boolean;
  meaningfulChange: boolean;
  triggerMatched: boolean;
}

export interface ObservationBoundaryEventSummary {
  observerId: number;
  handleId: number;
  matchedNodes: ReadonlyArray<string>;
  touched: boolean;
  recomputed: boolean;
  meaningfulChange: boolean;
  triggerMatched: boolean;
}

export interface ObservationBoundarySummary {
  branchId: number;
  deliveredEventCount: number;
  rollbackSuppressedEventCount: number;
  boundaryEvents: ReadonlyArray<ObservationBoundaryEventSummary>;
}

export interface WebPerformanceSummary {
  activeHandleCount: number;
  activeCallbackCount: number;
  matchedWatcherBreadth: number;
  deliveredObservationCount: number;
  rollbackSuppressedDeliveryCount: number;
  serialExecutorUsageCount: number;
  parallelExecutorUsageCount: number;
  outputSerializationCount: number;
  outputSerializationBreadth: number;
  jsCallbackInvocationCount: number;
  jsCallbackFailureCount: number;
  compatibilityReadCount: number;
  compatibilityReadBreadth: number;
}

export type SignalTarget = string | InputSignal | ComputedSignal | OutputSignal;

export class InputSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  readonly id: string;
}

export class ComputedSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  readonly id: string;
}

export class OutputSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  readonly id: string;
}

export class DisposableHandle {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
}

export class SignalsTransaction {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  set(input: InputSignal, value: SignalValue): void;
  setWithRegions(input: InputSignal, value: SignalValue, changedRegions: unknown): void;
}

export class Signals {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  input(id: string, initial: SignalValue): InputSignal;
  computed(id: string, spec: ComputedSpec): ComputedSignal;
  output(id: string, spec: OutputSpec): OutputSignal;
  transaction(callback: (tx: SignalsTransaction) => void): RunSummary;
  batch(callback: (tx: SignalsTransaction) => void): RunSummary;
  watch(
    target: SignalTarget,
    callback: (notice: WebObservationNotice) => void,
  ): DisposableHandle;
  effect(target: SignalTarget, callback: () => void): DisposableHandle;
  nuke(handle: DisposableHandle): boolean;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
  compatibilityApp(): SignalApp;
  compatibilityRuntime(): SignalRuntime;
}

export class SignalDiagnostics {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  why(id: string): WhySummary;
  health(): HealthSummary;
  summaryNow(): unknown;
  historyNow(): unknown;
  latestFlow(): unknown | null;
  latestObservation(): ObservationBoundarySummary | null;
  performanceSummary(): WebPerformanceSummary;
  latestFailure(): unknown | null;
  latestRollback(): unknown | null;
  latestFrontierExecution(): unknown | null;
  latestInvalidationTraceRecords(): unknown;
  recentHistory(): unknown;
}

export class SignalHistory {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  replay_for(id: string): unknown;
  lineage_for(id: string): unknown;
  snapshot(): unknown;
  restore_snapshot(snapshot: unknown): void;
  current_branch(): unknown;
  branches(): unknown;
  create_branch(name: string): unknown;
  switch_branch(branchId: bigint): void;
  replay_for_branch(branchId: bigint): unknown;
  branch_snapshot(branchId: bigint): unknown;
  branch_snapshot_id(branchId: bigint): bigint;
  branch_snapshot_envelope(branchId: bigint): unknown;
  restore_branch_snapshot(branchId: bigint, snapshot: unknown): void;
  restore_branch_snapshot_by_id(branchId: bigint, snapshotId: bigint): void;
  merge_branches(sourceBranchId: bigint, targetBranchId: bigint): unknown;
  merge_branches_with_proof(sourceBranchId: bigint, targetBranchId: bigint): unknown;
  plan_merge_branches(sourceBranchId: bigint, targetBranchId: bigint): unknown;
  plan_merge_branches_with_proof(sourceBranchId: bigint, targetBranchId: bigint): unknown;
  plan_merge_policy_preview(request: unknown): unknown;
  plan_merge_policy_preview_with_proof(request: unknown): unknown;
  merge_branches_policy_preview(request: unknown): unknown;
  merge_branches_policy_preview_with_proof(request: unknown): unknown;
  branch_state_proof(branchId: bigint): unknown;
  replay_parity_proof(expectedBranchId: bigint, replayedBranchId: bigint): unknown;
  replay_artifact_proof(expected: unknown, replayedBranchId: bigint): unknown;
}

export class SignalSpecialist {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  evaluate_dirty(): unknown;
  graph_summary(): unknown;
  read_versions(ids: ReadonlyArray<string>): unknown;
}

export class SignalAdapters {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  export_definitions(): unknown;
  export_runtime_envelope(): unknown;
  replace_runtime_envelope(envelope: unknown): void;
  runtime_proof_report(): unknown;
}

export class SignalApp {
  constructor();
  free(): void;
  [Symbol.dispose](): void;
  source(spec: unknown): void;
  recipe(spec: unknown): void;
  source_family(spec: unknown): void;
  recipe_family(spec: unknown): void;
  batch(ops: unknown): RunSummary;
  transaction_with_packed_grid_rgba(
    prefixOps: unknown,
    familyId: string,
    width: number,
    height: number,
    rgba: Uint8Array,
    suffixOps: unknown,
  ): RunSummary;
  read(id: string): SignalValue;
  read_many(ids: ReadonlyArray<string>): ReadonlyArray<SignalValue>;
  read_keyed(familyId: string, key: string): SignalValue;
  set_keyed(familyId: string, key: string, value: SignalValue): RunSummary;
  read_keyed_many(familyId: string, keys: ReadonlyArray<string>): ReadonlyArray<SignalValue>;
  read_keyed_many_packed_fields(
    familyId: string,
    keys: ReadonlyArray<string>,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  read_keyed_grid_packed_fields(
    familyId: string,
    columns: number,
    rows: number,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  read_keyed_rect_packed_fields(
    familyId: string,
    columns: number,
    rows: number,
    row: number,
    startColumn: number,
    width: number,
    height: number,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  prewarm_keyed_grid(familyId: string, columns: number, rows: number): void;
  seed_keyed_grid_coords(familyId: string, columns: number, rows: number): void;
  take_debug_events(): ReadonlyArray<string>;
  set_keyed_many(familyId: string, values: unknown): RunSummary;
  mark_changed_with_regions(id: string, changedRegions: unknown): RunSummary;
  mark_keyed_changed_with_regions(
    familyId: string,
    key: string,
    changedRegions: unknown,
  ): RunSummary;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export class SignalRuntime {
  constructor();
  free(): void;
  [Symbol.dispose](): void;
  define_source(spec: unknown): void;
  define_recipe(spec: unknown): void;
  define_source_family(spec: unknown): void;
  define_recipe_family(spec: unknown): void;
  read(id: string): SignalValue;
  read_many(ids: ReadonlyArray<string>): ReadonlyArray<SignalValue>;
  read_keyed(familyId: string, key: string): SignalValue;
  read_keyed_many(familyId: string, keys: ReadonlyArray<string>): ReadonlyArray<SignalValue>;
  read_keyed_many_packed_fields(
    familyId: string,
    keys: ReadonlyArray<string>,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  read_keyed_grid_packed_fields(
    familyId: string,
    columns: number,
    rows: number,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  read_keyed_rect_packed_fields(
    familyId: string,
    columns: number,
    rows: number,
    row: number,
    startColumn: number,
    width: number,
    height: number,
    fields: ReadonlyArray<string>,
  ): Uint8Array;
  prewarm_keyed_grid(familyId: string, columns: number, rows: number): void;
  seed_keyed_grid_coords(familyId: string, columns: number, rows: number): void;
  set_keyed(familyId: string, key: string, value: SignalValue): RunSummary;
  set_keyed_many(familyId: string, values: unknown): RunSummary;
  clear_keyed_family_cache(familyId: string): void;
  mark_changed_with_regions(id: string, changedRegions: unknown): RunSummary;
  mark_keyed_changed_with_regions(
    familyId: string,
    key: string,
    changedRegions: unknown,
  ): RunSummary;
  set_runtime_policy(policy: unknown): void;
  transaction(ops: unknown): RunSummary;
  transaction_with_packed_grid_rgba(
    prefixOps: unknown,
    familyId: string,
    width: number,
    height: number,
    rgba: Uint8Array,
    suffixOps: unknown,
  ): RunSummary;
  take_debug_events(): ReadonlyArray<string>;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export function createSignals(): Signals;

export function start(): void;
