use crate::courtroom::physical_integrity::physical_substrate_certification_authority::certify_physical_page_segment_extent_substrate;
use crate::PhysicalSubstrateCertificationDenial;
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_readiness::{
    close_physical_substrate_readiness, PhysicalSubstrateReadinessDenialKind,
};

#[test]
fn certification_cannot_close_the_heap_physical_substrate() {
    let denial = certify_physical_page_segment_extent_substrate()
        .expect_err("physical closeout remains unavailable during reconstruction");

    assert_eq!(
        denial,
        PhysicalSubstrateCertificationDenial::PhysicalFoundationReconstructionOpen
    );
}

#[test]
fn foundational_handoff_cannot_bypass_certification_quarantine() {
    let denial = close_physical_substrate_readiness(readiness())
        .expect_err("foundational digests do not prove persisted bytes");

    assert_eq!(
        denial.kind(),
        PhysicalSubstrateReadinessDenialKind::PhysicalFoundationReconstructionOpen
    );
}

fn readiness() -> AcceptedHandoffReadiness {
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
    .expect("S.1 foundational handoff")
}

fn digest(name: &str) -> StableDigest {
    StableDigest::new(format!("sha256:c2-certification-{name}"))
        .expect("non-empty test digest is structurally valid")
}
