use crate::backend::records::CompactionProductRecord;

pub(super) fn rebuild_superseded_families(
    record: &CompactionProductRecord,
) -> Vec<crate::SupersededPhysicalFamily> {
    record
        .superseded_families
        .iter()
        .cloned()
        .zip(record.superseded_artifact_ids.iter().cloned())
        .map(|(family_label, artifact_id)| {
            crate::SupersededPhysicalFamily::new(family_label, artifact_id, None)
        })
        .collect()
}
