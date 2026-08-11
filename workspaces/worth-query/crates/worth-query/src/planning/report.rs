use crate::identity::PlanDigest;

use super::route::{FallbackDisposition, PlannedExecutionRoute};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanningReport {
    plan_digest: PlanDigest,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    projection_count: usize,
    traversal_count: usize,
    predicate_count: usize,
    ordering_count: usize,
    result_shape_binding_count: usize,
}

impl PlanningReport {
    pub fn plan_digest(&self) -> &PlanDigest {
        &self.plan_digest
    }

    pub fn route(&self) -> &PlannedExecutionRoute {
        &self.route
    }

    pub fn fallback(&self) -> &FallbackDisposition {
        &self.fallback
    }

    pub fn projection_count(&self) -> usize {
        self.projection_count
    }

    pub fn traversal_count(&self) -> usize {
        self.traversal_count
    }

    pub fn predicate_count(&self) -> usize {
        self.predicate_count
    }

    pub fn ordering_count(&self) -> usize {
        self.ordering_count
    }

    pub fn result_shape_binding_count(&self) -> usize {
        self.result_shape_binding_count
    }

    pub(crate) fn new(
        plan_digest: PlanDigest,
        route: PlannedExecutionRoute,
        fallback: FallbackDisposition,
        projection_count: usize,
        traversal_count: usize,
        predicate_count: usize,
        ordering_count: usize,
        result_shape_binding_count: usize,
    ) -> Self {
        Self {
            plan_digest,
            route,
            fallback,
            projection_count,
            traversal_count,
            predicate_count,
            ordering_count,
            result_shape_binding_count,
        }
    }
}
