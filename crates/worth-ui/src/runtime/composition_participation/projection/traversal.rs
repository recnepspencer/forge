use super::digest::digest_parts;
use crate::runtime::{WorthUiMountedCompositionChildReceipt, WorthUiMountedCompositionTreeReceipt};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiCompositionParticipationTraversalReceipt {
    rows: Vec<WorthUiCompositionParticipationTraversalRow>,
    counters: WorthUiCompositionParticipationTraversalCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiCompositionParticipationTraversalRow {
    parent_id: String,
    node_id: String,
    authority_identity: String,
    graph_order: u32,
    mounted_child: WorthUiMountedCompositionChildReceipt,
    graph_child_row_digest: u64,
    row_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiCompositionParticipationTraversalCounters {
    graph_child_row_count: usize,
    mounted_child_lookup_count: usize,
    missing_mounted_child_count: usize,
    caller_owned_recursive_walk_count: usize,
    caller_owned_scan_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

impl WorthUiCompositionParticipationTraversalReceipt {
    pub(crate) fn from_tree(tree: &WorthUiMountedCompositionTreeReceipt) -> Self {
        let rows = tree
            .graph_access()
            .child_rows()
            .iter()
            .filter_map(|graph_row| {
                tree.child_for_node_id(graph_row.node().node_id().as_str())
                    .map(|mounted_child| {
                        WorthUiCompositionParticipationTraversalRow::new(
                            graph_row.parent_id(),
                            graph_row.order(),
                            graph_row.node().authority_identity(),
                            mounted_child.clone(),
                            graph_row.row_digest(),
                        )
                    })
            })
            .collect::<Vec<_>>();
        let graph_counters = tree.graph_access().counters();
        let counters = WorthUiCompositionParticipationTraversalCounters {
            graph_child_row_count: graph_counters.request_child_row_count(),
            mounted_child_lookup_count: graph_counters.request_child_row_count(),
            missing_mounted_child_count: tree.graph_access().child_rows().len() - rows.len(),
            caller_owned_recursive_walk_count: graph_counters.caller_owned_recursive_walk_count(),
            caller_owned_scan_count: graph_counters.caller_owned_scan_count(),
            source_reparse_count: graph_counters.source_reparse_count(),
            renderer_parse_count: graph_counters.renderer_parse_count(),
        };
        let receipt_digest = digest_parts(
            ["composition_participation_traversal".to_owned()]
                .into_iter()
                .chain(rows.iter().map(|row| row.row_digest().to_string()))
                .chain(std::iter::once(
                    counters.graph_child_row_count().to_string(),
                )),
        );
        Self {
            rows,
            counters,
            receipt_digest,
        }
    }

    pub fn rows(&self) -> &[WorthUiCompositionParticipationTraversalRow] {
        &self.rows
    }

    pub fn counters(&self) -> WorthUiCompositionParticipationTraversalCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionParticipationTraversalRow {
    fn new(
        parent_id: impl Into<String>,
        graph_order: u32,
        authority_identity: impl Into<String>,
        mounted_child: WorthUiMountedCompositionChildReceipt,
        graph_child_row_digest: u64,
    ) -> Self {
        let parent_id = parent_id.into();
        let node_id = mounted_child.node_id().to_owned();
        let authority_identity = authority_identity.into();
        let row_digest = digest_parts([
            "composition_participation_traversal_row",
            parent_id.as_str(),
            node_id.as_str(),
            authority_identity.as_str(),
            graph_order.to_string().as_str(),
            mounted_child.receipt_digest().to_string().as_str(),
            graph_child_row_digest.to_string().as_str(),
        ]);
        Self {
            parent_id,
            node_id,
            authority_identity,
            graph_order,
            mounted_child,
            graph_child_row_digest,
            row_digest,
        }
    }

    pub fn parent_id(&self) -> &str {
        &self.parent_id
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn authority_identity(&self) -> &str {
        &self.authority_identity
    }

    pub fn graph_order(&self) -> u32 {
        self.graph_order
    }

    pub fn mounted_child(&self) -> &WorthUiMountedCompositionChildReceipt {
        &self.mounted_child
    }

    pub fn graph_child_row_digest(&self) -> u64 {
        self.graph_child_row_digest
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiCompositionParticipationTraversalCounters {
    pub fn graph_child_row_count(self) -> usize {
        self.graph_child_row_count
    }

    pub fn mounted_child_lookup_count(self) -> usize {
        self.mounted_child_lookup_count
    }

    pub fn missing_mounted_child_count(self) -> usize {
        self.missing_mounted_child_count
    }

    pub fn caller_owned_recursive_walk_count(self) -> usize {
        self.caller_owned_recursive_walk_count
    }

    pub fn caller_owned_scan_count(self) -> usize {
        self.caller_owned_scan_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}
