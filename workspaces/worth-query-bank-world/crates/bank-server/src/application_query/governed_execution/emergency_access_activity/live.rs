use bank_domain::{
    model::BankPrincipalId,
    queries::{
        EstateEmergencyAccessActivity, EstateEmergencyAccessActivityLiveCause,
        EstateEmergencyAccessActivityQuery, EstateEmergencyAccessActivityQueryParameters,
    },
    schema::{
        BankSchema, EmergencyAccess, EstateCase, EstateCaseIdentityField, Principal,
        ViewEstateEmergencyProtectionCapability, ViewRestrictedEstateOperation,
    },
};
use worth_query_host::facade::{
    declaration::application_query::ApplicationQueryParameterSet,
    primary_graph::{
        WorthQueryApplicationLiveCauseDenialKind, WorthQueryApplicationLiveCloseOutcome,
        WorthQueryApplicationLiveControls, WorthQueryApplicationLiveLease,
        WorthQueryApplicationLiveOutcome, WorthQueryApplicationLiveOverflow,
        WorthQueryApplicationProjectionDenialKind, WorthQueryOperationAuthorizationDenial,
        WorthQueryPrincipalResolutionMode,
    },
    publication::domain_computation::{
        publish_application_result, WorthQueryPublishedApplicationResult,
    },
};

use super::admission::BankEstateEmergencyAccessActivityAdmission;
use crate::BankApplicationQueryDenial;

type ActivityLiveLease<'runtime, 'principal> = WorthQueryApplicationLiveLease<
    'runtime,
    'principal,
    BankSchema,
    EstateEmergencyAccessActivityQuery,
    EstateEmergencyAccessActivityQueryParameters,
    EstateEmergencyAccessActivity,
    Principal,
    BankPrincipalId,
    EstateCase,
    EmergencyAccess,
    EstateEmergencyAccessActivityLiveCause,
>;

pub struct BankEstateEmergencyAccessActivityLiveLease<'runtime, 'principal> {
    query: ActivityLiveLease<'runtime, 'principal>,
}

pub struct BankEstateEmergencyAccessActivityLiveUpdate {
    commit_ordinal: u64,
    published: WorthQueryPublishedApplicationResult<
        EstateEmergencyAccessActivityQuery,
        EstateEmergencyAccessActivity,
    >,
}

pub enum BankEstateEmergencyAccessActivityLiveOutcome {
    Delivered(BankEstateEmergencyAccessActivityLiveUpdate),
    Pending,
    Overflow(WorthQueryApplicationLiveOverflow),
    AuthorizationDenied(Box<WorthQueryOperationAuthorizationDenial>),
    StalePrincipal,
    StaleScope,
    ProjectionDenied(WorthQueryApplicationProjectionDenialKind),
    CauseDenied(WorthQueryApplicationLiveCauseDenialKind),
    Cancelled,
    DeadlineExceeded,
    Closed,
    Unavailable,
}

impl BankEstateEmergencyAccessActivityLiveUpdate {
    pub const fn commit_ordinal(&self) -> u64 {
        self.commit_ordinal
    }

    pub fn rows(&self) -> &[EstateEmergencyAccessActivity] {
        self.published.rows()
    }

    pub fn receipt(
        &self,
    ) -> &worth_query_host::facade::publication::domain_computation::WorthQueryApplicationQueryPublicationReceipt{
        self.published.receipt()
    }
}

impl BankEstateEmergencyAccessActivityLiveLease<'_, '_> {
    pub fn buffered_cause_count(&self) -> usize {
        self.query.buffered_cause_count()
    }

    pub fn poll(&mut self) -> BankEstateEmergencyAccessActivityLiveOutcome {
        match self.query.poll() {
            WorthQueryApplicationLiveOutcome::Delivered(update) => {
                let commit_ordinal = update.commit_ordinal();
                let (_, admitted) = update.into_admitted_disclosed();
                BankEstateEmergencyAccessActivityLiveOutcome::Delivered(
                    BankEstateEmergencyAccessActivityLiveUpdate {
                        commit_ordinal,
                        published: publish_application_result(admitted),
                    },
                )
            }
            WorthQueryApplicationLiveOutcome::Pending => {
                BankEstateEmergencyAccessActivityLiveOutcome::Pending
            }
            WorthQueryApplicationLiveOutcome::Overflow(overflow) => {
                BankEstateEmergencyAccessActivityLiveOutcome::Overflow(overflow)
            }
            WorthQueryApplicationLiveOutcome::AuthorizationDenied(kind) => {
                BankEstateEmergencyAccessActivityLiveOutcome::AuthorizationDenied(kind)
            }
            WorthQueryApplicationLiveOutcome::StalePrincipal => {
                BankEstateEmergencyAccessActivityLiveOutcome::StalePrincipal
            }
            WorthQueryApplicationLiveOutcome::StaleScope => {
                BankEstateEmergencyAccessActivityLiveOutcome::StaleScope
            }
            WorthQueryApplicationLiveOutcome::ProjectionDenied(kind) => {
                BankEstateEmergencyAccessActivityLiveOutcome::ProjectionDenied(kind)
            }
            WorthQueryApplicationLiveOutcome::CauseDenied(kind) => {
                BankEstateEmergencyAccessActivityLiveOutcome::CauseDenied(kind)
            }
            WorthQueryApplicationLiveOutcome::Cancelled => {
                BankEstateEmergencyAccessActivityLiveOutcome::Cancelled
            }
            WorthQueryApplicationLiveOutcome::DeadlineExceeded => {
                BankEstateEmergencyAccessActivityLiveOutcome::DeadlineExceeded
            }
            WorthQueryApplicationLiveOutcome::Closed => {
                BankEstateEmergencyAccessActivityLiveOutcome::Closed
            }
            WorthQueryApplicationLiveOutcome::Unavailable => {
                BankEstateEmergencyAccessActivityLiveOutcome::Unavailable
            }
        }
    }

    pub fn close(self) -> WorthQueryApplicationLiveCloseOutcome {
        self.query.close()
    }
}

impl<'runtime, 'principal>
    BankEstateEmergencyAccessActivityAdmission<'runtime, 'principal, '_, '_>
{
    pub(crate) fn subscribe(
        self,
        controls: WorthQueryApplicationLiveControls,
    ) -> Result<
        BankEstateEmergencyAccessActivityLiveLease<'runtime, 'principal>,
        BankApplicationQueryDenial,
    > {
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
                controls.request(),
            )
            .map_err(BankApplicationQueryDenial::CapabilityAdmission)?;
        let scope = application
            .resolve_entity(
                EstateCaseIdentityField::reference(),
                self.request.estate(),
                controls.request(),
                WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .map_err(BankApplicationQueryDenial::ScopeResolution)?;
        let query = application
            .open_governed_application_query_live::<
                EstateEmergencyAccessActivityQuery,
                EstateEmergencyAccessActivityQueryParameters,
                EstateEmergencyAccessActivity,
                Principal,
                BankPrincipalId,
                EstateCase,
                EmergencyAccess,
                EstateEmergencyAccessActivityLiveCause,
                _,
                _,
                _,
            >(
                query,
                self.principal.query(),
                scope,
                capability_access,
                ApplicationQueryParameterSet::<EstateEmergencyAccessActivityQuery>::new(),
                controls,
            )
            .map_err(BankApplicationQueryDenial::LiveOpen)?;
        Ok(BankEstateEmergencyAccessActivityLiveLease { query })
    }
}
