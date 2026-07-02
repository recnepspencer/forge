use crate::PhysicalReadStabilityAuthority;

use super::root_observation::{PostHazardRootObservation, PostProtectionPhysicalReadObservation};
use super::{
    PhysicalReadPlanAdmissionDenial, PhysicalReadReachabilityBarrier, ProtectedRootObservation,
    UnprotectedReadIntent,
};

#[derive(Debug, Clone)]
pub struct PublishedReaderHazard {
    intent: UnprotectedReadIntent,
    barrier: PhysicalReadReachabilityBarrier,
}

impl PublishedReaderHazard {
    pub fn publish(
        authority: &PhysicalReadStabilityAuthority,
        intent: UnprotectedReadIntent,
    ) -> Result<Self, PhysicalReadPlanAdmissionDenial> {
        require_intent_root_matches_authority(authority, &intent)?;
        match intent.release() {
            Some(release) if release.release_required() => {
                let barrier = PhysicalReadReachabilityBarrier::from_footprint_basis(
                    intent.protected_references().footprint_basis(),
                    release,
                );
                Ok(Self { intent, barrier })
            }
            _ => Err(PhysicalReadPlanAdmissionDenial::MissingReleaseSemantics),
        }
    }

    pub fn observe_authority_after_publication(
        self,
        authority: &PhysicalReadStabilityAuthority,
        observed: PostProtectionPhysicalReadObservation,
    ) -> Result<ProtectedRootObservation, PhysicalReadPlanAdmissionDenial> {
        if observed.hazard_barrier() != self.barrier {
            return Err(
                PhysicalReadPlanAdmissionDenial::PostProtectionObservationHazardMismatch {
                    expected_protected_references: self.barrier.protected_references(),
                    observed_protected_references: observed.hazard_barrier().protected_references(),
                },
            );
        }
        let observed =
            PostHazardRootObservation::from_published_hazard_authority(authority, observed)?;
        Ok(ProtectedRootObservation::from_published_hazard(
            self, observed,
        ))
    }

    pub(crate) fn intent(&self) -> &UnprotectedReadIntent {
        &self.intent
    }

    pub const fn reachability_barrier(&self) -> PhysicalReadReachabilityBarrier {
        self.barrier
    }
}

fn require_intent_root_matches_authority(
    authority: &PhysicalReadStabilityAuthority,
    intent: &UnprotectedReadIntent,
) -> Result<(), PhysicalReadPlanAdmissionDenial> {
    let expected = authority.root_epoch_basis();
    let observed = intent.root();
    if expected.epoch().get() == observed.epoch().get()
        && expected.manifest_epoch().get() == observed.manifest_epoch().get()
    {
        Ok(())
    } else {
        Err(PhysicalReadPlanAdmissionDenial::AuthorityRootMismatch {
            expected_root: expected.epoch().get(),
            observed_root: observed.epoch().get(),
            expected_manifest: expected.manifest_epoch().get(),
            observed_manifest: observed.manifest_epoch().get(),
        })
    }
}
