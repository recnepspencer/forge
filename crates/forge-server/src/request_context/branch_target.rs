#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}
