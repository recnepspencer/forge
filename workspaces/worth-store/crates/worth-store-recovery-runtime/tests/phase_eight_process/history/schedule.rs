use std::path::Path;

use sha2::{Digest, Sha256};

use super::{ExpectedWriterHistory, SubmittedOperationProgram};

#[path = "schedule/in_flight_mutation.rs"]
mod in_flight_mutation;
#[path = "schedule/parent_writer_durability_profile_selection.rs"]
mod parent_writer_durability_profile_selection;

pub(crate) use in_flight_mutation::ExpectedInFlightMutation;
pub(crate) use parent_writer_durability_profile_selection::ParentWriterDurabilityProfileSelection;

pub(super) const OPERATION_PROGRAM_MAGIC: &[u8] = b"WORTH-C8-SUBMITTED-OPERATIONS-V1\n";
pub(super) const OPERATION_COUNT: usize = 96;
pub(super) const OPERATION_PAYLOAD_BYTES: usize = 8 * 1024;

pub(super) fn payload(seed: u64, ordinal: u64) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(OPERATION_PAYLOAD_BYTES);
    for block in 0..(OPERATION_PAYLOAD_BYTES / 32) {
        let mut digest = Sha256::new();
        digest.update(b"worth.store.c8.parent-submitted-record.v1");
        digest.update(seed.to_le_bytes());
        digest.update(ordinal.to_le_bytes());
        digest.update((block as u64).to_le_bytes());
        bytes.extend_from_slice(&digest.finalize());
    }
    bytes
}

pub(super) fn mutation_material(seed: u64, ordinal: u64) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.c8.parent-submitted-operation.v1");
    digest.update(seed.to_le_bytes());
    digest.update(ordinal.to_le_bytes());
    digest.finalize().into()
}

pub(super) fn no_effect_material(seed: u64) -> [u8; 32] {
    let mut digest = Sha256::new();
    digest.update(b"worth.store.c8.proven-no-effect.v1");
    digest.update(seed.to_le_bytes());
    digest.finalize().into()
}

pub(super) fn scheduled_materials(seed: u64, count: usize) -> Vec<[u8; 32]> {
    let mut order = (0..count).collect::<Vec<_>>();
    let mut state = seed ^ 0xC8_5C_4A_01;
    for index in (1..order.len()).rev() {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let swap = (state as usize) % (index + 1);
        order.swap(index, swap);
    }
    order
        .into_iter()
        .map(|ordinal| mutation_material(seed, ordinal as u64))
        .collect()
}

pub(crate) fn create_checkpoint_operation_program(
    parent: &Path,
    schedule_seed: u64,
    dirty_seed: u64,
) -> Result<SubmittedOperationProgram, String> {
    create_checkpoint_operation_program_with_operation_count(
        parent,
        schedule_seed,
        dirty_seed,
        OPERATION_COUNT,
    )
}

pub(crate) fn create_checkpoint_operation_program_with_operation_count(
    parent: &Path,
    schedule_seed: u64,
    dirty_seed: u64,
    operation_count: usize,
) -> Result<SubmittedOperationProgram, String> {
    create_operation_program_with_profile(
        parent,
        schedule_seed,
        operation_count,
        ExpectedInFlightMutation::checkpoint_exact_prefix(dirty_seed),
        ParentWriterDurabilityProfileSelection::CheckpointWritebackV1,
    )
}

pub(crate) fn create_durable_before_ack_operation_program_with_operation_count(
    parent: &Path,
    schedule_seed: u64,
    dirty_seed: u64,
    operation_count: usize,
) -> Result<SubmittedOperationProgram, String> {
    create_operation_program_with_profile(
        parent,
        schedule_seed,
        operation_count,
        ExpectedInFlightMutation::durable_before_ack(dirty_seed),
        ParentWriterDurabilityProfileSelection::CheckpointWritebackV1,
    )
}

pub(crate) fn create_cleanup_rotation_operation_program_with_operation_count(
    parent: &Path,
    schedule_seed: u64,
    dirty_seed: u64,
    operation_count: usize,
) -> Result<SubmittedOperationProgram, String> {
    create_operation_program_with_profile(
        parent,
        schedule_seed,
        operation_count,
        ExpectedInFlightMutation::durable_before_ack(dirty_seed),
        ParentWriterDurabilityProfileSelection::CleanupRotationV1,
    )
}

fn create_operation_program_with_profile(
    parent: &Path,
    schedule_seed: u64,
    operation_count: usize,
    in_flight: ExpectedInFlightMutation,
    writer_profile_selection: ParentWriterDurabilityProfileSelection,
) -> Result<SubmittedOperationProgram, String> {
    let expected = ExpectedWriterHistory::from_profile(schedule_seed, operation_count, in_flight);
    let path = parent.join(format!("c8-submitted-operations-{schedule_seed}"));
    let identity_receipt = parent.join(format!("c8-completed-identities-{schedule_seed}"));
    let barrier_receipt = parent.join(format!("c8-checkpoint-barrier-{schedule_seed}"));
    let mut encoded = Vec::new();
    encoded.extend_from_slice(OPERATION_PROGRAM_MAGIC);
    encoded.extend_from_slice(&(operation_count as u32).to_le_bytes());
    for (ordinal, payload) in expected.payloads().iter().enumerate() {
        encoded.extend_from_slice(&mutation_material(schedule_seed, ordinal as u64));
        encoded.extend_from_slice(&(payload.len() as u64).to_le_bytes());
        encoded.extend_from_slice(payload);
    }
    std::fs::write(&path, encoded)
        .map_err(|error| format!("write submitted operation program: {error}"))?;
    Ok(SubmittedOperationProgram {
        path,
        identity_receipt,
        barrier_receipt,
        expected,
        writer_profile_selection,
    })
}
