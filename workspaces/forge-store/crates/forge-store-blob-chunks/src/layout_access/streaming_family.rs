use forge_store_contracts::DurableArtifactFamilyId;

use forge_store_layout_indexes::layout_strategy_admission::{
    phase24_streaming_rule, AdmittedStreamingLayoutRule,
};

use super::{
    read_counters_are_exact, BlobLayoutAccessDenial, BlobLayoutAccessDenialKind,
    BlobLayoutAccessPathEvidence,
};
use crate::{BlobStreamingReadRequest, BlobStreamingVerifiedRead};

use super::chunk_tree_family::ChunkTreeLayoutReport;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StreamingLayoutFamilyHome;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct StreamingLayoutAdmission {
    _rule: AdmittedStreamingLayoutRule,
}

impl StreamingLayoutFamilyHome {
    const fn s8() -> Self {
        Self
    }

    fn admit(self, rule: AdmittedStreamingLayoutRule) -> StreamingLayoutAdmission {
        let _ = self;
        StreamingLayoutAdmission { _rule: rule }
    }
}

fn streaming_layout() -> AdmittedStreamingLayoutFamily {
    let admission = StreamingLayoutFamilyHome::s8()
        .admit(phase24_streaming_rule().expect("phase 24 streaming rule must stay admitted"));
    AdmittedStreamingLayoutFamily::new(admission)
}

pub fn reject_full_blob_buffer_as_streaming_layout_authority(
    whole_blob: &[u8],
) -> Result<(), BlobLayoutAccessDenial> {
    let _ = whole_blob;
    Err(BlobLayoutAccessDenial::new(
        BlobLayoutAccessDenialKind::FullBlobBufferCannotStandInForStreamingLayoutAuthority,
    ))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct AdmittedStreamingLayoutFamily {
    _admission: StreamingLayoutAdmission,
}

impl AdmittedStreamingLayoutFamily {
    const fn new(admission: StreamingLayoutAdmission) -> Self {
        Self {
            _admission: admission,
        }
    }

    fn admit_streaming(
        &self,
        chunk_tree: &ChunkTreeLayoutReport,
        request: &BlobStreamingReadRequest,
        read: &BlobStreamingVerifiedRead,
    ) -> Result<StreamingLayoutReport, BlobLayoutAccessDenial> {
        let _ = self;
        if chunk_tree.chunk_tree_root() != request.chunk_tree_root()
            || chunk_tree.logical_content_digest() != request.logical_content_digest()
            || request.object_id() != read.object_id()
            || request.generation() != read.generation()
            || request.chunk_tree_root() != read.chunk_tree_root()
            || request.logical_content_digest() != read.logical_content_digest()
        {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::VerifiedReadDoesNotMatchStreamingRequest,
            ));
        }
        let report = StreamingLayoutReport::from_read(read)?;
        report.require_constant_memory_window()?;
        Ok(report)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingLayoutReport {
    family_id: DurableArtifactFamilyId,
    windows_observed: u64,
    bytes_read: u64,
    peak_resident_bytes: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StreamingResumeLayoutReport {
    family_id: DurableArtifactFamilyId,
    bytes_read: u64,
    windows_observed: u64,
    counter_evidence: BlobLayoutAccessPathEvidence,
}

impl StreamingLayoutReport {
    fn from_read(read: &BlobStreamingVerifiedRead) -> Result<Self, BlobLayoutAccessDenial> {
        if !read_counters_are_exact(read.counters()) {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::StreamingLayoutRequiresExactCounters,
            ));
        }
        let family_id = DurableArtifactFamilyId::BlobStream;
        Ok(Self {
            family_id,
            windows_observed: read.counters().windows_observed(),
            bytes_read: read.counters().bytes_read(),
            peak_resident_bytes: read.counters().peak_resident_bytes(),
            counter_evidence: BlobLayoutAccessPathEvidence::from_streaming(
                family_id,
                read.counters(),
            ),
        })
    }

    pub fn admit_resume_index_layout(&self) -> StreamingResumeLayoutReport {
        StreamingResumeLayoutReport {
            family_id: DurableArtifactFamilyId::SupportCursor,
            bytes_read: self.bytes_read,
            windows_observed: self.windows_observed,
            counter_evidence: self.counter_evidence,
        }
    }

    pub fn require_constant_memory_window(&self) -> Result<(), BlobLayoutAccessDenial> {
        if self.bytes_read > 0 && self.peak_resident_bytes >= self.bytes_read {
            return Err(BlobLayoutAccessDenial::new(
                BlobLayoutAccessDenialKind::StreamingLayoutRequiresConstantMemory,
            ));
        }
        Ok(())
    }

    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn windows_observed(&self) -> u64 {
        self.windows_observed
    }

    pub const fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub const fn peak_resident_bytes(&self) -> u64 {
        self.peak_resident_bytes
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl StreamingResumeLayoutReport {
    pub const fn family_id(&self) -> DurableArtifactFamilyId {
        self.family_id
    }

    pub const fn bytes_read(&self) -> u64 {
        self.bytes_read
    }

    pub const fn windows_observed(&self) -> u64 {
        self.windows_observed
    }

    pub const fn counter_evidence(&self) -> BlobLayoutAccessPathEvidence {
        self.counter_evidence
    }
}

impl ChunkTreeLayoutReport {
    pub fn admit_streaming_layout(
        &self,
        request: &BlobStreamingReadRequest,
        read: &BlobStreamingVerifiedRead,
    ) -> Result<StreamingLayoutReport, BlobLayoutAccessDenial> {
        streaming_layout().admit_streaming(self, request, read)
    }
}
