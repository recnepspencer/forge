use std::collections::BTreeMap;

use crate::lineage::data::{CorrespondenceCandidate, LineageEventRecord, LineageNode};
use crate::logic::runtime::state::subsystems::RuntimeSubsystem;

#[derive(Debug, Clone, Default)]
pub(crate) struct LineageSubsystem {
    pub(crate) nodes: BTreeMap<crate::identity::data::LineageId, LineageNode>,
    pub(crate) events: Vec<LineageEventRecord>,
    pub(crate) correspondence_candidates: Vec<CorrespondenceCandidate>,
    pub(crate) next_lineage_id: u64,
    pub(crate) next_event_id: u64,
}

impl LineageSubsystem {
    fn empty() -> Self {
        Self {
            nodes: BTreeMap::new(),
            events: Vec::new(),
            correspondence_candidates: Vec::new(),
            next_lineage_id: 1,
            next_event_id: 1,
        }
    }
}

impl RuntimeSubsystem for LineageSubsystem {
    type Config = ();

    fn new(_: &Self::Config) -> Self {
        Self::empty()
    }

    fn fork(&self) -> Self {
        self.clone()
    }
}
