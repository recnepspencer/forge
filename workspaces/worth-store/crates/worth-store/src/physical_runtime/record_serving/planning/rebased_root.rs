use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::{
    durable_artifact_checksum, BootstrapCatalog, CurrentRootCatalogEntry,
    CurrentRootCatalogGeneration, DurableFreeSpaceManifestHeader, DurablePhysicalRootManifest,
    PersistedRecordIdentity, RecordArtifactFile, SegmentGenerationCell,
};

use super::super::{
    access::{
        manifest_routing::{plan_manifest_updates, ManifestReader, RootManifestUpdateRequest},
        segment_membership::{
            plan_segment_membership_updates, SegmentMembershipPublicationPlan,
            SegmentMembershipUpdateContext,
        },
    },
    planning::{
        free_space_projection::{project_successor_free_space, FreeSpaceProjectionContext},
        free_space_routing::FreeSpacePublicationPlan,
        prepared_payload::PreparedRecordPayloadPlan,
    },
    publication::{append::ManifestCapacityTransition, PublicationPlan},
    AdmittedPhysicalRecordFormat, AdmittedRecordAccessPolicy, AdmittedRecordPlacementPolicy,
    RecordAllocationFrontier, RecordAppendDenial, RecordAppendError,
};

pub(in crate::physical_runtime::record_serving) struct RebasableRecordPublicationPlan {
    pub(in crate::physical_runtime::record_serving) publication: PublicationPlan,
    pub(in crate::physical_runtime::record_serving) prepared: PreparedRecordPayloadPlan,
}

pub(in crate::physical_runtime::record_serving) struct RootRebaseContext<'plan> {
    pub(in crate::physical_runtime::record_serving) media: &'plan QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) frame_load:
        &'plan (dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
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
        mut self,
        context: RootRebaseContext<'_>,
    ) -> Result<(PublicationPlan, DurableFreeSpaceManifestHeader), RecordAppendError> {
        let generation = successor_generation(context.current_root)?;
        require_capacity(&context)?;
        let free_space = project_successor_free_space(
            FreeSpaceProjectionContext {
                media: context.media,
                frame_load: context.frame_load,
                format: context.format,
                access: context.access,
                current: context.current_free_space,
                successor_generation: generation,
                successor_capacity: context.placement.manifest_capacity().get(),
                frontier: context.frontier,
            },
            &self.prepared.inline_allocations,
        )?;
        let FreeSpacePublicationPlan {
            header: free_space,
            blocks: free_space_blocks,
            discovery: free_space_discovery,
        } = free_space;
        let free_space_bytes = free_space.encode(context.format.declaration());
        let segment_routed = plan_segment_membership_updates(
            SegmentMembershipUpdateContext {
                media: context.media,
                frame_load: context.frame_load,
                format: context.format,
                access: context.access,
                current: context.current_root,
                successor_generation: generation,
                successor_capacity: context.placement.manifest_capacity().get(),
            },
            self.prepared.segment_updates,
        )
        .map_err(|_| damaged())?;
        let SegmentMembershipPublicationPlan {
            root: segment_root,
            next_block: next_segment_block,
            blocks: segment_blocks,
            discovery: segment_discovery,
        } = segment_routed;
        let reader = ManifestReader::with_loader(
            context.media,
            context.frame_load,
            context.format,
            context.access,
            context.current_root,
        );
        let (last_inline_record, last_inline_segment) = successor_inline_tail(
            context.current_root,
            self.prepared.last_inline_record,
            self.prepared.last_inline_segment,
        );
        let routed = plan_manifest_updates(
            &reader,
            context.current_root,
            RootManifestUpdateRequest {
                successor_generation: generation,
                successor_capacity: context.placement.manifest_capacity().get(),
                free_space_checksum: durable_artifact_checksum(&free_space_bytes),
                free_space_root: free_space.root(),
                segment_root,
                next_segment_block,
                placements: self.prepared.placements,
                last_inline_record,
                last_inline_segment,
            },
        )
        .map_err(|_| damaged())?;
        observe_manifest_discovery(&mut self.publication.observation, free_space_discovery);
        observe_manifest_discovery(&mut self.publication.observation, segment_discovery);
        observe_manifest_discovery(&mut self.publication.observation, routed.discovery);
        let mut manifests = free_space_blocks;
        manifests.push((
            RecordArtifactFile::FreeSpaceManifest { generation },
            free_space_bytes,
        ));
        manifests.extend(segment_blocks);
        manifests.extend(routed.blocks);
        self.publication.generation = generation;
        self.publication.recovery_basis =
            super::super::RecordPublicationRecoveryBasis::RootCandidate {
                prior_root_generation: context.current_root.generation(),
                candidate_root_generation: generation,
            };
        self.publication.root = RecordArtifactFile::RootManifest { generation };
        self.publication.manifest = routed.root;
        self.publication.root_bytes = self
            .publication
            .manifest
            .encode(context.format.declaration());
        self.publication.catalog_bytes = BootstrapCatalog::new(
            context.media.store_identity(),
            context.format.declaration(),
            CurrentRootCatalogEntry::new(
                CurrentRootCatalogGeneration::new(generation).expect("successor is nonzero"),
            ),
        )
        .encode()
        .to_vec();
        self.publication.manifests = manifests;
        Ok((self.publication, free_space))
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

fn successor_inline_tail(
    current: &DurablePhysicalRootManifest,
    prepared_record: Option<PersistedRecordIdentity>,
    prepared_segment: Option<SegmentGenerationCell>,
) -> (
    Option<PersistedRecordIdentity>,
    Option<SegmentGenerationCell>,
) {
    let current_segment = current.last_inline_segment();
    let prepared_wins = match (prepared_segment, current_segment) {
        (Some(prepared), Some(current)) => {
            (prepared.segment_id().get(), prepared.generation().get())
                >= (current.segment_id().get(), current.generation().get())
        }
        (Some(_), None) => true,
        _ => false,
    };
    if prepared_wins {
        (prepared_record, prepared_segment)
    } else {
        (current.last_inline_record(), current_segment)
    }
}

fn damaged() -> RecordAppendError {
    RecordAppendError::Denied(RecordAppendDenial::PublishedLayoutDamaged)
}

fn observe_manifest_discovery(
    observation: &mut super::super::publication::append_observation::PublicationObservation,
    discovery: super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot,
) {
    observation.manifest_blocks_read = observation
        .manifest_blocks_read
        .saturating_add(discovery.blocks_read());
    observation.manifest_comparisons = observation
        .manifest_comparisons
        .saturating_add(discovery.comparisons());
    observation.manifest_bytes_read = observation
        .manifest_bytes_read
        .saturating_add(discovery.bytes_read());
}
