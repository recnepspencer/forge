use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateEmergencyAccessActivity, EstateEmergencyAccessActivityQuery,
        EstateEmergencyAccessActivityQueryParameters,
    },
    schema::{BankSchema, EstateCase, Principal},
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{
        WorthQueryAdmittedApplicationQueryPlan, WorthQueryApplicationHistoricalRead,
        WorthQueryApplicationQueryControls, WorthQueryPrimaryGraphApplicationRuntime,
    },
    publication::domain_computation::{
        publish_application_result, WorthQueryPublishedApplicationResult,
    },
};

use super::admission::BankEstateEmergencyAccessActivityAdmission;
use crate::{BankApplicationQueryDenial, BankPreviewSession};

pub type BankEstateEmergencyAccessActivityResult = WorthQueryPublishedApplicationResult<
    EstateEmergencyAccessActivityQuery,
    EstateEmergencyAccessActivity,
>;

pub(super) type ActivityPlan<'a> = WorthQueryAdmittedApplicationQueryPlan<
    'a,
    BankSchema,
    EstateEmergencyAccessActivityQuery,
    EstateEmergencyAccessActivityQueryParameters,
    EstateEmergencyAccessActivity,
    Principal,
    BankPrincipalId,
    EstateCase,
>;

pub struct BankAdmittedEstateEmergencyAccessActivityHistorical<'a> {
    application: &'a WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    plan: ActivityPlan<'a>,
}

pub struct BankAdmittedEstateEmergencyAccessActivityPreview<'a> {
    application: &'a WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    plan: ActivityPlan<'a>,
}

impl BankAdmittedEstateEmergencyAccessActivityHistorical<'_> {
    pub fn execute(
        self,
    ) -> Result<BankEstateEmergencyAccessActivityResult, BankApplicationQueryDenial> {
        let result = self
            .application
            .execute_application_query_historical(self.plan)
            .map_err(BankApplicationQueryDenial::from_historical_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    }
}

impl BankAdmittedEstateEmergencyAccessActivityPreview<'_> {
    pub fn execute(
        self,
    ) -> Result<BankEstateEmergencyAccessActivityResult, BankApplicationQueryDenial> {
        let result = self
            .application
            .execute_application_query_preview(self.plan)
            .map_err(BankApplicationQueryDenial::from_preview_execution)?;
        Ok(publish_application_result(result.into_admitted_disclosed()))
    }
}

impl BankEstateEmergencyAccessActivityAdmission<'_, '_, '_, '_> {
    pub(crate) fn one_shot(
        self,
    ) -> Result<BankEstateEmergencyAccessActivityResult, BankApplicationQueryDenial> {
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
            BankAdmittedEstateEmergencyAccessActivityHistorical<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let basis = self
            .runtime
            .application_runtime()
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
            after_admission(BankAdmittedEstateEmergencyAccessActivityHistorical {
                application,
                plan,
            })
        })
    }

    pub(crate) fn preview<Output>(
        self,
        session: &BankPreviewSession,
        after_admission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccessActivityPreview<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let basis =
            session.admit_basis(self.runtime.application_runtime(), self.controls.request())?;
        let controls = WorthQueryApplicationQueryControls::preview(
            basis,
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        );
        self.with_admitted(controls, |application, plan| {
            after_admission(BankAdmittedEstateEmergencyAccessActivityPreview { application, plan })
        })
    }

    pub(super) fn with_admitted<Output>(
        self,
        controls: WorthQueryApplicationQueryControls<'_, BankSchema>,
        after_admission: impl for<'admitted> FnOnce(
            &'admitted WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
            ActivityPlan<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let query = application
            .installed_schema()
            .application_query(EstateEmergencyAccessActivityQuery::reference())
            .map_err(BankApplicationQueryDenial::from_installation)?;
        let capability = application
            .installed_schema()
            .capability(
                bank_domain::schema::ViewEstateEmergencyProtectionCapability::reference(),
                bank_domain::schema::ViewRestrictedEstateOperation::reference(),
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
                bank_domain::schema::EstateCaseIdentityField::reference(),
                self.request.estate(),
                controls.request_scope(),
                worth_query_host::facade::primary_graph::WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankApplicationQueryDenial::from_scope_resolution)?;
        let access =
            worth_query_host::facade::primary_graph::WorthQueryApplicationQueryAccessContext::<
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
                ApplicationQueryParameterSet::<EstateEmergencyAccessActivityQuery>::new(),
                controls,
            )
            .map_err(BankApplicationQueryDenial::from_admission)?;
        after_admission(application, plan)
    }
}
