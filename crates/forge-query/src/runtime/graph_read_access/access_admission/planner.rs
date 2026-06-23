use super::super::streaming_frontier_is_admissible;
use super::{
    ForgeQueryAdmittedGraphReadAccessPlan, ForgeQueryGraphReadAccessAdmission,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessCaseRegistry,
    ForgeQueryGraphReadAccessDenial, ForgeQueryGraphReadBudgetExceededDenial,
};
use crate::runtime::{
    derive_graph_read_cost_evidence, estimate_graph_read_access_cost,
    explain_graph_read_access_requirements_for_family_in_authority,
    forge_query_graph_index_inventory, match_graph_index_inventory_for_requirements,
    ForgeQueryGraphIndexInventory, ForgeQueryGraphReadAccessAuthorityContext,
    ForgeQueryGraphReadAccessShapeExplanationError, ForgeQueryGraphReadBudget,
    ForgeQueryGraphReadBudgetClassKind, ForgeQueryReadFamily,
};

pub fn admit_graph_read_access_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError> {
    let graph_index_inventory = forge_query_graph_index_inventory();
    admit_graph_read_access_for_family_with_inventory(family, graph_index_inventory)
}

pub(crate) fn admit_graph_read_access_for_family_with_inventory(
    family: &ForgeQueryReadFamily,
    graph_index_inventory: ForgeQueryGraphIndexInventory,
) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError> {
    let authority = ForgeQueryGraphReadAccessAuthorityContext::runtime_current_compatibility();
    admit_graph_read_access_for_family_in_authority_with_inventory(
        family,
        &authority,
        graph_index_inventory,
    )
}

pub fn admit_graph_read_access_for_family_in_authority(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError> {
    let graph_index_inventory = forge_query_graph_index_inventory();
    admit_graph_read_access_for_family_in_authority_with_inventory(
        family,
        authority,
        graph_index_inventory,
    )
}

pub(crate) fn admit_graph_read_access_for_family_in_authority_with_inventory(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
    graph_index_inventory: ForgeQueryGraphIndexInventory,
) -> Result<ForgeQueryGraphReadAccessAdmission, ForgeQueryGraphReadAccessShapeExplanationError> {
    let requirements =
        explain_graph_read_access_requirements_for_family_in_authority(family, authority)?;
    let evidence = derive_graph_read_cost_evidence(&requirements);
    let estimate = estimate_graph_read_access_cost(&requirements, evidence);
    let budget = ForgeQueryGraphReadBudget::inline_ephemeral_default();
    let budget_check = budget.check_supported_cost(&estimate);
    let case_registry = ForgeQueryGraphReadAccessCaseRegistry::exhaustive();
    let graph_index_inventory_match_report =
        match_graph_index_inventory_for_requirements(&requirements, &graph_index_inventory);

    if budget_check.class().kind()
        == &ForgeQueryGraphReadBudgetClassKind::ExceedsInlineEphemeralBudget
    {
        if streaming_frontier_is_admissible(&requirements) {
            return Ok(ForgeQueryGraphReadAccessAdmission::admitted_in_authority(
                requirements,
                estimate,
                budget_check,
                case_registry,
                graph_index_inventory,
                graph_index_inventory_match_report,
                authority.receipt().clone(),
                ForgeQueryGraphReadAccessAdmissionPosture::AdmittedPagedStreaming,
            ));
        }
        if required_cases_include_posture(
            &graph_index_inventory_match_report,
            &ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
        ) {
            return Ok(ForgeQueryGraphReadAccessAdmission::denied_in_authority(
                requirements,
                estimate,
                budget_check,
                case_registry,
                graph_index_inventory,
                graph_index_inventory_match_report,
                authority.receipt().clone(),
                ForgeQueryGraphReadAccessDenial::required_persistent_index(),
            ));
        }
        return Ok(ForgeQueryGraphReadAccessAdmission::denied_in_authority(
            requirements,
            estimate.clone(),
            budget_check,
            case_registry,
            graph_index_inventory,
            graph_index_inventory_match_report,
            authority.receipt().clone(),
            ForgeQueryGraphReadAccessDenial::from_budget_exceeded(
                ForgeQueryGraphReadBudgetExceededDenial::new(
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
        return Ok(ForgeQueryGraphReadAccessAdmission::denied_in_authority(
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

    Ok(ForgeQueryGraphReadAccessAdmission::admitted_in_authority(
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
    report: &crate::runtime::ForgeQueryGraphIndexInventoryMatchReport,
) -> Option<ForgeQueryGraphReadAccessDenial> {
    if required_cases_include_posture(
        report,
        &ForgeQueryGraphReadAccessAdmissionPosture::AsyncMaterializationRequired,
    ) {
        return Some(ForgeQueryGraphReadAccessDenial::required_async_materialization());
    }
    if required_cases_include_posture(
        report,
        &ForgeQueryGraphReadAccessAdmissionPosture::AccessCapabilityRegistrationRequired,
    ) {
        return Some(ForgeQueryGraphReadAccessDenial::required_access_capability_registration());
    }
    if required_cases_include_posture(
        report,
        &ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    ) {
        return Some(ForgeQueryGraphReadAccessDenial::required_persistent_index());
    }
    if required_cases_include_posture(report, &ForgeQueryGraphReadAccessAdmissionPosture::Denied) {
        return Some(ForgeQueryGraphReadAccessDenial::unsupported_graph_index_support());
    }
    None
}

fn admitted_graph_read_access_posture(
    report: &crate::runtime::ForgeQueryGraphIndexInventoryMatchReport,
) -> ForgeQueryGraphReadAccessAdmissionPosture {
    if report.includes_admission_posture(
        &ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex,
    ) {
        return ForgeQueryGraphReadAccessAdmissionPosture::BoundedEphemeralIndex;
    }
    ForgeQueryGraphReadAccessAdmissionPosture::InlineIndexed
}

pub fn plan_admitted_graph_read_access_for_family(
    family: &ForgeQueryReadFamily,
) -> Result<
    Option<ForgeQueryAdmittedGraphReadAccessPlan>,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    let admission = admit_graph_read_access_for_family(family)?;
    Ok(ForgeQueryAdmittedGraphReadAccessPlan::from_admission(
        admission,
    ))
}

pub fn plan_admitted_graph_read_access_for_family_in_authority(
    family: &ForgeQueryReadFamily,
    authority: &ForgeQueryGraphReadAccessAuthorityContext,
) -> Result<
    Option<ForgeQueryAdmittedGraphReadAccessPlan>,
    ForgeQueryGraphReadAccessShapeExplanationError,
> {
    let admission = admit_graph_read_access_for_family_in_authority(family, authority)?;
    Ok(ForgeQueryAdmittedGraphReadAccessPlan::from_admission(
        admission,
    ))
}

fn required_cases_include_posture(
    report: &crate::runtime::ForgeQueryGraphIndexInventoryMatchReport,
    posture: &ForgeQueryGraphReadAccessAdmissionPosture,
) -> bool {
    report.includes_admission_posture(posture)
}
