use crate::basis_lifecycle::BasisOperationLane;
use crate::runtime::WorthQueryWorkspace;

use super::super::projection_lifecycle::WorthQueryOperationalProjection;
use super::super::{WorthQueryCurrentDomainProjection, WorthQueryLiveBoundDomainProjection};
use super::{
    admission::stopped, WorthQueryAdmittedProjectionSharing,
    WorthQueryCheckedSharedOwnerRegistration, WorthQueryProjectionSharingCounters,
    WorthQueryProjectionSharingDenialKind, WorthQueryProjectionSharingOutcome,
    WorthQuerySharedLiveProjectionLease, WorthQuerySharedLiveProjectionPair,
};

pub(super) fn register_shared_pair<D, O, F, L: BasisOperationLane>(
    live: WorthQueryLiveBoundDomainProjection<D, O, F, L>,
    candidate: WorthQueryCurrentDomainProjection<D, O, F, L>,
    admitted: WorthQueryAdmittedProjectionSharing,
    workspace: &mut WorthQueryWorkspace,
    mut counters: WorthQueryProjectionSharingCounters,
) -> WorthQueryProjectionSharingOutcome<D, O, F, L> {
    let live_identity = live.identity().to_string();
    let candidate_identity = candidate.identity().to_string();
    let live_generation = live.lifecycle_basis().capability_generation();
    let candidate_generation = candidate.lifecycle_basis().capability_generation();
    let (source, proof, handle, receipt, provenance) = live.into_owner().into_parts();
    let capability = std::sync::Arc::clone(handle.workspace_capability());
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
                candidate,
                WorthQueryProjectionSharingDenialKind::LiveOwnerRegistration,
                "live owner receipt does not bind its exact source and admitted closure",
                counters,
            );
        }
    };
    let registration = match workspace.register_shared_projection_owner(bundle) {
        Ok(registration) => registration,
        Err(bundle) => {
            let (handle, receipt, provenance, _closure, _admitted) = bundle.into_parts();
            let owner = WorthQueryOperationalProjection::from_parts(
                source, proof, handle, receipt, provenance,
            );
            return stopped(
                WorthQueryLiveBoundDomainProjection::from_owner(owner),
                candidate,
                WorthQueryProjectionSharingDenialKind::LiveOwnerRegistration,
                "live projection is not an active route in this workspace",
                counters,
            );
        }
    };
    let (candidate_source, _, _) = candidate.into_live_parts();
    counters.owner_registrations = 1;
    counters.lease_issues = 2;
    let subject = WorthQuerySharedLiveProjectionLease::new(
        source,
        live_identity,
        live_generation,
        std::sync::Arc::clone(&capability),
        registration.subject,
    );
    let candidate = WorthQuerySharedLiveProjectionLease::new(
        candidate_source,
        candidate_identity,
        candidate_generation,
        capability,
        registration
            .candidate
            .expect("pair admission must issue a candidate lease"),
    );
    debug_assert_eq!(subject.owner_identity(), registration.owner);
    WorthQueryProjectionSharingOutcome::Shared(WorthQuerySharedLiveProjectionPair::new(
        subject, candidate, counters,
    ))
}
