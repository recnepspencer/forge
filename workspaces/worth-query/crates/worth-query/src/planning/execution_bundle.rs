use crate::collection::CollectionPlanBundle;
use crate::live::LivePromotionDescriptor;
use crate::validation::ValidatedQueryBundle;

use super::artifacts::{PlannedQueryArtifact, PlannedResultShapeArtifact};
use super::counters::PlanningCounters;
use super::errors::PlanningError;
use super::report::PlanningReport;
use super::request_context::PlanningRequestContext;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionPlanBundle {
    query: PlannedQueryArtifact,
    result_shape: PlannedResultShapeArtifact,
    collection: Option<CollectionPlanBundle>,
    live_promotion: LivePromotionDescriptor,
    request_context: PlanningRequestContext,
    report: PlanningReport,
    counters: PlanningCounters,
}

impl ExecutionPlanBundle {
    pub fn query(&self) -> &PlannedQueryArtifact {
        &self.query
    }

    pub fn result_shape(&self) -> &PlannedResultShapeArtifact {
        &self.result_shape
    }

    pub fn collection(&self) -> Option<&CollectionPlanBundle> {
        self.collection.as_ref()
    }

    pub fn live_promotion(&self) -> &LivePromotionDescriptor {
        &self.live_promotion
    }

    pub fn request_context(&self) -> &PlanningRequestContext {
        &self.request_context
    }

    pub fn report(&self) -> &PlanningReport {
        &self.report
    }

    pub fn counters(&self) -> &PlanningCounters {
        &self.counters
    }

    pub fn check_invariants(&self) -> Result<(), PlanningError> {
        if self.report.plan_digest() != self.query.plan_digest() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report digest does not match planned query digest",
            });
        }

        if self.report.route() != self.query.route() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report route does not match planned query route",
            });
        }

        if self.report.fallback() != self.query.fallback() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report fallback does not match planned query fallback",
            });
        }

        if self.report.projection_count() != self.query.projection_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report projection count does not match planned query projection count",
            });
        }

        if self.report.traversal_count() != self.query.traversal_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report traversal count does not match planned query traversal count",
            });
        }

        if self.report.predicate_count() != self.query.predicate_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report predicate count does not match planned query predicate count",
            });
        }

        if self.report.ordering_count() != self.query.ordering_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message:
                    "planning report ordering count does not match planned query ordering count",
            });
        }

        if self.report.result_shape_binding_count() != self.result_shape.binding_count() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planning report binding count does not match planned result-shape binding count",
            });
        }

        Ok(())
    }

    pub(crate) fn new(
        bundle: &ValidatedQueryBundle,
        query: PlannedQueryArtifact,
        result_shape: PlannedResultShapeArtifact,
        collection: Option<CollectionPlanBundle>,
        request_context: PlanningRequestContext,
        counters: PlanningCounters,
    ) -> Result<Self, PlanningError> {
        if query.plan_digest().as_str().is_empty() {
            return Err(PlanningError::PlanningInvariantViolation {
                message: "planned query digest must not be empty",
            });
        }

        let report = PlanningReport::new(
            query.plan_digest().clone(),
            query.route().clone(),
            query.fallback().clone(),
            query.projection_count(),
            query.traversal_count(),
            query.predicate_count(),
            query.ordering_count(),
            result_shape.binding_count(),
        );
        let live_promotion = LivePromotionDescriptor::for_plan(
            bundle,
            query.plan_digest().clone(),
            collection.as_ref(),
        );
        let bundle = Self {
            query,
            result_shape,
            collection,
            live_promotion,
            request_context,
            report,
            counters,
        };
        bundle.check_invariants()?;
        Ok(bundle)
    }
}
