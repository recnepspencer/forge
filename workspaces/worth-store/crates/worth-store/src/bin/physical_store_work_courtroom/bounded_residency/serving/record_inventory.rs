use worth_store::physical_runtime::{
    PhysicalRecordId, RecordByteLimit, RecordCountLimit, RecordReadLimits, RecordScanOutcome,
    RecordScanRequest, ServingPhysicalRuntime,
};

use super::super::configuration::BoundedResidencyConfiguration;

const INVENTORY_SCRATCH_BYTES: usize = 8 * 1024;

pub(super) fn discover(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
) -> Result<Box<[PhysicalRecordId]>, String> {
    let mut scan = serving
        .records()
        .scan(
            RecordScanRequest::from_start()
                .with_batch_limit(RecordCountLimit::new(1).expect("one record is nonzero")),
        )
        .map_err(|failure| format!("bounded-residency discovery scan denied: {failure:?}"))?;
    let mut scratch = [0_u8; INVENTORY_SCRATCH_BYTES];
    let mut records = Vec::with_capacity(configuration.record_count());
    while let RecordScanOutcome::Batch(batch) = scan
        .read_next_into(&mut scratch)
        .map_err(|failure| format!("bounded-residency discovery scan failed: {failure:?}"))?
    {
        let record = batch
            .records()
            .first()
            .ok_or_else(|| "bounded-residency discovery emitted an empty batch".to_owned())?;
        let identity = record.record_id();
        let declared_bytes = record.declared_payload_bytes();
        let ordinal = match batch.payload(0) {
            Some(payload) => {
                super::super::workload::identify_record(configuration, declared_bytes, payload)?
            }
            None => identify_deferred_record(serving, configuration, identity, declared_bytes)?,
        };
        records.push((ordinal, identity));
    }
    require_complete_inventory(records, configuration)
}

fn identify_deferred_record(
    serving: &ServingPhysicalRuntime,
    configuration: BoundedResidencyConfiguration,
    record: PhysicalRecordId,
    declared_bytes: u64,
) -> Result<usize, String> {
    let maximum = u32::try_from(declared_bytes)
        .ok()
        .and_then(RecordByteLimit::new)
        .ok_or_else(|| "bounded-residency deferred payload limit is invalid".to_owned())?;
    let mut session = serving
        .records()
        .open(record, RecordReadLimits::new(maximum))
        .map_err(|failure| format!("bounded-residency deferred open failed: {failure:?}"))?;
    let ordinal = {
        let chunk = session
            .next_chunk()
            .map_err(|failure| format!("bounded-residency deferred view failed: {failure:?}"))?
            .ok_or_else(|| "bounded-residency deferred record had no payload".to_owned())?;
        super::super::workload::identify_record(configuration, declared_bytes, chunk.bytes())?
    };
    if session.observation().explicit_copy_count() != 0 {
        return Err("bounded-residency deferred inventory copied record bytes".to_owned());
    }
    Ok(ordinal)
}

fn require_complete_inventory(
    mut records: Vec<(usize, PhysicalRecordId)>,
    configuration: BoundedResidencyConfiguration,
) -> Result<Box<[PhysicalRecordId]>, String> {
    if records.len() != configuration.record_count() {
        return Err("bounded-residency discovery omitted records".to_owned());
    }
    records.sort_unstable_by_key(|(ordinal, _)| *ordinal);
    if records
        .iter()
        .enumerate()
        .any(|(expected, (actual, _))| expected != *actual)
    {
        return Err(
            "bounded-residency discovery duplicated or omitted a workload ordinal".to_owned(),
        );
    }
    Ok(records
        .into_iter()
        .map(|(_, record)| record)
        .collect::<Vec<_>>()
        .into_boxed_slice())
}
