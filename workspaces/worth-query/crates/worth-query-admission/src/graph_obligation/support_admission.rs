use worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphObligationResourcePosture,
    WorthQueryInstalledGraphObligationSelectionBasis,
};

use crate::domain_computation::execution_resource_admission::{
    admit_execution_resource_plan, reserve_execution_resource_plan,
    reserve_graph_provider_capacity, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceSupportSnapshot,
};
use crate::graph_read_access::plan_review::review_graph_read_access;
use crate::graph_read_access::{
    WorthQueryGraphIndexInventory, WorthQueryGraphReadAccessRequirementSet,
    WorthQueryGraphReadBudget, WorthQueryGraphReadPlanReview,
};

use super::admission_identity::derive_graph_work_plan_identity;
use super::admitted_plan::WorthQueryGraphWorkAdmissionMechanics;
use super::{
    WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial,
    WorthQueryGraphWorkIntentKind, WorthQueryRequiredGraphWork,
};

/// Sealed proof that graph-read review consumed the exact required-obligation
/// phase that capacity admission will later consume.
///
/// ```compile_fail
/// use worth_query_admission::integration::WorthQueryReviewedApplicationQueryGraphWork;
/// let counterfeit = WorthQueryReviewedApplicationQueryGraphWork {};
/// ```
pub struct WorthQueryReviewedApplicationQueryGraphWork {
    required: WorthQueryRequiredGraphWork,
    review: WorthQueryGraphReadPlanReview,
}

impl WorthQueryReviewedApplicationQueryGraphWork {
    pub fn review(&self) -> &WorthQueryGraphReadPlanReview {
        &self.review
    }
}

pub fn review_application_query_graph_work(
    required: WorthQueryRequiredGraphWork,
    requirements: WorthQueryGraphReadAccessRequirementSet,
    inventory: WorthQueryGraphIndexInventory,
    budget: WorthQueryGraphReadBudget,
) -> Result<WorthQueryReviewedApplicationQueryGraphWork, WorthQueryGraphWorkAdmissionDenial> {
    if required.intent().kind() != WorthQueryGraphWorkIntentKind::ApplicationQueryRead {
        return Err(WorthQueryGraphWorkAdmissionDenial::IntentMismatch);
    }
    let review = review_graph_read_access(requirements, inventory, budget);
    validate_query_requirement_binding(&required, &review)?;
    Ok(WorthQueryReviewedApplicationQueryGraphWork { required, review })
}

pub fn admit_application_query_graph_work(
    reviewed: WorthQueryReviewedApplicationQueryGraphWork,
    support: &WorthQueryExecutionResourceSupportSnapshot,
) -> Result<WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial> {
    let WorthQueryReviewedApplicationQueryGraphWork { required, review } = reviewed;
    if let Some(denial) = review.denial() {
        return Err(WorthQueryGraphWorkAdmissionDenial::GraphReadPlan(
            denial.kind(),
        ));
    }
    let provider = support
        .graph_provider("primary")
        .ok_or(WorthQueryGraphWorkAdmissionDenial::ProviderSupportUnavailable)?;
    let capacity = reserve_graph_provider_capacity(provider)
        .ok_or(WorthQueryGraphWorkAdmissionDenial::CapacityUnavailable)?;
    let (identity, canonical_work) = derive_graph_work_plan_identity(
        &required,
        capacity.support_identity(),
        Some(review.requirements().digest().as_digest()),
    )
    .map_err(WorthQueryGraphWorkAdmissionDenial::CanonicalIdentity)?;
    Ok(WorthQueryAdmittedGraphWorkPlan::seal(
        identity,
        required,
        WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { review, capacity },
        canonical_work,
    ))
}

pub fn admit_application_operation_graph_work(
    required: WorthQueryRequiredGraphWork,
    operation_binding_identity: &str,
    request: &WorthQueryExecutionResourceRequest,
    support: WorthQueryExecutionResourceSupportSnapshot,
) -> Result<WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial> {
    if required.intent().kind() == WorthQueryGraphWorkIntentKind::ApplicationQueryRead {
        return Err(WorthQueryGraphWorkAdmissionDenial::IntentMismatch);
    }
    let contract = operation_resource_contract(&required)?;
    let admitted = admit_execution_resource_plan(
        operation_binding_identity,
        contract,
        request,
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .map_err(WorthQueryGraphWorkAdmissionDenial::ExecutionResource)?;
    let reserved = reserve_execution_resource_plan(admitted)
        .ok_or(WorthQueryGraphWorkAdmissionDenial::CapacityUnavailable)?;
    let (identity, canonical_work) =
        derive_graph_work_plan_identity(&required, reserved.resources().identity(), None)
            .map_err(WorthQueryGraphWorkAdmissionDenial::CanonicalIdentity)?;
    Ok(WorthQueryAdmittedGraphWorkPlan::seal(
        identity,
        required,
        WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation {
            capacity: Some(reserved),
        },
        canonical_work,
    ))
}

fn validate_query_requirement_binding(
    required: &WorthQueryRequiredGraphWork,
    review: &WorthQueryGraphReadPlanReview,
) -> Result<(), WorthQueryGraphWorkAdmissionDenial> {
    let expected = review.requirements().read_graph_digest();
    let matches = required.selected().rows().iter().any(|row| {
        matches!(
            row.selection_basis(),
            WorthQueryInstalledGraphObligationSelectionBasis::ApplicationQueryGraph(graph)
                if graph.canonical_planning_basis().digest() == expected
        )
    });
    matches
        .then_some(())
        .ok_or(WorthQueryGraphWorkAdmissionDenial::GraphReadRequirementMismatch)
}

fn operation_resource_contract(
    required: &WorthQueryRequiredGraphWork,
) -> Result<
    &worth_query_installation::facade::WorthQueryExecutionResourceContract,
    WorthQueryGraphWorkAdmissionDenial,
> {
    required
        .selected()
        .rows()
        .iter()
        .find_map(|row| match row.resource_posture() {
            WorthQueryInstalledGraphObligationResourcePosture::ApplicationOperation(contract) => {
                Some(contract)
            }
            WorthQueryInstalledGraphObligationResourcePosture::ApplicationQuery { .. } => None,
        })
        .ok_or(WorthQueryGraphWorkAdmissionDenial::IntentMismatch)
}
