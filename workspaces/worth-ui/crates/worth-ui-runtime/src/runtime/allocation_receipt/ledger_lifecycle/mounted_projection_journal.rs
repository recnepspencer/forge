use std::collections::VecDeque;

const MOUNTED_PROJECTION_DELTA_HISTORY_LIMIT: usize = 32;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct UiMountedAllocationProjectionJournal {
    entries: VecDeque<UiMountedAllocationProjectionJournalEntry>,
}

#[derive(Clone, Debug, PartialEq)]
struct UiMountedAllocationProjectionJournalEntry {
    predecessor_revision: u64,
    successor_revision: u64,
    changed_graph_nodes: Box<[crate::graph::UiGraphNodeIdentity]>,
}

#[derive(Clone, Debug)]
pub(crate) struct UiMountedAllocationProjectionSource {
    catalog: super::UiMountedAllocationProjectionCatalog,
    changed_graph_nodes: Box<[crate::graph::UiGraphNodeIdentity]>,
    journal_entries_touched: usize,
    reconstructive: bool,
}

impl UiMountedAllocationProjectionJournal {
    pub(super) fn record(
        &mut self,
        predecessor_revision: u64,
        successor_revision: u64,
        mut changed_graph_nodes: Vec<crate::graph::UiGraphNodeIdentity>,
    ) {
        changed_graph_nodes.sort();
        changed_graph_nodes.dedup();
        self.entries
            .push_back(UiMountedAllocationProjectionJournalEntry {
                predecessor_revision,
                successor_revision,
                changed_graph_nodes: changed_graph_nodes.into_boxed_slice(),
            });
        while self.entries.len() > MOUNTED_PROJECTION_DELTA_HISTORY_LIMIT {
            self.entries.pop_front();
        }
    }

    pub(super) fn source(
        &self,
        catalog: super::UiMountedAllocationProjectionCatalog,
        predecessor_revision: Option<u64>,
        current_revision: u64,
    ) -> UiMountedAllocationProjectionSource {
        let Some(predecessor_revision) = predecessor_revision else {
            return UiMountedAllocationProjectionSource::reconstructive(catalog);
        };
        if predecessor_revision == current_revision {
            return UiMountedAllocationProjectionSource::exact(catalog, Vec::new(), 0);
        }
        let mut cursor = predecessor_revision;
        let mut changed = Vec::new();
        let mut touched = 0usize;
        for entry in &self.entries {
            if entry.predecessor_revision != cursor {
                continue;
            }
            touched += 1;
            changed.extend_from_slice(&entry.changed_graph_nodes);
            cursor = entry.successor_revision;
            if cursor == current_revision {
                changed.sort();
                changed.dedup();
                return UiMountedAllocationProjectionSource::exact(catalog, changed, touched);
            }
        }
        UiMountedAllocationProjectionSource::reconstructive(catalog)
    }
}

impl Default for UiMountedAllocationProjectionJournal {
    fn default() -> Self {
        Self {
            entries: VecDeque::with_capacity(MOUNTED_PROJECTION_DELTA_HISTORY_LIMIT),
        }
    }
}

impl UiMountedAllocationProjectionSource {
    pub(crate) fn from_catalog(catalog: super::UiMountedAllocationProjectionCatalog) -> Self {
        Self::exact(catalog, Vec::new(), 0)
    }

    pub(crate) fn receipt(
        &self,
        graph_node: crate::graph::UiGraphNodeIdentity,
    ) -> Option<&super::UiAllocationReceipt> {
        self.catalog.receipt(graph_node)
    }

    pub(crate) fn changed_graph_nodes(&self) -> &[crate::graph::UiGraphNodeIdentity] {
        &self.changed_graph_nodes
    }

    pub(crate) fn journal_entries_touched(&self) -> usize {
        self.journal_entries_touched
    }

    pub(crate) fn is_reconstructive(&self) -> bool {
        self.reconstructive
    }

    fn exact(
        catalog: super::UiMountedAllocationProjectionCatalog,
        changed_graph_nodes: Vec<crate::graph::UiGraphNodeIdentity>,
        journal_entries_touched: usize,
    ) -> Self {
        Self {
            catalog,
            changed_graph_nodes: changed_graph_nodes.into_boxed_slice(),
            journal_entries_touched,
            reconstructive: false,
        }
    }

    fn reconstructive(catalog: super::UiMountedAllocationProjectionCatalog) -> Self {
        Self {
            catalog,
            changed_graph_nodes: Box::new([]),
            journal_entries_touched: 0,
            reconstructive: true,
        }
    }
}

impl Default for UiMountedAllocationProjectionSource {
    fn default() -> Self {
        Self::from_catalog(Default::default())
    }
}
