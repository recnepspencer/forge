use bank_domain::estate::{
    EstateAction, EstateCapabilityPurpose, EstateCaseId, RestrictedBankField,
};
use bank_domain::model::BankPrincipalId;
use bank_domain::queries::{
    EstateCustomerDisclosure, EstateCustomerDisclosureQuery,
    EstateCustomerDisclosureQueryParameters,
};
use bank_domain::schema::{
    BankSchema, EstateCase, EstateCaseIdentityField, Principal,
    ViewEstateIdentityVerificationCapability, ViewRestrictedEstateOperation,
};
use worth_query_host::facade::declaration::application_query::ApplicationQueryParameterSet;
use worth_query_host::facade::primary_graph::{
    WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationQueryAccessContext,
    WorthQueryApplicationQueryControls, WorthQueryPrincipalResolutionMode,
};
use worth_query_host::facade::publication::domain_computation::{
    publish_application_result, WorthQueryPublishedApplicationResult,
};

use super::BankApplicationQueryDenial;
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime};

type EstateCustomerDisclosurePlan<'a> = WorthQueryAdmittedApplicationQueryPlan<
    'a,
    BankSchema,
    EstateCustomerDisclosureQuery,
    EstateCustomerDisclosureQueryParameters,
    EstateCustomerDisclosure,
    Principal,
    BankPrincipalId,
    EstateCase,
>;

pub(crate) fn execute_estate_customer_disclosure(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    estate: EstateCaseId,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
) -> Result<
    WorthQueryPublishedApplicationResult<EstateCustomerDisclosureQuery, EstateCustomerDisclosure>,
    BankApplicationQueryDenial,
> {
    execute_estate_customer_disclosure_with(runtime, principal, estate, controls, |_| ())
        .map(|(published, ())| published)
}

pub(crate) fn execute_estate_customer_disclosure_with<Observation>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    estate: EstateCaseId,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
    observe: impl FnOnce(&EstateCustomerDisclosurePlan<'_>) -> Observation,
) -> Result<
    (
        WorthQueryPublishedApplicationResult<
            EstateCustomerDisclosureQuery,
            EstateCustomerDisclosure,
        >,
        Observation,
    ),
    BankApplicationQueryDenial,
> {
    execute_estate_customer_disclosure_action_with(
        runtime,
        principal,
        estate,
        EstateAction::ViewRestrictedEstate {
            estate,
            field: RestrictedBankField::CustomerIdentity,
            purpose: EstateCapabilityPurpose::IdentityVerification,
        },
        controls,
        observe,
    )
}

pub(crate) fn execute_estate_customer_disclosure_action_with<Observation>(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    estate: EstateCaseId,
    action: EstateAction,
    controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
    observe: impl FnOnce(&EstateCustomerDisclosurePlan<'_>) -> Observation,
) -> Result<
    (
        WorthQueryPublishedApplicationResult<
            EstateCustomerDisclosureQuery,
            EstateCustomerDisclosure,
        >,
        Observation,
    ),
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
    let capability_access = application
        .prepare_capability_access(
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
    let observation = observe(&plan);
    let result = application
        .execute_application_query_one_shot(plan)
        .map_err(BankApplicationQueryDenial::Execution)?;

    Ok((
        publish_application_result(result.into_admitted_disclosed()),
        observation,
    ))
}
