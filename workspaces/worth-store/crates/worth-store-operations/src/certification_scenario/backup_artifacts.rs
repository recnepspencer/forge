use std::path::Path;

use sha2::{Digest, Sha256};
use worth_store_blob_chunks::certification_test_authority::blob_backup_artifact_for_bytes;
use worth_store_contracts::{
    AcceptedHandoffReadiness, HandoffEvidenceDigestSet, StableDigest, ROADMAP_2_S1_SCOPE,
};
use worth_store_layout_indexes::encode_baseline_btree_leaf_record;
use worth_store_physical_backend::observe_physical_backup_artifact;
use worth_store_physical_format::{
    BackupBundleArtifactFormat, PageGenerationCell, PhysicalGeneration,
    PhysicalGenerationAuthority, PhysicalReferenceAuthority, PhysicalStoreRuntime,
    PlatformPhysicalAppendRequest, PlatformPhysicalOpenRequest,
    PlatformPhysicalRootPublicationReport, RootPublicationCell,
};
#[cfg(test)]
use worth_store_physical_format::{
    PhysicalBinaryEncodingWitness, PhysicalHeaderAuthority, PhysicalPageKind,
    PHYSICAL_HEADER_LENGTH,
};
use worth_store_physical_isolation::{
    BackupArtifactCoverage, BackupArtifactFamily, BackupArtifactReference,
    CurrentGenerationPhysicalReference,
};
use worth_store_recovery_physics::{
    CheckpointBackupArtifact, CheckpointCoveredLsnRange, CheckpointManifest,
    CheckpointPageLsnFrontier, CheckpointRedoBoundary, CheckpointRootPosture, LogSequenceNumber,
    PageLsn, SharpCheckpointCertificationMode,
};

mod physical_reference;
use physical_reference::*;

pub(crate) struct CanonicalBackupArtifacts {
    pub(crate) references: Vec<BackupArtifactReference>,
    pub(crate) checkpoint_identity: String,
}

pub(crate) fn canonical_backup_artifacts_at_root_generation(
    case: &str,
    source: &Path,
    root_generation: u64,
) -> CanonicalBackupArtifacts {
    assert!(root_generation > 0, "fixture root generation is nonzero");
    let mut world = CanonicalBackupArtifactWorld::new(case);
    let mut artifacts = None;
    for _ in 0..root_generation {
        artifacts = Some(world.publish(source));
    }
    artifacts.expect("nonzero publication count")
}

#[cfg(test)]
pub(crate) fn canonical_backup_artifacts_across_one_root_publication(
    case: &str,
    older_source: &Path,
    newer_source: &Path,
) -> (CanonicalBackupArtifacts, CanonicalBackupArtifacts) {
    let mut world = CanonicalBackupArtifactWorld::new(case);
    let older = world.publish(older_source);
    let newer = world.publish(newer_source);
    (older, newer)
}

struct CanonicalBackupArtifactWorld {
    case: String,
    runtime: PhysicalStoreRuntime,
    generation: PhysicalGeneration,
}

impl CanonicalBackupArtifactWorld {
    fn new(case: &str) -> Self {
        let mut runtime = open_physical_runtime();
        let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
        let generation = PhysicalGeneration::from_raw(1).expect("generation");
        let slot = generations
            .slot_cell(segment(200), page(4), record_slot(1))
            .with_slot_generation(generation);
        runtime
            .append_physical_record(PlatformPhysicalAppendRequest::page_slot(
                slot,
                format!("{case}:page-record").as_bytes(),
            ))
            .expect("physical page append");
        let appended_extent = generations
            .extent_cell(segment(100), extent(5))
            .with_extent_generation(generation);
        runtime
            .append_physical_record(PlatformPhysicalAppendRequest::extent(
                appended_extent,
                format!("{case}:extent-record").as_bytes(),
            ))
            .expect("physical extent append");
        Self {
            case: case.to_owned(),
            runtime,
            generation,
        }
    }

    fn publish(&mut self, source: &Path) -> CanonicalBackupArtifacts {
        let publication = self
            .runtime
            .publish_physical_root()
            .expect("root publication");
        materialize_canonical_backup_artifacts(
            &self.case,
            source,
            &self.runtime,
            publication,
            self.generation,
        )
    }
}

fn materialize_canonical_backup_artifacts(
    case: &str,
    source: &Path,
    runtime: &PhysicalStoreRuntime,
    publication: PlatformPhysicalRootPublicationReport,
    generation: PhysicalGeneration,
) -> CanonicalBackupArtifacts {
    let generations = PhysicalGenerationAuthority::for_canonical_physical_format();
    let current = runtime
        .current_physical_reachability_source()
        .expect("published current-root closure");
    let root = current.manifest().root_publication();
    let layout = publication.persisted_layout();
    let persisted_page = layout.pages().first().expect("persisted page");
    let persisted_extent = layout.extents().first().expect("persisted extent");

    let checkpoint_manifest = checkpoint_manifest(root, persisted_page.cell());
    let checkpoint = CheckpointBackupArtifact::from_sharp_manifest(&checkpoint_manifest, 1, 10)
        .expect("sharp checkpoint backup artifact");
    let checkpoint_identity = checkpoint.checkpoint_identity().to_owned();

    let mut references = Vec::with_capacity(7);
    references.push(observe_reference(
        source,
        "root.media",
        layout
            .root_manifest_candidates()
            .first()
            .expect("root manifest"),
        BackupArtifactFamily::RootManifest,
        BackupBundleArtifactFormat::PhysicalRootManifestV1,
        root_identity(root),
        BackupArtifactCoverage::root_manifest(root.generation().get()).expect("root coverage"),
        current_root_reference(root),
    ));

    let mut checkpoint_bytes = Vec::new();
    checkpoint
        .encode(&mut checkpoint_bytes)
        .expect("checkpoint encoding");
    references.push(observe_reference(
        source,
        "checkpoint.media",
        &checkpoint_bytes,
        BackupArtifactFamily::CheckpointManifest,
        BackupBundleArtifactFormat::RecoveryCheckpointManifestV1,
        checkpoint_identity.clone(),
        BackupArtifactCoverage::checkpoint_manifest(&checkpoint_identity, 1, 10)
            .expect("checkpoint coverage"),
        current_slot_reference(segment(200), page(1), record_slot(2), generation),
    ));

    let wal_owner_segment = segment(3);
    let wal = worth_store_wal::prepare_wal_frame_append(
        source,
        wal_owner_segment.get(),
        generation.get(),
        10,
        12,
        &format!("{case}:wal-frame"),
        format!("{case}:wal-payload").as_bytes(),
    )
    .expect("canonical WAL frame");
    references.push(observe_reference(
        source,
        "wal.media",
        wal.encoded_frame(),
        BackupArtifactFamily::WalSegment,
        BackupBundleArtifactFormat::WalSegmentV1,
        format!("wal:{}:{}:10-12", wal_owner_segment.get(), generation.get()),
        BackupArtifactCoverage::wal_segment(10, 12).expect("WAL coverage"),
        current_segment_reference(wal_owner_segment, generation),
    ));

    references.push(observe_reference(
        source,
        "page.media",
        persisted_page.bytes(),
        BackupArtifactFamily::Page,
        BackupBundleArtifactFormat::PhysicalDataPageV1,
        page_identity(persisted_page.cell()),
        BackupArtifactCoverage::physical_reachability(),
        current_page_reference(persisted_page.cell()),
    ));
    references.push(observe_reference(
        source,
        "extent.media",
        persisted_extent.bytes(),
        BackupArtifactFamily::Extent,
        BackupBundleArtifactFormat::PhysicalExtentRecordV1,
        extent_identity(persisted_extent.cell()),
        BackupArtifactCoverage::physical_reachability(),
        current_extent_reference(persisted_extent.cell()),
    ));

    let index_bytes =
        encode_baseline_btree_leaf_record([record_slot(20), record_slot(21)], true, false);
    references.push(observe_reference(
        source,
        "index.media",
        &index_bytes,
        BackupArtifactFamily::Index,
        BackupBundleArtifactFormat::LayoutBTreeLeafV1,
        format!("index:sha256:{}", hex(&Sha256::digest(index_bytes))),
        BackupArtifactCoverage::physical_reachability(),
        current_slot_reference(segment(200), page(1), record_slot(6), generation),
    ));

    let blob_payload = format!("{case}:blob-payload");
    let blob = blob_backup_artifact_for_bytes(case, blob_payload.as_bytes());
    let blob_identity = blob.chunk_identity().to_owned();
    let mut blob_bytes = Vec::new();
    blob.encode(&mut blob_bytes).expect("blob backup encoding");
    references.push(observe_reference(
        source,
        "blob.media",
        &blob_bytes,
        BackupArtifactFamily::BlobChunk,
        BackupBundleArtifactFormat::BlobChunkV1,
        blob_identity,
        BackupArtifactCoverage::physical_reachability(),
        current_extent_reference(
            generations
                .extent_cell(segment(100), extent(7))
                .with_extent_generation(generation),
        ),
    ));

    CanonicalBackupArtifacts {
        references,
        checkpoint_identity,
    }
}

pub(crate) fn open_physical_runtime() -> PhysicalStoreRuntime {
    PhysicalStoreRuntime::open_physical_format(
        physical_readiness(),
        PlatformPhysicalOpenRequest::physical_format_canonical(),
    )
    .expect("physical runtime")
}

#[cfg(test)]
pub(crate) fn copy_page_to_owner(
    source: &Path,
    target: &Path,
    owner: CurrentGenerationPhysicalReference,
) {
    let owner = owner.owner();
    let cell = PhysicalGenerationAuthority::for_canonical_physical_format()
        .page_cell(
            owner.segment_id().expect("page segment"),
            owner.page_id().expect("page id"),
        )
        .with_page_generation(owner.generation());
    let source_bytes = std::fs::read(source).expect("source page bytes");
    let payload = source_bytes
        .get(usize::from(PHYSICAL_HEADER_LENGTH)..)
        .expect("canonical page header");
    let binary = PhysicalBinaryEncodingWitness::physical_format_canonical().expect("encoding");
    let headers = PhysicalHeaderAuthority::for_canonical_physical_format(binary);
    let mut bytes = Vec::with_capacity(usize::from(PHYSICAL_HEADER_LENGTH) + payload.len());
    bytes.extend_from_slice(&headers.encode_page_header(
        cell,
        PhysicalPageKind::DataPage,
        payload.len().try_into().expect("bounded fixture payload"),
    ));
    bytes.extend_from_slice(payload);
    std::fs::write(target, bytes).expect("owner-bound page bytes");
}

fn checkpoint_manifest(root: RootPublicationCell, page: PageGenerationCell) -> CheckpointManifest {
    let references = PhysicalReferenceAuthority::for_canonical_physical_format();
    let root_reference = references.admit_root_publication(root).reference();
    let redo = LogSequenceNumber::new(10);
    CheckpointManifest::sharp(
        CheckpointRootPosture::root_present(root_reference),
        CheckpointPageLsnFrontier::from_pages([(page, PageLsn::from_lsn(redo))])
            .expect("page-LSN frontier"),
        CheckpointCoveredLsnRange::new(redo, LogSequenceNumber::new(12)).expect("checkpoint range"),
        CheckpointRedoBoundary::from_page_lsn(PageLsn::from_lsn(redo)),
        SharpCheckpointCertificationMode::certified(),
    )
    .expect("sharp checkpoint")
}

#[allow(clippy::too_many_arguments)]
fn observe_reference(
    source: &Path,
    name: &str,
    bytes: &[u8],
    family: BackupArtifactFamily,
    format: BackupBundleArtifactFormat,
    identity: String,
    coverage: BackupArtifactCoverage,
    reclaim_reference: CurrentGenerationPhysicalReference,
) -> BackupArtifactReference {
    let path = source.join(name);
    std::fs::write(&path, bytes).expect("owner artifact bytes");
    BackupArtifactReference::declare_untrusted_physical_observation(
        family,
        format,
        identity,
        reclaim_reference.generation().get(),
        coverage,
        observe_physical_backup_artifact(path, 4 * 1024).expect("physical observation"),
        reclaim_reference,
    )
    .expect("owner-bound backup reference")
}

fn physical_readiness() -> AcceptedHandoffReadiness {
    let digest = |name: &str| StableDigest::new(format!("sha256:{name}")).expect("fixture digest");
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
    .expect("physical-format readiness")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
