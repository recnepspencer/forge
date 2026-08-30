use super::super::failure_classification::read_failure;
use super::super::PhysicalRecordReader;
use crate::physical_runtime::record_serving::work_semantics::integrity_admission::{
    admit_extent_manifest as admit_clean_extent_manifest, CleanExtentAdmissionDenial,
};
use crate::physical_runtime::record_serving::{
    residency::record_frame_reader::RecordFrameReader, PhysicalRecordId, RecordReadDenial,
    RecordReadObservation, StalePhysicalRecordPlacement,
};
use worth_store_physical_format::{
    DurableExtentManifest, DurableExtentRecordPlacement, RecordArtifactFile,
    DURABLE_EXTENT_FRAME_HEADER_BYTES, EXTENT_CHUNK_METADATA_BYTES,
};

pub(super) struct AdmittedExtentManifest {
    pub(super) artifact: RecordArtifactFile,
    pub(super) manifest: DurableExtentManifest,
    pub(super) artifact_bytes: std::num::NonZeroU64,
    pub(super) integrity_membership:
        worth_store_physical_integrity::IntegrityValidatedExtentMembership,
}

pub(super) struct ExtentManifestAdmission<'admission, 'media> {
    pub(super) reader: &'admission PhysicalRecordReader,
    pub(super) record: PhysicalRecordId,
    pub(super) placement: DurableExtentRecordPlacement,
    pub(super) observation: &'admission mut RecordReadObservation,
    pub(super) allocation: &'admission worth_store_buffer_pool::OperationAllocationGrant,
    pub(super) artifacts: &'admission RecordFrameReader<'media>,
}

pub(super) fn admit_extent_manifest(
    mut admission: ExtentManifestAdmission<'_, '_>,
) -> Result<AdmittedExtentManifest, RecordReadDenial> {
    let (manifest, integrity_membership) = admission.load_manifest()?;
    let artifact = RecordArtifactFile::Extent {
        extent: admission.placement.extent().get(),
        generation: admission.placement.extent_generation(),
    };
    let artifact_bytes = complete_extent_bytes(&manifest);
    Ok(AdmittedExtentManifest {
        artifact,
        manifest,
        artifact_bytes,
        integrity_membership,
    })
}

impl ExtentManifestAdmission<'_, '_> {
    fn load_manifest(
        &mut self,
    ) -> Result<
        (
            DurableExtentManifest,
            worth_store_physical_integrity::IntegrityValidatedExtentMembership,
        ),
        RecordReadDenial,
    > {
        let bytes = self
            .artifacts
            .load_bounded(
                self.allocation,
                RecordArtifactFile::ExtentManifest {
                    extent: self.placement.extent().get(),
                    generation: self.placement.extent_generation(),
                },
                self.reader
                    .access
                    .transfer_limit()
                    .get()
                    .min(self.reader.format.declaration().page_size().bytes()),
            )
            .map_err(|failure| {
                self.observation.observe_physical_work(failure.work_trace());
                read_failure(failure)
            })?;
        self.observation.observe_physical_work(bytes.work_trace());
        self.observation.observe_manifest_block(bytes.len());
        self.observation.observe_transfer(bytes.len());
        match self.project_manifest(&bytes) {
            Ok(manifest) => Ok(manifest),
            Err(denial) => {
                bytes.reject_projection_failure();
                Err(denial)
            }
        }
    }

    fn project_manifest(
        &mut self,
        frame: &crate::physical_runtime::record_serving::residency::frame_loading::LoadedPhysicalFrame,
    ) -> Result<
        (
            DurableExtentManifest,
            worth_store_physical_integrity::IntegrityValidatedExtentMembership,
        ),
        RecordReadDenial,
    > {
        let admitted = admit_clean_extent_manifest(
            frame,
            self.reader.residency.resident_admission_context(),
            self.reader.store,
            self.reader.format.declaration(),
            self.placement,
        )
        .map_err(|denial| classify_denial(denial, self.observation))?;
        let manifest = admitted.manifest;
        if manifest.record() != self.record.persisted()
            || manifest.logical_bytes() != self.placement.payload_bytes()
        {
            return Err(RecordReadDenial::FormatMismatch);
        }
        self.observation.check_generation(true);
        Ok((manifest, admitted.membership))
    }
}

fn classify_denial(
    denial: CleanExtentAdmissionDenial,
    observation: &mut RecordReadObservation,
) -> RecordReadDenial {
    match denial {
        CleanExtentAdmissionDenial::ExtentMembership => {
            observation.check_generation(false);
            RecordReadDenial::StalePlacement(StalePhysicalRecordPlacement::ExtentMembership)
        }
        CleanExtentAdmissionDenial::Format => RecordReadDenial::FormatMismatch,
        CleanExtentAdmissionDenial::Damaged => RecordReadDenial::ArtifactDamaged,
    }
}

fn complete_extent_bytes(manifest: &DurableExtentManifest) -> std::num::NonZeroU64 {
    let bytes = manifest.logical_bytes()
        + u64::from(manifest.chunk_count())
            * (DURABLE_EXTENT_FRAME_HEADER_BYTES + EXTENT_CHUNK_METADATA_BYTES) as u64;
    std::num::NonZeroU64::new(bytes).expect("an admitted extent has nonzero artifact bytes")
}
