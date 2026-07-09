use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ImmutableReadContract {
    pub immutable_snapshots_required: bool,
    pub hidden_mutation_forbidden: bool,
    pub lazy_writeback_forbidden: bool,
}

impl Default for ImmutableReadContract {
    fn default() -> Self {
        Self {
            immutable_snapshots_required: true,
            hidden_mutation_forbidden: true,
            lazy_writeback_forbidden: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SerializedAuthorityContract {
    pub single_writer_required: bool,
    pub deterministic_commit_order_required: bool,
    pub deterministic_visibility_boundary_required: bool,
}

impl Default for SerializedAuthorityContract {
    fn default() -> Self {
        Self {
            single_writer_required: true,
            deterministic_commit_order_required: true,
            deterministic_visibility_boundary_required: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct RelationalBoundaryContract {
    pub reads: ImmutableReadContract,
    pub authority: SerializedAuthorityContract,
}
