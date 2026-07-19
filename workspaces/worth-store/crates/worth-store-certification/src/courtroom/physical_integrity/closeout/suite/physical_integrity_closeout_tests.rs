use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_readiness::{
    close_physical_substrate_readiness, PhysicalSubstrateReadinessDenialKind,
};

#[test]
fn physical_integrity_closeout_cannot_start_from_foundational_digests() {
    let denial = close_physical_substrate_readiness(foundational_readiness())
        .expect_err("physical-integrity certification has no synthetic substrate authority");

    assert_eq!(
        denial.kind(),
        PhysicalSubstrateReadinessDenialKind::PhysicalFoundationReconstructionOpen
    );
}

fn foundational_readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(
        ROADMAP_2_S1_SCOPE,
        HandoffEvidenceDigestSet::new(
            digest("backend"),
            digest("deferred"),
            digest("harness"),
            digest("terms"),
            digest("audit"),
            digest("complexity"),
            digest("provenance"),
        ),
    )
    .expect("foundational handoff fixture is structurally valid")
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:c2-integrity-{name}"))
        .expect("non-empty test digest is structurally valid")
}
