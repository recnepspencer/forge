use crate::BranchDeltaLayerId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadRequest {
    layer_id: BranchDeltaLayerId,
    branch_lineage: String,
}

impl BranchDeltaReadRequest {
    pub fn new(layer_id: BranchDeltaLayerId, branch_lineage: impl Into<String>) -> Self {
        Self {
            layer_id,
            branch_lineage: branch_lineage.into(),
        }
    }

    pub const fn layer_id(&self) -> BranchDeltaLayerId {
        self.layer_id
    }

    pub fn branch_lineage(&self) -> &str {
        &self.branch_lineage
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadPlan {
    request: BranchDeltaReadRequest,
    declared_delta_rows: u32,
}

impl BranchDeltaReadPlan {
    pub const fn new(request: BranchDeltaReadRequest, declared_delta_rows: u32) -> Self {
        Self {
            request,
            declared_delta_rows,
        }
    }

    pub const fn request(&self) -> &BranchDeltaReadRequest {
        &self.request
    }

    pub const fn declared_delta_rows(&self) -> u32 {
        self.declared_delta_rows
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BranchDeltaReadResult {
    layer_id: BranchDeltaLayerId,
    returned_delta_rows: u32,
}

impl BranchDeltaReadResult {
    pub const fn new(layer_id: BranchDeltaLayerId, returned_delta_rows: u32) -> Self {
        Self {
            layer_id,
            returned_delta_rows,
        }
    }
}
