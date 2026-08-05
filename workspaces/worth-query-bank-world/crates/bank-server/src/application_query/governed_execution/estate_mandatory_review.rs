use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateMandatoryReviewQuery, EstateMandatoryReviewRequest, EstateMandatoryReviewResult,
    },
    schema::{
        BankSchema, EstateCase, EstateCaseIdentityField, Principal,
        ViewEstateMandatoryReviewCapability, ViewRestrictedEstateOperation,
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

pub(crate) fn execute_estate_mandatory_review(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    request: EstateMandatoryReviewRequest,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
) -> Result<
    WorthQueryApplicationOneShotResult<EstateMandatoryReviewQuery, EstateMandatoryReviewResult>,
    BankApplicationQueryDenial,
> {
    let application = runtime.application_runtime();
    let query = application
        .installed_schema()
        .application_query(EstateMandatoryReviewQuery::reference())
        .map_err(BankApplicationQueryDenial::Installation)?;
    let capability = application
        .installed_schema()
        .capability(
            ViewEstateMandatoryReviewCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .map_err(BankApplicationQueryDenial::CapabilityInstallation)?;
    let capability_access = application
        .admit_capability_access(
            principal.query(),
            &capability,
            request.capability_request(),
            controls.request_scope(),
        )
        .map_err(BankApplicationQueryDenial::CapabilityAdmission)?;
    let scope = application
        .resolve_entity(
            EstateCaseIdentityField::reference(),
            request.estate(),
            controls.request_scope(),
            WorthQueryPrincipalResolutionMode::Ordinary,
        )
        .map_err(BankApplicationQueryDenial::ScopeResolution)?;
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
            ApplicationQueryParameterSet::<EstateMandatoryReviewQuery>::new(),
            controls,
        )
        .map_err(BankApplicationQueryDenial::Admission)?;
    application
        .execute_application_query_one_shot(plan)
        .map_err(BankApplicationQueryDenial::Execution)
}
