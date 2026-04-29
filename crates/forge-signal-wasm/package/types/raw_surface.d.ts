import type {
  AspectId,
  ComputedSpec,
  InputOptions,
  KeyedSetValue,
  KeyedRecipeFamilySpec,
  KeyedSourceFamilySpec,
  OutputSpec,
  RecipeSpec,
  RunSummary,
  SignalValue,
  SourceSpec,
  TransactionOp,
  VersionSummary,
  WebObservationNotice,
} from "./model.js";
import type {
  HealthSummary,
  ObservationBoundarySummary,
  RuntimeDefinitionEnvelope,
  WebPerformanceSummary,
  WhySummary,
} from "./diagnostics.js";

export class InputSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  peek(): SignalValue;
  readonly id: string;
}

export class ComputedSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  peek(): SignalValue;
  readonly id: string;
}

export class OutputSignal {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  get(): SignalValue;
  peek(): SignalValue;
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
  setWithAspects(input: InputSignal, value: SignalValue, aspects: ReadonlyArray<AspectId>): void;
  setWithRegions(input: InputSignal, value: SignalValue, changedRegions: unknown): void;
  setWithRegionsAndAspects(
    input: InputSignal,
    value: SignalValue,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): void;
}

export type SignalTarget = string | InputSignal | ComputedSignal | OutputSignal;

export class Signals {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  input(id: string, initial: SignalValue, options?: InputOptions): InputSignal;
  computedSpec(id: string, spec: ComputedSpec): ComputedSignal;
  computedCallback(id: string, callback: () => SignalValue): ComputedSignal;
  computed(id: string, spec: ComputedSpec): ComputedSignal;
  outputSpec(id: string, spec: OutputSpec): OutputSignal;
  outputCallback(id: string, callback: () => SignalValue): never;
  output(id: string, spec: OutputSpec): OutputSignal;
  transaction(callback: (tx: SignalsTransaction) => void): RunSummary;
  batch(callback: (tx: SignalsTransaction) => void): RunSummary;
  watch(target: SignalTarget, callback: (notice: WebObservationNotice) => void): DisposableHandle;
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
  read_versions(ids: ReadonlyArray<string>): ReadonlyArray<VersionSummary>;
}

export class SignalAdapters {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  export_definitions(): RuntimeDefinitionEnvelope;
  export_runtime_envelope(): never;
  replace_runtime_envelope(envelope: unknown): never;
  runtime_proof_report(): unknown;
}

export class SignalApp {
  constructor();
  free(): void;
  [Symbol.dispose](): void;
  source(spec: SourceSpec): void;
  recipe(spec: RecipeSpec): void;
  source_family(spec: KeyedSourceFamilySpec): void;
  recipe_family(spec: KeyedRecipeFamilySpec): void;
  batch(ops: ReadonlyArray<TransactionOp>): RunSummary;
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
  setKeyedWithAspects(
    familyId: string,
    key: string,
    value: SignalValue,
    aspects: ReadonlyArray<AspectId>,
  ): RunSummary;
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
  set_keyed_many(familyId: string, values: ReadonlyArray<KeyedSetValue>): RunSummary;
  mark_changed_with_regions(id: string, changedRegions: unknown): RunSummary;
  markChanged(id: string, aspects: ReadonlyArray<AspectId>): RunSummary;
  markChangedWithRegionsAndAspects(
    id: string,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): RunSummary;
  mark_keyed_changed_with_regions(
    familyId: string,
    key: string,
    changedRegions: unknown,
  ): RunSummary;
  markKeyedChanged(familyId: string, key: string, aspects: ReadonlyArray<AspectId>): RunSummary;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export class SignalRuntime {
  constructor();
  free(): void;
  [Symbol.dispose](): void;
  define_source(spec: SourceSpec): void;
  define_recipe(spec: RecipeSpec): void;
  define_source_family(spec: KeyedSourceFamilySpec): void;
  define_recipe_family(spec: KeyedRecipeFamilySpec): void;
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
  setKeyedWithAspects(
    familyId: string,
    key: string,
    value: SignalValue,
    aspects: ReadonlyArray<AspectId>,
  ): RunSummary;
  set_keyed_many(familyId: string, values: ReadonlyArray<KeyedSetValue>): RunSummary;
  clear_keyed_family_cache(familyId: string): void;
  mark_changed_with_regions(id: string, changedRegions: unknown): RunSummary;
  markChanged(id: string, aspects: ReadonlyArray<AspectId>): RunSummary;
  markChangedWithRegionsAndAspects(
    id: string,
    changedRegions: unknown,
    aspects: ReadonlyArray<AspectId>,
  ): RunSummary;
  mark_keyed_changed_with_regions(
    familyId: string,
    key: string,
    changedRegions: unknown,
  ): RunSummary;
  markKeyedChanged(familyId: string, key: string, aspects: ReadonlyArray<AspectId>): RunSummary;
  set_runtime_policy(policy: unknown): void;
  transaction(ops: ReadonlyArray<TransactionOp>): RunSummary;
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
