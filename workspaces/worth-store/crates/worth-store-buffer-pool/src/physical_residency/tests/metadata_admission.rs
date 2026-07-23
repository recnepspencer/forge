use super::*;

#[test]
fn impossible_entry_metadata_is_denied_before_hash_table_allocation() {
    let identity = store(14);
    let limits =
        PhysicalResidencyLimits::new_with_metadata_budget(1024, 1, 1, 1, 64, u32::MAX).unwrap();
    assert_eq!(
        PhysicalResidencyPool::open(identity, limits).unwrap_err(),
        PhysicalResidencyDenial::MetadataBudgetExceeded
    );
}
