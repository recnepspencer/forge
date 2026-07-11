#[cfg(any(test, feature = "certification-test-support"))]
use crate::PhysicalSimulationPlan;
#[cfg(any(test, feature = "certification-test-support"))]
use forge_store_blob_chunks::HeavyBlobFixtureMaterializationMode;
#[cfg(any(test, feature = "certification-test-support"))]
use forge_store_blob_chunks::{BlobHarnessSecurityScopeClass, BlobHarnessSizeClass};
#[cfg(any(test, feature = "certification-test-support"))]
use forge_store_physical_backend::HeavyFixtureBackendProfile;

#[cfg(any(test, feature = "certification-test-support"))]
use forge_store_blob_chunks::certification_test_authority::BlobHarnessExecutedWitness as S7BlobHarnessExecutedActorEvidence;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S7BlobHarnessOracleObservation {
    byte_equality_verified: bool,
    chunk_ordering_verified: bool,
    digest_checksum_distinction_verified: bool,
    reachability_verified: bool,
    cross_scope_dedupe_guarded: bool,
    constant_memory_envelope_held: bool,
    no_sidecar_path_verified: bool,
    heavy_evidence_verified: bool,
    heavy_cleanup_verified: bool,
    heavy_pattern_lane_verified: bool,
}

impl S7BlobHarnessOracleObservation {
    #[cfg(any(test, feature = "certification-test-support"))]
    pub(crate) fn from_executed_witness(
        plan: &PhysicalSimulationPlan,
        witness: &S7BlobHarnessExecutedActorEvidence,
    ) -> Option<Self> {
        let metadata = plan.s7_blob_harness_metadata()?;
        let memory_envelope_requires_streaming = matches!(
            metadata.size_class(),
            BlobHarnessSizeClass::MemoryEnvelopeExceeding
                | BlobHarnessSizeClass::HeavyMultiGbDeclared
        );
        let allocation_bytes = witness.allocation_bytes();
        let logical_bytes = witness.executed_topology().logical_bytes();
        let constant_memory_envelope_held = if memory_envelope_requires_streaming {
            allocation_bytes < logical_bytes
        } else {
            allocation_bytes <= logical_bytes
        };
        let requires_phase23_heavy_evidence =
            metadata.size_class() == BlobHarnessSizeClass::HeavyMultiGbDeclared;
        let heavy_fixture = witness.heavy_fixture_evidence();
        let heavy_evidence_verified = heavy_fixture
            .map(|evidence| {
                evidence.expected_digest_basis().logical_bytes() == logical_bytes
                    && evidence.expected_digest_basis().expected_chunk_count()
                        == witness.executed_topology().chunk_count()
                    && evidence.verification_pass_basis().actual_bytes_streamed() == logical_bytes
                    && evidence.verification_pass_basis().actual_chunk_count()
                        == witness.executed_topology().chunk_count()
                    && evidence.peak_allocation_count() > 0
                    && evidence.backend_profile() == HeavyFixtureBackendProfile::StoreOwnedLocalDisk
            })
            .unwrap_or(!requires_phase23_heavy_evidence);
        let heavy_cleanup_verified = heavy_fixture
            .map(|evidence| {
                if evidence.materialization_mode() == HeavyBlobFixtureMaterializationMode::TempFile
                {
                    evidence
                        .cleanup_receipt()
                        .map(|receipt| receipt.completed())
                        .unwrap_or(false)
                        && evidence.temporary_file_bytes() == evidence.disk_bytes_written()
                } else {
                    evidence.temporary_file_bytes() == 0
                        && evidence.disk_bytes_written() == 0
                        && evidence.cleanup_receipt().is_none()
                }
            })
            .unwrap_or(true);
        let heavy_pattern_lane_verified = heavy_fixture
            .map(|evidence| {
                evidence
                    .expected_digest_basis()
                    .byte_pattern_profile()
                    .is_canonical_phase23_pattern()
            })
            .unwrap_or(true);
        Some(Self {
            byte_equality_verified: witness.export_logical_digest_matches_lifecycle()
                && witness.export_declared_total_bytes() == logical_bytes,
            chunk_ordering_verified: witness.export_declared_chunk_count()
                == witness.executed_topology().chunk_count(),
            digest_checksum_distinction_verified: witness
                .export_checksum_distinct_from_stored_digest(),
            reachability_verified: witness.reachability_stored_digest_matches_lifecycle()
                && witness.reachability_reference_edges()
                    >= witness.executed_topology().chunk_count(),
            cross_scope_dedupe_guarded: !matches!(
                metadata.security_scope_class(),
                BlobHarnessSecurityScopeClass::CrossScopeDenied
            ) || witness.cross_scope_dedupe_denied(),
            constant_memory_envelope_held,
            no_sidecar_path_verified: constant_memory_envelope_held
                && witness.export_declared_total_bytes() == logical_bytes,
            heavy_evidence_verified,
            heavy_cleanup_verified,
            heavy_pattern_lane_verified,
        })
    }

    pub const fn byte_equality_verified(self) -> bool {
        self.byte_equality_verified
    }

    pub const fn chunk_ordering_verified(self) -> bool {
        self.chunk_ordering_verified
    }

    pub const fn digest_checksum_distinction_verified(self) -> bool {
        self.digest_checksum_distinction_verified
    }

    pub const fn reachability_verified(self) -> bool {
        self.reachability_verified
    }

    pub const fn cross_scope_dedupe_guarded(self) -> bool {
        self.cross_scope_dedupe_guarded
    }

    pub const fn constant_memory_envelope_held(self) -> bool {
        self.constant_memory_envelope_held
    }

    pub const fn no_sidecar_path_verified(self) -> bool {
        self.no_sidecar_path_verified
    }

    pub const fn heavy_evidence_verified(self) -> bool {
        self.heavy_evidence_verified
    }

    pub const fn heavy_cleanup_verified(self) -> bool {
        self.heavy_cleanup_verified
    }

    pub const fn heavy_pattern_lane_verified(self) -> bool {
        self.heavy_pattern_lane_verified
    }
}
