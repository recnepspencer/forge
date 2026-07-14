use super::{decode_leaf_record, BaselineBTreeExecutionDenial, BaselineBTreeLeafRecord};
use worth_store_physical_format::{
    access::page::PageAccess, PhysicalLayoutAccessCounterSnapshot, PhysicalReferenceAuthority,
    PhysicalStoreRuntime, SlotGenerationCell,
};

pub(super) struct ObservedBTreeLeafRead {
    pub(super) leaf: BaselineBTreeLeafRecord,
    pub(super) counters: PhysicalLayoutAccessCounterSnapshot,
}

pub(super) fn read_leaf(
    facade: &mut PhysicalStoreRuntime,
    cell: SlotGenerationCell,
) -> Result<ObservedBTreeLeafRead, BaselineBTreeExecutionDenial> {
    let reference = PhysicalReferenceAuthority::for_canonical_physical_format()
        .admit_page_slot(cell)
        .reference();
    let mut page_access = facade.page_access();
    let report = page_access.read_record(reference)?;
    let counters = PageAccess::access_counters(report);
    let leaf = decode_leaf_record(report.record_view().payload().as_bytes())
        .ok_or(BaselineBTreeExecutionDenial::InvalidLeafNode)?;
    Ok(ObservedBTreeLeafRead { leaf, counters })
}
