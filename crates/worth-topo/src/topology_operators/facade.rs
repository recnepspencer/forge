use forge_relational::facade::history::BranchId;
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TopologyEditApplicationMode {
    Mainline,
    BranchLocal(BranchId),
}
