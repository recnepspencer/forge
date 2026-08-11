use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateEmergencyAccountDetails, EstateEmergencyAccountDetailsQuery,
        EstateEmergencyAccountDetailsQueryParameters, EstateEmergencyAccountDetailsRequest,
    },
    schema::{
        BankSchema, EstateCase, EstateCaseIdentityField, Principal,
        ViewEstateEmergencyProtectionCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationHistoricalRead,
        WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryControls,
        WorthQueryApprovedElevation, WorthQueryPrimaryGraphApplicationRuntime,
        WorthQueryPrincipalResolutionMode,
    },
    publication::domain_computation::{
        publish_application_result, WorthQueryPublishedApplicationResult,
    },
};

use super::super::{BankApplicationQueryDenial, BankPreviewSession};
use crate::{BankAuthenticatedPrincipal, BankIdentityRuntime, BankReadControls};

type EmergencyAccountDetailsPlan<'a> = WorthQueryAdmittedApplicationQueryPlan<
    'a,
    BankSchema,
    EstateEmergencyAccountDetailsQuery,
    EstateEmergencyAccountDetailsQueryParameters,
    EstateEmergencyAccountDetails,
    Principal,
    BankPrincipalId,
    EstateCase,
>;

pub type BankEstateEmergencyAccountDetailsResult = WorthQueryPublishedApplicationResult<
    EstateEmergencyAccountDetailsQuery,
    EstateEmergencyAccountDetails,
>;

/// Opaque historical-lane authority admitted by Query for one exact elevation.
pub struct BankAdmittedEstateEmergencyAccountDetailsHistorical<'a> {
    application: &'a WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    plan: EmergencyAccountDetailsPlan<'a>,
}

/// Opaque preview-lane authority admitted by Query for one exact elevation.
pub struct BankAdmittedEstateEmergencyAccountDetailsPreview<'a> {
    application: &'a WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    plan: EmergencyAccountDetailsPlan<'a>,
}

pub(crate) struct BankEstateEmergencyAccountDetailsAdmission<'a> {
    runtime: &'a BankIdentityRuntime,
    principal: &'a BankAuthenticatedPrincipal,
    request: EstateEmergencyAccountDetailsRequest,
    approved: &'a WorthQueryApprovedElevation,
    controls: &'a BankReadControls,
}

impl BankAdmittedEstateEmergencyAccountDetailsHistorical<'_> {
    pub fn execute(
        self,
    ) -> Result<BankEstateEmergencyAccountDetailsResult, BankApplicationQueryDenial> {
        let result = self
            .application
            .execute_application_query_historical(self.plan)
            .map_err(BankApplicationQueryDenial::from_historical_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    }
}

impl BankAdmittedEstateEmergencyAccountDetailsPreview<'_> {
    pub fn execute(
        self,
    ) -> Result<BankEstateEmergencyAccountDetailsResult, BankApplicationQueryDenial> {
        let result = self
            .application
            .execute_application_query_preview(self.plan)
            .map_err(BankApplicationQueryDenial::from_preview_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    }
}

impl<'a> BankEstateEmergencyAccountDetailsAdmission<'a> {
    pub(crate) const fn new(
        runtime: &'a BankIdentityRuntime,
        principal: &'a BankAuthenticatedPrincipal,
        request: EstateEmergencyAccountDetailsRequest,
        approved: &'a WorthQueryApprovedElevation,
        controls: &'a BankReadControls,
    ) -> Self {
        Self {
            runtime,
            principal,
            request,
            approved,
            controls,
        }
    }

    pub(crate) fn one_shot(
        self,
    ) -> Result<BankEstateEmergencyAccountDetailsResult, BankApplicationQueryDenial> {
        let controls = WorthQueryApplicationQueryControls::current_one_shot(
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        );
        self.with_admitted(controls, |application, plan| {
            let result = application
                .execute_application_query_one_shot(plan)
                .map_err(BankApplicationQueryDenial::from_execution)?;
            Ok(publish_application_result(result.into_admitted_disclosed()))
        })
    }

    pub(crate) fn historical<Output>(
        self,
        after_admission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccountDetailsHistorical<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let basis = application
            .admit_application_historical_basis(
                WorthQueryApplicationHistoricalRead::at_application_commit(
                    self.approved.approval_commit_receipt(),
                ),
                self.controls.request(),
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        let controls = WorthQueryApplicationQueryControls::historical(
            basis,
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        );
        self.with_admitted(controls, |application, plan| {
            after_admission(BankAdmittedEstateEmergencyAccountDetailsHistorical {
                application,
                plan,
            })
        })
    }

    pub(crate) fn preview<Output>(
        self,
        session: &BankPreviewSession,
        after_admission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccountDetailsPreview<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let basis = session.admit_basis(application, self.controls.request())?;
        let controls = WorthQueryApplicationQueryControls::preview(
            basis,
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        );
        self.with_admitted(controls, |application, plan| {
            after_admission(BankAdmittedEstateEmergencyAccountDetailsPreview { application, plan })
        })
    }

    fn with_admitted<Output>(
        self,
        controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
        after_admission: impl for<'admitted> FnOnce(
            &'admitted WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
            EmergencyAccountDetailsPlan<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let query = application
            .installed_schema()
            .application_query(EstateEmergencyAccountDetailsQuery::reference())
            .map_err(BankApplicationQueryDenial::from_installation)?;
        let capability = application
            .installed_schema()
            .capability(
                ViewEstateEmergencyProtectionCapability::reference(),
                ViewRestrictedEstateOperation::reference(),
            )
            .map_err(BankApplicationQueryDenial::from_capability_installation)?;
        let capability_access = application
            .admit_approved_elevation_access(
                self.approved,
                self.principal.query(),
                &capability,
                self.request.capability_request(),
                controls.request_scope(),
            )
            .map_err(BankApplicationQueryDenial::from_capability_admission)?;
        let scope = application
            .resolve_entity(
                EstateCaseIdentityField::reference(),
                self.request.estate(),
                controls.request_scope(),
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankApplicationQueryDenial::from_scope_resolution)?;
        let access = WorthQueryApplicationQueryAccessContext::<
            BankSchema,
            Principal,
            BankPrincipalId,
            EstateCase,
        >::new(self.principal.query(), &scope);
        let plan = application
            .admit_governed_application_query(
                &query,
                &access,
                capability_access,
                ApplicationQueryParameterSet::<EstateEmergencyAccountDetailsQuery>::new(),
                controls,
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        after_admission(application, plan)
    }
}

pub(crate) fn execute_estate_emergency_account_details(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
    request: EstateEmergencyAccountDetailsRequest,
    approved: &WorthQueryApprovedElevation,
    controls: &BankReadControls,
) -> Result<BankEstateEmergencyAccountDetailsResult, BankApplicationQueryDenial> {
    BankEstateEmergencyAccountDetailsAdmission::new(runtime, principal, request, approved, controls)
        .one_shot()
}
