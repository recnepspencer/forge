import type {
  BranchMergePlan,
  BranchMergeResult,
  HealthSummary,
  KeyedRecipeFamilySpec,
  KeyedSourceFamilySpec,
  LineageSummary,
  MergePlanReport,
  MergeResultReport,
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
  set(value: T): RunSummary;
  why(): WhySummary;
}

export class KeyedRecipeHandle<T = SignalValue> {
  readonly familyId: string;
  readonly key: string;
  readonly id: string;
  read(): T;
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
  batch<T = SignalValue>(ops: Array<{ kind: "set"; id: string; value: T } | { kind: "setMany"; values: Array<{ id: string; value: T }> }>): RunSummary;
  read<T = SignalValue>(id: string): T;
  readKeyed<T = SignalValue>(familyId: string, key: string): T;
  setKeyed<T = SignalValue>(familyId: string, key: string, value: T): RunSummary;
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
  transaction<T = SignalValue>(ops: Array<{ kind: "set"; id: string; value: T } | { kind: "setMany"; values: Array<{ id: string; value: T }> }>): RunSummary;
  read<T = SignalValue>(id: string): T;
  readKeyed<T = SignalValue>(familyId: string, key: string): T;
  setKeyed<T = SignalValue>(familyId: string, key: string, value: T): RunSummary;
  diagnostics(): SignalDiagnostics;
  history(): SignalHistory;
  specialist(): SignalSpecialist;
  adapters(): SignalAdapters;
}

export class SignalDiagnostics {
  why(id: string): WhySummary;
  health(): HealthSummary;
}

export class SignalHistory {
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
  planMergeBranches(sourceBranchId: number, targetBranchId: number): MergePlanReport;
  planMergeBranchesDetailed(sourceBranchId: number, targetBranchId: number): BranchMergePlan;
  mergeBranches(sourceBranchId: number, targetBranchId: number): MergeResultReport;
  mergeBranchesDetailed(sourceBranchId: number, targetBranchId: number): BranchMergeResult;
}

export class SignalSpecialist {
  graphSummary(): unknown;
  evaluateDirty(): RunSummary;
  readVersions(ids: string[]): VersionSummary[];
}

export class SignalAdapters {
  exportDefinitions(): RuntimeDefinitionEnvelope;
  exportRuntimeEnvelope(): RuntimeEnvelope;
  replaceRuntimeEnvelope(envelope: RuntimeEnvelope): void;
}
