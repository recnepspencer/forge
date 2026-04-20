mod authority;
mod core;
mod cursor;
mod cursor_resume;
mod delta;
mod durable_runtime;
mod durable_runtime_recovery;
mod layout_materialization;
mod layout_reads;
mod layout_rebuild;
mod layout_support;
mod live_query;
mod maintenance;
mod publication_recovery;
mod snapshots;
mod support;
mod tiering;

use crate::evidence::{
    Milestone6AccessStructureVerification, Milestone7AccessStructureVerification, StoreCounters,
};
use crate::failure::StoreError;
use crate::media::DurableMediaReport;
use std::collections::HashMap;

use super::records::StoreState;

pub(crate) trait StatePersistence: std::fmt::Debug {
    fn load_state(&mut self) -> Result<StoreState, StoreError>;
    fn persist_state(&mut self, state: &StoreState) -> Result<DurableMediaReport, StoreError>;
    fn durable_media_report(&self) -> DurableMediaReport;
}

#[derive(Debug)]
pub(crate) struct StateBackedStoreBackend<P> {
    pub(super) persistence: P,
    pub(super) state: StoreState,
    pub(super) milestone_6_access_structure_verification: Milestone6AccessStructureVerification,
    pub(super) milestone_7_access_structure_verification: Milestone7AccessStructureVerification,
    pub(super) milestone_6_scope_prepare_counts: HashMap<String, u64>,
    pub(super) counters: StoreCounters,
}
