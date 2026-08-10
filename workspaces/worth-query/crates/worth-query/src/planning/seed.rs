use crate::collection::{CollectionPlanBundle, CollectionPlanningMode};
use crate::policy_plan::PolicyAwareCurrentPlan;
use crate::validation::ValidatedQueryBundle;

use super::artifacts::{PlannedQueryArtifact, PlannedResultShapeArtifact};
use super::counters::PlanningCounters;
use super::errors::PlanningError;
use super::execution_bundle::ExecutionPlanBundle;
use super::request_context::PlanningRequestContext;
use super::route::{
    planned_read_surface_count, route_candidate_count, FallbackDisposition, PlannedExecutionRoute,
};

pub fn seed_execution_plan(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
) -> Result<ExecutionPlanBundle, PlanningError> {
    seed_execution_plan_for_collection_mode(
        bundle,
        request_context,
        route,
        fallback,
        CollectionPlanningMode::Ordinary,
        None,
    )
}

pub(in crate::planning) fn seed_execution_plan_for_collection_mode(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    route: PlannedExecutionRoute,
    fallback: FallbackDisposition,
    collection_mode: CollectionPlanningMode,
    policy_plan: Option<&PolicyAwareCurrentPlan>,
) -> Result<ExecutionPlanBundle, PlanningError> {
    if !bundle.query().identity_bindings().is_empty()
        && request_context.semantic().binding_resolution().is_none()
    {
        return Err(PlanningError::MissingBindingResolutionForIdentityBoundQuery);
    }

    let binding_digest = request_context
        .semantic()
        .binding_resolution()
        .and_then(|resolution| {
            if resolution.requirements().requirements().is_empty() {
                None
            } else {
                Some(resolution.digest())
            }
        });
    reject_unsupported_collection_shape(bundle, &collection_mode)?;
    let planned_projection_count = policy_plan
        .map(|plan| plan.core().work_budget().authorized_field_width())
        .unwrap_or_else(|| bundle.query().projection().len());
    let result_shape = PlannedResultShapeArtifact::from_validated_bundle_for_collection_mode(
        bundle,
        &collection_mode,
    );
    let collection = CollectionPlanBundle::from_validated_bundle_for_mode(
        bundle,
        collection_mode,
        planned_projection_count,
    );
    let query = PlannedQueryArtifact::new(
        bundle.query().digest().clone(),
        bundle.query().canonical_query_digest().clone(),
        bundle.query().canonical_authority(),
        result_shape.validated_result_shape_digest(),
        route.clone(),
        fallback.clone(),
        planned_projection_count,
        bundle.query().traversal().len(),
        bundle.query().predicates().entries().len(),
        bundle.query().ordering().entries().len(),
        collection.as_ref().map(CollectionPlanBundle::digest),
        binding_digest,
        policy_plan.map(|plan| plan.core().seam().source_narrowed_artifact_digest()),
    );
    let counters = PlanningCounters::new(
        planned_projection_count,
        bundle.query().traversal().len(),
        route_candidate_count(&route),
        planned_read_surface_count(
            &route,
            planned_projection_count,
            bundle.query().traversal().len(),
            bundle.query().predicates().entries().len(),
            bundle.query().ordering().entries().len(),
        ),
        usize::from(fallback != FallbackDisposition::Forbidden),
        0,
        collection
            .as_ref()
            .map(|collection| collection.traversal_bound().edge_classes().len())
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| collection.traversal_bound().depth_limit().value() as usize)
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| {
                collection
                    .post_read_shaping()
                    .aggregate_shape()
                    .input_breadth()
                    .value()
            })
            .unwrap_or(0),
        collection
            .as_ref()
            .map(|collection| {
                usize::from(matches!(
                    collection.post_read_shaping().result_family(),
                    crate::collection::CollectionResultFamily::CdcCollection
                ))
            })
            .unwrap_or(0),
    );

    ExecutionPlanBundle::new(
        bundle,
        query,
        result_shape,
        collection,
        request_context,
        counters,
    )
}

fn reject_unsupported_collection_shape(
    bundle: &ValidatedQueryBundle,
    collection_mode: &CollectionPlanningMode,
) -> Result<(), PlanningError> {
    match bundle.query().family() {
        crate::authoring::QueryFamily::Detail => {
            if !matches!(collection_mode, CollectionPlanningMode::Ordinary) {
                return Err(PlanningError::UnsupportedCollectionResultFamily);
            }
        }
        crate::authoring::QueryFamily::Collection => {
            if bundle.query().ordering().entries().is_empty() {
                return Err(PlanningError::UnsupportedCursorShape);
            }
            if bundle.query().ordering().entries().len() > 1 {
                return Err(PlanningError::UnsupportedOrderingFamily);
            }
        }
    }

    Ok(())
}
