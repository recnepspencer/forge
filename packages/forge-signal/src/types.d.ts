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

export type GraphMetricsSummary = {
  activeNodeCount: number;
  edgeCount: number;
  dirtyNodeCount: number;
  maybeStaleNodeCount: number;
  cleanNodeCount: number;
  tombstoneCount: number;
  dirtyRatio: number;
  maxDependencyFanIn: number;
  maxSubscriberFanOut: number;
  averageDependencyFanIn: number;
  averageSubscriberFanOut: number;
  partitionInternerSize: number;
};

export type DiagnosticsGraphSummary = {
  profile: string;
  activeNodeCount: number;
  arenaCapacity: number;
  tombstoneCount: number;
  cleanNodeCount: number;
  maybeStaleNodeCount: number;
  dirtyNodeCount: number;
  dependencyEdgeCount: number;
  subscriberEdgeCount: number;
  nodesWithPartitionScopes: number;
  nodesWithTraceSummary: number;
  nodesWithExecutionRecord: number;
  nodesWithCausality: number;
  partitionInternerSize: number;
  sampleDirtyNodes: string[];
  sampleNodesWithExecutionRecord: string[];
  metrics: GraphMetricsSummary;
};

export type DiagnosticsExecutionHistoryNodeSummary = {
  node: string;
  executionRecordId?: number | null;
  semanticSegmentId?: number | null;
  outputChange?: string | null;
  memoizedOrigin?: string | null;
  reuseBasis?: string | null;
  reuseOrigin?: string | null;
  persistentCorrespondenceKind?: string | null;
  compositionRegionCount: number;
  reuseCertificationProofCount: number;
  changedPartitionCount: number;
  causalityKind?: string | null;
};

export type DiagnosticsExecutionHistorySummary = {
  profile: string;
  tracedNodeCount: number;
  executionRecordCount: number;
  latestExecutionRecordId?: number | null;
  reuseOriginCounts: Record<string, number>;
  nodes: DiagnosticsExecutionHistoryNodeSummary[];
};

export type DiagnosticsEvaluationPlanSummary = {
  profile: string;
  requestedTargetCount: number;
  stageCount: number;
  taskCount: number;
  maxStageWidth: number;
  contractPrunedCount: number;
  stageWidths: number[];
  directRequestCount: number;
  transitiveTaskCount: number;
  taskReasonCounts: Record<string, number>;
};

export type DiagnosticsExecutionReportSummary = {
  profile: string;
  stageCount: number;
  taskCount: number;
  tasksExecuted: number;
  tasksPruned: number;
  tasksValidatedClean: number;
  tasksDeferredByCondition: number;
  tasksRevertedCleanByCondition: number;
  tasksSatisfiedByMemoization: number;
  tasksWithSuppressedPropagation: number;
  preparedEvaluationsProduced: number;
  preparedEvaluationsApplied: number;
  dependencyCaptureUpdates: number;
  semanticSegmentCount: number;
  taskOutcomeCounts: Record<string, number>;
  stageOutcomeCounts: Record<string, number>;
};

export type DiagnosticsExplanationSummary = {
  profile: string;
  node: string;
  materializationMode: string;
  state: string;
  dirtyAspectCount: number;
  upstreamCount: number;
  changedUpstreamCount: number;
  skippedUpstreamCount: number;
  conditionDeferredCount: number;
  cleanUpstreamCount: number;
  missingSnapshotCount: number;
  dependencyRemovedCount: number;
  conservativeCauseCount: number;
  directScopeCount: number;
  translatedScopeCount: number;
  discardedScopeCount: number;
  insufficientScopeCount: number;
  rewiredDependencyCount: number;
  directCauseKinds: string[];
  scopeProvenanceKinds: string[];
  causeNoteSamples: string[];
  triageClasses: string[];
  propagationSuppressed: boolean;
  contractReadsMask: string | number;
  contractProducesMask: string | number;
  contractPartitionScopeCount: number;
  requiredContext: string;
  executionRecordId?: number | null;
  semanticSegmentId?: number | null;
  outputChange?: string | null;
  memoizedOrigin?: string | null;
  reuseBasis?: string | null;
  reuseOrigin?: string | null;
  reuseCertificationProofCount: number;
  changedRegionCount: number;
  causalityKind?: string | null;
};

export type DiagnosticsChangeInputSummary = {
  changedNodes: string[];
  changedAspects: number[];
  changedRegionCount: number;
  causalityKind?: string | null;
};

export type DiagnosticsInvalidationSummary = {
  invalidatedDirectSubscribers: number;
  maybeStaleDirectSubscribers: number;
  partitionScopedChecks: number;
  narrowedFrontierWidth: number;
  transitiveFrontierWidth: number;
  frontierSeedCount: number;
  frontierGroupCount: number;
  frontierDirectWaveCount: number;
  frontierTransitiveWaveCount: number;
  frontierPartitionMatchCount: number;
  frontierDetailMatchCount: number;
  frontierCycleCheckCandidateCount: number;
  frontierCycleCheckVisitedCount: number;
  frontierTraceRetainedCount: number;
};

export type DiagnosticsFlowCauseSample = {
  node: string;
  causeKinds: string[];
  scopeKinds: string[];
  scopeNotes: string[];
  suspectClasses: string[];
  rewired: boolean;
  conservativeRecompute: boolean;
};

export type DiagnosticsEventSubscriberOutcome = {
  subscriberName: string;
  outcome: string;
  requiresDataIds: string[];
  providesDataIds: string[];
  stagedDataIds: string[];
};

export type DiagnosticsEventEpochSummary = {
  ordinal: number;
  barrier: string;
  emittedEventCount: number;
  subscriberCount: number;
  committedSubscriberCount: number;
  failedSubscriberPosition?: number | null;
  subscriberOutcomes: DiagnosticsEventSubscriberOutcome[];
  outcome: string;
  failureSubscriber?: string | null;
  message?: string | null;
};

export type DiagnosticsPlanningSummary = {
  plan: DiagnosticsEvaluationPlanSummary;
};

export type DiagnosticsPrecomputeSummary = {
  executor?: string | null;
  stageCount: number;
  taskCount: number;
  preparedEvaluationsProduced: number;
  tasksDeferredByCondition: number;
  tasksSatisfiedByMemoization: number;
};

export type DiagnosticsApplySummary = {
  report: DiagnosticsExecutionReportSummary;
  preparedEvaluationsApplied: number;
  dependencyCaptureUpdates: number;
  tasksValidatedClean: number;
  tasksPruned: number;
  tasksWithSuppressedPropagation: number;
};

export type DiagnosticsRollbackSummary = {
  rolledBack: boolean;
  stagedNodePatchCount: number;
  maxTouchedNodesInTxn: number;
  reason?: string | null;
};

export type DiagnosticsFlowSummary = {
  profile: string;
  change: DiagnosticsChangeInputSummary;
  invalidation: DiagnosticsInvalidationSummary;
  planning: DiagnosticsPlanningSummary;
  precompute: DiagnosticsPrecomputeSummary;
  apply: DiagnosticsApplySummary;
  causeSamples: DiagnosticsFlowCauseSample[];
  eventEpochs: DiagnosticsEventEpochSummary[];
  rollback?: DiagnosticsRollbackSummary | null;
  explanation?: DiagnosticsExplanationSummary | null;
};

export type DiagnosticsFailureSummary = {
  profile: string;
  phase: string;
  stageIndex?: number | null;
  node?: string | null;
  executor?: string | null;
  executionRecordId?: number | null;
  hasPlanSummary: boolean;
  rolledBack: boolean;
  stagedNodePatchCount?: number | null;
  maxTouchedNodesInTxn?: number | null;
  eventEpochs: DiagnosticsEventEpochSummary[];
  message: string;
};

export type DiagnosticsRollbackDiagnostic = {
  rolledBack: boolean;
  stagedNodePatchCount: number;
  maxTouchedNodesInTxn: number;
  reason?: string | null;
  eventEpochs: DiagnosticsEventEpochSummary[];
};

export type DiagnosticsPartitionScopeSet = {
  scopes?: string[];
};

export type DiagnosticsDedupedNodeBatch = {
  nodes?: string[];
};

export type DiagnosticsSortedSourceBatch = {
  sources?: string[];
};

export type DiagnosticsTouchedScopeSummary = {
  seedScopes: DiagnosticsPartitionScopeSet;
  inclusionScopes: DiagnosticsPartitionScopeSet;
  transitiveReachedScopes: DiagnosticsPartitionScopeSet;
  directDirtyScopes: DiagnosticsPartitionScopeSet;
  maybeStaleScopes: DiagnosticsPartitionScopeSet;
  touchedNodes: DiagnosticsDedupedNodeBatch;
  touchedSources: DiagnosticsSortedSourceBatch;
};

export type DiagnosticsFrontierExecutionCounters = {
  frontierSeedCount: number;
  frontierGroupCount: number;
  frontierDirectWaveCount: number;
  frontierTransitiveWaveCount: number;
  frontierPartitionScopedCheckCount: number;
  frontierDirectDirtyCount: number;
  frontierMaybeStaleCount: number;
  frontierPartitionMatchCount: number;
  frontierDetailMatchCount: number;
  frontierCycleCheckCandidateCount: number;
  frontierCycleCheckVisitedCount: number;
  frontierTraceRetainedCount: number;
};

export type DiagnosticsFrontierWaveEntrySummary = {
  node: string;
  classification: string;
  inclusionBasis: string;
  narrowedScopes: DiagnosticsPartitionScopeSet;
};

export type DiagnosticsFrontierWaveSummary = {
  waveIndex: number;
  aspect: number;
  entries: DiagnosticsFrontierWaveEntrySummary[];
};

export type DiagnosticsFrontierExecutionSummary = {
  seedCount: number;
  directWaves: DiagnosticsFrontierWaveSummary[];
  transitiveWaves: DiagnosticsFrontierWaveSummary[];
  touchedScopeSummary: DiagnosticsTouchedScopeSummary;
  counters: DiagnosticsFrontierExecutionCounters;
};

export type DiagnosticsInvalidationTraceRecord = {
  node: string;
  aspect: number;
  waveIndex: number;
  classification: string;
  inclusionBasis: string;
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

export type RuntimeProofReport = {
  proofSchemaVersion: string;
  schemaRegistryDigest: string;
  mergeStrategyRegistryDigest: string;
  mergeBaseStrategyRegistryDigest: string;
  aspectMergePolicyRegistryDigest: string;
  conflictIsolationRegistryDigest: string;
  conflictPolicyRegistryDigest: string;
  identityMatcherRegistryDigest: string;
  sourceOnlyPolicyRegistryDigest: string;
  deletionPolicyRegistryDigest: string;
  registryBundleDigest: string;
};

export type MergePlanProofReport = {
  proofSchemaVersion: string;
  registryBundleDigest: string;
  planDigest: string;
  semanticsDigest: string;
  loweredStrategyBundleDigest: string;
  selectedStrategyDigest: string;
  selectedMergeBaseDigest: string;
  selectedConflictPolicyDigest: string;
  selectedConflictIsolationDigest: string;
  selectedIdentityMatcherDigest: string;
  selectedSourceOnlyPolicyDigest: string;
  selectedDeletionPolicyDigest: string;
};

export type MergeResultProofReport = {
  proofSchemaVersion: string;
  registryBundleDigest: string;
  resultDigest: string;
  semanticsDigest: string;
  loweredStrategyBundleDigest: string;
  lineageDigest: string;
  selectedStrategyDigest: string;
  selectedMergeBaseDigest: string;
  selectedConflictPolicyDigest: string;
  selectedConflictIsolationDigest: string;
  selectedIdentityMatcherDigest: string;
  selectedSourceOnlyPolicyDigest: string;
  selectedDeletionPolicyDigest: string;
};

export type BranchStateProofReport = {
  proofSchemaVersion: string;
  branchId: number;
  branchName: string;
  snapshotId?: number | null;
  stateDigest: string;
};

export type ReplayParityProofReport = {
  proofSchemaVersion: string;
  expectedBranchId: number;
  expectedBranchName: string;
  expectedSnapshotId?: number | null;
  expectedStateDigest: string;
  replayedBranchId: number;
  replayedBranchName: string;
  replayedSnapshotId?: number | null;
  replayedStateDigest: string;
  parity: boolean;
  mismatchClasses: string[];
};

export type ReplayArtifactProofInput = {
  proofSchemaVersion: string;
  registryBundleDigest?: string | null;
  loweredStrategyBundleDigest?: string | null;
  mergePlanDigest?: string | null;
  mergeResultDigest?: string | null;
  lineageDigest?: string | null;
  branchStateDigest: string;
};

export type ReplayArtifactProofReport = {
  proofSchemaVersion: string;
  expected: ReplayArtifactProofInput;
  replayed: ReplayArtifactProofInput;
  parity: boolean;
  mismatchClasses: string[];
};

export type RawConflictIsolationWitness = {
  granularity: string;
  conflict_record_count: number;
};

export type RawRegionIsolationSummary = {
  isolated_region_count: number;
  host_declared_region_count: number;
};

export type RawConservativeIsolationExpansion = {
  expanded_node_count: number;
};

export type RawSelectedMergeSemantics = {
  strategy_name?: string;
  strategy_basis?: string;
  merge_base_name?: string;
  merge_base_basis?: string;
  conflict_policy_name?: string;
  conflict_policy_basis?: string;
  conflict_isolation_name?: string;
  conflict_isolation_basis?: string;
  identity_matcher_name?: string;
  identity_matcher_basis?: string;
  source_only_policy_name?: string;
  source_only_policy_basis?: string;
  deletion_policy_name?: string;
  deletion_policy_basis?: string;
};

export type MergePlanProofEnvelope = {
  plan: BranchMergePlan;
  proof: MergePlanProofReport;
};

export type MergeResultProofEnvelope = {
  result: BranchMergeResult;
  proof: MergeResultProofReport;
};

export type RawIdentityCorrespondenceRecord = {
  source_node: string | number;
  target_node?: string | number | null;
  basis?: string | null;
  status: string;
  candidate_count: number;
  candidate_target_nodes?: Array<string | number>;
  admissibility_rejection?: string | null;
};

export type RawLoweredIdentityCorrespondencePlan = {
  target_candidate_count: number;
  source_lookup_count: number;
  ambiguous_match_count: number;
  rejected_admissibility_count: number;
  records: RawIdentityCorrespondenceRecord[];
};

export type RawLoweredDeletionPolicyPlan = {
  target_only_nodes: Array<string | number>;
  target_only_count: number;
  rejected_target_only_count: number;
};

export type RawLoweredConflictIsolationRecord = {
  source_node: string | number;
  target_node?: string | number | null;
  granularity: string;
  isolated_aspects: number[];
};

export type RawLoweredConflictIsolationPlan = {
  selected_policy_name?: string | null;
  selected_policy_digest?: string | null;
  selected_policy_basis?: string | null;
  expansion_breadth: number;
  witness?: RawConflictIsolationWitness | null;
  region_summary?: RawRegionIsolationSummary;
  conservative_expansion?: RawConservativeIsolationExpansion;
  records: RawLoweredConflictIsolationRecord[];
};

export type RawLoweredAspectMergePolicyRecord = {
  aspect: number;
  selected_policy_name: string;
  selected_policy_basis: string;
  affected_source_nodes: Array<string | number>;
};

export type RawLoweredAspectMergePolicyPlan = {
  records: RawLoweredAspectMergePolicyRecord[];
};

export type RawLoweredAspectMergeDecisionRecord = {
  aspect: number;
  source_node: string | number;
  target_node?: string | number | null;
  selected_policy_name: string;
  selected_policy_basis: string;
  outcome: string;
};

export type RawLoweredAspectMergeDecisionPlan = {
  records: RawLoweredAspectMergeDecisionRecord[];
};

export type RawBranchMergePlan = {
  source_branch_id: number;
  target_branch_id: number;
  schema_registry_digest?: string;
  registry_bundle_digest?: string;
  lowered_strategy_bundle_digest?: string;
  merge_kind: string;
  divergence: string;
  merge_strategy: string;
  source_snapshot_id?: number | null;
  target_snapshot_id_before?: number | null;
  selected_strategy_name?: string;
  selected_strategy_basis?: string;
  selected_merge_base_name?: string;
  selected_merge_base_basis?: string;
  selected_conflict_policy_name?: string;
  selected_conflict_policy_basis?: string;
  selected_conflict_isolation_name?: string;
  selected_conflict_isolation_basis?: string;
  selected_identity_matcher_name?: string;
  selected_identity_matcher_basis?: string;
  selected_source_only_policy_name?: string;
  selected_source_only_policy_basis?: string;
  selected_deletion_policy_name?: string;
  selected_deletion_policy_basis?: string;
  selected_semantics?: RawSelectedMergeSemantics;
  identity_correspondence: RawLoweredIdentityCorrespondencePlan;
  deletion_plan: RawLoweredDeletionPolicyPlan;
  conflict_isolation_plan: RawLoweredConflictIsolationPlan;
  aspect_policy_plan: RawLoweredAspectMergePolicyPlan;
  aspect_decision_plan: RawLoweredAspectMergeDecisionPlan;
  planned_candidates: { nodes: Array<string | number> };
  proof_minimal_overlap: { shared_nodes: Array<string | number> };
  conservative_overlap: { expanded_nodes: Array<string | number>; support_nodes: Array<string | number> };
  node_plan: unknown[];
  adoption_core: unknown[];
  resolution_plan?: unknown | null;
};

export type RawBranchMergeResultRecord = {
  source_node: string | number;
  target_node?: string | number | null;
  action: string;
  basis: string;
  identity_basis?: string | null;
  identity_status?: string | null;
  identity_candidate_count: number;
  resolved_conflict_kinds?: string[];
};

export type RawBranchMergeCounters = {
  source_slice_breadth: number;
  proof_minimal_overlap_breadth: number;
  conservative_overlap_expansion_breadth: number;
  final_candidate_breadth: number;
  reconciliation_breadth: number;
  adopted_count: number;
  introduced_node_count: number;
  replaced_count: number;
  preserved_target_count: number;
  equivalent_unchanged_count: number;
  skipped_non_adoptable_count: number;
  target_only_count: number;
  identity_target_candidates_indexed: number;
  identity_source_lookups: number;
  identity_ambiguous_match_count: number;
  identity_rejected_admissibility_count: number;
  conflict_isolation_record_count: number;
  conflict_isolation_expansion_breadth: number;
};

export type RawBranchMergeResult = {
  source_branch: number;
  target_branch: number;
  schema_registry_digest?: string;
  registry_bundle_digest?: string;
  lowered_strategy_bundle_digest?: string;
  merge_kind: string;
  divergence: string;
  merge_strategy: string;
  merged_snapshot_id?: number | null;
  target_snapshot_id_before?: number | null;
  target_snapshot_id_after?: number | null;
  source_snapshot_id?: number | null;
  selected_strategy_name?: string;
  selected_strategy_basis?: string;
  selected_merge_base_name?: string;
  selected_merge_base_basis?: string;
  selected_conflict_policy_name?: string;
  selected_conflict_policy_basis?: string;
  selected_conflict_isolation_name?: string;
  selected_conflict_isolation_basis?: string;
  selected_identity_matcher_name?: string;
  selected_identity_matcher_basis?: string;
  selected_source_only_policy_name?: string;
  selected_source_only_policy_basis?: string;
  selected_deletion_policy_name?: string;
  selected_deletion_policy_basis?: string;
  selected_semantics?: RawSelectedMergeSemantics;
  resolution_plan?: unknown | null;
  identity_correspondence: RawLoweredIdentityCorrespondencePlan;
  deletion_plan: RawLoweredDeletionPolicyPlan;
  conflict_isolation_plan: RawLoweredConflictIsolationPlan;
  aspect_policy_plan: RawLoweredAspectMergePolicyPlan;
  aspect_decision_plan: RawLoweredAspectMergeDecisionPlan;
  counters: RawBranchMergeCounters;
  records: RawBranchMergeResultRecord[];
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

export type BranchMergePlan = RawBranchMergePlan;
export type BranchMergeResult = RawBranchMergeResult;
