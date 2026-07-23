use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::super::projection_lifecycle::{
    admit_projection_promotion_core, WorthQueryOperationalProjection, WorthQueryProjectionCoreStop,
};
use super::super::WorthQueryLiveBoundDomainProjection;
use super::{
    WorthQueryAdmittedProjectionSharing, WorthQueryCheckedSharedOwnerRegistration,
    WorthQuerySharedLiveProjectionLease,
};

#[must_use = "a stopped singleton admission retains the exact live projection"]
pub enum WorthQueryProjectionLeaseAdmissionOutcome<D, O, F, L: BasisOperationLane> {
    Admitted(WorthQuerySharedLiveProjectionLease<D, O, F, L>),
    Stopped(WorthQueryProjectionLeaseAdmissionStop<D, O, F, L>),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryProjectionLeaseAdmissionDenialKind {
    ConsumerSupport,
    CoreAuthority,
    OwnerAuthority,
    Route,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryProjectionLeaseAdmissionCounters {
    pub core_preflight: super::super::WorthQueryProjectionPromotionCounters,
    pub support_checks: usize,
    pub owner_authority_checks: usize,
    pub owner_registration_attempts: usize,
    pub lease_issues: usize,
    pub unrelated_route_scans: usize,
}

pub struct WorthQueryProjectionLeaseAdmissionStop<D, O, F, L: BasisOperationLane> {
    live: WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    kind: WorthQueryProjectionLeaseAdmissionDenialKind,
    detail: String,
    counters: WorthQueryProjectionLeaseAdmissionCounters,
}

impl<D, O, F, L: BasisOperationLane> WorthQueryProjectionLeaseAdmissionStop<D, O, F, L> {
    pub const fn kind(&self) -> WorthQueryProjectionLeaseAdmissionDenialKind {
        self.kind
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub const fn counters(&self) -> WorthQueryProjectionLeaseAdmissionCounters {
        self.counters
    }

    pub fn into_live(self) -> WorthQueryLiveBoundDomainProjection<D, O, F, L> {
        self.live
    }
}

impl<D: 'static, O, F, L: BasisOperationLane> WorthQueryLiveBoundDomainProjection<D, O, F, L> {
    pub fn into_managed_lease(
        self,
        workspace: &mut WorthQueryWorkspace,
    ) -> WorthQueryProjectionLeaseAdmissionOutcome<D, O, F, L> {
        let mut counters = WorthQueryProjectionLeaseAdmissionCounters::default();
        match admit_projection_promotion_core(self.snapshot(), self.lifecycle_basis(), workspace) {
            Ok(admitted) => counters.core_preflight = admitted.counters,
            Err(stop) => {
                return stopped(
                    self,
                    WorthQueryProjectionLeaseAdmissionDenialKind::CoreAuthority,
                    core_stop_detail(stop),
                    counters,
                );
            }
        }
        counters.support_checks += 1;
        if self.snapshot().consumer_contract().support_posture(
            crate::domain_installation::WorthQueryConsumerSupportDimension::Sharing,
        ) != crate::domain_installation::WorthQueryConsumerSupportPosture::Supported
        {
            return stopped(
                self,
                WorthQueryProjectionLeaseAdmissionDenialKind::ConsumerSupport,
                "managed lease requires Supported consumer sharing posture",
                counters,
            );
        }
        let identity = self.identity().to_string();
        let capability_generation = self.lifecycle_basis().capability_generation();
        let admitted = WorthQueryAdmittedProjectionSharing::singleton(self.snapshot());
        let (source, proof, handle, receipt, provenance) = self.into_owner().into_parts();
        let capability = std::sync::Arc::clone(handle.workspace_capability());
        counters.owner_authority_checks += 1;
        let bundle = match WorthQueryCheckedSharedOwnerRegistration::admit(
            &source, &proof, handle, receipt, provenance, admitted,
        ) {
            Ok(bundle) => bundle,
            Err((handle, receipt, provenance, _closure, _admitted)) => {
                let owner = WorthQueryOperationalProjection::from_parts(
                    source, proof, handle, receipt, provenance,
                );
                return stopped(
                    WorthQueryLiveBoundDomainProjection::from_owner(owner),
                    WorthQueryProjectionLeaseAdmissionDenialKind::OwnerAuthority,
                    "live owner receipt does not bind its exact source and admitted closure",
                    counters,
                );
            }
        };
        counters.owner_registration_attempts += 1;
        match workspace.register_singleton_projection_owner(bundle) {
            Ok(registration) => {
                counters.lease_issues += 1;
                WorthQueryProjectionLeaseAdmissionOutcome::Admitted(
                    WorthQuerySharedLiveProjectionLease::new(
                        source,
                        identity,
                        capability_generation,
                        capability,
                        registration.subject,
                    )
                    .with_singleton_admission_counters(counters),
                )
            }
            Err(bundle) => {
                let (handle, receipt, provenance, _closure, _admitted) = bundle.into_parts();
                let owner = WorthQueryOperationalProjection::from_parts(
                    source, proof, handle, receipt, provenance,
                );
                stopped(
                    WorthQueryLiveBoundDomainProjection::from_owner(owner),
                    WorthQueryProjectionLeaseAdmissionDenialKind::Route,
                    "live projection is not an active route in this workspace",
                    counters,
                )
            }
        }
    }
}

fn stopped<D, O, F, L: BasisOperationLane>(
    live: WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    kind: WorthQueryProjectionLeaseAdmissionDenialKind,
    detail: impl Into<String>,
    counters: WorthQueryProjectionLeaseAdmissionCounters,
) -> WorthQueryProjectionLeaseAdmissionOutcome<D, O, F, L> {
    WorthQueryProjectionLeaseAdmissionOutcome::Stopped(WorthQueryProjectionLeaseAdmissionStop {
        live,
        kind,
        detail: detail.into(),
        counters,
    })
}

fn core_stop_detail(stop: WorthQueryProjectionCoreStop) -> &'static str {
    match stop {
        WorthQueryProjectionCoreStop::Stale(_) => "live installation is stale",
        WorthQueryProjectionCoreStop::RebindRequired(_) => "live projection requires rebind",
        WorthQueryProjectionCoreStop::AuthorityRevalidationRequired(_) => {
            "live projection authority requires revalidation"
        }
        WorthQueryProjectionCoreStop::Denied { detail, .. } => detail,
    }
}
