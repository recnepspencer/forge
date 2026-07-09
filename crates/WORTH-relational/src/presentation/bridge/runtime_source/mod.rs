use std::sync::Arc;

use crate::logic::runtime::RelationalRuntime;

mod branch_heads;
mod committed_patches;
mod continuity_lineage;
mod snapshot_authority;
mod snapshot_reads;

#[derive(Debug, Clone)]
pub struct RuntimeBridgeRelationalSource {
    runtime: Arc<RelationalRuntime>,
}

impl RuntimeBridgeRelationalSource {
    pub fn new(runtime: Arc<RelationalRuntime>) -> Self {
        Self { runtime }
    }
}
