use super::{
    FoundationalPerformanceAccessPatternDefinition, FoundationalPerformanceAllocationDefinition,
    FoundationalPerformanceBreadthLocalityDefinition,
    FoundationalPerformanceExecutionTemperatureDefinition,
    FoundationalPerformanceFallbackDebtDefinition,
    FoundationalPerformanceFreshnessRetentionDefinition,
    FoundationalPerformancePrimitiveDefinition,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceBreadthLocalityPosture {
    PointLocal,
    FamilyLocalBatch,
    BasisLocalBatch,
    BranchLocal,
    SnapshotBound,
    DeltaBound,
    CrossPartitionOrCrossRegion,
    PortabilityScope,
    OperatorOrGlobalDebt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceAllocationPosture {
    NoAllocation,
    ActionLocal,
    ArenaLocal,
    BatchLocal,
    ManifestScoped,
    RebuildScoped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceAccessPatternPosture {
    ScanHeavy,
    PointLookup,
    TraversalLocal,
    AppendHeavy,
    RebuildCapable,
    DensityAdaptive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceExecutionTemperature {
    HotPath,
    WarmPath,
    ColdPath,
    RecoveryOnly,
    SupportOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceFreshnessRetentionPosture {
    ExactBasisCurrent,
    HistoricalRetained,
    ReplayDerived,
    RestoredReadmitted,
    StaleSupport,
    ReducedRetention,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceFallbackDebtPosture {
    Verified,
    Deferred,
    Debt,
    Rejected,
    WidenedWithExplicitDisclosure,
    FreshFreezeRebuildReadmissionRequired,
}

pub fn foundational_performance_breadth_locality_definitions(
) -> [FoundationalPerformanceBreadthLocalityDefinition; 9] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::PointLocal,
            "point_local",
            "single-point or narrowly local work",
            "batch, global, or cross-partition breadth by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::FamilyLocalBatch,
            "family_local_batch",
            "batch work that stays within one family-local scope",
            "basis-wide or operator-global breadth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::BasisLocalBatch,
            "basis_local_batch",
            "batch work scoped to one canonical basis",
            "cross-basis or portability-wide breadth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::BranchLocal,
            "branch_local",
            "work scoped to one branch-local view or queue",
            "current-basis global authority",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::SnapshotBound,
            "snapshot_bound",
            "work scoped to a snapshot-bound subject or delivery",
            "live current-basis continuity across snapshots",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::DeltaBound,
            "delta_bound",
            "work shaped around delta-bounded breadth",
            "full-basis or retained-global coverage",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::CrossPartitionOrCrossRegion,
            "cross_partition_or_cross_region",
            "work that explicitly crosses partition, shard, or region seams",
            "point-local or family-local cost narrowness",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::PortabilityScope,
            "portability_scope",
            "work shaped by portability or export compatibility scope",
            "local operational hot-path narrowness",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceBreadthLocalityPosture::OperatorOrGlobalDebt,
            "operator_or_global_debt",
            "global or operator-facing debt that remains explicit",
            "narrow verified execution posture",
        ),
    ]
}

pub fn foundational_performance_allocation_definitions(
) -> [FoundationalPerformanceAllocationDefinition; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::NoAllocation,
            "no_allocation",
            "work that does not allocate within the named boundary",
            "proof about wider report or rebuild scopes",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::ActionLocal,
            "action_local",
            "allocation scoped to one action or operation",
            "batch, report, or lifecycle-wide storage claims",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::ArenaLocal,
            "arena_local",
            "allocation scoped to an arena or lifecycle-managed locality",
            "proof that allocation is absent or durable beyond the arena",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::BatchLocal,
            "batch_local",
            "allocation scoped to one batch or grouped execution",
            "point-local zero-allocation semantics",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::ManifestScoped,
            "manifest_scoped",
            "allocation scoped to boundary manifests or report assembly",
            "hot-path action-local allocation semantics",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAllocationPosture::RebuildScoped,
            "rebuild_scoped",
            "allocation scoped to rebuild, replay, or recovery work",
            "proof that foreground work shares the same allocation story",
        ),
    ]
}

pub fn foundational_performance_access_pattern_definitions(
) -> [FoundationalPerformanceAccessPatternDefinition; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::ScanHeavy,
            "scan_heavy",
            "work optimized around scans or broad sequential access",
            "point lookup or append locality guarantees",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::PointLookup,
            "point_lookup",
            "work optimized around direct point lookups",
            "batch scans or traversal-heavy semantics",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::TraversalLocal,
            "traversal_local",
            "work optimized around neighborhood or traversal-local movement",
            "global scan or direct point-lookup equivalence",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::AppendHeavy,
            "append_heavy",
            "work optimized around append-heavy updates or growth",
            "rebuild-free random-access guarantees",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::RebuildCapable,
            "rebuild_capable",
            "work that can pay explicit rebuild or recovery costs when needed",
            "cheap hot-path access under all conditions",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAccessPatternPosture::DensityAdaptive,
            "density_adaptive",
            "work that shifts posture based on density or sparsity pressure",
            "one static cost story across all densities",
        ),
    ]
}

pub fn foundational_performance_execution_temperature_definitions(
) -> [FoundationalPerformanceExecutionTemperatureDefinition; 5] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceExecutionTemperature::HotPath,
            "hot_path",
            "foreground operational work that must stay narrow by default",
            "cold-path replay, support, or forensic expansion by default",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceExecutionTemperature::WarmPath,
            "warm_path",
            "near-operational work that remains visible but may be broader than the hot path",
            "fully cold replay/support expansion or zero-cost narrowness claims",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceExecutionTemperature::ColdPath,
            "cold_path",
            "cold-path work that may materialize richer, wider, or slower surfaces explicitly",
            "the same narrowness guarantees as hot-path execution",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceExecutionTemperature::RecoveryOnly,
            "recovery_only",
            "restore and recovery work that only runs when recovery is required",
            "normal hot-path operational delivery",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceExecutionTemperature::SupportOnly,
            "support_only",
            "support, operator, or forensic work outside the operational lane",
            "fresh authoritative execution truth by default",
        ),
    ]
}

pub fn foundational_performance_freshness_retention_definitions(
) -> [FoundationalPerformanceFreshnessRetentionDefinition; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::ExactBasisCurrent,
            "exact_basis_current",
            "current-basis truth with no replay or staleness downgrade",
            "historical, replay-derived, or stale-support posture",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::HistoricalRetained,
            "historical_retained",
            "retained historical evidence that remains explicitly historical",
            "fresh current-basis authority",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::ReplayDerived,
            "replay_derived",
            "replay-derived reconstruction or comparison work",
            "fresh current-basis execution truth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::RestoredReadmitted,
            "restored_readmitted",
            "restored or readmitted work that crossed a freshness boundary",
            "untouched current-basis execution",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::StaleSupport,
            "stale_support",
            "support evidence that may be useful while remaining stale explicitly",
            "fresh operational current-basis truth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFreshnessRetentionPosture::ReducedRetention,
            "reduced_retention",
            "intentionally reduced retained evidence or summary support",
            "full historical or current-basis richness",
        ),
    ]
}

pub fn foundational_performance_fallback_debt_definitions(
) -> [FoundationalPerformanceFallbackDebtDefinition; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::Verified,
            "verified",
            "the named lane stayed within its declared verified posture",
            "deferred, widened, debt-shaped, or rejected disclosure",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::Deferred,
            "deferred",
            "part of the work or evidence is explicitly deferred",
            "already executed verified evidence",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::Debt,
            "debt",
            "the lane carries explicit performance debt that still needs stronger closure",
            "narrow verified execution truth",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::Rejected,
            "rejected",
            "the requested path was explicitly rejected",
            "admitted or executed success",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::WidenedWithExplicitDisclosure,
            "widened_with_explicit_disclosure",
            "the lane widened and disclosed the broader included work explicitly",
            "verified narrow posture",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceFallbackDebtPosture::FreshFreezeRebuildReadmissionRequired,
            "fresh_freeze_rebuild_readmission_required",
            "freshness or trust boundaries require freeze, rebuild, or readmission before stronger reuse",
            "current-basis immediate verification",
        ),
    ]
}
