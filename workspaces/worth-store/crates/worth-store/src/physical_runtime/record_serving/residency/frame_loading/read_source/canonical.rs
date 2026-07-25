use worth_store_physical_format::{RecordArtifactFile, RecordFrameCoordinate};

use super::{FrameReadSource, FrameReadSourceFailure, ObservedArtifactLength, PreparedFrameRead};
use crate::physical_runtime::{
    record_serving::{
        residency::frame_work_trace::FrameWorkTrace, CanonicalRecordReadFailure,
        CanonicalRecordReadPort, PreparedCanonicalRecordRead, RecordReadPartition,
    },
    PhysicalWorkIdentity,
};

#[derive(Clone)]
pub(in crate::physical_runtime::record_serving) struct CanonicalFrameReadSource {
    port: std::sync::Arc<CanonicalRecordReadPort>,
    context: CanonicalReadContext,
}

#[derive(Clone, Copy)]
enum CanonicalReadContext {
    Ordinary,
    Scan,
}

impl CanonicalFrameReadSource {
    pub(in crate::physical_runtime::record_serving) fn new(port: CanonicalRecordReadPort) -> Self {
        Self {
            port: std::sync::Arc::new(port),
            context: CanonicalReadContext::Ordinary,
        }
    }

    pub(in crate::physical_runtime::record_serving) fn for_scan(mut self) -> Self {
        self.context = CanonicalReadContext::Scan;
        self
    }

    fn range_partition(&self, artifact: RecordArtifactFile) -> RecordReadPartition {
        match self.context {
            CanonicalReadContext::Ordinary => RecordReadPartition::for_range(artifact),
            CanonicalReadContext::Scan => RecordReadPartition::Scan,
        }
    }

    fn metadata_partition(&self, artifact: RecordArtifactFile) -> RecordReadPartition {
        match self.context {
            CanonicalReadContext::Ordinary => RecordReadPartition::for_metadata(artifact),
            CanonicalReadContext::Scan => RecordReadPartition::Scan,
        }
    }
}

impl FrameReadSource for CanonicalFrameReadSource {
    fn prepare_exact(
        &self,
        artifact: RecordArtifactFile,
        offset: u64,
        length: u32,
    ) -> Result<Box<dyn PreparedFrameRead + '_>, FrameReadSourceFailure> {
        let coordinate = RecordFrameCoordinate::new(artifact, offset, length).ok_or_else(|| {
            FrameReadSourceFailure::work(
                CanonicalRecordReadFailure::InvalidCoordinate,
                FrameWorkTrace::default(),
            )
        })?;
        self.port
            .prepare(coordinate, self.range_partition(artifact))
            .map(|prepared| Box::new(prepared) as Box<dyn PreparedFrameRead>)
            .map_err(|failure| {
                FrameReadSourceFailure::work(
                    failure.failure(),
                    FrameWorkTrace::one(failure.identity()),
                )
            })
    }

    fn file_length(
        &self,
        artifact: RecordArtifactFile,
    ) -> Result<ObservedArtifactLength, FrameReadSourceFailure> {
        self.port
            .file_length(artifact, self.metadata_partition(artifact))
            .map(|(bytes, work, projection_failure)| {
                ObservedArtifactLength::admitted(
                    bytes,
                    FrameWorkTrace::one(Some(work)),
                    projection_failure,
                )
            })
            .map_err(|failure| {
                FrameReadSourceFailure::work(
                    failure.failure(),
                    FrameWorkTrace::one(failure.identity()),
                )
            })
    }
}

impl PreparedFrameRead for PreparedCanonicalRecordRead {
    fn identity(&self) -> Option<PhysicalWorkIdentity> {
        Some(self.identity())
    }

    fn execute(
        self: Box<Self>,
        target: &mut [u8],
    ) -> Result<
        Option<crate::physical_runtime::instance::PhysicalProjectionFailureCapability>,
        FrameReadSourceFailure,
    > {
        let identity = (*self).identity();
        let (bytes, projection_failure) = (*self).execute().map_err(|failure| {
            FrameReadSourceFailure::work(failure, FrameWorkTrace::one(Some(identity)))
        })?;
        if bytes.len() != target.len() {
            projection_failure.consume();
            return Err(FrameReadSourceFailure::work(
                CanonicalRecordReadFailure::SettlementMismatch,
                FrameWorkTrace::one(Some(identity)),
            ));
        }
        target.copy_from_slice(&bytes);
        Ok(Some(projection_failure))
    }
}
