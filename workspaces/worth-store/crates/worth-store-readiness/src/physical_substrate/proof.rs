use super::{
    PhysicalSubstrateReadiness, PhysicalSubstrateReadinessDenial,
    PhysicalSubstrateReadinessDenialKind,
};
use worth_store_contracts::{AcceptedHandoffReadiness, RoadmapScope, ROADMAP_2_S1_SCOPE};

/// Historical S.1 closeout authority retained only so downstream code receives
/// an explicit denial while the physical foundation is reconstructed.
#[derive(Debug)]
pub struct PhysicalSubstrateCloseoutReceipt {
    scope: RoadmapScope,
}

impl PhysicalSubstrateCloseoutReceipt {
    pub const fn scope(&self) -> RoadmapScope {
        self.scope
    }
}

/// Physical-substrate closeout is intentionally unavailable until C.13 can
/// derive it from the reconstructed file-backed runtime.
pub fn close_physical_substrate_readiness(
    readiness: AcceptedHandoffReadiness,
) -> Result<PhysicalSubstrateCloseoutReceipt, PhysicalSubstrateReadinessDenial> {
    if readiness.scope() != ROADMAP_2_S1_SCOPE {
        return Err(PhysicalSubstrateReadinessDenial::new(
            PhysicalSubstrateReadinessDenialKind::WrongRoadmapScope,
        ));
    }
    Err(reconstruction_open())
}

/// A historical receipt cannot be replayed into readiness while reconstruction
/// is open, even if one remains in a caller compiled against an older surface.
pub fn prove_physical_substrate_readiness(
    _closeout: PhysicalSubstrateCloseoutReceipt,
) -> Result<PhysicalSubstrateReadiness, PhysicalSubstrateReadinessDenial> {
    Err(reconstruction_open())
}

const fn reconstruction_open() -> PhysicalSubstrateReadinessDenial {
    PhysicalSubstrateReadinessDenial::new(
        PhysicalSubstrateReadinessDenialKind::PhysicalFoundationReconstructionOpen,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use worth_store_contracts::{HandoffEvidenceDigestSet, StableDigest};

    #[test]
    fn foundational_handoff_cannot_close_the_physical_substrate() {
        let denial = close_physical_substrate_readiness(readiness())
            .expect_err("digest handoff and heap substrate cannot close physical storage");

        assert_eq!(
            denial.kind(),
            PhysicalSubstrateReadinessDenialKind::PhysicalFoundationReconstructionOpen
        );
    }

    fn readiness() -> AcceptedHandoffReadiness {
        AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
            ROADMAP_2_S1_SCOPE,
            digest_set(),
        )
        .expect("S.1 foundational handoff")
    }

    fn digest_set() -> HandoffEvidenceDigestSet {
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        )
    }

    fn digest(name: &str) -> StableDigest {
        StableDigest::new(format!("sha256:c2-{name}"))
            .expect("non-empty test digest is structurally valid")
    }
}
