use crate::handoffs::{BlobHarnessChunkTopology, BlobHarnessSizeClass};

use worth_store_physical_backend::{
    HeavyFixtureBackendProfile, HeavyFixtureCleanupReceipt, HeavyFixtureDiskPreflightReceipt,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeavyBlobFixtureMaterializationMode {
    StreamOnly,
    TempFile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DeterministicBytePatternProfile {
    CanonicalMixed,
    IncompressibleSeeded,
    HighlyCompressibleRepeatedSpans,
    ChunkBoundaryAdversarial,
    RepeatedChunkDedupePressure,
    SparseDeclarationDenied,
    LogicalSizeOnlyDenied,
    HiddenTemporarySidecarDenied,
    WholeObjectExpectedBufferDenied,
    GeneratedExpectedByteArtifactDenied,
    AmbientChaosCorpus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum HeavyBlobQualificationDenial {
    SparseOnlyProofNotCanonical,
    LogicalSizeOnlyProofNotCanonical,
    WholeObjectExpectedBufferNotCanonical,
    GeneratedExpectedByteArtifactNotCanonical,
    HiddenTemporarySidecarNotCanonical,
    AmbientCorpusNotCanonical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeavyBlobExpectedDigestBasis {
    seed: u64,
    logical_bytes: u64,
    chunk_bytes: u64,
    expected_chunk_count: u64,
    byte_pattern_profile: DeterministicBytePatternProfile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HeavyBlobVerificationPassBasis {
    rolling_digest: u64,
    actual_bytes_streamed: u64,
    actual_chunk_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyBlobFixturePlan {
    seed: u64,
    topology: BlobHarnessChunkTopology,
    byte_pattern_profile: DeterministicBytePatternProfile,
    materialization_mode: HeavyBlobFixtureMaterializationMode,
    backend_profile: HeavyFixtureBackendProfile,
    expected_digest_basis: HeavyBlobExpectedDigestBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeavyBlobFixtureExecutionEvidence {
    expected_digest_basis: HeavyBlobExpectedDigestBasis,
    verification_pass_basis: HeavyBlobVerificationPassBasis,
    peak_resident_memory_bytes: u64,
    peak_allocation_count: u64,
    temporary_file_bytes: u64,
    disk_bytes_written: u64,
    cleanup_receipt: Option<HeavyFixtureCleanupReceipt>,
    backend_profile: HeavyFixtureBackendProfile,
    materialization_mode: HeavyBlobFixtureMaterializationMode,
    disk_preflight_receipt: Option<HeavyFixtureDiskPreflightReceipt>,
}

impl HeavyBlobFixturePlan {
    pub fn canonical_for_profile(
        size_class: BlobHarnessSizeClass,
        topology: BlobHarnessChunkTopology,
    ) -> Option<Self> {
        if size_class != BlobHarnessSizeClass::HeavyMultiGbDeclared {
            return None;
        }
        Some(Self::new(
            22,
            topology,
            DeterministicBytePatternProfile::CanonicalMixed,
            HeavyBlobFixtureMaterializationMode::StreamOnly,
            HeavyFixtureBackendProfile::StoreOwnedLocalDisk,
        ))
    }

    pub(crate) fn temp_file_smoke_for_topology(topology: BlobHarnessChunkTopology) -> Self {
        Self::new(
            22,
            topology,
            DeterministicBytePatternProfile::CanonicalMixed,
            HeavyBlobFixtureMaterializationMode::TempFile,
            HeavyFixtureBackendProfile::StoreOwnedLocalDisk,
        )
    }

    pub fn with_materialization_mode(mut self, mode: HeavyBlobFixtureMaterializationMode) -> Self {
        self.materialization_mode = mode;
        self
    }

    pub fn with_backend_profile(mut self, backend_profile: HeavyFixtureBackendProfile) -> Self {
        self.backend_profile = backend_profile;
        self
    }

    pub fn with_byte_pattern_profile(
        mut self,
        byte_pattern_profile: DeterministicBytePatternProfile,
    ) -> Self {
        self.byte_pattern_profile = byte_pattern_profile;
        self.expected_digest_basis = HeavyBlobExpectedDigestBasis {
            byte_pattern_profile,
            ..self.expected_digest_basis
        };
        self
    }

    pub fn ambient_chaos_corpus_stress_for_topology(topology: BlobHarnessChunkTopology) -> Self {
        Self::new(
            22,
            topology,
            DeterministicBytePatternProfile::AmbientChaosCorpus,
            HeavyBlobFixtureMaterializationMode::StreamOnly,
            HeavyFixtureBackendProfile::NonCanonicalChaosCorpus,
        )
    }

    fn new(
        seed: u64,
        topology: BlobHarnessChunkTopology,
        byte_pattern_profile: DeterministicBytePatternProfile,
        materialization_mode: HeavyBlobFixtureMaterializationMode,
        backend_profile: HeavyFixtureBackendProfile,
    ) -> Self {
        Self {
            seed,
            topology,
            byte_pattern_profile,
            materialization_mode,
            backend_profile,
            expected_digest_basis: HeavyBlobExpectedDigestBasis {
                seed,
                logical_bytes: topology.logical_bytes(),
                chunk_bytes: topology.chunk_bytes(),
                expected_chunk_count: topology.chunk_count(),
                byte_pattern_profile,
            },
        }
    }

    pub const fn topology(&self) -> BlobHarnessChunkTopology {
        self.topology
    }

    pub const fn materialization_mode(&self) -> HeavyBlobFixtureMaterializationMode {
        self.materialization_mode
    }

    pub const fn byte_pattern_profile(&self) -> DeterministicBytePatternProfile {
        self.byte_pattern_profile
    }

    pub const fn backend_profile(&self) -> HeavyFixtureBackendProfile {
        self.backend_profile
    }

    pub const fn expected_digest_basis(&self) -> HeavyBlobExpectedDigestBasis {
        self.expected_digest_basis
    }
}

impl HeavyBlobExpectedDigestBasis {
    pub const fn seed(self) -> u64 {
        self.seed
    }

    pub const fn logical_bytes(self) -> u64 {
        self.logical_bytes
    }

    pub const fn chunk_bytes(self) -> u64 {
        self.chunk_bytes
    }

    pub const fn expected_chunk_count(self) -> u64 {
        self.expected_chunk_count
    }

    pub const fn byte_pattern_profile(self) -> DeterministicBytePatternProfile {
        self.byte_pattern_profile
    }
}

impl DeterministicBytePatternProfile {
    pub const fn canonical_heavy_blob_patterns() -> [Self; 4] {
        [
            Self::IncompressibleSeeded,
            Self::HighlyCompressibleRepeatedSpans,
            Self::ChunkBoundaryAdversarial,
            Self::RepeatedChunkDedupePressure,
        ]
    }

    pub const fn is_canonical_pattern(self) -> bool {
        matches!(
            self,
            Self::CanonicalMixed
                | Self::IncompressibleSeeded
                | Self::HighlyCompressibleRepeatedSpans
                | Self::ChunkBoundaryAdversarial
                | Self::RepeatedChunkDedupePressure
        )
    }
}

impl HeavyBlobVerificationPassBasis {
    pub(crate) const fn new(
        rolling_digest: u64,
        actual_bytes_streamed: u64,
        actual_chunk_count: u64,
    ) -> Self {
        Self {
            rolling_digest,
            actual_bytes_streamed,
            actual_chunk_count,
        }
    }

    pub const fn rolling_digest(self) -> u64 {
        self.rolling_digest
    }

    pub const fn actual_bytes_streamed(self) -> u64 {
        self.actual_bytes_streamed
    }

    pub const fn actual_chunk_count(self) -> u64 {
        self.actual_chunk_count
    }
}

impl HeavyBlobFixtureExecutionEvidence {
    pub(crate) fn observed(
        plan: &HeavyBlobFixturePlan,
        verification_pass_basis: HeavyBlobVerificationPassBasis,
        peak_resident_memory_bytes: u64,
        peak_allocation_count: u64,
        temporary_file_bytes: u64,
        disk_bytes_written: u64,
        cleanup_receipt: Option<HeavyFixtureCleanupReceipt>,
        disk_preflight_receipt: Option<HeavyFixtureDiskPreflightReceipt>,
    ) -> Self {
        Self {
            expected_digest_basis: plan.expected_digest_basis,
            verification_pass_basis,
            peak_resident_memory_bytes,
            peak_allocation_count,
            temporary_file_bytes,
            disk_bytes_written,
            cleanup_receipt,
            backend_profile: plan.backend_profile,
            materialization_mode: plan.materialization_mode,
            disk_preflight_receipt,
        }
    }

    pub const fn expected_digest_basis(&self) -> HeavyBlobExpectedDigestBasis {
        self.expected_digest_basis
    }

    pub const fn verification_pass_basis(&self) -> HeavyBlobVerificationPassBasis {
        self.verification_pass_basis
    }

    pub const fn peak_resident_memory_bytes(&self) -> u64 {
        self.peak_resident_memory_bytes
    }

    pub const fn peak_allocation_count(&self) -> u64 {
        self.peak_allocation_count
    }

    pub const fn temporary_file_bytes(&self) -> u64 {
        self.temporary_file_bytes
    }

    pub const fn disk_bytes_written(&self) -> u64 {
        self.disk_bytes_written
    }

    pub fn cleanup_receipt(&self) -> Option<&HeavyFixtureCleanupReceipt> {
        self.cleanup_receipt.as_ref()
    }

    pub fn disk_preflight_receipt(&self) -> Option<&HeavyFixtureDiskPreflightReceipt> {
        self.disk_preflight_receipt.as_ref()
    }

    pub const fn backend_profile(&self) -> HeavyFixtureBackendProfile {
        self.backend_profile
    }

    pub const fn materialization_mode(&self) -> HeavyBlobFixtureMaterializationMode {
        self.materialization_mode
    }
}

pub const fn deny_sparse_only_heavy_qualification() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::SparseOnlyProofNotCanonical
}

pub const fn deny_logical_size_only_heavy_qualification() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::LogicalSizeOnlyProofNotCanonical
}

pub const fn deny_whole_object_expected_buffer() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::WholeObjectExpectedBufferNotCanonical
}

pub const fn deny_generated_expected_byte_artifact() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::GeneratedExpectedByteArtifactNotCanonical
}

pub const fn deny_hidden_temporary_sidecar() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::HiddenTemporarySidecarNotCanonical
}

pub const fn deny_ambient_chaos_corpus_as_canonical() -> HeavyBlobQualificationDenial {
    HeavyBlobQualificationDenial::AmbientCorpusNotCanonical
}

pub fn admit_canonical_qualification_lane(
    plan: &HeavyBlobFixturePlan,
) -> Result<(), HeavyBlobQualificationDenial> {
    match plan.byte_pattern_profile() {
        DeterministicBytePatternProfile::SparseDeclarationDenied => {
            Err(HeavyBlobQualificationDenial::SparseOnlyProofNotCanonical)
        }
        DeterministicBytePatternProfile::LogicalSizeOnlyDenied => {
            Err(HeavyBlobQualificationDenial::LogicalSizeOnlyProofNotCanonical)
        }
        DeterministicBytePatternProfile::WholeObjectExpectedBufferDenied => {
            Err(HeavyBlobQualificationDenial::WholeObjectExpectedBufferNotCanonical)
        }
        DeterministicBytePatternProfile::GeneratedExpectedByteArtifactDenied => {
            Err(HeavyBlobQualificationDenial::GeneratedExpectedByteArtifactNotCanonical)
        }
        DeterministicBytePatternProfile::HiddenTemporarySidecarDenied => {
            Err(HeavyBlobQualificationDenial::HiddenTemporarySidecarNotCanonical)
        }
        DeterministicBytePatternProfile::AmbientChaosCorpus => {
            Err(HeavyBlobQualificationDenial::AmbientCorpusNotCanonical)
        }
        _ => Ok(()),
    }
}
