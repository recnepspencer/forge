import type {
  BranchStateProofReport,
  BranchMergePlan,
  BranchMergeResult,
  DiagnosticsExecutionHistorySummary,
  DiagnosticsFailureSummary,
  DiagnosticsFlowSummary,
  DiagnosticsFrontierExecutionSummary,
  DiagnosticsGraphSummary,
  DiagnosticsInvalidationTraceRecord,
  DiagnosticsRollbackDiagnostic,
  HealthSummary,
  KeyedRecipeFamilySpec,
  KeyedSourceFamilySpec,
  LineageSummary,
  MergePlanReport,
  MergePlanProofEnvelope,
  ReplayArtifactProofInput,
  ReplayArtifactProofReport,
  ReplayParityProofReport,
  MergeResultReport,
  MergeResultProofEnvelope,
  ReplaySummary,
  RecipeSpec,
  RunSummary,
  RuntimeBranch,
  RuntimeDefinitionEnvelope,
  RuntimeEnvelope,
  RuntimePolicy,
  RuntimeSnapshotEnvelope,
  SignalValue,
  SourceSpec,
  VersionSummary,
  WhySummary
} from "./types";
import type {
  RecipeBuilder,
  RecipeFamilyBuilder,
  SourceBuilder,
  SourceFamilyBuilder
} from "./builders";

export class SignalHandle<T = SignalValue> {
  readonly id: string;
  read(): T;
  subscribe(listener: (value: T) => void, options?: { emitCurrent?: boolean }): () => void;
  why(): WhySummary;
}

export class SourceHandle<T = SignalValue> extends SignalHandle<T> {
  set(value: T): RunSummary;
}

export class RecipeHandle<T = SignalValue> extends SignalHandle<T> {}

export class KeyedSourceHandle<T = SignalValue> {
  readonly familyId: string;
  readonly key: string;
  readonly id: string;
  read(): T;
  subscribe(listener: (value: T) => void, options?: { emitCurrent?: boolean }): () => void;
  set(value: T): RunSummary;
  why(): WhySummary;
}

export class KeyedRecipeHandle<T = SignalValue> {
  readonly familyId: string;
  readonly key: string;
  readonly id: string;
  read(): T;
  subscribe(listener: (value: T) => void, options?: { emitCurrent?: boolean }): () => void;
  why(): WhySummary;
}

export class SourceFamilyHandle<T = SignalValue> {
  readonly familyId: string;
  toRead(): { familyId: string };
  key(key: string): KeyedSourceHandle<T>;
  read(key: string): T;
  set(key: string, value: T): RunSummary;
}

export class RecipeFamilyHandle<T = SignalValue> {
  readonly familyId: string;
  toRead(): { familyId: string };
  key(key: string): KeyedRecipeHandle<T>;
  read(key: string): T;
}

export class SignalApp {
  source<T = SignalValue>(spec: SourceSpec<T> | SourceBuilder<T>): SourceHandle<T>;
  recipe<T = SignalValue>(spec: RecipeSpec<T> | RecipeBuilder<T>): RecipeHandle<T>;
  sourceFamily<T = SignalValue>(
    spec: KeyedSourceFamilySpec<T> | SourceFamilyBuilder<T>
  ): SourceFamilyHandle<T>;
  recipeFamily<T = SignalValue>(
    spec: KeyedRecipeFamilySpec<T> | RecipeFamilyBuilder<T>
  ): RecipeFamilyHandle<T>;
  batch<T = SignalValue>(ops: Array<TransactionOp<T>>): RunSummary;
  handle<T = SignalValue>(id: string): SignalHandle<T>;
  read<T = SignalValue>(id: string): T;
  readKeyed<T = SignalValue>(familyId: string, key: string): T;
  setKeyed<T = SignalValue>(familyId: string, key: string, value: T): RunSummary;
  readKeyedMany<T = SignalValue>(familyId: string, keys: string[]): T[];
  setKeyedMany<T = SignalValue>(familyId: string, values: Array<{ key: string; value: T }>): RunSummary;
  subscribe(listener: () => void): () => void;
  watch<T = SignalValue>(
    id: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean }
  ): () => void;
  watchKeyed<T = SignalValue>(
    familyId: string,
    key: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean }
  ): () => void;
  clearKeyedFamilyCache(familyId: string): void;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export class SignalRuntime {
  setRuntimePolicy(policy: RuntimePolicy): this;
  defineSource<T = SignalValue>(spec: SourceSpec<T> | SourceBuilder<T>): SourceHandle<T>;
  defineRecipe<T = SignalValue>(spec: RecipeSpec<T> | RecipeBuilder<T>): RecipeHandle<T>;
  defineSourceFamily<T = SignalValue>(
    spec: KeyedSourceFamilySpec<T> | SourceFamilyBuilder<T>
  ): SourceFamilyHandle<T>;
  defineRecipeFamily<T = SignalValue>(
    spec: KeyedRecipeFamilySpec<T> | RecipeFamilyBuilder<T>
  ): RecipeFamilyHandle<T>;
  transaction<T = SignalValue>(ops: Array<TransactionOp<T>>): RunSummary;
  handle<T = SignalValue>(id: string): SignalHandle<T>;
  read<T = SignalValue>(id: string): T;
  readKeyed<T = SignalValue>(familyId: string, key: string): T;
  setKeyed<T = SignalValue>(familyId: string, key: string, value: T): RunSummary;
  readKeyedMany<T = SignalValue>(familyId: string, keys: string[]): T[];
  setKeyedMany<T = SignalValue>(familyId: string, values: Array<{ key: string; value: T }>): RunSummary;
  subscribe(listener: () => void): () => void;
  watch<T = SignalValue>(
    id: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean }
  ): () => void;
  watchKeyed<T = SignalValue>(
    familyId: string,
    key: string,
    listener: (value: T) => void,
    options?: { emitCurrent?: boolean }
  ): () => void;
  clearKeyedFamilyCache(familyId: string): void;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export class SignalDiagnostics {
  why(id: string): WhySummary;
  health(): HealthSummary;
  summaryNow(): DiagnosticsGraphSummary;
  historyNow(): DiagnosticsExecutionHistorySummary;
  latestFlow(): DiagnosticsFlowSummary | null;
  latestFailure(): DiagnosticsFailureSummary | null;
  latestRollback(): DiagnosticsRollbackDiagnostic | null;
  latestFrontierExecution(): DiagnosticsFrontierExecutionSummary | null;
  latestInvalidationTraceRecords(): DiagnosticsInvalidationTraceRecord[];
  recentHistory(): DiagnosticsExecutionHistorySummary[];
}

export class SignalHistory {
  snapshotOpaque(): unknown;
  restoreSnapshotOpaque(snapshot: unknown): void;
  replayFor(id: string): ReplaySummary;
  lineageFor(id: string): LineageSummary;
  snapshot(): RuntimeSnapshotEnvelope;
  restoreSnapshot(snapshot: RuntimeSnapshotEnvelope): void;
  currentBranch(): RuntimeBranch;
  branches(): RuntimeBranch[];
  createBranch(name: string): RuntimeBranch;
  switchBranch(branchId: number): void;
  replayForBranch(branchId: number): ReplaySummary;
  branchSnapshot(branchId: number): unknown;
  branchSnapshotId(branchId: number): number | bigint;
  branchSnapshotEnvelope(branchId: number): RuntimeSnapshotEnvelope;
  branchSnapshotEnvelopeOpaque(branchId: number): unknown;
  restoreBranchSnapshotOpaque(branchId: number, snapshot: unknown): void;
  restoreBranchSnapshotById(branchId: number, snapshotId: number | bigint): void;
  planMergeBranches(sourceBranchId: number, targetBranchId: number): MergePlanReport;
  planMergeBranchesDetailed(sourceBranchId: number, targetBranchId: number): BranchMergePlan;
  planMergeBranchesDetailedWithProof(sourceBranchId: number, targetBranchId: number): MergePlanProofEnvelope;
  planMergePolicyPreview(request: MergePolicyPreviewRequest): MergePlanReport;
  planMergePolicyPreviewDetailed(request: MergePolicyPreviewRequest): BranchMergePlan;
  planMergePolicyPreviewDetailedWithProof(request: MergePolicyPreviewRequest): MergePlanProofEnvelope;
  mergeBranchesPolicyPreview(request: MergePolicyPreviewRequest): MergeResultReport;
  mergeBranchesPolicyPreviewDetailed(request: MergePolicyPreviewRequest): BranchMergeResult;
  mergeBranchesPolicyPreviewDetailedWithProof(request: MergePolicyPreviewRequest): MergeResultProofEnvelope;
  mergeBranches(sourceBranchId: number, targetBranchId: number): MergeResultReport;
  mergeBranchesDetailed(sourceBranchId: number, targetBranchId: number): BranchMergeResult;
  mergeBranchesDetailedWithProof(sourceBranchId: number, targetBranchId: number): MergeResultProofEnvelope;
  branchStateProof(branchId: number): BranchStateProofReport;
  replayParityProof(expectedBranchId: number, replayedBranchId: number): ReplayParityProofReport;
  replayArtifactProof(expected: ReplayArtifactProofInput, replayedBranchId: number): ReplayArtifactProofReport;
}

export class SignalSpecialist {
  graphSummary(): DiagnosticsGraphSummary;
  evaluateDirty(): RunSummary;
  readVersions(ids: string[]): VersionSummary[];
}

export class SignalAdapters {
  exportDefinitions(): RuntimeDefinitionEnvelope;
  exportRuntimeEnvelope(): RuntimeEnvelope;
  runtimeProofReport(): RuntimeProofReport;
  replaceRuntimeEnvelope(envelope: RuntimeEnvelope): void;
}
