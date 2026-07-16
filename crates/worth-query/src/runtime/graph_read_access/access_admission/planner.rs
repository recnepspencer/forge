use super::super::streaming_frontier_is_admissible;
#[cfg(test)]
use super::WorthQueryAdmittedGraphReadAccessPlan;
use super::{
    WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessAdmissionPosture,
    WorthQueryGraphReadAccessCaseRegistry, WorthQueryGraphReadAccessDenial,
    WorthQueryGraphReadBudgetExceededDenial,
};
#[cfg(test)]
use crate::runtime::worth_query_graph_index_inventory;
use crate::runtime::{
    derive_graph_read_cost_evidence, estimate_graph_read_access_cost,
    explain_graph_read_access_requirements_for_family_in_authority_with_lookup,
    match_graph_index_inventory_for_requirements, WorthQueryGraphIndexInventory,
    WorthQueryGraphReadAccessAuthorityContext, WorthQueryGraphReadAccessShapeExplanationError,
    WorthQueryGraphReadBudget, WorthQueryGraphReadBudgetClassKind,
    WorthQueryGraphReadOperationLookup, WorthQueryReadFamily,
};
#[cfg(test)]
pub fn admit_graph_read_access_for_family(
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError> {
    let graph_index_inventory = worth_query_graph_index_inventory();
    admit_graph_read_access_for_family_with_inventory(family, graph_index_inventory)
}
#[cfg(test)]
pub(crate) fn admit_graph_read_access_for_family_with_inventory(
    family: &WorthQueryReadFamily,
    graph_index_inventory: WorthQueryGraphIndexInventory,
) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError> {
    let authority = WorthQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    admit_graph_read_access_for_family_in_authority_with_inventory(
        family,
        &authority,
        graph_index_inventory,
    )
}
#[cfg(test)]
pub(crate) fn admit_graph_read_access_for_family_in_authority_with_inventory(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    graph_index_inventory: WorthQueryGraphIndexInventory,
) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError> {
    admit_graph_read_access_for_family_in_authority_with_inventory_and_lookup(
        family,
        authority,
        graph_index_inventory,
        &crate::runtime::WorthQueryGraphReadOperationRegistry::empty(),
    )
}

pub(crate) fn admit_graph_read_access_for_family_in_authority_with_inventory_and_lookup(
    family: &WorthQueryReadFamily,
    authority: &WorthQueryGraphReadAccessAuthorityContext,
    graph_index_inventory: WorthQueryGraphIndexInventory,
    lookup: &impl WorthQueryGraphReadOperationLookup,
) -> Result<WorthQueryGraphReadAccessAdmission, WorthQueryGraphReadAccessShapeExplanationError> {
    let requirements = explain_graph_read_access_requirements_for_family_in_authority_with_lookup(
        family, authority, lookup,
    )?;
    let evidence = derive_graph_read_cost_evidence(&requirements);
    let estimate = estimate_graph_read_access_cost(&requirements, evidence);
    let budget = WorthQueryGraphReadBudget::inline_ephemeral_default();
    let budget_check = budget.check_supported_cost(&estimate);
    let case_registry = WorthQueryGraphReadAccessCaseRegistry::exhaustive();
    let graph_index_inventory_match_report =
        match_graph_index_inventory_for_requirements(&requirements, &graph_index_inventory);

    if budget_check.class().kind()
        == &WorthQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget
    {
        if streaming_frontier_is_admissible(&requirements) {
            return Ok(WorthQueryGraphReadAccessAdmission::admitted_in_authority(
                requirements,
                estimate,
                budget_check,
                case_registry,
                graph_index_inventory,
                graph_index_inventory_match_report,
                authority.receipt().clone(),
                WorthQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming,
            ));
        }
        if required_cases_include_posture(
            &graph_index_inventory_match_report,
            &WorthQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
        ) {
            return Ok(WorthQueryGraphReadAccessAdmission::denied_in_authority(
                requirements,
                estimate,
                budget_check,
                case_registry,
                graph_index_inventory,
                graph_index_inventory_match_report,
                authority.receipt().clone(),
                WorthQueryGraphReadAccessDenial::required_persistent_index(),
            ));
        }
        return Ok(WorthQueryGraphReadAccessAdmission::denied_in_authority(
            requirements,
            estimate.clone(),
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            authority.receipt().clone(),
            WorthQueryGraphReadAccessDenial::from_budget_exceeded(
                WorthQueryGraphReadBudgetExceededDenial::new(
                    budget.max_inline_index_bytes(),
                    estimate.supported().index_bytes(),
                    budget.max_inline_result_bytes(),
                    estimate.supported().result_bytes(),
                    budget.max_inline_intermediate_set_size(),
                    estimate.intrinsic().intermediate_set_size(),
                ),
            ),
        ));
    }

    if let Some(graph_index_denial) =
        required_graph_index_inventory_denial(&graph_index_inventory_match_report)
    {
        return Ok(WorthQueryGraphReadAccessAdmission::denied_in_authority(
            requirements,
            estimate,
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            authority.receipt().clone(),
            graph_index_denial,
        ));
    }

    Ok(WorthQueryGraphReadAccessAdmission::admitted_in_authority(
        requirements,
        estimate,
        budget_check,
        case_registry,
        graph_index_inventory,
        graph_index_inventory_match_report.clone(),
        authority.receipt().clone(),
        admitted_graph_read_access_posture(&graph_index_inventory_match_report),
    ))
}

fn required_graph_index_inventory_denial(
    report: &crate::runtime::WorthQueryGraphIndexInventoryMatchReport,
) -> Option<WorthQueryGraphReadAccessDenial> {
    if required_cases_include_posture(
        report,
        &WorthQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired,
    ) {
        return Some(WorthQueryGraphReadAccessDenial::required_async_materialization());
    }
    if required_cases_include_posture(
        report,
        &WorthQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
    ) {
        return Some(WorthQueryGraphReadAccessDenial::required_access_capability_registration());
    }
    if required_cases_include_posture(
        report,
        &WorthQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    ) {
        return Some(WorthQueryGraphReadAccessDenial::required_persistent_index());
    }
    if required_cases_include_posture(report, &WorthQueryGraphReadAccessAdmissionPosture::Denied) {
        return Some(WorthQueryGraphReadAccessDenial::unsupported_graph_index_support());
    }
    None
}

fn admitted_graph_read_access_posture(
    report: &crate::runtime::WorthQueryGraphIndexInventoryMatchReport,
) -> WorthQueryGraphReadAccessAdmissionPosture {
    if report.includes_admission_posture(
        &WorthQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
    ) {
        return WorthQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex;
    }
    WorthQueryGraphReadAccessAdmissionPosture::InlineIndexed
}
#[cfg(test)]
pub fn plan_admitted_graph_read_access_for_family(
    family: &WorthQueryReadFamily,
) -> Result<
    Option<WorthQueryAdmittedGraphReadAccessPlan>,
    WorthQueryGraphReadAccessShapeExplanationError,
> {
    let admission = admit_graph_read_access_for_family(family)?;
    Ok(WorthQueryAdmittedGraphReadAccessPlan::from_admission(
        admission,
    ))
}
fn required_cases_include_posture(
    report: &crate::runtime::WorthQueryGraphIndexInventoryMatchReport,
    posture: &WorthQueryGraphReadAccessAdmissionPosture,
) -> bool {
    report.includes_admission_posture(posture)
}
