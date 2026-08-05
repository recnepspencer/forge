use crate::execution::ExecutionCounters;

use super::super::{FrontierPlanningCounters, FrontierRouteCounters};

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
