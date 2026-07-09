use crate::{
    PhysicalManifestDiscoveryEvidenceDenial, PhysicalManifestDiscoveryEvidenceReport,
    PhysicalManifestDiscoveryEvidenceRow, PhysicalSubstrateLane,
};
use worth_store_physical_format::{
    AllocationClassKind, ManifestDiscoveryAuthority, PhysicalExtentId, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalRootManifest, PhysicalRootReference, PhysicalSegmentId,
};

#[test]
fn manifest_discovery_evidence_exports_real_root_report() {
    let fixture = manifest_fixture();
    let report = fixture.admitted_reopen();
    let evidence = PhysicalManifestDiscoveryEvidenceReport::from_manifest_report(
        PhysicalManifestDiscoveryEvidenceRow::RootManifestDiscovery,
        report,
    )
    .unwrap();

    assert_eq!(evidence.lane(), PhysicalSubstrateLane::HappyAuthority);
    assert_eq!(evidence.counters().root_manifest_read_count(), 1);
    assert_eq!(evidence.counters().segment_manifest_entry_count(), 1);
    assert_eq!(evidence.counters().extent_manifest_entry_count(), 1);
    assert_eq!(evidence.counters().free_space_map_entry_count(), 1);
}

#[test]
fn backend_residue_denial_certifies_from_real_manifest_denial() {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let residue_slot = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(2));
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = worth_store_physical_format::PhysicalManifestUniverseBuilder::s1(root)
        .segment(segment_cell)
        .publish();
    let report = discovery
        .reopen_from_root(&manifest, references.admit_root_publication(root))
        .unwrap();
    let denial = discovery.reject_backend_residue(report, references.admit_page_slot(residue_slot));

    let evidence = PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
        PhysicalManifestDiscoveryEvidenceRow::BackendResidueRejected,
        denial,
    )
    .unwrap();

    assert_eq!(evidence.lane(), PhysicalSubstrateLane::HostileFormat);
    assert_eq!(evidence.counters().backend_residue_rejection_count(), 1);
}

#[test]
fn free_space_reuse_stale_denial_certifies_generation_change() {
    let fixture = manifest_fixture();
    let old_free_space = fixture
        .generations
        .free_space_slot_cell(
            segment(7),
            page(3),
            slot(1),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(2));
    let denial = fixture
        .discovery
        .validate_free_space_reuse(
            fixture.admitted_reopen(),
            fixture.references.admit_free_space_reuse(old_free_space),
        )
        .unwrap_err();

    let evidence = PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
        PhysicalManifestDiscoveryEvidenceRow::FreeSpaceReuseGenerationChanged,
        denial,
    )
    .unwrap();

    assert_eq!(evidence.lane(), PhysicalSubstrateLane::HostileFormat);
    assert_eq!(evidence.counters().manifest_index_probe_count(), 1);
}

#[test]
fn stale_root_publication_denial_certifies_from_real_reopen_denial() {
    let fixture = manifest_fixture();
    let old_root = fixture
        .generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(4));
    let denial = fixture
        .discovery
        .reopen_from_root(
            &fixture.manifest,
            fixture.references.admit_root_publication(old_root),
        )
        .unwrap_err();

    let evidence = PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
        PhysicalManifestDiscoveryEvidenceRow::RootPublicationGenerationChanged,
        denial,
    )
    .unwrap();

    assert_eq!(evidence.lane(), PhysicalSubstrateLane::HostileFormat);
    assert_eq!(evidence.counters().root_manifest_read_count(), 1);
    assert_eq!(evidence.counters().root_manifest_entry_count(), 0);
}

#[test]
fn wrong_root_publication_denial_does_not_certify_generation_change() {
    let fixture = manifest_fixture();
    let wrong_root = fixture
        .generations
        .root_publication_cell(root_ref(2))
        .with_root_publication_generation(generation(5));
    let denial = fixture
        .discovery
        .reopen_from_root(
            &fixture.manifest,
            fixture.references.admit_root_publication(wrong_root),
        )
        .unwrap_err();

    let evidence_denial = PhysicalManifestDiscoveryEvidenceReport::from_manifest_denial(
        PhysicalManifestDiscoveryEvidenceRow::RootPublicationGenerationChanged,
        denial,
    )
    .unwrap_err();

    assert_eq!(
        evidence_denial,
        PhysicalManifestDiscoveryEvidenceDenial::MissingStaleRootPublicationGeneration
    );
}

struct ManifestFixture {
    generations: PhysicalGenerationAuthority,
    references: PhysicalReferenceAuthority,
    discovery: ManifestDiscoveryAuthority,
    manifest: PhysicalRootManifest,
}

impl ManifestFixture {
    fn admitted_reopen(&self) -> worth_store_physical_format::ManifestDiscoveryReport<'_> {
        self.discovery
            .reopen_from_root(
                &self.manifest,
                self.references
                    .admit_root_publication(self.manifest.root_publication()),
            )
            .unwrap()
    }
}

fn manifest_fixture() -> ManifestFixture {
    let generations = PhysicalGenerationAuthority::s1();
    let references = PhysicalReferenceAuthority::s1();
    let discovery = ManifestDiscoveryAuthority::s1();
    let segment_cell = generations
        .segment_cell(segment(7))
        .with_segment_generation(generation(1));
    let page_slot = generations
        .slot_cell(segment(7), page(3), slot(1))
        .with_slot_generation(generation(2));
    let extent_cell = generations
        .extent_cell(segment(7), extent(20))
        .with_extent_generation(generation(3));
    let free_space = generations
        .free_space_slot_cell(
            segment(7),
            page(3),
            slot(1),
            AllocationClassKind::OrdinaryRecordPage,
        )
        .unwrap()
        .with_free_space_generation(generation(4));
    let root = generations
        .root_publication_cell(root_ref(1))
        .with_root_publication_generation(generation(5));
    let manifest = worth_store_physical_format::PhysicalManifestUniverseBuilder::s1(root)
        .segment(segment_cell)
        .ordinary_page(page_slot)
        .extent(extent_cell)
        .free_space_reuse(free_space)
        .publish();
    ManifestFixture {
        generations,
        references,
        discovery,
        manifest,
    }
}

fn segment(value: u64) -> PhysicalSegmentId {
    PhysicalSegmentId::from_raw(value).unwrap()
}

fn page(value: u64) -> PhysicalPageId {
    PhysicalPageId::from_raw(value).unwrap()
}

fn extent(value: u64) -> PhysicalExtentId {
    PhysicalExtentId::from_raw(value).unwrap()
}

fn slot(value: u16) -> PhysicalRecordSlot {
    PhysicalRecordSlot::from_raw(value).unwrap()
}

fn root_ref(value: u64) -> PhysicalRootReference {
    PhysicalRootReference::from_raw(value).unwrap()
}

fn generation(value: u64) -> PhysicalGeneration {
    PhysicalGeneration::from_raw(value).unwrap()
}
