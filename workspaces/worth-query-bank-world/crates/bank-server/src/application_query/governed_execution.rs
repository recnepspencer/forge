use bank_domain::estate::{EstateAction, EstateCapabilityPurpose, EstateCaseId, RestrictedBankField};
use bank_domain::model::BankPrincipalId;
use bank_domain::queries::{
    EstateCustomerDisclosure, EstateCustomerDisclosureQuery,
};
use bank_domain::schema::{
    BankSchema, EstateCase, EstateCaseIdentityField, Principal,
    ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
};
use worth_query_host::facade::declaration::application_query::ApplicationQueryParameterSet;
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryControls,
    WorthQueryPrincipalResolutionMode,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_result, WorthQueryPublishedApplicationResult,
};

use super::BankApplicationQueryDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

pub(crate) fn execute_estate_customer_disclosure(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    estate: EstateCaseId,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
) -> Result<
    WorthQueryPublishedApplicationResult<EstateCustomerDisclosureQuery, EstateCustomerDisclosure>,
    BankApplicationQueryDenial,
> {
    let application = runtime.application_runtime();
    let query = application
        .installed_schema()
        .application_query(EstateCustomerDisclosureQuery::reference())
        .map_err(BankApplicationQueryDenial::Installation)?;
    let capability = application
        .installed_schema()
        .capability(
            ViewEstateIdentityVerificationCapability::reference(),
            ViewRestrictedEstateOperation::reference(),
        )
        .map_err(BankApplicationQueryDenial::CapabilityInstallation)?;
    let action = EstateAction::ViewRestrictedEstate {
        estate,
        field: RestrictedBankField::CustomerIdentity,
        purpose: EstateCapabilityPurpose::IdentityVerification,
    };
    let capability_access = application
        .admit_capability_access(
            principal.query(),
            &capability,
            action,
            controls.request_scope(),
        )
        .map_err(BankApplicationQueryDenial::CapabilityAdmission)?;
    let scope = application
        .resolve_entity(
            EstateCaseIdentityField::reference(),
            estate,
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
            ApplicationQueryParameterSet::<EstateCustomerDisclosureQuery>::new(),
            controls,
        )
        .map_err(BankApplicationQueryDenial::Admission)?;
    let result = application
        .execute_application_query_one_shot(plan)
        .map_err(BankApplicationQueryDenial::Execution)?;

    Ok(publish_application_result(result.into_admitted_disclosed()))
}
