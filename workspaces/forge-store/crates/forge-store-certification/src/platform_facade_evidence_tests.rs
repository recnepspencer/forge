use crate::{
    PlatformPhysicalFacadeEvidenceReport, PlatformPhysicalFacadeEvidenceRow,
    RuntimeVerifierRelationship, ScenarioDenialBoundary,
};
use forge_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use forge_store_layout_indexes::layout_strategy_admission::phase19_page_rule;
use forge_store_physical_format::{
    PhysicalGeneration, PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot,
    PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalFacade,
    PlatformPhysicalOpenRequest,
};

#[test]
fn facade_scan_evidence_materializes_runtime_verifier_parity() {
    let mut facade = open_facade();
    let append = facade
        .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
            slot_cell(),
            b"cert",
        ))
        .expect("append through facade");
    let page_rule = phase19_page_rule().expect("phase-19 page rule");
    facade
        .page_layout(&page_rule)
        .expect("admitted page layout")
        .locate_record(append.reference())
        .expect("locate through facade");
    facade
        .publish_physical_root()
        .expect("publish root through facade");
    let scan = facade
        .scan_physical_layout()
        .expect("scan through verifier");

    let report = PlatformPhysicalFacadeEvidenceReport::from_facade_evidence(
        PlatformPhysicalFacadeEvidenceRow::RuntimeVerifierParity,
        &scan.platform_evidence(),
    )
    .expect("facade evidence row");

    assert_eq!(
        report.parity().relationship(),
        RuntimeVerifierRelationship::RuntimeMustMatchVerifier
    );
    assert!(report.observed_references() > 0);
}

#[test]
fn facade_shortcut_rejections_materialize_certification_trace() {
    let mut facade = open_facade();
    facade
        .reject_full_store_heap_materialization()
        .expect_err("materialization rejected");
    facade
        .reject_backend_residue_guess()
        .expect_err("residue rejected");

    let report = PlatformPhysicalFacadeEvidenceReport::from_shortcut_counters(facade.counters())
        .expect("shortcut evidence row");

    assert!(report
        .shortcut_rejections()
        .contains(&ScenarioDenialBoundary::WholeStoreMaterialization));
    assert!(report
        .shortcut_rejections()
        .contains(&ScenarioDenialBoundary::BackendResidueGuessing));
}

fn open_facade() -> PlatformPhysicalFacade {
    PlatformPhysicalFacade::open_s1(readiness(), PlatformPhysicalOpenRequest::s1_canonical())
        .expect("open S.1 facade")
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_s0_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
        .expect("S.1 handoff readiness")
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
    StableDigest::new(format!("sha256:{name}")).expect("non-empty digest")
}

fn slot_cell() -> forge_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::s1()
        .slot_cell(segment(1), page(1), slot(1))
        .with_slot_generation(generation(5))
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
