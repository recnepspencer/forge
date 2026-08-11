use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateLegalComplianceQuery, EstateLegalComplianceRequest, EstateLegalComplianceResult,
    },
    schema::{
        BankSchema, EstateCase, EstateCaseIdentityField, Principal,
        ViewEstateLegalComplianceCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{
        WorthQueryApplicationOneShotResult, WorthQueryApplicationQueryAccessContext,
        WorthQueryApplicationQueryControls, WorthQueryPrincipalResolutionMode,
    },
};

use super::super::BankApplicationQueryDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub(crate) fn execute_estate_legal_compliance(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    request: EstateLegalComplianceRequest,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
) -> Result<
    WorthQueryApplicationOneShotResult<EstateLegalComplianceQuery, EstateLegalComplianceResult>,
    BankApplicationQueryDenial,
> {
    let application = runtime.application_runtime();
    let query = application
        .installed_schema()
        .application_query(EstateLegalComplianceQuery::reference())
        .map_err(BankApplicationQueryDenial::from_installation)?;
    let capability = application
        .installed_schema()
        .capability(
            ViewEstateLegalComplianceCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .map_err(BankApplicationQueryDenial::from_capability_installation)?;
    let capability_access = application
        .admit_capability_access(
            principal.query(),
            &capability,
            request.capability_request(),
            controls.request_scope(),
        )
        .map_err(BankApplicationQueryDenial::from_capability_admission)?;
    let scope = application
        .resolve_entity(
            EstateCaseIdentityField::reference(),
            request.estate(),
            controls.request_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(BankApplicationQueryDenial::from_scope_resolution)?;
    let access = WorthQueryApplicationQueryAccessContext::<
        BankSchema,
        Principal,
        BankPrincipalId,
        EstateCase,
    >::new(principal.query(), &scope);
    let plan = application
        .admit_governed_application_query(
            &query,
            &access,
            capability_access,
            ApplicationQueryParameterSet::<EstateLegalComplianceQuery>::new(),
            controls,
        )
        .map_err(BankApplicationQueryDenial::from_admission)?;
    application
        .execute_application_query_one_shot(plan)
        .map_err(BankApplicationQueryDenial::from_execution)
}
