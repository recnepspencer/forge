use super::super::BaselineBTreeLeafRecord;
use super::{
    BTreeSeparatorPartitionDenial, BaselineBTreeExecutionDenial, BaselineBTreeLookupBranch,
};
use worth_store_physical_format::PhysicalRecordSlot;

pub(in crate::strategy::btree::execution) fn verify_selected_leaf_partition(
    separator: PhysicalRecordSlot,
    branch: BaselineBTreeLookupBranch,
    leaf: BaselineBTreeLeafRecord,
) -> Result<(), BaselineBTreeExecutionDenial> {
    let [first, second] = leaf.slots();
    if first.get() >= second.get() {
        return Err(BaselineBTreeExecutionDenial::SeparatorPartition(
            BTreeSeparatorPartitionDenial::LeafSlotsNotCanonical,
        ));
    }
    match branch {
        BaselineBTreeLookupBranch::Left if second.get() >= separator.get() => {
            Err(BaselineBTreeExecutionDenial::SeparatorPartition(
                BTreeSeparatorPartitionDenial::LeftChildCrossesSeparator,
            ))
        }
        BaselineBTreeLookupBranch::Right if first.get() < separator.get() => {
            Err(BaselineBTreeExecutionDenial::SeparatorPartition(
                BTreeSeparatorPartitionDenial::RightChildPrecedesSeparator,
            ))
        }
        _ => Ok(()),
    }
}
