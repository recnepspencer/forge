use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest, RecordArtifactFile,
};

use super::super::{
    planning::prepared_payload::PreparedRecordPayloadPlan,
    publication::{append::ManifestCapacityTransition, PublicationPlan},
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    RecordAllocationFrontier, RecordAppendDenial, RecordAppendError,
};

mod assembly;
mod projection;

pub(in crate::physical_runtime::record_serving) struct RebasableRecordPublicationPlan {
    pub(in crate::physical_runtime::record_serving) publication: PublicationPlan,
    pub(in crate::physical_runtime::record_serving) prepared: PreparedRecordPayloadPlan,
}

pub(in crate::physical_runtime::record_serving) struct RootRebaseContext<'plan> {
    pub(in crate::physical_runtime::record_serving) media: &'plan QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) frame_ports:
        super::super::residency::frame_ports::RecordFramePorts,
    pub(in crate::physical_runtime::record_serving) source:
        super::super::residency::frame_loading::CanonicalFrameReadSource,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root:
        &'plan DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) current_free_space:
        &'plan DurableFreeSpaceManifestHeader,
    pub(in crate::physical_runtime::record_serving) frontier: &'plan RecordAllocationFrontier,
    pub(in crate::physical_runtime::record_serving) placement: AdmittedRecordPlacementPolicy,
    pub(in crate::physical_runtime::record_serving) capacity_transition: ManifestCapacityTransition,
}

impl RebasableRecordPublicationPlan {
    pub(in crate::physical_runtime::record_serving) fn begin(
        prepared: PreparedRecordPayloadPlan,
        current_root: &DurablePhysicalRootManifest,
        candidate: RecordArtifactFile,
    ) -> Result<Self, RecordAppendError> {
        let generation =
            current_root
                .generation()
                .checked_add(1)
                .ok_or(RecordAppendError::Denied(
                    RecordAppendDenial::RootGenerationExhausted,
                ))?;
        let publication = PublicationPlan {
            records: prepared.records.clone(),
            generation,
            data: Vec::new(),
            payload_manifests: Vec::new(),
            manifests: Vec::new(),
            root: RecordArtifactFile::RootManifest { generation },
            candidate,
            manifest: current_root.clone(),
            root_bytes: Vec::new(),
            catalog_bytes: Vec::new(),
            observation: prepared.observation,
            work: super::super::RecordPublicationWorkTrace::default(),
            recovery_basis: super::super::RecordPublicationRecoveryBasis::Preparation {
                root_generation: current_root.generation(),
            },
        };
        Ok(Self {
            publication,
            prepared,
        })
    }

    pub(in crate::physical_runtime::record_serving) fn attach_payload(mut self) -> Self {
        self.publication.data = std::mem::take(&mut self.prepared.data);
        self.publication.payload_manifests = std::mem::take(&mut self.prepared.payload_manifests);
        self
    }

    pub(in crate::physical_runtime::record_serving) fn resume(
        publication: PublicationPlan,
        prepared: PreparedRecordPayloadPlan,
    ) -> Self {
        Self {
            publication,
            prepared,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn rebase(
        self,
        context: RootRebaseContext<'_>,
    ) -> Result<(PublicationPlan, DurableFreeSpaceManifestHeader), RecordAppendError> {
        let generation = successor_generation(context.current_root)?;
        require_capacity(&context)?;
        let projected = projection::project_successor_root(&context, self.prepared, generation)?;
        Ok(assembly::assemble_rebased_publication(
            self.publication,
            context,
            generation,
            projected,
        ))
    }
}

fn successor_generation(current: &DurablePhysicalRootManifest) -> Result<u64, RecordAppendError> {
    current
        .generation()
        .checked_add(1)
        .ok_or(RecordAppendError::Denied(
            RecordAppendDenial::RootGenerationExhausted,
        ))
}

fn require_capacity(context: &RootRebaseContext<'_>) -> Result<(), RecordAppendError> {
    if context.capacity_transition == ManifestCapacityTransition::PreserveCurrent
        && context.placement.manifest_capacity().get() != context.current_root.node_capacity()
    {
        Err(RecordAppendError::Denied(
            RecordAppendDenial::ManifestCapacityMigrationRequired,
        ))
    } else {
        Ok(())
    }
}

pub(super) fn damaged() -> RecordAppendError {
    RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged)
}
