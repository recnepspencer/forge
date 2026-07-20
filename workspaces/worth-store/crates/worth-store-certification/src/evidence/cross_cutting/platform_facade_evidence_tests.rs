use crate::{
    InMemoryPhysicalFormatModelEvidenceReport, InMemoryPhysicalFormatModelEvidenceRow,
    RuntimeVerifierRelationship, ScenarioDenialBoundary,
};
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_physical_format::{
    InMemoryPhysicalFormatModel, InMemoryPhysicalFormatModelRequest, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalSegmentId,
    PlatformPhysicalAppendRequest,
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
    facade
        .page_access()
        .locate_record(append.reference())
        .expect("locate through facade");
    facade
        .publish_physical_root()
        .expect("publish root through facade");
    let scan = facade
        .scan_physical_layout()
        .expect("scan through verifier");

    let report = InMemoryPhysicalFormatModelEvidenceReport::from_facade_evidence(
        InMemoryPhysicalFormatModelEvidenceRow::RuntimeVerifierParity,
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

    let report =
        InMemoryPhysicalFormatModelEvidenceReport::from_shortcut_counters(facade.counters())
            .expect("shortcut evidence row");

    assert!(report
        .shortcut_rejections()
        .contains(&ScenarioDenialBoundary::WholeStoreMaterialization));
    assert!(report
        .shortcut_rejections()
        .contains(&ScenarioDenialBoundary::BackendResidueGuessing));
}

fn open_facade() -> InMemoryPhysicalFormatModel {
    InMemoryPhysicalFormatModel::start_empty_model(
        readiness(),
        InMemoryPhysicalFormatModelRequest::physical_format_canonical(),
    )
    .expect("open S.1 facade")
}

fn readiness() -> AcceptedHandoffReadiness {
    AcceptedHandoffReadiness::from_foundational_handoff_artifacts(ROADMAP_2_S1_SCOPE, digest_set())
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

fn slot_cell() -> worth_store_physical_format::SlotGenerationCell {
    PhysicalGenerationAuthority::for_canonical_physical_format()
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
