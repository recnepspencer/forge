use sha2::{Digest, Sha256};
use worth_store::physical_runtime::{
    PhysicalManifestCapacityTransition, PhysicalMutationIdempotencyMaterial,
    PhysicalRecordInitialization, RecordAppendBatch, ServingPhysicalRuntime,
};

use super::{configuration::BoundedResidencyConfiguration, workload::record_payload};

const INLINE_BATCH_WIDTH: usize = 64;
// Each extent frame carries enough WAL metadata that four 1-MiB records can
// exceed the admitted 8-MiB segment as one atomic group. Keep producer groups
// below that physical admission boundary so rotation remains a valid outcome.
const EXTENT_BATCH_WIDTH: usize = 2;

pub(super) fn run(
    invocation: super::super::arguments::BoundedResidencyProducerInvocation,
) -> Result<(), String> {
    let configuration = BoundedResidencyConfiguration::read(&invocation.configuration)?;
    let (format, placement, access) = super::super::configuration::record_configuration();
    let policy = configuration
        .producer_policy(format)
        .into_result()
        .map_err(|denial| format!("bounded-residency producer policy denied: {denial:?}"))?;
    let media = super::super::admission::admit_media(&invocation.root, None)?;
    let durability = super::super::admission::admit_durability_with_checkpoint_memory(
        &media,
        configuration.checkpoint_memory_limit(),
    )?;
    let serving = super::super::admission::require_serving(
        media.initialize_record_store(
            PhysicalRecordInitialization::new(format, placement, access, durability)
                .with_residency_policy(policy),
        ),
        "bounded-residency producer initialization",
    )?;
    let (published, digest) = publish_workload(&serving, configuration, placement)?;
    let observation = serving.residency_observation();
    let store = serving.store_identity();
    let runtime = serving.runtime_identity();
    let generation = observation.store_generation();
    let peak_resident_bytes = observation.counters().peak_resident_bytes();
    let close = serving.close();
    if close.residency().requires_inspection() {
        return Err("bounded-residency producer closed with residency inspection".to_owned());
    }
    println!(
        "BOUNDED_RESIDENCY_PRODUCER {} {} {} {} {} {} {} {}",
        std::process::id(),
        hex(&store.bytes()),
        runtime.get(),
        generation.get(),
        published,
        configuration.producer_payload_bytes()?,
        hex(&digest),
        peak_resident_bytes,
    );
    Ok(())
}

fn publish_workload(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    placement: worth_store::physical_runtime::AdmittedRecordPlacementPolicy,
) -> Result<(usize, [u8; 32]), String> {
    let mut published = 0_usize;
    let mut digest = Sha256::new();
    while published < configuration.producer_record_count() {
        let width = batch_width(configuration, published);
        let end = published
            .saturating_add(width)
            .min(configuration.producer_record_count());
        let mut batch = RecordAppendBatch::builder();
        for ordinal in published..end {
            let payload = record_payload(configuration, ordinal)?;
            update_digest(&mut digest, &payload);
            batch = batch.push_owned(payload);
        }
        let batch = batch
            .build()
            .map_err(|denial| format!("bounded-residency producer batch denied: {denial:?}"))?;
        let result = serving.certification_publish_single_durable_mutation(
            placement,
            PhysicalManifestCapacityTransition::PreserveCurrent,
            mutation_material(published, end),
            batch,
        );
        let published_records = result
            .settled_members()
            .iter()
            .map(|member| member.persisted_records().len())
            .sum::<usize>();
        if published_records != end - published {
            return Err("bounded-residency producer omitted published identities".to_owned());
        }
        published = end;
    }
    for ordinal in published..configuration.record_count() {
        let payload = record_payload(configuration, ordinal)?;
        update_digest(&mut digest, &payload);
    }
    Ok((published, digest.finalize().into()))
}

fn mutation_material(start: usize, end: usize) -> PhysicalMutationIdempotencyMaterial {
    let mut digest = Sha256::new();
    digest.update(b"worth-store.bounded-residency.producer.v1");
    digest.update((start as u64).to_le_bytes());
    digest.update((end as u64).to_le_bytes());
    PhysicalMutationIdempotencyMaterial::new(digest.finalize().into())
}

fn batch_width(configuration: BoundedResidencyConfiguration, ordinal: usize) -> usize {
    match configuration.record_bytes(ordinal) {
        Some(bytes) if bytes <= 3_000 => INLINE_BATCH_WIDTH,
        Some(_) => EXTENT_BATCH_WIDTH,
        None => 1,
    }
}

fn update_digest(digest: &mut Sha256, payload: &[u8]) {
    digest.update((payload.len() as u64).to_le_bytes());
    digest.update(payload);
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}
