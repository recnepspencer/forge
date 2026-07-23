use std::ops::Range;

use worth_store_physical_backend::QualifiedFilesystemMedia;
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_format::{
    decode_inline_record, inspect_inline_page, CurrentPhysicalRecordPlacement,
    DurableExtentManifest, DurablePhysicalRootManifest, RecordArtifactFile,
    DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

use super::super::{
    access::extent_read_session::ExtentReadState,
    residency::serving_artifacts::ServingRecordArtifacts, AdmittedPhysicalRecordFormat,
    AdmittedRecordAccessPolicy, ExternalPhysicalRecordLocator, PhysicalLocatorReadmissionOutcome,
    PhysicalRecordId, RecordReadDenial, RecordReadError, RecordReadLimits, RecordReadObservation,
    RecordStreamFailure, StalePhysicalRecordPlacement,
};

#[path = "locate/failure_classification.rs"]
mod failure_classification;
use failure_classification::{manifest_failure, read_failure};

enum ReadPlacement<'runtime> {
    Inline {
        frame: super::super::residency::frame_loading::LoadedPhysicalFrame,
        payload: Range<usize>,
        offset: usize,
    },
    Extent(Box<ExtentReadState<'runtime>>),
}

pub struct RecordReadSession<'runtime> {
    placement: ReadPlacement<'runtime>,
    observation: RecordReadObservation,
    health: &'runtime super::super::lifecycle::serving_health::ServingHealth,
    _lifecycle: super::super::lifecycle::record_lifecycle::RecordReadSessionLease,
    _allocation: worth_store_buffer_pool::OperationAllocationGrant,
}

pub type OpenedPhysicalRecord<'runtime> = RecordReadSession<'runtime>;

impl RecordReadSession<'_> {
    pub fn read_next(&mut self, target: &mut [u8]) -> Result<usize, RecordStreamFailure> {
        if target.is_empty() {
            return Ok(0);
        }
        let count = match &mut self.placement {
            ReadPlacement::Inline {
                frame,
                payload,
                offset,
            } => {
                let count = target.len().min(payload.len().saturating_sub(*offset));
                let start = payload.start + *offset;
                frame.copy_range_into(start..start + count, &mut target[..count]);
                *offset += count;
                count
            }
            ReadPlacement::Extent(state) => match state.read_next(target, &mut self.observation) {
                Ok(count) => count,
                Err(failure) => {
                    self.health.observe_stream_failure(failure.kind());
                    return Err(failure);
                }
            },
        };
        self.observation.observe_copy(count);
        self.observation.payload_bytes =
            self.observation.payload_bytes.saturating_add(count as u64);
        Ok(count)
    }

    pub const fn observation(&self) -> RecordReadObservation {
        self.observation
    }
}

pub struct PhysicalRecordReader<'runtime> {
    pub(in crate::physical_runtime::record_serving) media: &'runtime QualifiedFilesystemMedia,
    pub(in crate::physical_runtime::record_serving) format: AdmittedPhysicalRecordFormat,
    pub(in crate::physical_runtime::record_serving) access: AdmittedRecordAccessPolicy,
    pub(in crate::physical_runtime::record_serving) current_root:
        &'runtime DurablePhysicalRootManifest,
    pub(in crate::physical_runtime::record_serving) health:
        &'runtime super::super::lifecycle::serving_health::ServingHealth,
    pub(in crate::physical_runtime::record_serving) lifecycle:
        super::super::lifecycle::record_lifecycle::RecordReaderLease,
    pub(in crate::physical_runtime::record_serving) frame_load:
        &'runtime (dyn super::super::residency::frame_ports::FrameLoadPort + Send + Sync),
    pub(in crate::physical_runtime::record_serving) frame_ports:
        &'runtime super::super::residency::frame_ports::RecordFramePorts,
}

impl<'runtime> PhysicalRecordReader<'runtime> {
    pub fn open(
        &self,
        record: PhysicalRecordId,
        limits: RecordReadLimits,
    ) -> Result<RecordReadSession<'runtime>, RecordReadError> {
        let mut observation = RecordReadObservation::default();
        let allocation = self
            .frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundRead,
                u64::from(self.format.declaration().page_size().bytes()),
            )
            .map_err(|reason| {
                RecordReadError::new(RecordReadDenial::ResidencyUnavailable(reason), observation)
            })?;
        let mut discovery =
            super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
        let placement = super::super::access::manifest_routing::ManifestReader::with_loader(
            self.media,
            self.frame_load,
            self.format,
            self.access,
            self.current_root,
        )
        .locate(record.persisted(), &mut discovery);
        observation.observe_manifest(discovery);
        let placement =
            placement.map_err(|failure| self.read_error(manifest_failure(failure), observation))?;
        let placement = placement
            .ok_or_else(|| RecordReadError::new(RecordReadDenial::RecordNotFound, observation))?;
        observation.requested_bytes = placement.payload_bytes();
        if placement.payload_bytes() > u64::from(limits.maximum_payload.get()) {
            return Err(RecordReadError::new(
                RecordReadDenial::CallerLimitExceeded,
                observation,
            ));
        }
        self.open_known_placement_with_allocation(record, placement, observation, allocation)
    }

    pub(in crate::physical_runtime::record_serving) fn open_known_placement(
        &self,
        record: PhysicalRecordId,
        placement: CurrentPhysicalRecordPlacement,
        limits: RecordReadLimits,
        mut observation: RecordReadObservation,
    ) -> Result<RecordReadSession<'runtime>, RecordReadError> {
        observation.requested_bytes = placement.payload_bytes();
        if placement.payload_bytes() > u64::from(limits.maximum_payload.get()) {
            return Err(RecordReadError::new(
                RecordReadDenial::CallerLimitExceeded,
                observation,
            ));
        }
        let allocation = self
            .frame_ports
            .begin_operation(
                worth_store_buffer_pool::OperationAllocationScope::ForegroundRead,
                u64::from(self.format.declaration().page_size().bytes()),
            )
            .map_err(|reason| {
                RecordReadError::new(RecordReadDenial::ResidencyUnavailable(reason), observation)
            })?;
        self.open_known_placement_with_allocation(record, placement, observation, allocation)
    }

    fn open_known_placement_with_allocation(
        &self,
        record: PhysicalRecordId,
        placement: CurrentPhysicalRecordPlacement,
        mut observation: RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession<'runtime>, RecordReadError> {
        let result = match placement {
            CurrentPhysicalRecordPlacement::Inline(value) => {
                self.open_inline(record, value, &mut observation, allocation)
            }
            CurrentPhysicalRecordPlacement::Extent(value) => {
                self.open_extent(record, value, &mut observation, allocation)
            }
        };
        result.map_err(|denial| self.read_error(denial, observation))
    }

    fn open_inline(
        &self,
        record: PhysicalRecordId,
        placement: worth_store_physical_format::DurableInlineRecordPlacement,
        observation: &mut RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession<'runtime>, RecordReadDenial> {
        let artifacts = ServingRecordArtifacts::new(self.media, self.frame_load);
        let mut discovery =
            super::super::access::manifest_routing::ManifestDiscoveryCounterSnapshot::default();
        let page_entry =
            super::super::access::segment_membership::SegmentMembershipReader::with_loader(
                self.media,
                self.frame_load,
                self.format,
                self.access,
                self.current_root,
            )
            .locate(placement.segment(), placement.page(), &mut discovery);
        observation.observe_manifest(discovery);
        let page_entry =
            page_entry
                .map_err(manifest_failure)?
                .ok_or(RecordReadDenial::StalePlacement(
                    StalePhysicalRecordPlacement::SegmentMembership,
                ))?;
        if !observation.check_generation(page_entry.data_segment_cell() == placement.segment_cell())
        {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::SegmentGeneration,
            ));
        }
        if !observation.check_generation(page_entry.page_cell() == placement.page_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::PageGeneration,
            ));
        }
        let page_bytes = self.format.declaration().page_size().bytes();
        let segment_artifact = RecordArtifactFile::Segment {
            segment: placement.segment().get(),
            generation: page_entry.data_generation(),
        };
        if artifacts
            .file_length(segment_artifact)
            .map_err(read_failure)?
            != u64::from(page_entry.data_page_count()) * u64::from(page_bytes)
        {
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        let page = artifacts
            .load_exact(
                segment_artifact,
                u64::from(page_entry.frame_index()) * u64::from(page_bytes),
                page_bytes,
            )
            .map_err(read_failure)?;
        observation.observe_transfer(page.len());
        let geometry = inspect_inline_page(self.format.declaration(), &page)
            .map_err(|_| RecordReadDenial::ArtifactDamaged)?;
        if !observation.check_generation(geometry.page_cell() == placement.page_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::PageIdentity,
            ));
        }
        let decoded = decode_inline_record(
            &page,
            record.persisted(),
            placement.page_cell(),
            placement.slot_cell(),
        );
        match &decoded {
            Ok(_) => {
                observation.check_generation(true);
            }
            Err(worth_store_physical_format::InlinePageDenial::SlotGenerationMismatch) => {
                observation.check_generation(false);
            }
            Err(_) => {}
        }
        let (payload, format) = decoded.map_err(|denial| {
            if denial == worth_store_physical_format::InlinePageDenial::SlotGenerationMismatch {
                RecordReadDenial::StalePlacement(StalePhysicalRecordPlacement::SlotGeneration)
            } else {
                RecordReadDenial::ArtifactDamaged
            }
        })?;
        if format != self.format.declaration()
            || payload.range().len() as u64 != placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        observation.touched_segments = 1;
        observation.touched_pages = 1;
        Ok(RecordReadSession {
            placement: ReadPlacement::Inline {
                frame: page,
                payload: payload.range(),
                offset: 0,
            },
            observation: *observation,
            health: self.health,
            _lifecycle: self.lifecycle.read_session(),
            _allocation: allocation,
        })
    }

    fn open_extent(
        &self,
        record: PhysicalRecordId,
        placement: worth_store_physical_format::DurableExtentRecordPlacement,
        observation: &mut RecordReadObservation,
        allocation: worth_store_buffer_pool::OperationAllocationGrant,
    ) -> Result<RecordReadSession<'runtime>, RecordReadDenial> {
        let artifacts = ServingRecordArtifacts::new(self.media, self.frame_load);
        let bytes = artifacts
            .load_bounded(
                RecordArtifactFile::ExtentManifest {
                    extent: placement.extent().get(),
                    generation: placement.extent_generation(),
                },
                self.access
                    .transfer_limit()
                    .get()
                    .min(self.format.declaration().page_size().bytes()),
            )
            .map_err(read_failure)?;
        observation.observe_manifest_block(bytes.len());
        observation.observe_transfer(bytes.len());
        let (manifest, format) =
            DurableExtentManifest::decode(&bytes).map_err(|_| RecordReadDenial::ArtifactDamaged)?;
        if format != self.format.declaration()
            || manifest.record() != record.persisted()
            || manifest.logical_bytes() != placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        if !observation.check_generation(manifest.extent_cell() == placement.extent_cell()) {
            return Err(RecordReadDenial::StalePlacement(
                StalePhysicalRecordPlacement::ExtentMembership,
            ));
        }
        let expected = manifest.logical_bytes()
            + u64::from(manifest.chunk_count())
                * (DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES) as u64;
        let artifact = RecordArtifactFile::Extent {
            extent: placement.extent().get(),
            generation: placement.extent_generation(),
        };
        if artifacts.file_length(artifact).map_err(read_failure)? != expected {
            return Err(RecordReadDenial::ArtifactDamaged);
        }
        observation.touched_extents = 1;
        Ok(RecordReadSession {
            placement: ReadPlacement::Extent(Box::new(ExtentReadState::new(
                artifacts,
                artifact,
                manifest,
                self.format.declaration(),
            ))),
            observation: *observation,
            health: self.health,
            _lifecycle: self.lifecycle.read_session(),
            _allocation: allocation,
        })
    }

    pub fn readmit_locator(
        &self,
        locator: ExternalPhysicalRecordLocator,
    ) -> PhysicalLocatorReadmissionOutcome {
        super::super::access::readmission::readmit_locator(
            self.media,
            self.frame_load,
            self.format,
            self.access,
            self.current_root,
            self.health,
            locator,
        )
    }

    pub fn open_external(
        &self,
        locator: ExternalPhysicalRecordLocator,
        limits: RecordReadLimits,
    ) -> Result<RecordReadSession<'runtime>, RecordReadError> {
        let record = self
            .readmit_locator(locator)
            .into_result()
            .map_err(|denial| {
                let denial = match denial {
                    super::super::PhysicalLocatorReadmissionDenial::StoreIdentityMismatch => {
                        RecordReadDenial::StoreIdentityMismatch
                    }
                    super::super::PhysicalLocatorReadmissionDenial::RecordNotFound => {
                        RecordReadDenial::RecordNotFound
                    }
                    super::super::PhysicalLocatorReadmissionDenial::CurrentRootUnavailable => {
                        RecordReadDenial::ArtifactUnavailable
                    }
                };
                self.read_error(denial, RecordReadObservation::default())
            })?;
        self.open(record, limits)
    }

    pub const fn store_identity(&self) -> StableStoreIdentity {
        self.media.store_identity()
    }

    fn read_error(
        &self,
        denial: RecordReadDenial,
        observation: RecordReadObservation,
    ) -> RecordReadError {
        self.health.observe_read_denial(denial);
        RecordReadError::new(denial, observation)
    }
}
