use forge_relational::facade::history::BranchId;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyMutationApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}
