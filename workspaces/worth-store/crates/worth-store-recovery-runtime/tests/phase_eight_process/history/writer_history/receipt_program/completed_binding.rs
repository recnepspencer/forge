use std::collections::{BTreeMap, BTreeSet};

use super::super::super::{parent_oracle, schedule};
use super::super::ExpectedWriterHistory;
use super::decoding::IdentityReceiptEntry;

pub(super) fn bind(
    entries: &[IdentityReceiptEntry],
    expected: &ExpectedWriterHistory,
) -> Result<BTreeMap<[u8; 32], parent_oracle::ExpectedCanonicalRecord>, String> {
    let expected_by_material = expected
        .payloads()
        .iter()
        .enumerate()
        .map(|(ordinal, payload)| {
            (
                schedule::mutation_material(expected.seed, ordinal as u64),
                (ordinal, payload),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let scheduled_materials = schedule::scheduled_materials(expected.seed, entries.len());
    let mut seen_operations = BTreeSet::new();
    let mut seen_records = BTreeSet::new();
    let mut durable_bindings = BTreeMap::new();
    for (ordinal, entry) in entries.iter().enumerate() {
        if entry.material != scheduled_materials[ordinal] || entry.fate != 1 {
            return Err(format!(
                "C8 identity receipt schedule or fate mismatch at position {ordinal}"
            ));
        }
        let (submitted_ordinal, payload) =
            expected_by_material.get(&entry.material).ok_or_else(|| {
                format!("C8 identity receipt contains an unknown operation at position {ordinal}")
            })?;
        let (allocation_epoch, record_ordinal) = entry
            .record
            .ok_or_else(|| format!("C8 completed operation omitted its record at {ordinal}"))?;
        if allocation_epoch == [0; 16] || record_ordinal == 0 {
            return Err(format!(
                "C8 identity receipt has invalid completed record at {ordinal}"
            ));
        }
        if !seen_operations.insert(*submitted_ordinal) {
            return Err("C8 identity receipt contains a duplicate operation".to_owned());
        }
        if !seen_records.insert((allocation_epoch, record_ordinal)) {
            return Err("C8 identity receipt contains a duplicate record identity".to_owned());
        }
        durable_bindings.insert(
            entry.idempotency,
            parent_oracle::ExpectedCanonicalRecord {
                allocation_epoch,
                ordinal: record_ordinal,
                payload: (*payload).clone(),
                redo_digest: [0; 32],
            },
        );
    }
    Ok(durable_bindings)
}
