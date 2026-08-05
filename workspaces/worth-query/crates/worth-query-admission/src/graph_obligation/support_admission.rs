use worth_query_declaration::facade::domain_computation::WorthQueryExecutionResourceRequest;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationOwner,
    WorthQueryInstalledGraphObligationResourcePosture,
    WorthQueryInstalledGraphObligationSelectionBasis,
};

use crate::domain_computation::execution_resource_admission::{
    admit_execution_resource_plan, reserve_execution_resource_plan,
    reserve_graph_provider_capacity, WorthQueryExecutionResourceAdmissionCounters,
    WorthQueryExecutionResourceSupportSnapshot,
};
use crate::graph_read_access::{
    review_graph_read_access, WorthQueryGraphIndexInventory,
    WorthQueryGraphReadAccessRequirementSet, WorthQueryGraphReadBudget,
    WorthQueryGraphReadPlanReview,
};

use super::admitted_plan::WorthQueryGraphWorkAdmissionMechanics;
use super::{
    WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial,
    WorthQueryGraphWorkIntentKind, WorthQuerySelectedGraphObligations,
};

pub struct WorthQueryReviewedApplicationQueryGraphWork {
    selected: WorthQuerySelectedGraphObligations,
    review: WorthQueryGraphReadPlanReview,
}

impl WorthQueryReviewedApplicationQueryGraphWork {
    pub fn review(&self) -> &WorthQueryGraphReadPlanReview {
        &self.review
    }
}

pub fn review_application_query_graph_work(
    selected: WorthQuerySelectedGraphObligations,
    requirements: WorthQueryGraphReadAccessRequirementSet,
    inventory: WorthQueryGraphIndexInventory,
    budget: WorthQueryGraphReadBudget,
) -> Result<WorthQueryReviewedApplicationQueryGraphWork, WorthQueryGraphWorkAdmissionDenial> {
    if selected.intent().kind() != WorthQueryGraphWorkIntentKind::ApplicationQueryRead {
        return Err(WorthQueryGraphWorkAdmissionDenial::IntentMismatch);
    }
    validate_owner_requirements(&selected)?;
    let review = review_graph_read_access(requirements, inventory, budget);
    validate_query_requirement_binding(&selected, &review)?;
    Ok(WorthQueryReviewedApplicationQueryGraphWork { selected, review })
}

pub fn admit_application_query_graph_work(
    reviewed: WorthQueryReviewedApplicationQueryGraphWork,
    support: &WorthQueryExecutionResourceSupportSnapshot,
) -> Result<WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial> {
    let WorthQueryReviewedApplicationQueryGraphWork { selected, review } = reviewed;
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
    WorthQueryAdmittedGraphWorkPlan::seal(
        selected,
        WorthQueryGraphWorkAdmissionMechanics::ApplicationQuery { review, capacity },
    )
    .ok_or(WorthQueryGraphWorkAdmissionDenial::IdentityExhausted)
}

pub fn admit_application_operation_graph_work(
    selected: WorthQuerySelectedGraphObligations,
    operation_binding_identity: &str,
    request: &WorthQueryExecutionResourceRequest,
    support: WorthQueryExecutionResourceSupportSnapshot,
) -> Result<WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial> {
    if selected.intent().kind() == WorthQueryGraphWorkIntentKind::ApplicationQueryRead {
        return Err(WorthQueryGraphWorkAdmissionDenial::IntentMismatch);
    }
    validate_owner_requirements(&selected)?;
    let contract = operation_resource_contract(&selected)?;
    let admitted = admit_execution_resource_plan(
        operation_binding_identity,
        contract,
        request,
        support,
        WorthQueryExecutionResourceAdmissionCounters::default(),
    )
    .map_err(WorthQueryGraphWorkAdmissionDenial::ExecutionResource)?;
    let capacity = reserve_execution_resource_plan(admitted)
        .ok_or(WorthQueryGraphWorkAdmissionDenial::CapacityUnavailable)?;
    WorthQueryAdmittedGraphWorkPlan::seal(
        selected,
        WorthQueryGraphWorkAdmissionMechanics::ApplicationOperation {
            capacity: Some(capacity),
        },
    )
    .ok_or(WorthQueryGraphWorkAdmissionDenial::IdentityExhausted)
}

pub fn admit_application_operation_read_graph_work(
    selected: WorthQuerySelectedGraphObligations,
    support: &WorthQueryExecutionResourceSupportSnapshot,
) -> Result<WorthQueryAdmittedGraphWorkPlan, WorthQueryGraphWorkAdmissionDenial> {
    if selected.intent().kind() != WorthQueryGraphWorkIntentKind::ApplicationOperationRead {
        return Err(WorthQueryGraphWorkAdmissionDenial::IntentMismatch);
    }
    validate_owner_requirements(&selected)?;
    let provider = support
        .graph_provider("primary")
        .ok_or(WorthQueryGraphWorkAdmissionDenial::ProviderSupportUnavailable)?;
    let capacity = reserve_graph_provider_capacity(provider)
        .ok_or(WorthQueryGraphWorkAdmissionDenial::CapacityUnavailable)?;
    WorthQueryAdmittedGraphWorkPlan::seal(
        selected,
        WorthQueryGraphWorkAdmissionMechanics::ApplicationOperationRead { capacity },
    )
    .ok_or(WorthQueryGraphWorkAdmissionDenial::IdentityExhausted)
}

fn validate_owner_requirements(
    selected: &WorthQuerySelectedGraphObligations,
) -> Result<(), WorthQueryGraphWorkAdmissionDenial> {
    selected
        .rows()
        .iter()
        .all(|row| owner_requirement_is_exact(row.kind(), row.required_owners()))
        .then_some(())
        .ok_or(WorthQueryGraphWorkAdmissionDenial::UnsupportedOwner)
}

pub(super) fn owner_requirement_is_exact(
    kind: WorthQueryInstalledGraphObligationKind,
    owners: &[WorthQueryInstalledGraphObligationOwner],
) -> bool {
    use WorthQueryInstalledGraphObligationKind as Kind;
    use WorthQueryInstalledGraphObligationOwner as Owner;
    match kind {
        Kind::GraphRead | Kind::EffectApplication => owners == [Owner::RelationalGraph],
        Kind::AuthorizationObservation => {
            owners == [Owner::RelationalGraph]
                || owners
                    == [
                        Owner::RelationalGraph,
                        Owner::RuntimeBridgeCorrespondence,
                        Owner::SignalPolicy,
                    ]
        }
        Kind::MutationTouch => owners == [Owner::QueryApplicationProgram],
        Kind::InvariantExecution => {
            owners
                == [
                    Owner::RelationalGraph,
                    Owner::QueryInstalledInvariantProvider,
                ]
        }
    }
}

fn validate_query_requirement_binding(
    selected: &WorthQuerySelectedGraphObligations,
    review: &WorthQueryGraphReadPlanReview,
) -> Result<(), WorthQueryGraphWorkAdmissionDenial> {
    let expected = review.requirements().read_graph_digest();
    selected
        .rows()
        .iter()
        .any(|row| {
            matches!(
                row.selection_basis(),
                WorthQueryInstalledGraphObligationSelectionBasis::ApplicationQueryGraph(graph)
                    if graph.canonical_planning_basis().digest() == expected
            )
        })
        .then_some(())
        .ok_or(WorthQueryGraphWorkAdmissionDenial::GraphReadRequirementMismatch)
}

fn operation_resource_contract(
    selected: &WorthQuerySelectedGraphObligations,
) -> Result<
    &worth_query_installation::facade::WorthQueryExecutionResourceContract,
    WorthQueryGraphWorkAdmissionDenial,
> {
    selected
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
