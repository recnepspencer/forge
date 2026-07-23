use worth_store_physical_backend::{ArtifactTreeFailure, QualifiedFilesystemMedia};
use worth_store_physical_format::{
    BootstrapCatalog, CurrentRootCatalogEntry, CurrentRootCatalogGeneration,
    DurablePhysicalRootManifest, RecordArtifactFile,
};

use super::super::{
    planning::batch_placement::preflight_placement,
    planning::placement_context::PlacementPlanningContext,
    planning::placement_plan::lower_batch,
    publication::{execute_publication, PublicationPlan},
    AdmittedPhysicalRecordFormat, AdmittedRecordPlacementPolicy, IndeterminateRecordPublication,
    PublishedRecordBatch, RecordAllocationFrontier, RecordAppendBatch, RecordStreamFailure,
    UnpublishedRecordBatchFailure,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordPlacementClass {
    InlinePage,
    ExtentBacked,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RecordAppendDenial {
    EmptyBatch,
    BatchRecordLimitExceeded,
    BatchByteLimitExceeded,
    RecordTooLarge,
    InlinePageFull,
    RootGenerationExhausted,
    RecordIdentityExhausted,
    PhysicalIdentityExhausted,
    IdentityEntropyUnavailable,
    BackendUnavailable(ArtifactTreeFailure),
    ServingRequiresInspection,
    PlacementFormatMismatch,
    ManifestCapacityMigrationRequired,
    PublishedLayoutDamaged,
    ResidencyUnavailable(worth_store_buffer_pool::PhysicalResidencyDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::physical_runtime::record_serving) enum ManifestCapacityTransition {
    PreserveCurrent,
    ReconstructToRequested,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RecordAppendError {
    Denied(RecordAppendDenial),
    StreamFailed(RecordStreamFailure),
    Unpublished(UnpublishedRecordBatchFailure),
    Indeterminate(IndeterminateRecordPublication),
}

pub(in crate::physical_runtime::record_serving) struct RecordAppendExecutionContext<'runtime> {
    pub(in crate::physical_runtime::record_serving) media: &'runtime QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access:
        super::super::AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root:
        &'runtime DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) current_free_space:
        &'runtime worth_store_physical_format::DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime::record_serving) allocation_frontier:
        &'runtime mut RecordAllocationFrontier,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) frame_ports:
        &'runtime super::super::residency::frame_ports::RecordFramePorts,
    pub(in crate::physical_runtime::record_serving) capacity_transition: ManifestCapacityTransition,
}

pub(in crate::physical_runtime::record_serving) fn append(
    context: RecordAppendExecutionContext<'_>,
    batch: RecordAppendBatch,
) -> Result<
    (
        PublishedRecordBatch,
        DurablePhysicalRootManifest,
        worth_store_physical_format::DurableFreeSpaceManifestHeader,
    ),
    RecordAppendError,
> {
    let RecordAppendExecutionContext {
        media,
        format,
        access,
        current_root,
        current_free_space,
        allocation_frontier,
        placement,
        frame_ports,
        capacity_transition,
    } = context;
    if !placement.admits(format) {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::PlacementFormatMismatch,
        ));
    }
    if capacity_transition == ManifestCapacityTransition::PreserveCurrent
        && placement.manifest_capacity().get() != current_root.node_capacity()
    {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::ManifestCapacityMigrationRequired,
        ));
    }
    preflight_placement(format, placement, &batch)?;
    let admitted = batch.admit(access).map_err(RecordAppendError::Denied)?;
    let generation = current_root
        .generation()
        .checked_add(1)
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::RootGenerationExhausted,
        ))?;
    let counters_before = media.counters();
    let candidate = RecordArtifactFile::CatalogCandidate {
        publication: next_nonzero_random()?,
    };
    let mut candidate_frontier = allocation_frontier.clone();
    let lowered = lower_batch(
        PlacementPlanningContext {
            media,
            format,
            access,
            current_root,
            current_free_space,
            frontier: &mut candidate_frontier,
            placement,
            generation,
            frame_load: frame_ports.loader(),
        },
        admitted,
    )?;
    let super::super::planning::placement_plan::LoweredRecordPlacementPlan {
        records,
        data,
        manifests,
        manifest,
        free_space,
        observation,
    } = lowered;
    let root = RecordArtifactFile::RootManifest { generation };
    let plan = PublicationPlan {
        records,
        generation,
        data,
        manifests,
        root,
        candidate,
        root_bytes: manifest.encode(format.declaration()),
        catalog_bytes: BootstrapCatalog::new(
            media.store_identity(),
            format.declaration(),
            CurrentRootCatalogEntry::new(
                CurrentRootCatalogGeneration::new(generation).expect("nonzero successor"),
            ),
        )
        .encode()
        .to_vec(),
        manifest,
        observation,
    };
    match execute_publication(
        frame_ports.publisher(),
        media,
        format,
        plan,
        counters_before,
    ) {
        Ok(published) => {
            *allocation_frontier = candidate_frontier;
            Ok((published.0, published.1, free_space))
        }
        Err(error) => Err(error),
    }
}

fn next_nonzero_random() -> Result<u64, RecordAppendError> {
    let mut bytes = [0_u8; 8];
    getrandom::fill(&mut bytes)
        .map_err(|_| RecordAppendError::Denied(RecordAppendDenial::IdentityEntropyUnavailable))?;
    let value = u64::from_le_bytes(bytes);
    if value == 0 {
        return Err(RecordAppendError::Denied(
            RecordAppendDenial::IdentityEntropyUnavailable,
        ));
    }
    Ok(value)
}
