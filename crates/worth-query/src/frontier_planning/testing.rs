use crate::basis::ExecutionPreflightBundle;
use crate::execution::{ExecutionCounters, ExecutionResultEnvelope};
use crate::identity::{hash_parts, BasisDigest, PlanDigest, ResultDigest, ValidatedQueryDigest};
use crate::live::{LiveQueryFamily, LiveQueryPlan};

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct PlannedWorkPacketDigest(String);

impl PlannedWorkPacketDigest {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct BundleResolvedBasisDigest(String);

impl BundleResolvedBasisDigest {
    pub(crate) fn as_str(&self) -> &str {
        &self.0
    }

    fn from_basis_digest(digest: &BasisDigest) -> Self {
        Self(digest.as_str().to_string())
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrontierPostureDigest(String);

impl FrontierPostureDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    fn from_parts(parts: &[String]) -> Self {
        Self(hash_parts(parts))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrontierSurfaceDigest(String);

impl FrontierSurfaceDigest {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub(crate) fn from_label(label: &str) -> Self {
        Self(hash_parts(&[format!("frontier_surface:{label}")]))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum PacketEquivalenceContract {
    CollectionDigestAndBasis,
    BoundedTraversalDigestAndBasis,
    LiveDescriptorAndProgressBasis,
}

impl PacketEquivalenceContract {
    fn as_str(&self) -> &'static str {
        match self {
            Self::CollectionDigestAndBasis => "collection_digest_and_basis",
            Self::BoundedTraversalDigestAndBasis => "bounded_traversal_digest_and_basis",
            Self::LiveDescriptorAndProgressBasis => "live_descriptor_and_progress_basis",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum PacketMergeContract {
    OrderedCollectionResultBoundary,
    BoundedMaterializationResultBoundary,
    LiveDetailResultBoundary,
    LiveOrderedCollectionResultBoundary,
    LiveBoundedMaterializationResultBoundary,
}

impl PacketMergeContract {
    fn as_str(&self) -> &'static str {
        match self {
            Self::OrderedCollectionResultBoundary => "ordered_collection_result_boundary",
            Self::BoundedMaterializationResultBoundary => "bounded_materialization_result_boundary",
            Self::LiveDetailResultBoundary => "live_detail_result_boundary",
            Self::LiveOrderedCollectionResultBoundary => "live_ordered_collection_result_boundary",
            Self::LiveBoundedMaterializationResultBoundary => {
                "live_bounded_materialization_result_boundary"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PacketMergeBoundary {
    contract: PacketMergeContract,
    digest: FrontierPostureDigest,
}

impl PacketMergeBoundary {
    pub(crate) fn contract(&self) -> &PacketMergeContract {
        &self.contract
    }

    pub(crate) fn digest(&self) -> &FrontierPostureDigest {
        &self.digest
    }

    fn new(
        contract: PacketMergeContract,
        scope_summary: &str,
        basis: &BundleResolvedBasisDigest,
    ) -> Self {
        Self {
            digest: FrontierPostureDigest::from_parts(&[
                format!("merge_contract:{}", contract.as_str()),
                format!("scope:{scope_summary}"),
                format!("basis:{}", basis.as_str()),
            ]),
            contract,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FrontierPredictionDriftOutcome {
    WithinBudget,
    SerialFallbackRequired,
    DeniedByDrift,
}

impl FrontierPredictionDriftOutcome {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::WithinBudget => "within_budget",
            Self::SerialFallbackRequired => "serial_fallback_required",
            Self::DeniedByDrift => "denied_by_drift",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FrontierDisjointnessClass {
    CollectionWindowSurface,
    TraversalScopeSurface,
    LiveMaintenanceSurface,
}

impl FrontierDisjointnessClass {
    fn as_str(&self) -> &'static str {
        match self {
            Self::CollectionWindowSurface => "collection_window_surface",
            Self::TraversalScopeSurface => "traversal_scope_surface",
            Self::LiveMaintenanceSurface => "live_maintenance_surface",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrontierBreadthPrediction(usize);

impl FrontierBreadthPrediction {
    pub fn value(&self) -> usize {
        self.0
    }

    fn new(value: usize) -> Self {
        Self(value.max(1))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct FrontierComplexityContract(&'static str);

impl FrontierComplexityContract {
    pub fn as_str(&self) -> &str {
        self.0
    }

    fn ordered_collection() -> Self {
        Self("frontier_ordered_collection")
    }

    fn bounded_materialization() -> Self {
        Self("frontier_bounded_materialization")
    }

    fn live_detail() -> Self {
        Self("frontier_live_detail")
    }

    fn live_ordered_collection() -> Self {
        Self("frontier_live_ordered_collection")
    }

    fn live_bounded_materialization() -> Self {
        Self("frontier_live_bounded_materialization")
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum FrontierPerformanceStatus {
    Verified,
    Debt,
}

impl FrontierPerformanceStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Verified => "verified",
            Self::Debt => "debt",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum FrontierPlanFamily {
    OrderedCollection,
    BoundedMaterialization,
    LiveDetail,
    LiveOrderedCollection,
    LiveBoundedMaterialization,
}

impl FrontierPlanFamily {
    fn as_str(&self) -> &'static str {
        match self {
            Self::OrderedCollection => "ordered_collection",
            Self::BoundedMaterialization => "bounded_materialization",
            Self::LiveDetail => "live_detail",
            Self::LiveOrderedCollection => "live_ordered_collection",
            Self::LiveBoundedMaterialization => "live_bounded_materialization",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) enum PlannedWorkPacketFamily {
    OrderedCollectionRoot,
    BoundedMaterializationRoot,
    LiveDetailRoot,
    LiveOrderedCollectionRoot,
    LiveBoundedMaterializationRoot,
}

impl PlannedWorkPacketFamily {
    fn as_str(&self) -> &'static str {
        match self {
            Self::OrderedCollectionRoot => "ordered_collection_root",
            Self::BoundedMaterializationRoot => "bounded_materialization_root",
            Self::LiveDetailRoot => "live_detail_root",
            Self::LiveOrderedCollectionRoot => "live_ordered_collection_root",
            Self::LiveBoundedMaterializationRoot => "live_bounded_materialization_root",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlannedWorkPacket {
    source_plan_digest: PlanDigest,
    family: PlannedWorkPacketFamily,
    ordinal: usize,
    digest: PlannedWorkPacketDigest,
    scope_summary: String,
    merge_boundary: PacketMergeBoundary,
}

impl PlannedWorkPacket {
    pub(crate) fn family(&self) -> &PlannedWorkPacketFamily {
        &self.family
    }

    pub(crate) fn digest(&self) -> &PlannedWorkPacketDigest {
        &self.digest
    }

    pub(crate) fn merge_boundary(&self) -> &PacketMergeBoundary {
        &self.merge_boundary
    }

    fn new(
        source_plan_digest: PlanDigest,
        family: PlannedWorkPacketFamily,
        ordinal: usize,
        scope_summary: String,
        merge_boundary: PacketMergeBoundary,
        basis_digest: &BundleResolvedBasisDigest,
    ) -> Self {
        let digest = PlannedWorkPacketDigest::from_parts(&[
            format!("plan:{}", source_plan_digest.as_str()),
            format!("family:{}", family.as_str()),
            format!("ordinal:{ordinal}"),
            format!("scope:{scope_summary}"),
            format!("merge:{}", merge_boundary.digest().as_str()),
            format!("basis:{}", basis_digest.as_str()),
        ]);
        Self {
            source_plan_digest,
            family,
            ordinal,
            digest,
            scope_summary,
            merge_boundary,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlannedWorkPacketSet {
    packets: Vec<PlannedWorkPacket>,
    equivalence_contract: PacketEquivalenceContract,
}

impl PlannedWorkPacketSet {
    pub(crate) fn packets(&self) -> &[PlannedWorkPacket] {
        &self.packets
    }

    pub(crate) fn equivalence_contract(&self) -> &PacketEquivalenceContract {
        &self.equivalence_contract
    }

    fn new(
        packets: Vec<PlannedWorkPacket>,
        equivalence_contract: PacketEquivalenceContract,
    ) -> Self {
        Self {
            packets,
            equivalence_contract,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierPlanningCounters {
    frontier_planning_invocation_count: usize,
    planned_packet_count: usize,
    planned_bundle_route_count: usize,
    mixed_basis_denial_count: usize,
    predicted_breadth: usize,
    planned_packet_merge_boundary_count: usize,
}

impl FrontierPlanningCounters {
    pub fn frontier_planning_invocation_count(&self) -> usize {
        self.frontier_planning_invocation_count
    }

    pub fn planned_packet_count(&self) -> usize {
        self.planned_packet_count
    }

    pub fn planned_bundle_route_count(&self) -> usize {
        self.planned_bundle_route_count
    }

    pub fn mixed_basis_denial_count(&self) -> usize {
        self.mixed_basis_denial_count
    }

    pub fn predicted_breadth(&self) -> usize {
        self.predicted_breadth
    }

    pub fn planned_packet_merge_boundary_count(&self) -> usize {
        self.planned_packet_merge_boundary_count
    }

    fn single_route(
        predicted_breadth: usize,
        packet_count: usize,
        merge_boundary_count: usize,
    ) -> Self {
        Self {
            frontier_planning_invocation_count: 1,
            planned_packet_count: packet_count,
            planned_bundle_route_count: 1,
            mixed_basis_denial_count: 0,
            predicted_breadth,
            planned_packet_merge_boundary_count: merge_boundary_count,
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierRouteCounters {
    route_lowering_invocation_count: usize,
    route_surface_digest_count: usize,
    route_parallel_admission_count: usize,
    route_serial_fallback_count: usize,
    route_prediction_drift_count: usize,
}

impl FrontierRouteCounters {
    pub fn route_lowering_invocation_count(&self) -> usize {
        self.route_lowering_invocation_count
    }

    pub fn route_surface_digest_count(&self) -> usize {
        self.route_surface_digest_count
    }

    pub fn route_parallel_admission_count(&self) -> usize {
        self.route_parallel_admission_count
    }

    pub fn route_serial_fallback_count(&self) -> usize {
        self.route_serial_fallback_count
    }

    pub fn route_prediction_drift_count(&self) -> usize {
        self.route_prediction_drift_count
    }

    fn parallel(drift_outcome: &FrontierPredictionDriftOutcome) -> Self {
        Self {
            route_lowering_invocation_count: 1,
            route_surface_digest_count: 1,
            route_parallel_admission_count: 1,
            route_serial_fallback_count: 0,
            route_prediction_drift_count: usize::from(
                *drift_outcome != FrontierPredictionDriftOutcome::WithinBudget,
            ),
        }
    }

    fn serial(drift_outcome: &FrontierPredictionDriftOutcome) -> Self {
        Self {
            route_lowering_invocation_count: 1,
            route_surface_digest_count: 1,
            route_parallel_admission_count: 0,
            route_serial_fallback_count: 1,
            route_prediction_drift_count: usize::from(
                *drift_outcome != FrontierPredictionDriftOutcome::WithinBudget,
            ),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FrontierCounterSnapshot {
    frontier_lookup_count: usize,
    frontier_prediction_count: usize,
    frontier_predicted_breadth: usize,
    frontier_realized_breadth: usize,
    parallel_admission_route_count: usize,
    parallel_admission_batch_count: usize,
    parallel_admission_denial_count: usize,
    serial_fallback_plan_count: usize,
    serial_fallback_execution_count: usize,
    bundle_parallel_route_count: usize,
    bundle_serial_route_count: usize,
    mixed_basis_bundle_denial_count: usize,
    packet_merge_width: usize,
    packet_merge_reduction_count: usize,
    frontier_prediction_drift_count: usize,
    executor_parallel_rediscovery_count: usize,
    work_avoided_by_parallel_admission_count: usize,
    work_preserved_by_serial_fallback_count: usize,
}

impl FrontierCounterSnapshot {
    pub(crate) fn serial_control(
        planning: &FrontierPlanningCounters,
        execution: &ExecutionCounters,
    ) -> Self {
        Self {
            frontier_lookup_count: planning.frontier_planning_invocation_count(),
            frontier_prediction_count: planning.frontier_planning_invocation_count(),
            frontier_predicted_breadth: planning.predicted_breadth(),
            frontier_realized_breadth: execution.execution_records_examined_count(),
            parallel_admission_route_count: 0,
            parallel_admission_batch_count: 0,
            parallel_admission_denial_count: 0,
            serial_fallback_plan_count: 0,
            serial_fallback_execution_count: 0,
            bundle_parallel_route_count: 0,
            bundle_serial_route_count: 0,
            mixed_basis_bundle_denial_count: planning.mixed_basis_denial_count(),
            packet_merge_width: planning.planned_packet_merge_boundary_count(),
            packet_merge_reduction_count: planning.planned_packet_merge_boundary_count(),
            frontier_prediction_drift_count: 0,
            executor_parallel_rediscovery_count: execution.executor_semantic_rediscovery_count(),
            work_avoided_by_parallel_admission_count: 0,
            work_preserved_by_serial_fallback_count: 0,
        }
    }

    pub(crate) fn parallel_admission(
        planning: &FrontierPlanningCounters,
        route: &FrontierRouteCounters,
        execution: &ExecutionCounters,
    ) -> Self {
        Self {
            frontier_lookup_count: planning.frontier_planning_invocation_count(),
            frontier_prediction_count: planning.frontier_planning_invocation_count(),
            frontier_predicted_breadth: planning.predicted_breadth(),
            frontier_realized_breadth: execution.execution_records_examined_count(),
            parallel_admission_route_count: route.route_parallel_admission_count(),
            parallel_admission_batch_count: usize::from(route.route_parallel_admission_count() > 0),
            parallel_admission_denial_count: 0,
            serial_fallback_plan_count: route.route_serial_fallback_count(),
            serial_fallback_execution_count: 0,
            bundle_parallel_route_count: 0,
            bundle_serial_route_count: 0,
            mixed_basis_bundle_denial_count: planning.mixed_basis_denial_count(),
            packet_merge_width: planning.planned_packet_merge_boundary_count(),
            packet_merge_reduction_count: planning.planned_packet_merge_boundary_count(),
            frontier_prediction_drift_count: route.route_prediction_drift_count(),
            executor_parallel_rediscovery_count: execution.executor_semantic_rediscovery_count(),
            work_avoided_by_parallel_admission_count: planning
                .predicted_breadth()
                .saturating_sub(1),
            work_preserved_by_serial_fallback_count: 0,
        }
    }

    pub(crate) fn serial_fallback(
        planning: &FrontierPlanningCounters,
        route: &FrontierRouteCounters,
        execution: &ExecutionCounters,
    ) -> Self {
        Self {
            frontier_lookup_count: planning.frontier_planning_invocation_count(),
            frontier_prediction_count: planning.frontier_planning_invocation_count(),
            frontier_predicted_breadth: planning.predicted_breadth(),
            frontier_realized_breadth: execution.execution_records_examined_count(),
            parallel_admission_route_count: route.route_parallel_admission_count(),
            parallel_admission_batch_count: 0,
            parallel_admission_denial_count: 0,
            serial_fallback_plan_count: route.route_serial_fallback_count(),
            serial_fallback_execution_count: usize::from(route.route_serial_fallback_count() > 0),
            bundle_parallel_route_count: 0,
            bundle_serial_route_count: 0,
            mixed_basis_bundle_denial_count: planning.mixed_basis_denial_count(),
            packet_merge_width: planning.planned_packet_merge_boundary_count(),
            packet_merge_reduction_count: planning.planned_packet_merge_boundary_count(),
            frontier_prediction_drift_count: route.route_prediction_drift_count(),
            executor_parallel_rediscovery_count: execution.executor_semantic_rediscovery_count(),
            work_avoided_by_parallel_admission_count: 0,
            work_preserved_by_serial_fallback_count: execution
                .execution_records_examined_count()
                .max(1),
        }
    }

    pub(crate) fn serial_fallback_bundle(
        planning: &FrontierPlanningCounters,
        route: &FrontierRouteCounters,
        execution: &ExecutionCounters,
        bundle_serial_route_count: usize,
    ) -> Self {
        let mut snapshot = Self::serial_fallback(planning, route, execution);
        snapshot.bundle_serial_route_count = bundle_serial_route_count;
        snapshot
    }

    pub(crate) fn parallel_admission_bundle(
        planning: &FrontierPlanningCounters,
        route: &FrontierRouteCounters,
        execution: &ExecutionCounters,
        bundle_parallel_route_count: usize,
    ) -> Self {
        let mut snapshot = Self::parallel_admission(planning, route, execution);
        snapshot.bundle_parallel_route_count = bundle_parallel_route_count;
        snapshot
    }

    pub(crate) fn parallel_admission_denial() -> Self {
        Self {
            frontier_lookup_count: 1,
            parallel_admission_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn mixed_basis_bundle_denial() -> Self {
        Self {
            frontier_lookup_count: 1,
            mixed_basis_bundle_denial_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn compile_fail() -> Self {
        Self {
            frontier_lookup_count: 1,
            ..Self::default()
        }
    }

    pub(crate) fn absorb(&mut self, other: &Self) {
        self.frontier_lookup_count += other.frontier_lookup_count;
        self.frontier_prediction_count += other.frontier_prediction_count;
        self.frontier_predicted_breadth += other.frontier_predicted_breadth;
        self.frontier_realized_breadth += other.frontier_realized_breadth;
        self.parallel_admission_route_count += other.parallel_admission_route_count;
        self.parallel_admission_batch_count += other.parallel_admission_batch_count;
        self.parallel_admission_denial_count += other.parallel_admission_denial_count;
        self.serial_fallback_plan_count += other.serial_fallback_plan_count;
        self.serial_fallback_execution_count += other.serial_fallback_execution_count;
        self.bundle_parallel_route_count += other.bundle_parallel_route_count;
        self.bundle_serial_route_count += other.bundle_serial_route_count;
        self.mixed_basis_bundle_denial_count += other.mixed_basis_bundle_denial_count;
        self.packet_merge_width += other.packet_merge_width;
        self.packet_merge_reduction_count += other.packet_merge_reduction_count;
        self.frontier_prediction_drift_count += other.frontier_prediction_drift_count;
        self.executor_parallel_rediscovery_count += other.executor_parallel_rediscovery_count;
        self.work_avoided_by_parallel_admission_count +=
            other.work_avoided_by_parallel_admission_count;
        self.work_preserved_by_serial_fallback_count +=
            other.work_preserved_by_serial_fallback_count;
    }

    pub(crate) fn digest_parts(&self, label: &str) -> Vec<String> {
        vec![
            format!(
                "{label}.frontier_lookup_count:{}",
                self.frontier_lookup_count
            ),
            format!(
                "{label}.frontier_prediction_count:{}",
                self.frontier_prediction_count
            ),
            format!(
                "{label}.frontier_predicted_breadth:{}",
                self.frontier_predicted_breadth
            ),
            format!(
                "{label}.frontier_realized_breadth:{}",
                self.frontier_realized_breadth
            ),
            format!(
                "{label}.parallel_admission_route_count:{}",
                self.parallel_admission_route_count
            ),
            format!(
                "{label}.parallel_admission_batch_count:{}",
                self.parallel_admission_batch_count
            ),
            format!(
                "{label}.parallel_admission_denial_count:{}",
                self.parallel_admission_denial_count
            ),
            format!(
                "{label}.serial_fallback_plan_count:{}",
                self.serial_fallback_plan_count
            ),
            format!(
                "{label}.serial_fallback_execution_count:{}",
                self.serial_fallback_execution_count
            ),
            format!(
                "{label}.bundle_parallel_route_count:{}",
                self.bundle_parallel_route_count
            ),
            format!(
                "{label}.bundle_serial_route_count:{}",
                self.bundle_serial_route_count
            ),
            format!(
                "{label}.mixed_basis_bundle_denial_count:{}",
                self.mixed_basis_bundle_denial_count
            ),
            format!("{label}.packet_merge_width:{}", self.packet_merge_width),
            format!(
                "{label}.packet_merge_reduction_count:{}",
                self.packet_merge_reduction_count
            ),
            format!(
                "{label}.frontier_prediction_drift_count:{}",
                self.frontier_prediction_drift_count
            ),
            format!(
                "{label}.executor_parallel_rediscovery_count:{}",
                self.executor_parallel_rediscovery_count
            ),
            format!(
                "{label}.work_avoided_by_parallel_admission_count:{}",
                self.work_avoided_by_parallel_admission_count
            ),
            format!(
                "{label}.work_preserved_by_serial_fallback_count:{}",
                self.work_preserved_by_serial_fallback_count
            ),
        ]
    }

    pub fn executor_parallel_rediscovery_count(&self) -> usize {
        self.executor_parallel_rediscovery_count
    }

    pub fn frontier_lookup_count(&self) -> usize {
        self.frontier_lookup_count
    }

    pub fn frontier_prediction_count(&self) -> usize {
        self.frontier_prediction_count
    }

    pub fn frontier_predicted_breadth(&self) -> usize {
        self.frontier_predicted_breadth
    }

    pub fn frontier_realized_breadth(&self) -> usize {
        self.frontier_realized_breadth
    }

    pub fn parallel_admission_route_count(&self) -> usize {
        self.parallel_admission_route_count
    }

    pub fn parallel_admission_batch_count(&self) -> usize {
        self.parallel_admission_batch_count
    }

    pub fn parallel_admission_denial_count(&self) -> usize {
        self.parallel_admission_denial_count
    }

    pub fn serial_fallback_plan_count(&self) -> usize {
        self.serial_fallback_plan_count
    }

    pub fn serial_fallback_execution_count(&self) -> usize {
        self.serial_fallback_execution_count
    }

    pub fn bundle_parallel_route_count(&self) -> usize {
        self.bundle_parallel_route_count
    }

    pub fn bundle_serial_route_count(&self) -> usize {
        self.bundle_serial_route_count
    }

    pub fn mixed_basis_bundle_denial_count(&self) -> usize {
        self.mixed_basis_bundle_denial_count
    }

    pub fn packet_merge_width(&self) -> usize {
        self.packet_merge_width
    }

    pub fn packet_merge_reduction_count(&self) -> usize {
        self.packet_merge_reduction_count
    }

    pub fn frontier_prediction_drift_count(&self) -> usize {
        self.frontier_prediction_drift_count
    }

    pub fn work_avoided_by_parallel_admission_count(&self) -> usize {
        self.work_avoided_by_parallel_admission_count
    }

    pub fn work_preserved_by_serial_fallback_count(&self) -> usize {
        self.work_preserved_by_serial_fallback_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum PlannedRouteFamily {
    FrontierSerialControl,
    FrontierParallelAdmitted,
    FrontierParallelAdmittedBundle,
    FrontierSerialFallback,
    FrontierSerialFallbackBundle,
}

impl PlannedRouteFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FrontierSerialControl => "frontier_serial_control",
            Self::FrontierParallelAdmitted => "frontier_parallel_admitted",
            Self::FrontierParallelAdmittedBundle => "frontier_parallel_admitted_bundle",
            Self::FrontierSerialFallback => "frontier_serial_fallback",
            Self::FrontierSerialFallbackBundle => "frontier_serial_fallback_bundle",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierParityBundleError {
    BundleRouteIndexOutOfRange {
        route_count: usize,
        route_index: usize,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierParityBundle {
    query_digest: ValidatedQueryDigest,
    plan_digest: PlanDigest,
    result_digest: ResultDigest,
    basis_digest: String,
    route_family: PlannedRouteFamily,
    route_posture_digest: FrontierPostureDigest,
    predicted_breadth: FrontierBreadthPrediction,
    realized_breadth: usize,
    counter_snapshot: FrontierCounterSnapshot,
}

impl FrontierParityBundle {
    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn result_digest(&self) -> &ResultDigest {
        &self.result_digest
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn route_family(&self) -> &PlannedRouteFamily {
        &self.route_family
    }

    pub fn route_posture_digest(&self) -> &FrontierPostureDigest {
        &self.route_posture_digest
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub fn realized_breadth(&self) -> usize {
        self.realized_breadth
    }

    pub fn counter_snapshot(&self) -> &FrontierCounterSnapshot {
        &self.counter_snapshot
    }

    pub fn from_serial_control(
        frontier_plan: &FrontierAwarePlan,
        preflight: &ExecutionPreflightBundle,
        execution: &ExecutionResultEnvelope,
    ) -> Self {
        Self {
            query_digest: frontier_plan.query_digest().clone(),
            plan_digest: frontier_plan.source_plan_digest().clone(),
            result_digest: execution.report().result_digest().clone(),
            basis_digest: preflight.basis().proof().digest().as_str().to_string(),
            route_family: PlannedRouteFamily::FrontierSerialControl,
            route_posture_digest: frontier_plan.report().posture_digest().clone(),
            predicted_breadth: frontier_plan.predicted_breadth().clone(),
            realized_breadth: execution.counters().execution_records_examined_count(),
            counter_snapshot: FrontierCounterSnapshot::serial_control(
                frontier_plan.counters(),
                execution.counters(),
            ),
        }
    }

    pub fn from_parallel_admission(
        route: &ParallelAdmissionRoute,
        execution: &ExecutionResultEnvelope,
    ) -> Self {
        Self {
            query_digest: route.query_digest().clone(),
            plan_digest: route.source_plan_digest().clone(),
            result_digest: execution.report().result_digest().clone(),
            basis_digest: route
                .preflight()
                .basis()
                .proof()
                .digest()
                .as_str()
                .to_string(),
            route_family: PlannedRouteFamily::FrontierParallelAdmitted,
            route_posture_digest: route.posture_digest().clone(),
            predicted_breadth: route.decision().predicted_breadth().clone(),
            realized_breadth: execution.counters().execution_records_examined_count(),
            counter_snapshot: FrontierCounterSnapshot::parallel_admission(
                route.planning_counters(),
                route.counters(),
                execution.counters(),
            ),
        }
    }

    pub fn from_parallel_admission_bundle(
        bundle: &ParallelAdmissionRouteSet,
        route_index: usize,
        execution: &ExecutionResultEnvelope,
    ) -> Result<Self, FrontierParityBundleError> {
        let route = bundle.routes().get(route_index).ok_or(
            FrontierParityBundleError::BundleRouteIndexOutOfRange {
                route_count: bundle.routes().len(),
                route_index,
            },
        )?;
        Ok(Self {
            query_digest: route.query_digest().clone(),
            plan_digest: route.source_plan_digest().clone(),
            result_digest: execution.report().result_digest().clone(),
            basis_digest: bundle.bundle_basis_digest().to_string(),
            route_family: PlannedRouteFamily::FrontierParallelAdmittedBundle,
            route_posture_digest: bundle.bundle_posture_digest().clone(),
            predicted_breadth: route.decision().predicted_breadth().clone(),
            realized_breadth: execution.counters().execution_records_examined_count(),
            counter_snapshot: FrontierCounterSnapshot::parallel_admission_bundle(
                bundle.planning_counters(),
                route.counters(),
                execution.counters(),
                bundle.routes().len(),
            ),
        })
    }

    pub fn from_serial_fallback(
        route: &SerialFallbackRoute,
        execution: &ExecutionResultEnvelope,
    ) -> Self {
        Self {
            query_digest: route.query_digest().clone(),
            plan_digest: route.source_plan_digest().clone(),
            result_digest: execution.report().result_digest().clone(),
            basis_digest: route
                .preflight()
                .basis()
                .proof()
                .digest()
                .as_str()
                .to_string(),
            route_family: PlannedRouteFamily::FrontierSerialFallback,
            route_posture_digest: route.posture_digest().clone(),
            predicted_breadth: route.report().predicted_breadth().clone(),
            realized_breadth: execution.counters().execution_records_examined_count(),
            counter_snapshot: FrontierCounterSnapshot::serial_fallback(
                route.planning_counters(),
                route.counters(),
                execution.counters(),
            ),
        }
    }

    pub fn from_serial_fallback_bundle(
        bundle: &SerialFallbackBundleRoutes,
        route_index: usize,
        execution: &ExecutionResultEnvelope,
    ) -> Result<Self, FrontierParityBundleError> {
        let route = bundle.routes().get(route_index).ok_or(
            FrontierParityBundleError::BundleRouteIndexOutOfRange {
                route_count: bundle.routes().len(),
                route_index,
            },
        )?;
        Ok(Self {
            query_digest: route.query_digest().clone(),
            plan_digest: route.source_plan_digest().clone(),
            result_digest: execution.report().result_digest().clone(),
            basis_digest: bundle.bundle_basis_digest().to_string(),
            route_family: PlannedRouteFamily::FrontierSerialFallbackBundle,
            route_posture_digest: bundle.bundle_posture_digest().clone(),
            predicted_breadth: route.report().predicted_breadth().clone(),
            realized_breadth: execution.counters().execution_records_examined_count(),
            counter_snapshot: FrontierCounterSnapshot::serial_fallback_bundle(
                bundle.planning_counters(),
                route.counters(),
                execution.counters(),
                bundle.routes().len(),
            ),
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierPlanningReport {
    posture_digest: FrontierPostureDigest,
    family: FrontierPlanFamily,
    source_plan_digest: PlanDigest,
    bundle_basis_digest: BundleResolvedBasisDigest,
    predicted_breadth: FrontierBreadthPrediction,
    packet_merge_contract: PacketMergeContract,
    packet_count: usize,
    packet_merge_boundary_count: usize,
}

impl FrontierPlanningReport {
    pub fn posture_digest(&self) -> &FrontierPostureDigest {
        &self.posture_digest
    }

    pub fn source_plan_digest(&self) -> &PlanDigest {
        &self.source_plan_digest
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub(crate) fn packet_merge_contract(&self) -> &PacketMergeContract {
        &self.packet_merge_contract
    }

    fn new(
        family: FrontierPlanFamily,
        source_plan_digest: PlanDigest,
        bundle_basis_digest: BundleResolvedBasisDigest,
        predicted_breadth: FrontierBreadthPrediction,
        packet_set: &PlannedWorkPacketSet,
    ) -> Self {
        let mut parts = vec![
            format!("family:{}", family.as_str()),
            format!("plan:{}", source_plan_digest.as_str()),
            format!("basis:{}", bundle_basis_digest.as_str()),
            format!("predicted_breadth:{}", predicted_breadth.value()),
            format!(
                "packet_equivalence:{}",
                packet_set.equivalence_contract().as_str()
            ),
        ];
        for packet in packet_set.packets() {
            parts.push(format!("packet:{}", packet.digest().as_str()));
            parts.push(format!(
                "merge:{}",
                packet.merge_boundary().digest().as_str()
            ));
        }

        Self {
            posture_digest: FrontierPostureDigest::from_parts(&parts),
            family,
            source_plan_digest,
            bundle_basis_digest,
            predicted_breadth,
            packet_merge_contract: packet_set.packets()[0].merge_boundary().contract().clone(),
            packet_count: packet_set.packets().len(),
            packet_merge_boundary_count: packet_set.packets().len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierRouteReport {
    posture_digest: FrontierPostureDigest,
    source_plan_digest: PlanDigest,
    route_surface_digest: FrontierSurfaceDigest,
    predicted_breadth: FrontierBreadthPrediction,
    drift_outcome: FrontierPredictionDriftOutcome,
    disjointness_class: Option<FrontierDisjointnessClass>,
    serial_fallback_reason: Option<SerialFallbackReason>,
}

impl FrontierRouteReport {
    pub fn posture_digest(&self) -> &FrontierPostureDigest {
        &self.posture_digest
    }

    pub fn source_plan_digest(&self) -> &PlanDigest {
        &self.source_plan_digest
    }

    pub fn route_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.route_surface_digest
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn disjointness_class(&self) -> Option<&FrontierDisjointnessClass> {
        self.disjointness_class.as_ref()
    }

    pub fn serial_fallback_reason(&self) -> Option<&SerialFallbackReason> {
        self.serial_fallback_reason.as_ref()
    }

    fn from_parallel_route(
        posture_digest: FrontierPostureDigest,
        frontier_plan: &FrontierAwarePlan,
        evidence: &FrontierRouteEvidence,
    ) -> Self {
        Self {
            posture_digest,
            source_plan_digest: frontier_plan.source_plan_digest().clone(),
            route_surface_digest: evidence.surface_digest.clone(),
            predicted_breadth: frontier_plan.predicted_breadth().clone(),
            drift_outcome: evidence.drift_outcome.clone(),
            disjointness_class: evidence.disjointness_class.clone(),
            serial_fallback_reason: None,
        }
    }

    fn from_serial_route(
        posture_digest: FrontierPostureDigest,
        frontier_plan: &FrontierAwarePlan,
        reason: SerialFallbackReason,
        evidence: &FrontierRouteEvidence,
    ) -> Self {
        Self {
            posture_digest,
            source_plan_digest: frontier_plan.source_plan_digest().clone(),
            route_surface_digest: evidence.surface_digest.clone(),
            predicted_breadth: frontier_plan.predicted_breadth().clone(),
            drift_outcome: evidence.drift_outcome.clone(),
            disjointness_class: None,
            serial_fallback_reason: Some(reason),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FrontierAwarePlan {
    query_digest: ValidatedQueryDigest,
    source_plan_digest: PlanDigest,
    family: FrontierPlanFamily,
    bundle_basis_digest: BundleResolvedBasisDigest,
    packet_set: PlannedWorkPacketSet,
    predicted_breadth: FrontierBreadthPrediction,
    drift_outcome: FrontierPredictionDriftOutcome,
    disjointness_class: FrontierDisjointnessClass,
    complexity_contract: FrontierComplexityContract,
    performance_status: FrontierPerformanceStatus,
    report: FrontierPlanningReport,
    counters: FrontierPlanningCounters,
}

impl FrontierAwarePlan {
    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        &self.query_digest
    }

    pub fn source_plan_digest(&self) -> &PlanDigest {
        &self.source_plan_digest
    }

    pub(crate) fn family(&self) -> &FrontierPlanFamily {
        &self.family
    }

    pub(crate) fn bundle_basis_digest(&self) -> &BundleResolvedBasisDigest {
        &self.bundle_basis_digest
    }

    pub(crate) fn packet_set(&self) -> &PlannedWorkPacketSet {
        &self.packet_set
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn disjointness_class(&self) -> &FrontierDisjointnessClass {
        &self.disjointness_class
    }

    pub fn complexity_contract(&self) -> &FrontierComplexityContract {
        &self.complexity_contract
    }

    pub fn performance_status(&self) -> &FrontierPerformanceStatus {
        &self.performance_status
    }

    pub fn report(&self) -> &FrontierPlanningReport {
        &self.report
    }

    pub fn counters(&self) -> &FrontierPlanningCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontierBundlePlan {
    bundle_basis_digest: BundleResolvedBasisDigest,
    route_plans: Vec<FrontierAwarePlan>,
    counters: FrontierPlanningCounters,
}

impl FrontierBundlePlan {
    pub(crate) fn bundle_basis_digest(&self) -> &BundleResolvedBasisDigest {
        &self.bundle_basis_digest
    }

    pub(crate) fn route_plans(&self) -> &[FrontierAwarePlan] {
        &self.route_plans
    }

    pub(crate) fn counters(&self) -> &FrontierPlanningCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FrontierPlanningError {
    UnsupportedFrontierFamily,
    UnsupportedBundleComposition,
    MixedBasisBundle {
        expected_basis_digest: BundleResolvedBasisDigest,
        found_basis_digest: BundleResolvedBasisDigest,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum FrontierPlanningInput {
    ExecutionPreflight(ExecutionPreflightBundle),
    LivePlan(LiveQueryPlan),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionDecision {
    disjointness_class: FrontierDisjointnessClass,
    predicted_breadth: FrontierBreadthPrediction,
    packet_count: usize,
}

impl ParallelAdmissionDecision {
    pub fn disjointness_class(&self) -> &FrontierDisjointnessClass {
        &self.disjointness_class
    }

    pub fn predicted_breadth(&self) -> &FrontierBreadthPrediction {
        &self.predicted_breadth
    }

    pub fn packet_count(&self) -> usize {
        self.packet_count
    }

    fn from_frontier_plan(plan: &FrontierAwarePlan) -> Self {
        Self {
            disjointness_class: plan.disjointness_class().clone(),
            predicted_breadth: plan.predicted_breadth().clone(),
            packet_count: plan.packet_set().packets().len(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SerialFallbackReason {
    DeterministicAdmissionDenied,
    PredictionDriftRequiresSerialRoute,
    SerialExecutor,
    BelowMinStageWidth,
    BelowPolicyWorkThreshold,
    ValidationHeavyStage,
    BelowFullParallelThreshold,
    FullParallelUnsupportedByMutableEngine,
}

impl SerialFallbackReason {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DeterministicAdmissionDenied => "deterministic_admission_denied",
            Self::PredictionDriftRequiresSerialRoute => "prediction_drift_requires_serial_route",
            Self::SerialExecutor => "serial_executor",
            Self::BelowMinStageWidth => "below_min_stage_width",
            Self::BelowPolicyWorkThreshold => "below_policy_work_threshold",
            Self::ValidationHeavyStage => "validation_heavy_stage",
            Self::BelowFullParallelThreshold => "below_full_parallel_threshold",
            Self::FullParallelUnsupportedByMutableEngine => {
                "full_parallel_unsupported_by_mutable_engine"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FrontierRouteEvidence {
    basis_digest: String,
    surface_digest: FrontierSurfaceDigest,
    drift_outcome: FrontierPredictionDriftOutcome,
    disjointness_class: Option<FrontierDisjointnessClass>,
    serial_fallback_reason: Option<SerialFallbackReason>,
}

impl FrontierRouteEvidence {
    pub(crate) fn parallel_admission(
        basis_digest: String,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
    ) -> Self {
        Self {
            basis_digest,
            surface_digest,
            drift_outcome: FrontierPredictionDriftOutcome::WithinBudget,
            disjointness_class: Some(disjointness_class),
            serial_fallback_reason: None,
        }
    }

    pub(crate) fn serial_fallback(
        basis_digest: String,
        surface_digest: FrontierSurfaceDigest,
        reason: SerialFallbackReason,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            basis_digest,
            surface_digest,
            drift_outcome,
            disjointness_class: None,
            serial_fallback_reason: Some(reason),
        }
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.surface_digest
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        &self.drift_outcome
    }

    pub fn serial_fallback_reason(&self) -> Option<&SerialFallbackReason> {
        self.serial_fallback_reason.as_ref()
    }

    fn route_posture_digest(&self, frontier_plan: &FrontierAwarePlan) -> FrontierPostureDigest {
        FrontierPostureDigest::from_parts(&[
            format!(
                "frontier_plan_posture:{}",
                frontier_plan.report().posture_digest().as_str()
            ),
            format!("evidence_basis:{}", self.basis_digest),
            format!("frontier_surface:{}", self.surface_digest.as_str()),
            format!("drift_outcome:{}", self.drift_outcome.as_str()),
            format!(
                "disjointness:{}",
                self.disjointness_class
                    .as_ref()
                    .map(FrontierDisjointnessClass::as_str)
                    .unwrap_or("none")
            ),
            format!(
                "serial_fallback_reason:{}",
                self.serial_fallback_reason
                    .as_ref()
                    .map(SerialFallbackReason::as_str)
                    .unwrap_or("none")
            ),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionEvidence {
    route_evidence: FrontierRouteEvidence,
}

impl ParallelAdmissionEvidence {
    pub fn basis_digest(&self) -> &str {
        self.route_evidence.basis_digest()
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        self.route_evidence.surface_digest()
    }

    pub(crate) fn from_surface(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence::parallel_admission(
                basis_digest.into(),
                surface_digest,
                disjointness_class,
            ),
        }
    }

    #[cfg(test)]
    pub(crate) fn from_surface_with_drift_for_test(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        disjointness_class: FrontierDisjointnessClass,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence {
                basis_digest: basis_digest.into(),
                surface_digest,
                drift_outcome,
                disjointness_class: Some(disjointness_class),
                serial_fallback_reason: None,
            },
        }
    }

    fn route_evidence(&self) -> &FrontierRouteEvidence {
        &self.route_evidence
    }

    #[cfg(test)]
    pub(crate) fn route_posture_digest_for_test(
        &self,
        frontier_plan: &FrontierAwarePlan,
    ) -> FrontierPostureDigest {
        self.route_evidence.route_posture_digest(frontier_plan)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionBundleEvidence {
    basis_digest: String,
    bundle_surface_digest: FrontierSurfaceDigest,
    route_evidences: Vec<ParallelAdmissionEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ParallelAdmissionBundleEvidenceError {
    EmptyRouteEvidence,
    MixedBasisDigest {
        expected_basis_digest: String,
        found_basis_digest: String,
    },
}

impl ParallelAdmissionBundleEvidence {
    pub(crate) fn from_routes(
        bundle_surface_digest: FrontierSurfaceDigest,
        route_evidences: Vec<ParallelAdmissionEvidence>,
    ) -> Result<Self, ParallelAdmissionBundleEvidenceError> {
        let first = route_evidences
            .first()
            .ok_or(ParallelAdmissionBundleEvidenceError::EmptyRouteEvidence)?;
        let expected_basis_digest = first.basis_digest().to_string();
        for route in route_evidences.iter().skip(1) {
            let found_basis_digest = route.basis_digest();
            if found_basis_digest != expected_basis_digest {
                return Err(ParallelAdmissionBundleEvidenceError::MixedBasisDigest {
                    expected_basis_digest,
                    found_basis_digest: found_basis_digest.to_string(),
                });
            }
        }

        Ok(Self {
            basis_digest: expected_basis_digest,
            bundle_surface_digest,
            route_evidences,
        })
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn bundle_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.bundle_surface_digest
    }

    pub fn route_evidences(&self) -> &[ParallelAdmissionEvidence] {
        &self.route_evidences
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackEvidence {
    route_evidence: FrontierRouteEvidence,
}

impl SerialFallbackEvidence {
    pub fn basis_digest(&self) -> &str {
        self.route_evidence.basis_digest()
    }

    pub fn surface_digest(&self) -> &FrontierSurfaceDigest {
        self.route_evidence.surface_digest()
    }

    pub fn drift_outcome(&self) -> &FrontierPredictionDriftOutcome {
        self.route_evidence.drift_outcome()
    }

    pub fn reason(&self) -> &SerialFallbackReason {
        self.route_evidence
            .serial_fallback_reason()
            .expect("serial fallback evidence must carry fallback reason")
    }

    pub(crate) fn from_surface(
        basis_digest: impl Into<String>,
        surface_digest: FrontierSurfaceDigest,
        reason: SerialFallbackReason,
        drift_outcome: FrontierPredictionDriftOutcome,
    ) -> Self {
        Self {
            route_evidence: FrontierRouteEvidence::serial_fallback(
                basis_digest.into(),
                surface_digest,
                reason,
                drift_outcome,
            ),
        }
    }

    fn route_evidence(&self) -> &FrontierRouteEvidence {
        &self.route_evidence
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackBundleEvidence {
    basis_digest: String,
    bundle_surface_digest: FrontierSurfaceDigest,
    route_evidences: Vec<SerialFallbackEvidence>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum SerialFallbackBundleEvidenceError {
    EmptyRouteEvidence,
    MixedBasisDigest {
        expected_basis_digest: String,
        found_basis_digest: String,
    },
}

impl SerialFallbackBundleEvidence {
    pub(crate) fn from_routes(
        bundle_surface_digest: FrontierSurfaceDigest,
        route_evidences: Vec<SerialFallbackEvidence>,
    ) -> Result<Self, SerialFallbackBundleEvidenceError> {
        let first = route_evidences
            .first()
            .ok_or(SerialFallbackBundleEvidenceError::EmptyRouteEvidence)?;
        let expected_basis_digest = first.basis_digest().to_string();
        for route in route_evidences.iter().skip(1) {
            let found_basis_digest = route.basis_digest();
            if found_basis_digest != expected_basis_digest {
                return Err(SerialFallbackBundleEvidenceError::MixedBasisDigest {
                    expected_basis_digest,
                    found_basis_digest: found_basis_digest.to_string(),
                });
            }
        }

        Ok(Self {
            basis_digest: expected_basis_digest,
            bundle_surface_digest,
            route_evidences,
        })
    }

    pub fn basis_digest(&self) -> &str {
        &self.basis_digest
    }

    pub fn bundle_surface_digest(&self) -> &FrontierSurfaceDigest {
        &self.bundle_surface_digest
    }

    pub fn route_evidences(&self) -> &[SerialFallbackEvidence] {
        &self.route_evidences
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OrderedCollectionFrontierPreflight {
    preflight: ExecutionPreflightBundle,
}

impl OrderedCollectionFrontierPreflight {
    fn new(preflight: ExecutionPreflightBundle) -> Self {
        Self { preflight }
    }

    pub(crate) fn as_preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedMaterializationFrontierPreflight {
    preflight: ExecutionPreflightBundle,
}

impl BoundedMaterializationFrontierPreflight {
    fn new(preflight: ExecutionPreflightBundle) -> Self {
        Self { preflight }
    }

    pub(crate) fn as_preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierPreflightAdmissionError {
    UnsupportedFrontierFamily,
    OrderedCollectionRequired,
    BoundedMaterializationRequired,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionRoute {
    preflight: ExecutionPreflightBundle,
    frontier_plan: FrontierAwarePlan,
    decision: ParallelAdmissionDecision,
    report: FrontierRouteReport,
    planning_counters: FrontierPlanningCounters,
    counters: FrontierRouteCounters,
    route_posture_digest: FrontierPostureDigest,
}

impl ParallelAdmissionRoute {
    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        self.frontier_plan.query_digest()
    }

    pub fn source_plan_digest(&self) -> &PlanDigest {
        self.frontier_plan.source_plan_digest()
    }

    pub fn posture_digest(&self) -> &FrontierPostureDigest {
        &self.route_posture_digest
    }

    pub fn decision(&self) -> &ParallelAdmissionDecision {
        &self.decision
    }

    pub fn report(&self) -> &FrontierRouteReport {
        &self.report
    }

    pub(crate) fn planning_counters(&self) -> &FrontierPlanningCounters {
        &self.planning_counters
    }

    pub fn counters(&self) -> &FrontierRouteCounters {
        &self.counters
    }

    pub(crate) fn preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }

    fn new(
        preflight: ExecutionPreflightBundle,
        frontier_plan: FrontierAwarePlan,
        evidence: &ParallelAdmissionEvidence,
    ) -> Self {
        let route_evidence = evidence.route_evidence();
        let decision = ParallelAdmissionDecision::from_frontier_plan(&frontier_plan);
        let route_posture_digest = route_evidence.route_posture_digest(&frontier_plan);
        let report = FrontierRouteReport::from_parallel_route(
            route_posture_digest.clone(),
            &frontier_plan,
            route_evidence,
        );
        let counters = FrontierRouteCounters::parallel(route_evidence.drift_outcome());
        Self {
            preflight,
            planning_counters: frontier_plan.counters().clone(),
            frontier_plan,
            decision,
            report,
            counters,
            route_posture_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackRoute {
    preflight: ExecutionPreflightBundle,
    frontier_plan: FrontierAwarePlan,
    reason: SerialFallbackReason,
    report: FrontierRouteReport,
    planning_counters: FrontierPlanningCounters,
    counters: FrontierRouteCounters,
    route_posture_digest: FrontierPostureDigest,
}

impl SerialFallbackRoute {
    pub fn query_digest(&self) -> &ValidatedQueryDigest {
        self.frontier_plan.query_digest()
    }

    pub fn source_plan_digest(&self) -> &PlanDigest {
        self.frontier_plan.source_plan_digest()
    }

    pub fn posture_digest(&self) -> &FrontierPostureDigest {
        &self.route_posture_digest
    }

    pub fn reason(&self) -> &SerialFallbackReason {
        &self.reason
    }

    pub fn report(&self) -> &FrontierRouteReport {
        &self.report
    }

    pub(crate) fn planning_counters(&self) -> &FrontierPlanningCounters {
        &self.planning_counters
    }

    pub fn counters(&self) -> &FrontierRouteCounters {
        &self.counters
    }

    pub(crate) fn preflight(&self) -> &ExecutionPreflightBundle {
        &self.preflight
    }

    fn new(
        preflight: ExecutionPreflightBundle,
        frontier_plan: FrontierAwarePlan,
        reason: SerialFallbackReason,
        evidence: &SerialFallbackEvidence,
    ) -> Self {
        let route_evidence = evidence.route_evidence();
        let route_posture_digest = route_evidence.route_posture_digest(&frontier_plan);
        let report = FrontierRouteReport::from_serial_route(
            route_posture_digest.clone(),
            &frontier_plan,
            reason.clone(),
            route_evidence,
        );
        let counters = FrontierRouteCounters::serial(route_evidence.drift_outcome());
        Self {
            preflight,
            planning_counters: frontier_plan.counters().clone(),
            frontier_plan,
            reason,
            report,
            counters,
            route_posture_digest,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SerialFallbackBundleRoutes {
    bundle_basis_digest: BundleResolvedBasisDigest,
    bundle_posture_digest: FrontierPostureDigest,
    planning_counters: FrontierPlanningCounters,
    routes: Vec<SerialFallbackRoute>,
}

impl SerialFallbackBundleRoutes {
    pub fn bundle_basis_digest(&self) -> &str {
        self.bundle_basis_digest.as_str()
    }

    pub fn bundle_posture_digest(&self) -> &FrontierPostureDigest {
        &self.bundle_posture_digest
    }

    pub fn routes(&self) -> &[SerialFallbackRoute] {
        &self.routes
    }

    pub(crate) fn planning_counters(&self) -> &FrontierPlanningCounters {
        &self.planning_counters
    }

    fn new(
        bundle_basis_digest: BundleResolvedBasisDigest,
        planning_counters: FrontierPlanningCounters,
        bundle_evidence: &SerialFallbackBundleEvidence,
        routes: Vec<SerialFallbackRoute>,
    ) -> Self {
        let mut parts = vec![
            format!("bundle_basis:{}", bundle_basis_digest.as_str()),
            format!(
                "bundle_surface:{}",
                bundle_evidence.bundle_surface_digest().as_str()
            ),
        ];
        for (index, route) in routes.iter().enumerate() {
            parts.push(format!(
                "route[{index}]:{}",
                route.posture_digest().as_str()
            ));
        }
        Self {
            bundle_basis_digest,
            bundle_posture_digest: FrontierPostureDigest::from_parts(&parts),
            planning_counters,
            routes,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParallelAdmissionRouteSet {
    bundle_basis_digest: BundleResolvedBasisDigest,
    bundle_posture_digest: FrontierPostureDigest,
    planning_counters: FrontierPlanningCounters,
    routes: Vec<ParallelAdmissionRoute>,
}

impl ParallelAdmissionRouteSet {
    pub fn bundle_basis_digest(&self) -> &str {
        self.bundle_basis_digest.as_str()
    }

    pub fn bundle_posture_digest(&self) -> &FrontierPostureDigest {
        &self.bundle_posture_digest
    }

    pub fn routes(&self) -> &[ParallelAdmissionRoute] {
        &self.routes
    }

    pub(crate) fn planning_counters(&self) -> &FrontierPlanningCounters {
        &self.planning_counters
    }

    fn new(
        bundle_basis_digest: BundleResolvedBasisDigest,
        planning_counters: FrontierPlanningCounters,
        bundle_evidence: &ParallelAdmissionBundleEvidence,
        routes: Vec<ParallelAdmissionRoute>,
    ) -> Self {
        let mut parts = vec![
            format!("bundle_basis:{}", bundle_basis_digest.as_str()),
            format!(
                "bundle_surface:{}",
                bundle_evidence.bundle_surface_digest().as_str()
            ),
        ];
        for (index, route) in routes.iter().enumerate() {
            parts.push(format!(
                "route[{index}]:{}",
                route.posture_digest().as_str()
            ));
        }
        Self {
            bundle_basis_digest,
            bundle_posture_digest: FrontierPostureDigest::from_parts(&parts),
            planning_counters,
            routes,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierRoutePlanningError {
    UnsupportedFrontierFamily,
    ParallelAdmissionDenied {
        reason: SerialFallbackReason,
        posture_digest: FrontierPostureDigest,
    },
    PredictionDriftDenied {
        posture_digest: FrontierPostureDigest,
    },
    SerialFallbackUnavailable {
        posture_digest: FrontierPostureDigest,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FrontierBundleRoutePlanningError {
    UnsupportedBundleComposition,
    MixedBasisBundle {
        expected_basis_digest: String,
        found_basis_digest: String,
    },
    EvidenceCountMismatch {
        expected: usize,
        found: usize,
    },
    RoutePlanningFailed {
        route_index: usize,
        error: FrontierRoutePlanningError,
    },
}

impl From<FrontierPlanningError> for FrontierRoutePlanningError {
    fn from(value: FrontierPlanningError) -> Self {
        match value {
            FrontierPlanningError::UnsupportedFrontierFamily
            | FrontierPlanningError::UnsupportedBundleComposition
            | FrontierPlanningError::MixedBasisBundle { .. } => Self::UnsupportedFrontierFamily,
        }
    }
}

impl From<ExecutionPreflightBundle> for FrontierPlanningInput {
    fn from(value: ExecutionPreflightBundle) -> Self {
        Self::ExecutionPreflight(value)
    }
}

impl From<LiveQueryPlan> for FrontierPlanningInput {
    fn from(value: LiveQueryPlan) -> Self {
        Self::LivePlan(value)
    }
}

pub fn admit_ordered_collection_frontier_preflight(
    preflight: ExecutionPreflightBundle,
) -> Result<OrderedCollectionFrontierPreflight, FrontierPreflightAdmissionError> {
    let collection = preflight
        .plan()
        .collection()
        .ok_or(FrontierPreflightAdmissionError::UnsupportedFrontierFamily)?;
    if collection.traversal_bound().edge_classes().is_empty() {
        Ok(OrderedCollectionFrontierPreflight::new(preflight))
    } else {
        Err(FrontierPreflightAdmissionError::OrderedCollectionRequired)
    }
}

pub fn admit_bounded_materialization_frontier_preflight(
    preflight: ExecutionPreflightBundle,
) -> Result<BoundedMaterializationFrontierPreflight, FrontierPreflightAdmissionError> {
    let collection = preflight
        .plan()
        .collection()
        .ok_or(FrontierPreflightAdmissionError::UnsupportedFrontierFamily)?;
    if collection.traversal_bound().edge_classes().is_empty() {
        Err(FrontierPreflightAdmissionError::BoundedMaterializationRequired)
    } else {
        Ok(BoundedMaterializationFrontierPreflight::new(preflight))
    }
}

pub(crate) fn lower_preflight_to_frontier_plan(
    preflight: &ExecutionPreflightBundle,
) -> Result<FrontierAwarePlan, FrontierPlanningError> {
    let collection = preflight
        .plan()
        .collection()
        .ok_or(FrontierPlanningError::UnsupportedFrontierFamily)?;
    let basis_digest =
        BundleResolvedBasisDigest::from_basis_digest(preflight.basis().proof().digest());
    let (
        family,
        packet_family,
        equivalence_contract,
        merge_contract,
        disjointness_class,
        complexity_contract,
        performance_status,
        scope_summary,
        predicted_breadth,
    ) = if collection.traversal_bound().edge_classes().is_empty() {
        (
            FrontierPlanFamily::OrderedCollection,
            PlannedWorkPacketFamily::OrderedCollectionRoot,
            PacketEquivalenceContract::CollectionDigestAndBasis,
            PacketMergeContract::OrderedCollectionResultBoundary,
            FrontierDisjointnessClass::CollectionWindowSurface,
            FrontierComplexityContract::ordered_collection(),
            FrontierPerformanceStatus::Verified,
            format!(
                "collection:{}:result_family:{}:ordering:{}",
                collection.digest().as_str(),
                collection
                    .post_read_shaping()
                    .result_family()
                    .digest_label(),
                collection.ordering_basis().entries().len()
            ),
            FrontierBreadthPrediction::new(
                preflight.plan().counters().planned_read_surface_count(),
            ),
        )
    } else {
        (
            FrontierPlanFamily::BoundedMaterialization,
            PlannedWorkPacketFamily::BoundedMaterializationRoot,
            PacketEquivalenceContract::BoundedTraversalDigestAndBasis,
            PacketMergeContract::BoundedMaterializationResultBoundary,
            FrontierDisjointnessClass::TraversalScopeSurface,
            FrontierComplexityContract::bounded_materialization(),
            FrontierPerformanceStatus::Debt,
            format!(
                "collection:{}:edge_classes:{}:depth:{}",
                collection.digest().as_str(),
                collection.traversal_bound().edge_classes().len(),
                collection.traversal_bound().depth_limit().value()
            ),
            FrontierBreadthPrediction::new(
                preflight.plan().counters().planned_read_surface_count()
                    + preflight
                        .plan()
                        .counters()
                        .planned_materialization_edge_class_count(),
            ),
        )
    };

    let packet_merge_boundary =
        PacketMergeBoundary::new(merge_contract, &scope_summary, &basis_digest);
    let packet = PlannedWorkPacket::new(
        preflight.plan().query().plan_digest().clone(),
        packet_family,
        0,
        scope_summary,
        packet_merge_boundary,
        &basis_digest,
    );
    let packet_set = PlannedWorkPacketSet::new(vec![packet], equivalence_contract);
    let report = FrontierPlanningReport::new(
        family.clone(),
        preflight.plan().query().plan_digest().clone(),
        basis_digest.clone(),
        predicted_breadth.clone(),
        &packet_set,
    );
    let counters = FrontierPlanningCounters::single_route(
        predicted_breadth.value(),
        packet_set.packets().len(),
        packet_set.packets().len(),
    );

    Ok(FrontierAwarePlan {
        query_digest: preflight.plan().query().validated_query_digest().clone(),
        source_plan_digest: preflight.plan().query().plan_digest().clone(),
        family,
        bundle_basis_digest: basis_digest,
        packet_set,
        predicted_breadth,
        drift_outcome: FrontierPredictionDriftOutcome::WithinBudget,
        disjointness_class,
        complexity_contract,
        performance_status,
        report,
        counters,
    })
}

pub(crate) fn lower_live_plan_to_frontier_plan(
    live: &LiveQueryPlan,
) -> Result<FrontierAwarePlan, FrontierPlanningError> {
    let basis_digest = BundleResolvedBasisDigest::from_basis_digest(
        live.progress_basis().current_basis().proof().digest(),
    );
    let relevance = live.descriptor().relevance_contract();
    let (
        family,
        packet_family,
        equivalence_contract,
        merge_contract,
        complexity_contract,
        performance_status,
        scope_summary,
        predicted_breadth,
    ) = match live.descriptor().family() {
        LiveQueryFamily::Detail => (
            FrontierPlanFamily::LiveDetail,
            PlannedWorkPacketFamily::LiveDetailRoot,
            PacketEquivalenceContract::LiveDescriptorAndProgressBasis,
            PacketMergeContract::LiveDetailResultBoundary,
            FrontierComplexityContract::live_detail(),
            FrontierPerformanceStatus::Verified,
            format!(
                "live_detail:{}:fields:{}",
                live.descriptor().plan_digest().as_str(),
                relevance.projected_fields().len()
            ),
            FrontierBreadthPrediction::new(relevance.projected_fields().len()),
        ),
        LiveQueryFamily::OrderedCollection => (
            FrontierPlanFamily::LiveOrderedCollection,
            PlannedWorkPacketFamily::LiveOrderedCollectionRoot,
            PacketEquivalenceContract::LiveDescriptorAndProgressBasis,
            PacketMergeContract::LiveOrderedCollectionResultBoundary,
            FrontierComplexityContract::live_ordered_collection(),
            FrontierPerformanceStatus::Verified,
            format!(
                "live_ordered_collection:{}:projected:{}:ordering:{}",
                live.descriptor().plan_digest().as_str(),
                relevance.projected_fields().len(),
                relevance.ordering_fields().len()
            ),
            FrontierBreadthPrediction::new(
                relevance.projected_fields().len() + relevance.ordering_fields().len(),
            ),
        ),
        LiveQueryFamily::BoundedMaterialization => (
            FrontierPlanFamily::LiveBoundedMaterialization,
            PlannedWorkPacketFamily::LiveBoundedMaterializationRoot,
            PacketEquivalenceContract::LiveDescriptorAndProgressBasis,
            PacketMergeContract::LiveBoundedMaterializationResultBoundary,
            FrontierComplexityContract::live_bounded_materialization(),
            FrontierPerformanceStatus::Debt,
            format!(
                "live_bounded_materialization:{}:projected:{}:ordering:{}:relations:{}",
                live.descriptor().plan_digest().as_str(),
                relevance.projected_fields().len(),
                relevance.ordering_fields().len(),
                relevance.traversal_relations().len()
            ),
            FrontierBreadthPrediction::new(
                relevance.projected_fields().len()
                    + relevance.ordering_fields().len()
                    + relevance.traversal_relations().len(),
            ),
        ),
    };

    let packet_merge_boundary =
        PacketMergeBoundary::new(merge_contract, &scope_summary, &basis_digest);
    let packet = PlannedWorkPacket::new(
        live.descriptor().plan_digest().clone(),
        packet_family,
        0,
        scope_summary,
        packet_merge_boundary,
        &basis_digest,
    );
    let packet_set = PlannedWorkPacketSet::new(vec![packet], equivalence_contract);
    let report = FrontierPlanningReport::new(
        family.clone(),
        live.descriptor().plan_digest().clone(),
        basis_digest.clone(),
        predicted_breadth.clone(),
        &packet_set,
    );
    let counters = FrontierPlanningCounters::single_route(
        predicted_breadth.value(),
        packet_set.packets().len(),
        packet_set.packets().len(),
    );

    Ok(FrontierAwarePlan {
        query_digest: live.descriptor().query_digest().clone(),
        source_plan_digest: live.descriptor().plan_digest().clone(),
        family,
        bundle_basis_digest: basis_digest,
        packet_set,
        predicted_breadth,
        drift_outcome: FrontierPredictionDriftOutcome::WithinBudget,
        disjointness_class: FrontierDisjointnessClass::LiveMaintenanceSurface,
        complexity_contract,
        performance_status,
        report,
        counters,
    })
}

pub(crate) fn lower_frontier_bundle(
    inputs: &[FrontierPlanningInput],
) -> Result<FrontierBundlePlan, FrontierPlanningError> {
    if inputs.is_empty() {
        return Err(FrontierPlanningError::UnsupportedBundleComposition);
    }

    let first_input_kind = frontier_input_kind(&inputs[0]);
    if inputs
        .iter()
        .skip(1)
        .any(|input| frontier_input_kind(input) != first_input_kind)
    {
        return Err(FrontierPlanningError::UnsupportedBundleComposition);
    }

    let mut route_plans = Vec::with_capacity(inputs.len());
    for input in inputs {
        let plan = match input {
            FrontierPlanningInput::ExecutionPreflight(preflight) => {
                lower_preflight_to_frontier_plan(preflight)
            }
            FrontierPlanningInput::LivePlan(live) => lower_live_plan_to_frontier_plan(live),
        }
        .map_err(|err| match err {
            FrontierPlanningError::UnsupportedFrontierFamily => {
                FrontierPlanningError::UnsupportedBundleComposition
            }
            other => other,
        })?;
        route_plans.push(plan);
    }

    let expected_basis = route_plans[0].bundle_basis_digest().clone();
    for route_plan in route_plans.iter().skip(1) {
        if route_plan.bundle_basis_digest() != &expected_basis {
            return Err(FrontierPlanningError::MixedBasisBundle {
                expected_basis_digest: expected_basis.clone(),
                found_basis_digest: route_plan.bundle_basis_digest().clone(),
            });
        }
    }

    Ok(FrontierBundlePlan {
        bundle_basis_digest: route_plans[0].bundle_basis_digest().clone(),
        counters: FrontierPlanningCounters {
            frontier_planning_invocation_count: 1,
            planned_packet_count: route_plans
                .iter()
                .map(|route| route.packet_set().packets().len())
                .sum(),
            planned_bundle_route_count: route_plans.len(),
            mixed_basis_denial_count: 0,
            predicted_breadth: route_plans
                .iter()
                .map(|route| route.predicted_breadth().value())
                .sum(),
            planned_packet_merge_boundary_count: route_plans
                .iter()
                .map(|route| route.packet_set().packets().len())
                .sum(),
        },
        route_plans,
    })
}

pub fn lower_preflight_to_parallel_admission_route(
    preflight: &OrderedCollectionFrontierPreflight,
    evidence: &ParallelAdmissionEvidence,
) -> Result<ParallelAdmissionRoute, FrontierRoutePlanningError> {
    let preflight = preflight.as_preflight();
    let frontier_plan = lower_preflight_to_frontier_plan(preflight)?;
    let route_evidence = evidence.route_evidence();
    if evidence.basis_digest() != preflight.basis().proof().digest().as_str() {
        return Err(FrontierRoutePlanningError::ParallelAdmissionDenied {
            reason: SerialFallbackReason::DeterministicAdmissionDenied,
            posture_digest: route_evidence.route_posture_digest(&frontier_plan),
        });
    }
    match route_evidence.drift_outcome() {
        FrontierPredictionDriftOutcome::WithinBudget => {}
        FrontierPredictionDriftOutcome::SerialFallbackRequired => {
            return Err(FrontierRoutePlanningError::ParallelAdmissionDenied {
                reason: SerialFallbackReason::PredictionDriftRequiresSerialRoute,
                posture_digest: route_evidence.route_posture_digest(&frontier_plan),
            });
        }
        FrontierPredictionDriftOutcome::DeniedByDrift => {
            return Err(FrontierRoutePlanningError::PredictionDriftDenied {
                posture_digest: route_evidence.route_posture_digest(&frontier_plan),
            });
        }
    }
    match frontier_plan.family() {
        FrontierPlanFamily::OrderedCollection => {
            if route_evidence.disjointness_class.as_ref()
                != Some(&FrontierDisjointnessClass::CollectionWindowSurface)
            {
                return Err(FrontierRoutePlanningError::ParallelAdmissionDenied {
                    reason: SerialFallbackReason::DeterministicAdmissionDenied,
                    posture_digest: route_evidence.route_posture_digest(&frontier_plan),
                });
            }
            Ok(ParallelAdmissionRoute::new(
                preflight.clone(),
                frontier_plan,
                evidence,
            ))
        }
        FrontierPlanFamily::BoundedMaterialization => {
            Err(FrontierRoutePlanningError::ParallelAdmissionDenied {
                reason: SerialFallbackReason::DeterministicAdmissionDenied,
                posture_digest: route_evidence.route_posture_digest(&frontier_plan),
            })
        }
        _ => Err(FrontierRoutePlanningError::UnsupportedFrontierFamily),
    }
}

pub fn lower_preflight_to_serial_fallback_route(
    preflight: &BoundedMaterializationFrontierPreflight,
    evidence: &SerialFallbackEvidence,
) -> Result<SerialFallbackRoute, FrontierRoutePlanningError> {
    let preflight = preflight.as_preflight();
    let frontier_plan = lower_preflight_to_frontier_plan(preflight)?;
    let route_evidence = evidence.route_evidence();
    if evidence.basis_digest() != preflight.basis().proof().digest().as_str() {
        return Err(FrontierRoutePlanningError::SerialFallbackUnavailable {
            posture_digest: route_evidence.route_posture_digest(&frontier_plan),
        });
    }
    if route_evidence.drift_outcome() == &FrontierPredictionDriftOutcome::DeniedByDrift {
        return Err(FrontierRoutePlanningError::PredictionDriftDenied {
            posture_digest: route_evidence.route_posture_digest(&frontier_plan),
        });
    }
    match frontier_plan.family() {
        FrontierPlanFamily::BoundedMaterialization => Ok(SerialFallbackRoute::new(
            preflight.clone(),
            frontier_plan,
            evidence.reason().clone(),
            evidence,
        )),
        FrontierPlanFamily::OrderedCollection => {
            Err(FrontierRoutePlanningError::SerialFallbackUnavailable {
                posture_digest: route_evidence.route_posture_digest(&frontier_plan),
            })
        }
        _ => Err(FrontierRoutePlanningError::UnsupportedFrontierFamily),
    }
}

pub fn lower_preflight_bundle_to_parallel_admission_routes(
    preflights: &[OrderedCollectionFrontierPreflight],
    evidence: &ParallelAdmissionBundleEvidence,
) -> Result<ParallelAdmissionRouteSet, FrontierBundleRoutePlanningError> {
    if preflights.is_empty() {
        return Err(FrontierBundleRoutePlanningError::UnsupportedBundleComposition);
    }
    if preflights.len() != evidence.route_evidences().len() {
        return Err(FrontierBundleRoutePlanningError::EvidenceCountMismatch {
            expected: preflights.len(),
            found: evidence.route_evidences().len(),
        });
    }

    let raw_preflights = preflights
        .iter()
        .map(|preflight| preflight.as_preflight().clone())
        .map(FrontierPlanningInput::from)
        .collect::<Vec<_>>();
    let bundle_plan = lower_frontier_bundle(&raw_preflights).map_err(|error| match error {
        FrontierPlanningError::UnsupportedFrontierFamily
        | FrontierPlanningError::UnsupportedBundleComposition => {
            FrontierBundleRoutePlanningError::UnsupportedBundleComposition
        }
        FrontierPlanningError::MixedBasisBundle {
            expected_basis_digest,
            found_basis_digest,
        } => FrontierBundleRoutePlanningError::MixedBasisBundle {
            expected_basis_digest: expected_basis_digest.as_str().to_string(),
            found_basis_digest: found_basis_digest.as_str().to_string(),
        },
    })?;
    if evidence.basis_digest() != bundle_plan.bundle_basis_digest().as_str() {
        return Err(FrontierBundleRoutePlanningError::MixedBasisBundle {
            expected_basis_digest: bundle_plan.bundle_basis_digest().as_str().to_string(),
            found_basis_digest: evidence.basis_digest().to_string(),
        });
    }

    let mut routes = Vec::with_capacity(preflights.len());
    for (index, (preflight, route_evidence)) in preflights
        .iter()
        .zip(evidence.route_evidences().iter())
        .enumerate()
    {
        let route = lower_preflight_to_parallel_admission_route(preflight, route_evidence)
            .map_err(
                |error| FrontierBundleRoutePlanningError::RoutePlanningFailed {
                    route_index: index,
                    error,
                },
            )?;
        routes.push(route);
    }

    Ok(ParallelAdmissionRouteSet::new(
        bundle_plan.bundle_basis_digest().clone(),
        bundle_plan.counters().clone(),
        evidence,
        routes,
    ))
}

pub fn lower_preflight_bundle_to_serial_fallback_routes(
    preflights: &[BoundedMaterializationFrontierPreflight],
    evidence: &SerialFallbackBundleEvidence,
) -> Result<SerialFallbackBundleRoutes, FrontierBundleRoutePlanningError> {
    if preflights.is_empty() {
        return Err(FrontierBundleRoutePlanningError::UnsupportedBundleComposition);
    }
    if preflights.len() != evidence.route_evidences().len() {
        return Err(FrontierBundleRoutePlanningError::EvidenceCountMismatch {
            expected: preflights.len(),
            found: evidence.route_evidences().len(),
        });
    }

    let raw_preflights = preflights
        .iter()
        .map(|preflight| preflight.as_preflight().clone())
        .map(FrontierPlanningInput::from)
        .collect::<Vec<_>>();
    let bundle_plan = lower_frontier_bundle(&raw_preflights).map_err(|error| match error {
        FrontierPlanningError::UnsupportedFrontierFamily
        | FrontierPlanningError::UnsupportedBundleComposition => {
            FrontierBundleRoutePlanningError::UnsupportedBundleComposition
        }
        FrontierPlanningError::MixedBasisBundle {
            expected_basis_digest,
            found_basis_digest,
        } => FrontierBundleRoutePlanningError::MixedBasisBundle {
            expected_basis_digest: expected_basis_digest.as_str().to_string(),
            found_basis_digest: found_basis_digest.as_str().to_string(),
        },
    })?;
    if evidence.basis_digest() != bundle_plan.bundle_basis_digest().as_str() {
        return Err(FrontierBundleRoutePlanningError::MixedBasisBundle {
            expected_basis_digest: bundle_plan.bundle_basis_digest().as_str().to_string(),
            found_basis_digest: evidence.basis_digest().to_string(),
        });
    }

    let mut routes = Vec::with_capacity(preflights.len());
    for (index, (preflight, route_evidence)) in preflights
        .iter()
        .zip(evidence.route_evidences().iter())
        .enumerate()
    {
        let route = lower_preflight_to_serial_fallback_route(preflight, route_evidence).map_err(
            |error| FrontierBundleRoutePlanningError::RoutePlanningFailed {
                route_index: index,
                error,
            },
        )?;
        routes.push(route);
    }

    Ok(SerialFallbackBundleRoutes::new(
        bundle_plan.bundle_basis_digest().clone(),
        bundle_plan.counters().clone(),
        evidence,
        routes,
    ))
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FrontierInputKind {
    ExecutionPreflight,
    LivePlan,
}

fn frontier_input_kind(input: &FrontierPlanningInput) -> FrontierInputKind {
    match input {
        FrontierPlanningInput::ExecutionPreflight(_) => FrontierInputKind::ExecutionPreflight,
        FrontierPlanningInput::LivePlan(_) => FrontierInputKind::LivePlan,
    }
}
