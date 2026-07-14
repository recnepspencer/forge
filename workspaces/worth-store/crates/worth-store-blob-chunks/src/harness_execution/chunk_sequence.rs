use crate::handoffs::{BlobHarnessChunkTopology, BlobHarnessSecurityScopeClass};
use crate::heavy_fixture::{
    DeterministicBytePatternProfile, HeavyBlobFixtureExecutionEvidence, HeavyBlobFixturePlan,
    HeavyBlobVerificationPassBasis,
};
use crate::{
    BlobChunkSequenceAdmission, BlobChunkSize, BlobChunkingRuleAdmission,
    HeavyBlobFixtureMaterializationMode,
};
use worth_store_physical_backend::{
    cleanup_heavy_fixture_materialization, preflight_heavy_fixture_directory,
    HeavyFixtureDiskPreflightReceipt, HeavyFixtureMaterializationDirectory,
    HeavyFixtureTempFileMaterialization,
};

use super::backend::physical_payload_for_bytes;
use super::scope_admission::blob_scope;

#[derive(Debug, Clone)]
pub(super) struct GeneratedBlobSequence {
    pub(super) sequence: crate::AdmittedBlobChunkSequence,
    pub(super) executed_topology: BlobHarnessChunkTopology,
    pub(super) declared_topology: BlobHarnessChunkTopology,
    pub(super) byte_pattern_profile: DeterministicBytePatternProfile,
    pub(super) peak_window_bytes: u64,
    pub(super) heavy_fixture_evidence: Option<HeavyBlobFixtureExecutionEvidence>,
}

pub(super) fn build_chunk_sequence(
    case: &str,
    scope_class: BlobHarnessSecurityScopeClass,
    declared_topology: BlobHarnessChunkTopology,
    heavy_fixture_plan: Option<&HeavyBlobFixturePlan>,
) -> GeneratedBlobSequence {
    let executed_topology = execution_topology(declared_topology, heavy_fixture_plan);
    let byte_pattern_profile = heavy_fixture_plan
        .map(HeavyBlobFixturePlan::byte_pattern_profile)
        .unwrap_or(DeterministicBytePatternProfile::CanonicalMixed);
    let rule = BlobChunkingRuleAdmission::fixed_size(
        BlobChunkSize::from_bytes(executed_topology.chunk_bytes()).expect("chunk size"),
    )
    .expect("rule");
    let mut admission = BlobChunkSequenceAdmission::start(
        blob_scope(case, scope_class),
        rule,
        executed_topology.logical_bytes(),
    )
    .expect("sequence");
    let (mut temp_file, disk_preflight_receipt) =
        begin_temp_materialization(heavy_fixture_plan, case);
    let mut actual_bytes_streamed = 0_u64;
    let mut rolling_digest = 0_u64;
    let mut peak_window_bytes = 0_u64;
    let mut peak_allocation_count = 0_u64;
    for ordinal in 0..executed_topology.chunk_count() {
        let offset = chunk_offset(executed_topology, ordinal);
        let bytes = deterministic_chunk_bytes(executed_topology, byte_pattern_profile, ordinal);
        actual_bytes_streamed += bytes.len() as u64;
        rolling_digest = update_rolling_digest(rolling_digest, &bytes);
        if let Some(materialization) = temp_file.as_mut() {
            materialization
                .append_chunk(&bytes)
                .expect("temp fixture chunk");
        }
        admission = admission
            .push_payload(offset, physical_payload_for_bytes(&bytes))
            .expect("payload");
        peak_window_bytes = peak_window_bytes.max(bytes.len() as u64);
        peak_allocation_count = peak_allocation_count.max(1);
    }
    let heavy_fixture_evidence = heavy_fixture_plan.map(|plan| {
        let (temporary_file_bytes, disk_bytes_written, cleanup_receipt) =
            finalize_temp_materialization(temp_file);
        HeavyBlobFixtureExecutionEvidence::observed(
            plan,
            HeavyBlobVerificationPassBasis::new(
                rolling_digest,
                actual_bytes_streamed,
                executed_topology.chunk_count(),
            ),
            peak_window_bytes,
            peak_allocation_count,
            temporary_file_bytes,
            disk_bytes_written,
            cleanup_receipt,
            disk_preflight_receipt,
        )
    });
    GeneratedBlobSequence {
        sequence: admission.finish().expect("finish"),
        executed_topology,
        declared_topology,
        byte_pattern_profile,
        peak_window_bytes,
        heavy_fixture_evidence,
    }
}

pub(super) fn chunk_window_for_ordinal(
    generated: &GeneratedBlobSequence,
    ordinal: u64,
) -> (u64, Vec<u8>) {
    let offset = chunk_offset(generated.executed_topology, ordinal);
    (
        offset,
        deterministic_chunk_bytes(
            generated.executed_topology,
            generated.byte_pattern_profile,
            ordinal,
        ),
    )
}

fn blob_harness_execution_topology(
    declared_topology: BlobHarnessChunkTopology,
) -> BlobHarnessChunkTopology {
    const HEAVY_DECLARED_EXECUTION_THRESHOLD: u64 = 1024 * 1024 * 1024;
    const REPRESENTATIVE_EXECUTED_BYTES: u64 = 8 * 1024 * 1024;
    if declared_topology.logical_bytes() < HEAVY_DECLARED_EXECUTION_THRESHOLD {
        return declared_topology;
    }
    let logical_bytes = declared_topology
        .chunk_bytes()
        .min(REPRESENTATIVE_EXECUTED_BYTES);
    BlobHarnessChunkTopology::from_executed_projection(
        logical_bytes.div_ceil(declared_topology.chunk_bytes()),
        logical_bytes,
        declared_topology.chunk_bytes(),
    )
    .expect("phase 22 execution projection")
}

fn execution_topology(
    declared_topology: BlobHarnessChunkTopology,
    heavy_fixture_plan: Option<&HeavyBlobFixturePlan>,
) -> BlobHarnessChunkTopology {
    heavy_fixture_plan
        .map(HeavyBlobFixturePlan::topology)
        .unwrap_or_else(|| blob_harness_execution_topology(declared_topology))
}

fn begin_temp_materialization(
    heavy_fixture_plan: Option<&HeavyBlobFixturePlan>,
    case: &str,
) -> (
    Option<HeavyFixtureTempFileMaterialization>,
    Option<HeavyFixtureDiskPreflightReceipt>,
) {
    let Some(plan) = heavy_fixture_plan else {
        return (None, None);
    };
    if plan.materialization_mode() != HeavyBlobFixtureMaterializationMode::TempFile {
        return (None, None);
    }
    let directory = HeavyFixtureMaterializationDirectory::named_heavy_fixture_root();
    let preflight = preflight_heavy_fixture_directory(
        directory,
        plan.topology().logical_bytes(),
        plan.backend_profile(),
    )
    .expect("heavy fixture preflight");
    let temp = HeavyFixtureTempFileMaterialization::begin(&preflight, case).expect("temp fixture");
    (Some(temp), Some(preflight))
}

fn finalize_temp_materialization(
    temp_file: Option<HeavyFixtureTempFileMaterialization>,
) -> (
    u64,
    u64,
    Option<worth_store_physical_backend::HeavyFixtureCleanupReceipt>,
) {
    let Some(materialization) = temp_file else {
        return (0, 0, None);
    };
    let bytes_written = materialization.bytes_written();
    let cleanup = cleanup_heavy_fixture_materialization(materialization).expect("cleanup");
    (bytes_written, bytes_written, Some(cleanup))
}

fn update_rolling_digest(current: u64, bytes: &[u8]) -> u64 {
    bytes
        .iter()
        .fold(current.rotate_left(9) ^ 0x9e37_79b9, |digest, byte| {
            digest
                .wrapping_mul(1_099_511_628_211)
                .wrapping_add(u64::from(*byte) + 1)
        })
}

fn deterministic_chunk_bytes(
    topology: BlobHarnessChunkTopology,
    pattern: DeterministicBytePatternProfile,
    ordinal: u64,
) -> Vec<u8> {
    let offset = chunk_offset(topology, ordinal);
    let len = topology
        .chunk_bytes()
        .min(topology.logical_bytes() - offset) as usize;
    let seed = topology.chunk_bytes()
        ^ topology.logical_bytes()
        ^ ordinal.rotate_left(7)
        ^ offset.rotate_left(13);
    match pattern {
        DeterministicBytePatternProfile::CanonicalMixed => (0..len)
            .map(|index| {
                let pattern = DeterministicBytePatternProfile::canonical_heavy_blob_patterns()[index
                    % DeterministicBytePatternProfile::canonical_heavy_blob_patterns().len()];
                patterned_byte(pattern, seed, index, len)
            })
            .collect(),
        _ => (0..len)
            .map(|index| patterned_byte(pattern, seed, index, len))
            .collect(),
    }
}

fn patterned_byte(
    pattern: DeterministicBytePatternProfile,
    seed: u64,
    index: usize,
    len: usize,
) -> u8 {
    match pattern {
        DeterministicBytePatternProfile::CanonicalMixed
        | DeterministicBytePatternProfile::IncompressibleSeeded => {
            ((seed.wrapping_add(index as u64).wrapping_mul(17)) & 0xff) as u8
        }
        DeterministicBytePatternProfile::HighlyCompressibleRepeatedSpans => {
            ((index / 4096) % 4) as u8 * 0x33
        }
        DeterministicBytePatternProfile::ChunkBoundaryAdversarial => {
            if index < 32 || index + 32 >= len {
                0xffu8.wrapping_sub((index & 0xff) as u8)
            } else {
                ((seed.rotate_left(5).wrapping_add(index as u64)) & 0xff) as u8
            }
        }
        DeterministicBytePatternProfile::RepeatedChunkDedupePressure => {
            (((seed >> 3).wrapping_add((index % 1024) as u64)) & 0xff) as u8
        }
        DeterministicBytePatternProfile::SparseDeclarationDenied
        | DeterministicBytePatternProfile::LogicalSizeOnlyDenied
        | DeterministicBytePatternProfile::HiddenTemporarySidecarDenied
        | DeterministicBytePatternProfile::WholeObjectExpectedBufferDenied
        | DeterministicBytePatternProfile::GeneratedExpectedByteArtifactDenied
        | DeterministicBytePatternProfile::AmbientChaosCorpus => {
            ((seed.wrapping_add(index as u64).wrapping_mul(29)) & 0xff) as u8
        }
    }
}

const fn chunk_offset(topology: BlobHarnessChunkTopology, ordinal: u64) -> u64 {
    ordinal * topology.chunk_bytes()
}
