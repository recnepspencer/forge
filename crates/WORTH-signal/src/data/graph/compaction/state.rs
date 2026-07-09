use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(in crate::data::graph) struct CompactionState {
    pub(in crate::data::graph) tombstone_count: u32,
    pub(in crate::data::graph) gc_threshold: u32,
    pub(in crate::data::graph) debt: u32,
    pub(in crate::data::graph) cursor: u8,
}

impl Default for CompactionState {
    fn default() -> Self {
        Self::new(1024)
    }
}

impl CompactionState {
    pub(in crate::data::graph) fn new(gc_threshold: u32) -> Self {
        Self {
            tombstone_count: 0,
            gc_threshold,
            debt: 0,
            cursor: 0,
        }
    }
}
