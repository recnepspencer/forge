use crate::collection::{CollectionPlanningMode, CollectionResultFamily};
use crate::policy_plan::PolicyAwareCurrentPlan;
use crate::validation::ValidatedQueryBundle;

use super::errors::PlanningError;
use super::execution_bundle::ExecutionPlanBundle;
use super::request_context::PlanningRequestContext;
use super::route::{FallbackDisposition, PlannedExecutionRoute};
use super::seed::seed_execution_plan_for_collection_mode;

pub(crate) fn plan_validated_bundle(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family(
        bundle,
        request_context,
        CollectionResultFamily::OrdinaryCollection,
    )
}

pub(crate) fn plan_validated_bundle_with_policy_authority(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    policy_plan: &PolicyAwareCurrentPlan,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family_with_policy_authority(
        bundle,
        request_context,
        CollectionResultFamily::OrdinaryCollection,
        Some(policy_plan),
    )
}

pub(crate) fn plan_validated_bundle_for_count_aggregate(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family_with_policy_authority(
        bundle,
        request_context,
        CollectionResultFamily::CountAggregate,
        None,
    )
}

pub(crate) fn plan_validated_bundle_for_count_aggregate_with_policy_authority(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    policy_plan: &PolicyAwareCurrentPlan,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family_with_policy_authority(
        bundle,
        request_context,
        CollectionResultFamily::CountAggregate,
        Some(policy_plan),
    )
}

pub(crate) fn plan_validated_bundle_for_collection_family(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    collection_result_family: CollectionResultFamily,
) -> Result<ExecutionPlanBundle, PlanningError> {
    plan_validated_bundle_for_collection_family_with_policy_authority(
        bundle,
        request_context,
        collection_result_family,
        None,
    )
}

fn plan_validated_bundle_for_collection_family_with_policy_authority(
    bundle: &ValidatedQueryBundle,
    request_context: PlanningRequestContext,
    collection_result_family: CollectionResultFamily,
    policy_plan: Option<&PolicyAwareCurrentPlan>,
) -> Result<ExecutionPlanBundle, PlanningError> {
    if request_context.semantic().basis_intent().fallback_allowed() {
        return Err(PlanningError::UnsupportedFallbackShape);
    }

    if matches!(
        request_context.semantic().basis_intent().authority_family(),
        crate::basis::BasisAuthorityFamily::Store
    ) {
        return Err(PlanningError::UnsupportedBackendParityRequest);
    }

    let route = select_route(bundle, &request_context);
    let fallback = if request_context.semantic().basis_intent().fallback_allowed() {
        FallbackDisposition::AdmittedButUnused
    } else {
        FallbackDisposition::Forbidden
    };

    let collection_mode = match collection_result_family {
        CollectionResultFamily::OrdinaryCollection => CollectionPlanningMode::Ordinary,
        CollectionResultFamily::CdcCollection => CollectionPlanningMode::Cdc,
        CollectionResultFamily::CountAggregate => CollectionPlanningMode::CountRows,
    };
    seed_execution_plan_for_collection_mode(
        bundle,
        request_context,
        route,
        fallback,
        collection_mode,
        policy_plan,
    )
}

fn select_route(
    bundle: &ValidatedQueryBundle,
    request_context: &PlanningRequestContext,
) -> PlannedExecutionRoute {
    match request_context.semantic().basis_intent().authority_family() {
        crate::basis::BasisAuthorityFamily::Store => PlannedExecutionRoute::StoreSnapshotRead,
        crate::basis::BasisAuthorityFamily::Runtime => {
            if bundle.query().traversal().is_empty()
                && bundle.query().predicates().entries().is_empty()
                && bundle.query().ordering().entries().is_empty()
            {
                PlannedExecutionRoute::RuntimeSnapshotRead
            } else {
                PlannedExecutionRoute::RuntimeExpandedSnapshotRead
            }
        }
    }
}
