use super::{BTreeSeparatorPartitionDenial, BaselineBTreeExecutionDenialKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BTreeLookupExecutionCaseId {
    Found,
    Absent,
    Denied(BaselineBTreeExecutionDenialKind),
}

impl BTreeLookupExecutionCaseId {
    pub const fn name(self) -> &'static str {
        match self {
            Self::Found => "layout.btree_lookup.execution.found",
            Self::Absent => "layout.btree_lookup.execution.absent",
            Self::Denied(BaselineBTreeExecutionDenialKind::Physical) => {
                "layout.btree_lookup.execution.denied.physical"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::InvalidRootNode) => {
                "layout.btree_lookup.execution.denied.invalid_root_node"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::InvalidLeafNode) => {
                "layout.btree_lookup.execution.denied.invalid_leaf_node"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::InvalidPhysicalReferenceForBTree) => {
                "layout.btree_lookup.execution.denied.invalid_physical_reference"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::WrongSelectedOperation) => {
                "layout.btree_lookup.execution.denied.wrong_selected_operation"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::StableReadPlan) => {
                "layout.btree_lookup.execution.denied.stable_read_plan"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::Recovery) => {
                "layout.btree_lookup.execution.denied.recovery"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::CounterEnvelope) => {
                "layout.btree_lookup.execution.denied.counter_envelope"
            }
            Self::Denied(BaselineBTreeExecutionDenialKind::SeparatorPartition(
                BTreeSeparatorPartitionDenial::LeafSlotsNotCanonical,
            )) => "layout.btree_lookup.execution.denied.leaf_slots_not_canonical",
            Self::Denied(BaselineBTreeExecutionDenialKind::SeparatorPartition(
                BTreeSeparatorPartitionDenial::LeftChildCrossesSeparator,
            )) => "layout.btree_lookup.execution.denied.left_child_crosses_separator",
            Self::Denied(BaselineBTreeExecutionDenialKind::SeparatorPartition(
                BTreeSeparatorPartitionDenial::RightChildPrecedesSeparator,
            )) => "layout.btree_lookup.execution.denied.right_child_precedes_separator",
        }
    }
}

pub fn btree_lookup_execution_cases() -> impl Iterator<Item = BTreeLookupExecutionCaseId> {
    use BTreeSeparatorPartitionDenial as Partition;
    use BaselineBTreeExecutionDenialKind as Denial;

    [
        BTreeLookupExecutionCaseId::Found,
        BTreeLookupExecutionCaseId::Absent,
        BTreeLookupExecutionCaseId::Denied(Denial::Physical),
        BTreeLookupExecutionCaseId::Denied(Denial::InvalidLeafNode),
        BTreeLookupExecutionCaseId::Denied(Denial::SeparatorPartition(
            Partition::LeafSlotsNotCanonical,
        )),
        BTreeLookupExecutionCaseId::Denied(Denial::SeparatorPartition(
            Partition::LeftChildCrossesSeparator,
        )),
        BTreeLookupExecutionCaseId::Denied(Denial::SeparatorPartition(
            Partition::RightChildPrecedesSeparator,
        )),
    ]
    .into_iter()
}
