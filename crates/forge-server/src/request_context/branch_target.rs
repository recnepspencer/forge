#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerBranchTarget {
    Main,
    Branch { branch_id: String },
    Preview { preview_id: String },
}

impl ForgeServerBranchTarget {
    pub fn canonical_label(&self) -> String {
        match self {
            Self::Main => "main".to_string(),
            Self::Branch { branch_id } => format!("branch:{branch_id}"),
            Self::Preview { preview_id } => format!("preview:{preview_id}"),
        }
    }

    pub fn branch_digest(&self) -> String {
        format!("forge-server-branch-target-v1:{}", self.canonical_label())
    }
}
