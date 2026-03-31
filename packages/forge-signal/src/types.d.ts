export type SignalPrimitive = string | number | boolean | null;
export type SignalValue =
  | SignalPrimitive
  | SignalValue[]
  | { [key: string]: SignalValue };

export type ExprInput<T = SignalValue> = Expr<T> | T;

export type Expr<T = SignalValue> =
  | { kind: "value"; value: T }
  | { kind: "read"; id: string }
  | { kind: "get"; target: Expr<Record<string, SignalValue>>; field: string }
  | { kind: "at"; target: Expr<SignalValue[]>; index: ExprInput<number> }
  | { kind: "first"; target: Expr<SignalValue[]> }
  | { kind: "last"; target: Expr<SignalValue[]> }
  | { kind: "slice"; target: Expr<SignalValue[]>; start: ExprInput<number>; end?: ExprInput<number> }
  | { kind: "join"; target: Expr<SignalValue[]>; separator: ExprInput<string> }
  | { kind: "flatten"; target: Expr<SignalValue[][]> }
  | { kind: "object"; fields: Record<string, ExprInput<SignalValue>> | Array<[string, ExprInput<SignalValue>]> }
  | { kind: "array"; items: Array<ExprInput<SignalValue>> }
  | { kind: "sum"; args: Array<ExprInput<number>> }
  | { kind: "multiply"; args: Array<ExprInput<number>> }
  | { kind: "concat"; args: Array<ExprInput<SignalPrimitive>> }
  | { kind: "coalesce"; args: Array<ExprInput<SignalValue>> }
  | { kind: "length"; target: ExprInput<SignalValue> }
  | { kind: "contains"; target: ExprInput<SignalValue>; value: ExprInput<SignalValue> }
  | { kind: "mergeObjects"; args: Array<ExprInput<Record<string, SignalValue>>> }
  | { kind: "keys"; target: Expr<Record<string, SignalValue>> }
  | { kind: "values"; target: Expr<Record<string, SignalValue>> }
  | { kind: "hasField"; target: Expr<Record<string, SignalValue>>; field: string }
  | { kind: "pick"; target: Expr<Record<string, SignalValue>>; fields: string[] }
  | { kind: "omit"; target: Expr<Record<string, SignalValue>>; fields: string[] }
  | { kind: "append"; target: Expr<SignalValue[]>; value: ExprInput<SignalValue> }
  | { kind: "abs"; target: ExprInput<number> }
  | { kind: "min"; args: Array<ExprInput<number>> }
  | { kind: "max"; args: Array<ExprInput<number>> }
  | { kind: "sqrt"; target: ExprInput<number> }
  | { kind: "sin"; target: ExprInput<number> }
  | { kind: "cos"; target: ExprInput<number> }
  | { kind: "floor"; target: ExprInput<number> }
  | { kind: "mod"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "clamp"; value: ExprInput<number>; min: ExprInput<number>; max: ExprInput<number> }
  | { kind: "atan2"; y: ExprInput<number>; x: ExprInput<number> }
  | { kind: "subtract"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "divide"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "eq"; left: ExprInput<SignalValue>; right: ExprInput<SignalValue> }
  | { kind: "neq"; left: ExprInput<SignalValue>; right: ExprInput<SignalValue> }
  | { kind: "gt"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "gte"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "lt"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "lte"; left: ExprInput<number>; right: ExprInput<number> }
  | { kind: "and"; args: Array<ExprInput<boolean>> }
  | { kind: "or"; args: Array<ExprInput<boolean>> }
  | { kind: "not"; arg: ExprInput<boolean> }
  | { kind: "if"; condition: ExprInput<boolean>; thenExpr: ExprInput<SignalValue>; elseExpr: ExprInput<SignalValue> };

export type ConditionSpec = { expr: Expr<boolean> | ExprInput<boolean> };
export type IdentitySpec = { kind: "exact" } | { kind: "expr"; expr: ExprInput<SignalValue> };

export type SourceSpec<T = SignalValue> = {
  id: string;
  initial?: T;
};

export type RecipeSpec<T = SignalValue> = {
  id: string;
  reads?: string[];
  expr: Expr<T>;
  when?: ConditionSpec | null;
  identity?: IdentitySpec | null;
};

export type KeyedSourceFamilySpec<T = SignalValue> = {
  familyId: string;
  initial?: T;
};

export type RecipeFamilyReadSpec =
  | { kind: "signal"; id: string }
  | { kind: "keyed"; familyId: string };

export type KeyedRecipeFamilySpec<T = SignalValue> = {
  familyId: string;
  reads?: RecipeFamilyReadSpec[];
  expr: Expr<T>;
  when?: ConditionSpec | null;
  identity?: IdentitySpec | null;
};

export type TransactionOp<T = SignalValue> =
  | { kind: "set"; id: string; value: T }
  | { kind: "setMany"; values: Array<{ id: string; value: T }> }
  | { kind: "setManyKeyed"; familyId: string; values: Array<{ key: string; value: T }> }
  | { kind: "setPackedGridRgba"; familyId: string; width: number; height: number; rgba: Uint8ClampedArray | Uint8Array };

export type RuntimePolicyPreset =
  | "development"
  | "operational"
  | "forensic"
  | "webDevelopment"
  | "fintech"
  | "kernel"
  | "gameEngine";

export type RuntimePolicy = {
  preset: RuntimePolicyPreset;
};

export type RunSummary = {
  touchedNodes: number;
  nodesEvaluated: number;
  nodesRecomputed: number;
  nodesSuppressed: number;
  plansBuilt: number;
  stagesExecuted: number;
  totalNanos: string;
  evaluationNanos: string;
  commitNanos: string;
};

export type WhySummary = {
  id: string;
  node: string;
  state: string;
  upstream: string[];
  changedRegions: string[];
  propagationSuppressed: boolean;
  outputChange?: string | null;
  outputIdentity?: string | null;
};

export type HealthSummary = {
  activeNodeCount: number;
  cleanNodeCount: number;
  maybeStaleNodeCount: number;
  dirtyNodeCount: number;
  dependencyEdgeCount: number;
  subscriberEdgeCount: number;
};

export type ReplayFrameSummary = {
  cursor: number;
  kind: string;
  branchId: number;
  snapshotId?: number | null;
  node?: string | null;
  detail?: string | null;
};

export type ReplaySummary = {
  frames: ReplayFrameSummary[];
};

export type LineageEventSummary = {
  sequence: number;
  label: string;
  emittedOnBranchId: number;
  node?: string | null;
  subjectArtifactId?: number | null;
  parentArtifactId?: number | null;
  snapshotId?: number | null;
};

export type LineageSummary = {
  events: LineageEventSummary[];
};

export type RuntimeBranch = {
  id: number;
  name: string;
  parentBranchId?: number | null;
  headSnapshotId?: number | null;
};

export type VersionSummary = {
  id: string;
  version: number;
};

export type RuntimeDefinitionEnvelope = {
  policy: RuntimePolicy;
  sources: SourceSpec[];
  recipes: RecipeSpec[];
  sourceFamilies: KeyedSourceFamilySpec[];
  recipeFamilies: KeyedRecipeFamilySpec[];
};

export type RuntimeSnapshotEnvelope = {
  snapshot: unknown;
  state: {
    sources: Array<{ id: string; value: SignalValue; version: number }>;
    recipes: Array<{
      id: string;
      value: SignalValue;
      version: number;
      initialized: boolean;
      outputIdentity?: string | null;
    }>;
  };
};

export type RuntimeEnvelope = {
  definitions: RuntimeDefinitionEnvelope;
  snapshot: RuntimeSnapshotEnvelope;
};

export type MergePlanReport = {
  sourceBranchId: number | null;
  targetBranchId: number | null;
  mergeKind: string | null;
  divergence: string | null;
  mergeStrategy: string | null;
  sourceSnapshotId: number | null;
  targetSnapshotIdBefore: number | null;
  candidateCount: number;
  sharedNodeCount: number;
  expandedNodeCount: number;
  supportNodeCount: number;
  nodePlanCount: number;
  adoptionCount: number;
  hasResolutionPlan: boolean;
};

export type MergeResultReport = {
  sourceBranchId: number | null;
  targetBranchId: number | null;
  mergeKind: string | null;
  divergence: string | null;
  mergeStrategy: string | null;
  mergedSnapshotId: number | null;
  targetSnapshotIdBefore: number | null;
  targetSnapshotIdAfter: number | null;
  sourceSnapshotId: number | null;
  recordCount: number;
  adoptedCount: number;
  introducedCount: number;
  replacedCount: number;
  preservedTargetCount: number;
  equivalentUnchangedCount: number;
  skippedNonAdoptableCount: number;
  conflictCount: number;
  hasResolutionPlan: boolean;
};

export type BranchMergePlan = Record<string, unknown>;
export type BranchMergeResult = Record<string, unknown>;
