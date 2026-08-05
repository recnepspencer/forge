use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateEmergencyAccessActivity, EstateEmergencyAccessActivityQuery,
        EstateEmergencyAccessActivityQueryParameters,
    },
    schema::{
        BankSchema, EstateCase, EstateCaseIdentityField, Principal,
        ViewEstateEmergencyProtectionCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{
        WorthQueryApplicationQueryAccessContext, WorthQueryApplicationQueryContinuation,
        WorthQueryApplicationQueryResumeControls, WorthQueryPrimaryGraphApplicationRuntime,
        WorthQueryPrincipalResolutionMode,
    },
    publication::domain_computation::{
        publish_application_result, WorthQueryPublishedApplicationResult,
    },
};

use super::{admission::BankEstateEmergencyAccessActivityAdmission, bounded::ActivityPlan};
use crate::BankApplicationQueryDenial;

pub type BankEstateEmergencyAccessActivityContinuation = WorthQueryApplicationQueryContinuation<
    BankSchema,
    EstateEmergencyAccessActivityQuery,
    EstateEmergencyAccessActivityQueryParameters,
    EstateEmergencyAccessActivity,
    EstateCase,
>;

pub struct BankEstateEmergencyAccessActivityPageResult {
    published: WorthQueryPublishedApplicationResult<
        EstateEmergencyAccessActivityQuery,
        EstateEmergencyAccessActivity,
    >,
    continuation: Option<BankEstateEmergencyAccessActivityContinuation>,
}

pub struct BankAdmittedEstateEmergencyAccessActivityContinuation<'a> {
    application: &'a WorthQueryPrimaryGraphApplicationRuntime<BankSchema>,
    plan: ActivityPlan<'a>,
}

impl BankEstateEmergencyAccessActivityPageResult {
    pub fn rows(&self) -> &[EstateEmergencyAccessActivity] {
        self.published.rows()
    }

    pub fn receipt(
        &self,
    ) -> &worth_query_host::facade::publication::domain_computation::WorthQueryApplicationQueryPublicationReceipt{
        self.published.receipt()
    }

    pub const fn continuation(&self) -> Option<&BankEstateEmergencyAccessActivityContinuation> {
        self.continuation.as_ref()
    }

    pub fn into_parts(
        self,
    ) -> (
        WorthQueryPublishedApplicationResult<
            EstateEmergencyAccessActivityQuery,
            EstateEmergencyAccessActivity,
        >,
        Option<BankEstateEmergencyAccessActivityContinuation>,
    ) {
        (self.published, self.continuation)
    }
}

impl BankAdmittedEstateEmergencyAccessActivityContinuation<'_> {
    pub fn execute(
        self,
    ) -> Result<BankEstateEmergencyAccessActivityPageResult, BankApplicationQueryDenial> {
        let page = self
            .application
            .execute_application_query_continuation_page(self.plan)
            .map_err(BankApplicationQueryDenial::ContinuationExecution)?;
        Ok(publish_page(page))
    }
}

impl BankEstateEmergencyAccessActivityAdmission<'_, '_, '_, '_> {
    pub(crate) fn page(
        self,
    ) -> Result<BankEstateEmergencyAccessActivityPageResult, BankApplicationQueryDenial> {
        let controls = worth_query_host::facade::primary_graph::WorthQueryApplicationQueryControls::current_continuation_page(
            self.controls.maximum_result_count(),
            self.controls.maximum_work(),
            self.controls.request(),
        );
        self.with_admitted(controls, |application, plan| {
            let page = application
                .execute_application_query_continuation_page(plan)
                .map_err(BankApplicationQueryDenial::ContinuationExecution)?;
            Ok(publish_page(page))
        })
    }

    pub(crate) fn resume(
        self,
        continuation: BankEstateEmergencyAccessActivityContinuation,
        controls: WorthQueryApplicationQueryResumeControls<'_>,
    ) -> Result<BankEstateEmergencyAccessActivityPageResult, BankApplicationQueryDenial> {
        self.readmit_resume(continuation, controls, |admitted| admitted.execute())
    }

    pub(crate) fn readmit_resume<Output>(
        self,
        continuation: BankEstateEmergencyAccessActivityContinuation,
        controls: WorthQueryApplicationQueryResumeControls<'_>,
        after_readmission: impl for<'admitted> FnOnce(
            BankAdmittedEstateEmergencyAccessActivityContinuation<'admitted>,
        )
            -> Result<Output, BankApplicationQueryDenial>,
    ) -> Result<Output, BankApplicationQueryDenial> {
        let application = self.runtime.application_runtime();
        let query = application
            .installed_schema()
            .application_query(EstateEmergencyAccessActivityQuery::reference())
            .map_err(BankApplicationQueryDenial::Installation)?;
        let capability = application
            .installed_schema()
            .capability(
                ViewEstateEmergencyProtectionCapability::reference(),
                ViewRestrictedEstateOperation::reference(),
            )
            .map_err(BankApplicationQueryDenial::CapabilityInstallation)?;
        let capability_access = application
            .admit_approved_elevation_access(
                self.approved,
                self.principal.query(),
                &capability,
                self.request.capability_request(),
                controls.request_scope(),
            )
            .map_err(BankApplicationQueryDenial::CapabilityAdmission)?;
        let scope = application
            .resolve_entity(
                EstateCaseIdentityField::reference(),
                self.request.estate(),
                controls.request_scope(),
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankApplicationQueryDenial::ScopeResolution)?;
        let access = WorthQueryApplicationQueryAccessContext::<
            BankSchema,
            Principal,
            BankPrincipalId,
            EstateCase,
        >::new(self.principal.query(), &scope);
        let plan = application
            .readmit_governed_application_query_continuation(
                &query,
                &access,
                capability_access,
                ApplicationQueryParameterSet::<EstateEmergencyAccessActivityQuery>::new(),
                continuation,
                controls,
            )
            .map_err(BankApplicationQueryDenial::Admission)?;
        after_readmission(BankAdmittedEstateEmergencyAccessActivityContinuation {
            application,
            plan,
        })
    }
}

fn publish_page(
    page: worth_query_host::facade::primary_graph::WorthQueryApplicationContinuationPageResult<
        BankSchema,
        EstateEmergencyAccessActivityQuery,
        EstateEmergencyAccessActivityQueryParameters,
        EstateEmergencyAccessActivity,
        EstateCase,
    >,
) -> BankEstateEmergencyAccessActivityPageResult {
    let (admitted, continuation) = page.into_admitted_disclosed();
    BankEstateEmergencyAccessActivityPageResult {
        published: publish_application_result(admitted),
        continuation,
    }
}
