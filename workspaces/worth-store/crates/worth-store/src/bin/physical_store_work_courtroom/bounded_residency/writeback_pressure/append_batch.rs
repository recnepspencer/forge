use worth_store::physical_runtime::RecordAppendBatch;

use super::super::{configuration::BoundedResidencyConfiguration, workload::record_payload};

pub(super) fn build(
    configuration: BoundedResidencyConfiguration,
    ordinal: usize,
) -> Result<RecordAppendBatch, String> {
    let payload = record_payload(configuration, ordinal)?;
    RecordAppendBatch::builder()
        .push_owned(payload)
        .build()
        .map_err(|denial| format!("serving append batch denied: {denial:?}"))
}
