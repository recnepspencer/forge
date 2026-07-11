use crate::BranchDeltaLayerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaRewritePlan {
    layer_id: BranchDeltaLayerId,
    rewritten_delta_rows: u32,
}

impl BranchDeltaRewritePlan {
    pub const fn new(layer_id: BranchDeltaLayerId, rewritten_delta_rows: u32) -> Self {
        Self {
            layer_id,
            rewritten_delta_rows,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaRebuildReceipt {
    layer_id: BranchDeltaLayerId,
    rebuilt_delta_rows: u32,
}

impl BranchDeltaRebuildReceipt {
    pub const fn new(layer_id: BranchDeltaLayerId, rebuilt_delta_rows: u32) -> Self {
        Self {
            layer_id,
            rebuilt_delta_rows,
        }
    }
}
