use std::path::Path;

use worth_store_physical_backend::observe_physical_backup_artifact;
use worth_store_physical_format::{
    PhysicalCurrentReachabilitySource, PhysicalExtentId, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalPageId, PhysicalRecordSlot, PhysicalReferenceAuthority,
    PhysicalSegmentId, PlatformPhysicalAppendRequest, PlatformPhysicalRootPublicationReport,
};
use worth_store_physical_isolation::{
    BackupArtifactCoverage, BackupArtifactFamily, BackupArtifactReference,
    CurrentGenerationPhysicalReference, GenerationCountedPhysicalReference,
};

pub(super) struct CurrentRootFixture {
    source: PhysicalCurrentReachabilitySource,
    publication: PlatformPhysicalRootPublicationReport,
}

impl CurrentRootFixture {
    pub(super) const fn source(&self) -> &PhysicalCurrentReachabilitySource {
        &self.source
    }
}

pub(super) fn current_root_fixture_with_shared_page() -> CurrentRootFixture {
    let mut runtime = crate::certification_scenario::backup_artifacts::open_physical_runtime();
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let segment = PhysicalSegmentId::from_raw(41).expect("segment");
    let page = PhysicalPageId::from_raw(9).expect("page");
    for slot in [1, 2] {
        let cell = generations
            .slot_cell(
                segment,
                page,
                PhysicalRecordSlot::from_raw(slot).expect("slot"),
            )
            .with_slot_generation(PhysicalGeneration::from_raw(7).expect("generation"));
        runtime
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(cell, b"record"))
            .expect("append page slot");
    }
    let extent = generations
        .extent_cell(segment, PhysicalExtentId::from_raw(12).expect("extent"))
        .with_extent_generation(PhysicalGeneration::from_raw(9).expect("generation"));
    runtime
        .append_physical_record(PlatformPhysicalAppendRequest::extent(
            extent,
            b"large-record",
        ))
        .expect("append extent");
    let publication = runtime.publish_physical_root().expect("publish root");
    let source = runtime
        .current_physical_reachability_source()
        .expect("runtime-issued current-root source");
    CurrentRootFixture {
        source,
        publication,
    }
}

pub(super) fn core_references_for_source(
    fixture: &CurrentRootFixture,
    directory: &Path,
) -> Vec<BackupArtifactReference> {
    let source = fixture.source();
    let layout = fixture.publication.persisted_layout();
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let root = source.manifest().root_publication();
    let root_reference = GenerationCountedPhysicalReference::from_admitted_reference(
        references.admit_root_publication(root),
    )
    .require_current_generation(root.generation())
    .expect("current root");
    let mut artifacts = vec![observed_reference(
        directory,
        BackupArtifactFamily::RootManifest,
        "runtime-root",
        layout
            .root_manifest_candidates()
            .first()
            .expect("runtime root bytes"),
        BackupArtifactCoverage::root_manifest(root.generation().get()).expect("root coverage"),
        root_reference,
    )];
    artifacts.extend(source.page_cells().iter().enumerate().map(|(index, cell)| {
        observed_reference(
            directory,
            BackupArtifactFamily::Page,
            &format!("runtime-page-{index}"),
            layout
                .pages()
                .iter()
                .find(|persisted| persisted.cell() == *cell)
                .expect("runtime page bytes")
                .bytes(),
            BackupArtifactCoverage::physical_reachability(),
            GenerationCountedPhysicalReference::from_page_cell(*cell)
                .require_current_generation(cell.generation())
                .expect("current page"),
        )
    }));
    artifacts.extend(
        source
            .extent_cells()
            .iter()
            .enumerate()
            .map(|(index, cell)| {
                observed_reference(
                    directory,
                    BackupArtifactFamily::Extent,
                    &format!("runtime-extent-{index}"),
                    layout
                        .extents()
                        .iter()
                        .find(|persisted| persisted.cell() == *cell)
                        .expect("runtime extent bytes")
                        .bytes(),
                    BackupArtifactCoverage::physical_reachability(),
                    GenerationCountedPhysicalReference::from_admitted_reference(
                        references.admit_extent(*cell),
                    )
                    .require_current_generation(cell.generation())
                    .expect("current extent"),
                )
            }),
    );
    artifacts
}

fn observed_reference(
    directory: &Path,
    family: BackupArtifactFamily,
    identity: &str,
    bytes: &[u8],
    coverage: BackupArtifactCoverage,
    reference: CurrentGenerationPhysicalReference,
) -> BackupArtifactReference {
    let path = directory.join(format!("{identity}.media"));
    std::fs::write(&path, bytes).expect("runtime-owned source artifact");
    BackupArtifactReference::declare_untrusted_physical_observation(
        family,
        super::support::artifact_format(family),
        identity,
        reference.generation().get(),
        coverage,
        observe_physical_backup_artifact(path, 4 * 1024).expect("physical observation"),
        reference,
    )
    .expect("owner-bound artifact")
}
