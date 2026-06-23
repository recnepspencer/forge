use forge_query::facade::consumer_kit::{
    project_support_snapshot, project_workspace_support_snapshot, ForgeQuerySupportSnapshot,
};
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryApplicationFacade, ForgeQueryWorkspace,
};

use super::authority_receipt::PrimitiveConstructionQueryAuthorityReceipt;
use super::authority_request::PrimitiveConstructionQueryAuthorityRequest;
use super::domain::PrimitiveConstructionQueryDomain;
use super::errors::PrimitiveConstructionQueryAuthorityError;
use super::operating_context::PrimitiveConstructionOperatingContext;
use super::support_summary::PrimitiveConstructionQueryAuthoritySupportSummary;
use crate::construction::query_support_pins::primitive_construction_query_support_pins;

pub(crate) fn default_primitive_construction_query_authority_receipt(
    workspace: &ForgeQueryWorkspace,
    request: PrimitiveConstructionQueryAuthorityRequest,
) -> Result<PrimitiveConstructionQueryAuthorityReceipt, PrimitiveConstructionQueryAuthorityError> {
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveConstructionQueryDomain)
        .with_operating_context(PrimitiveConstructionOperatingContext::current_head_authoritative())
        .validate()?
        .admit()?;
    let support_snapshot = project_workspace_support_snapshot(workspace);

    require_authority_support_snapshot_receipt(&handle, request, support_snapshot)
}

pub(crate) fn require_primitive_construction_query_authority(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveConstructionQueryDomain,
        PrimitiveConstructionOperatingContext,
    >,
    request: PrimitiveConstructionQueryAuthorityRequest,
) -> Result<PrimitiveConstructionQueryAuthorityReceipt, PrimitiveConstructionQueryAuthorityError> {
    let support_snapshot =
        project_support_snapshot(handle.support_snapshot().runtime_support_matrix());

    require_authority_support_snapshot_receipt(handle, request, support_snapshot)
}

fn require_authority_support_snapshot_receipt(
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<
        PrimitiveConstructionQueryDomain,
        PrimitiveConstructionOperatingContext,
    >,
    request: PrimitiveConstructionQueryAuthorityRequest,
    support_snapshot: ForgeQuerySupportSnapshot,
) -> Result<PrimitiveConstructionQueryAuthorityReceipt, PrimitiveConstructionQueryAuthorityError> {
    let contract = primitive_construction_query_support_pins()?;
    let report = contract.evaluate_snapshot(&support_snapshot)?;
    let support_summary = PrimitiveConstructionQueryAuthoritySupportSummary::from_report(&report);
    report.assert_satisfied()?;

    Ok(PrimitiveConstructionQueryAuthorityReceipt::new(
        &request,
        handle.handle_identity_digest(),
        handle.operating_context_identity_digest(),
        handle.support_snapshot().snapshot_digest(),
        handle.support_snapshot().validated_config_digest(),
        contract.contract_digest(),
        report.report_digest(),
        report.observed_snapshot_digest(),
        report.observed_source_matrix_digest(),
        support_summary,
    ))
}
