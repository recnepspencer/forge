use crate::physical_substrate_certification_authority::{
    certify_physical_page_segment_extent_substrate, closeout_run_without_legacy_overclaim_row,
    closeout_run_without_shortcut_row,
};
use crate::{
    PhysicalPageSegmentExtentSubstrateCloseout, PhysicalSubstrateCloseoutDenial,
    PhysicalSubstrateCloseoutStoryRow, PlatformPhysicalFacadeEvidenceRow,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_physical_format::PhysicalOperationKind;
use forge_store_readiness::{
    close_s1_physical_substrate_readiness, prove_s2_physical_substrate_readiness,
};

#[test]
fn physical_page_segment_extent_substrate_closeout_mints_s2_readiness() {
    let closeout = certify_physical_page_segment_extent_substrate().unwrap();
    let s2_readiness = closeout.into_s2_readiness();

    assert!(s2_readiness.is_sealed());
    assert_eq!(s2_readiness.scope(), ROADMAP_2_S1_SCOPE);
}

#[test]
fn physical_substrate_authority_exports_interpretable_closeout_evidence() {
    let closeout = certify_physical_page_segment_extent_substrate().unwrap();
    let evidence = closeout.evidence();

    assert_eq!(closeout.scope(), ROADMAP_2_S1_SCOPE);
    assert_eq!(
        closeout.run().run_id().as_str(),
        "physical_page_segment_extent_substrate_run"
    );
    assert!(!evidence.story().is_empty());
    assert!(!evidence.facade().is_empty());
    assert!(!evidence.manifest().is_empty());
    assert!(!evidence.offline_verifier().is_empty());
    assert!(!evidence.page_records().is_empty());
    assert!(!evidence.extent_records().is_empty());
    assert!(!evidence.identity().is_empty());
    assert_eq!(
        evidence.complexity().len(),
        PhysicalOperationKind::s1_required().len()
    );
    assert_eq!(evidence.foundation().scope(), ROADMAP_2_S1_SCOPE);
    assert_eq!(
        evidence.platform_grade_witness().scope(),
        ROADMAP_2_S1_SCOPE
    );
}

#[test]
fn physical_substrate_authority_mints_s2_readiness_without_raw_descriptors() {
    let s2_readiness = prove_s2_physical_substrate_readiness(
        close_s1_physical_substrate_readiness(
            AcceptedHandoffReadiness::from_s0_artifacts(
                ROADMAP_2_S1_SCOPE,
                HandoffEvidenceDigestSet::new(
                    StableDigest::new("sha256:physical-substrate-backend").unwrap(),
                    StableDigest::new("sha256:physical-substrate-deferred").unwrap(),
                    StableDigest::new("sha256:physical-substrate-harness").unwrap(),
                    StableDigest::new("sha256:physical-substrate-terms").unwrap(),
                    StableDigest::new("sha256:physical-substrate-audit").unwrap(),
                    StableDigest::new("sha256:physical-substrate-complexity").unwrap(),
                    StableDigest::new("sha256:physical-substrate-provenance").unwrap(),
                ),
            )
            .unwrap(),
        )
        .unwrap(),
    )
    .unwrap();

    assert!(s2_readiness.is_sealed());
    assert_eq!(s2_readiness.scope(), ROADMAP_2_S1_SCOPE);
}

#[test]
fn closeout_rejects_missing_shortcut_evidence_before_s2_readiness() {
    let run = closeout_run_without_shortcut_row().unwrap();
    let denial = PhysicalPageSegmentExtentSubstrateCloseout::admit(run).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSubstrateCloseoutDenial::MissingFacadeRow(
            PlatformPhysicalFacadeEvidenceRow::ShortcutRejections
        )
    );
}

#[test]
fn closeout_rejects_missing_legacy_overclaim_story_before_s2_readiness() {
    let run = closeout_run_without_legacy_overclaim_row().unwrap();
    let denial = PhysicalPageSegmentExtentSubstrateCloseout::admit(run).unwrap_err();

    assert_eq!(
        denial,
        PhysicalSubstrateCloseoutDenial::MissingStoryRow(
            PhysicalSubstrateCloseoutStoryRow::LegacyOverclaimRejected
        )
    );
}
