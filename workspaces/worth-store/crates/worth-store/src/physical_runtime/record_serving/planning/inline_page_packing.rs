use super::super::{
    planning::batch_placement::MaterializedInlineInput, AdmittedPhysicalRecordFormat,
    AdmittedRecordPlacementPolicy,
};

pub(in crate::physical_runtime::record_serving) fn fitting_prefix(
    records: &[MaterializedInlineInput],
    physical_free: usize,
    policy_free: usize,
) -> usize {
    let limit = physical_free.min(policy_free);
    let mut used = 0_usize;
    records
        .iter()
        .take_while(|record| {
            let required = worth_store_physical_format::DURABLE_INLINE_SLOT_BYTES
                .saturating_add(record.bytes.len());
            if used.saturating_add(required) > limit {
                false
            } else {
                used += required;
                true
            }
        })
        .count()
}

pub(in crate::physical_runtime::record_serving) fn new_page_fill_capacity(
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
) -> usize {
    let payload = format.declaration().page_size().bytes() as usize
        - worth_store_physical_format::DURABLE_FRAME_HEADER_BYTES;
    payload * placement.page_fill().get() as usize / 100
        - worth_store_physical_format::DURABLE_INLINE_PAGE_PREFIX_BYTES
}

pub(in crate::physical_runtime::record_serving) fn remaining_policy_capacity(
    format: AdmittedPhysicalRecordFormat,
    placement: AdmittedRecordPlacementPolicy,
    physical_free: usize,
) -> usize {
    let payload = format.declaration().page_size().bytes() as usize
        - worth_store_physical_format::DURABLE_FRAME_HEADER_BYTES;
    let initially_free = payload - worth_store_physical_format::DURABLE_INLINE_PAGE_PREFIX_BYTES;
    let already_used = initially_free.saturating_sub(physical_free);
    new_page_fill_capacity(format, placement).saturating_sub(already_used)
}
