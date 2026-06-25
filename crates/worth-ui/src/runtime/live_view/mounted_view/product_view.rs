use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    resolve_composition_participation, WorthUiCompositionParticipationReceipt,
    WorthUiCompositionRootMountReceipt, WorthUiGraphBackedLiveViewProjectionReceipt,
    WorthUiMountedCompositionTreeReceipt, WorthUiMountedGraphChildSelectionCounters,
    WorthUiMountedViewReceipt, WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId,
    WorthUiRuntimeHost,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedProductViewReceipt {
    semantic_slice: WorthUiMountedProductViewSemanticSlice,
    mounted_view: WorthUiMountedViewReceipt,
    root_entries: Vec<WorthUiMountedProductRootEntryReceipt>,
    composition_tree: WorthUiMountedCompositionTreeReceipt,
    composition_participation: WorthUiCompositionParticipationReceipt,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    graph_obligation_execution_digests: Vec<u64>,
    composition_graph_digest: u64,
    counters: WorthUiMountedProductViewCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiMountedProductRootEntryReceipt {
    root_mount: WorthUiCompositionRootMountReceipt,
    composition_tree_digest: u64,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMountedProductViewSemanticSlice {
    LiveView,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiMountedProductViewCounters {
    selected_graph_obligation_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
    root_entry_count: usize,
    mounted_node_count: usize,
    composition_node_count: usize,
    composition_edge_count: usize,
    composition_policy_attachment_count: usize,
    graph_child_selection: WorthUiMountedGraphChildSelectionCounters,
}

impl WorthUiMountedProductViewReceipt {
    pub(in crate::runtime::live_view) fn from_live_view_projection(
        runtime: &WorthUiRuntimeHost,
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        mounted_view: WorthUiMountedViewReceipt,
        root_mounts: Vec<WorthUiCompositionRootMountReceipt>,
    ) -> Self {
        let composition_tree =
            WorthUiMountedCompositionTreeReceipt::from_mounted_view(&mounted_view);
        let composition_participation = resolve_composition_participation(
            runtime.graph_authority(),
            &composition_tree,
            projection.accessibility_associations(),
        )
        .expect("mounted product composition participation must admit for graph-backed projection");
        let root_entries = root_entries_from_mounts(root_mounts, composition_tree.receipt_digest());
        let root_mount_obligation_count = root_entries
            .iter()
            .map(|entry| {
                entry
                    .root_mount()
                    .query_graph_execution()
                    .selected_obligation_count()
            })
            .sum::<usize>();
        let mut graph_obligation_execution_digests = live_view_graph_execution_digests(projection);
        graph_obligation_execution_digests.push(
            mounted_view
                .composition_graph()
                .query_graph_execution()
                .execution_digest(),
        );
        graph_obligation_execution_digests.push(
            mounted_view
                .context_propagation()
                .query_graph_execution()
                .execution_digest(),
        );
        graph_obligation_execution_digests.push(
            composition_participation
                .query_graph_execution()
                .execution_digest(),
        );
        let counters = WorthUiMountedProductViewCounters::from_projection(
            projection,
            mounted_view.nodes().len(),
            mounted_view.composition_graph().counters().node_count(),
            mounted_view.composition_graph().counters().edge_count(),
            mounted_view
                .composition_graph()
                .counters()
                .policy_attachment_count(),
            mounted_view
                .composition_graph()
                .counters()
                .selected_graph_obligation_count(),
            mounted_view.child_reconciliation().counters(),
            mounted_view
                .context_propagation()
                .query_graph_execution()
                .selected_obligation_count(),
            composition_participation
                .query_graph_execution()
                .selected_obligation_count(),
            root_entries.len(),
            root_mount_obligation_count,
        );
        let composition_graph_digest = mounted_view.composition_graph().receipt_digest();
        graph_obligation_execution_digests.extend(root_entries.iter().map(|entry| {
            entry
                .root_mount()
                .query_graph_execution()
                .execution_digest()
        }));
        let mut consumed_facts = mounted_view.consumed_facts().to_vec();
        consumed_facts.extend(
            mounted_view
                .child_reconciliation()
                .consumed_facts()
                .iter()
                .cloned(),
        );
        consumed_facts.extend(composition_participation.consumed_facts().iter().cloned());
        consumed_facts.extend(
            root_entries
                .iter()
                .flat_map(|entry| entry.root_mount().consumed_facts().iter().cloned()),
        );
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            std::iter::once("mounted_product_view:live_view".to_owned())
                .chain(std::iter::once(mounted_view.receipt_digest().to_string()))
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned()))
                .chain(
                    graph_obligation_execution_digests
                        .iter()
                        .map(u64::to_string),
                )
                .chain(std::iter::once(composition_graph_digest.to_string()))
                .chain(std::iter::once(
                    composition_tree.receipt_digest().to_string(),
                ))
                .chain(std::iter::once(
                    composition_participation.receipt_digest().to_string(),
                ))
                .chain(
                    root_entries
                        .iter()
                        .map(|entry| entry.receipt_digest().to_string()),
                )
                .chain(std::iter::once(
                    counters.selected_graph_obligation_count().to_string(),
                )),
        );
        Self {
            semantic_slice: WorthUiMountedProductViewSemanticSlice::LiveView,
            mounted_view,
            root_entries,
            composition_tree,
            composition_participation,
            consumed_facts,
            graph_obligation_execution_digests,
            composition_graph_digest,
            counters,
            receipt_digest,
        }
    }

    pub fn semantic_slice(&self) -> WorthUiMountedProductViewSemanticSlice {
        self.semantic_slice
    }

    pub fn composition_tree(&self) -> &WorthUiMountedCompositionTreeReceipt {
        &self.composition_tree
    }

    pub fn composition_participation(&self) -> &WorthUiCompositionParticipationReceipt {
        &self.composition_participation
    }

    pub fn child_reconciliation(
        &self,
    ) -> &crate::runtime::WorthUiLiveViewCompositionSubjectReconciliationReceipt {
        self.mounted_view.child_reconciliation()
    }

    pub fn root_entries(&self) -> &[WorthUiMountedProductRootEntryReceipt] {
        &self.root_entries
    }

    pub fn live_view_id(&self) -> &str {
        self.mounted_view.live_view_id()
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn graph_obligation_execution_digests(&self) -> &[u64] {
        &self.graph_obligation_execution_digests
    }

    pub fn composition_graph_digest(&self) -> u64 {
        self.composition_graph_digest
    }

    pub fn counters(&self) -> WorthUiMountedProductViewCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedProductRootEntryReceipt {
    fn new(root_mount: WorthUiCompositionRootMountReceipt, composition_tree_digest: u64) -> Self {
        let receipt_digest = digest_parts([
            "mounted_product_root_entry",
            root_mount.receipt_digest().to_string().as_str(),
            composition_tree_digest.to_string().as_str(),
        ]);
        Self {
            root_mount,
            composition_tree_digest,
            receipt_digest,
        }
    }

    pub fn root_mount(&self) -> &WorthUiCompositionRootMountReceipt {
        &self.root_mount
    }

    pub fn composition_tree_digest(&self) -> u64 {
        self.composition_tree_digest
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMountedProductViewCounters {
    fn from_projection(
        projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
        mounted_node_count: usize,
        composition_node_count: usize,
        composition_edge_count: usize,
        composition_policy_attachment_count: usize,
        composition_obligation_count: usize,
        graph_child_selection: WorthUiMountedGraphChildSelectionCounters,
        composition_context_obligation_count: usize,
        composition_participation_obligation_count: usize,
        root_entry_count: usize,
        root_mount_obligation_count: usize,
    ) -> Self {
        let selected_graph_obligation_count = projection
            .controls()
            .iter()
            .map(|receipt| receipt.query_graph_execution().selected_obligation_count())
            .chain(
                projection
                    .conditionals()
                    .iter()
                    .map(|receipt| receipt.query_graph_execution().selected_obligation_count()),
            )
            .chain(
                projection
                    .readinesses()
                    .iter()
                    .map(|receipt| receipt.query_graph_execution().selected_obligation_count()),
            )
            .chain(
                projection
                    .payloads()
                    .iter()
                    .map(|receipt| receipt.query_graph_execution().selected_obligation_count()),
            )
            .chain(
                projection
                    .interactions()
                    .iter()
                    .map(|receipt| receipt.query_graph_execution().selected_obligation_count()),
            )
            .sum::<usize>()
            + composition_obligation_count
            + composition_context_obligation_count
            + composition_participation_obligation_count
            + root_mount_obligation_count;
        let admission = projection.counters();
        Self {
            selected_graph_obligation_count,
            source_reparse_count: admission.source_reparse_count(),
            renderer_parse_count: admission.renderer_parse_count(),
            root_entry_count,
            mounted_node_count,
            composition_node_count,
            composition_edge_count,
            composition_policy_attachment_count,
            graph_child_selection,
        }
    }

    pub fn selected_graph_obligation_count(self) -> usize {
        self.selected_graph_obligation_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }

    pub fn mounted_node_count(self) -> usize {
        self.mounted_node_count
    }

    pub fn root_entry_count(self) -> usize {
        self.root_entry_count
    }

    pub fn composition_node_count(self) -> usize {
        self.composition_node_count
    }

    pub fn composition_edge_count(self) -> usize {
        self.composition_edge_count
    }

    pub fn composition_policy_attachment_count(self) -> usize {
        self.composition_policy_attachment_count
    }

    pub fn graph_child_selection(self) -> WorthUiMountedGraphChildSelectionCounters {
        self.graph_child_selection
    }
}

fn live_view_graph_execution_digests(
    projection: &WorthUiGraphBackedLiveViewProjectionReceipt,
) -> Vec<u64> {
    projection
        .controls()
        .iter()
        .map(|receipt| receipt.query_graph_execution())
        .chain(
            projection
                .conditionals()
                .iter()
                .map(|receipt| receipt.query_graph_execution()),
        )
        .chain(
            projection
                .readinesses()
                .iter()
                .map(|receipt| receipt.query_graph_execution()),
        )
        .chain(
            projection
                .payloads()
                .iter()
                .map(|receipt| receipt.query_graph_execution()),
        )
        .chain(
            projection
                .interactions()
                .iter()
                .map(|receipt| receipt.query_graph_execution()),
        )
        .map(WorthUiQueryGraphExecutionReceipt::execution_digest)
        .collect()
}

fn root_entries_from_mounts(
    root_mounts: Vec<WorthUiCompositionRootMountReceipt>,
    composition_tree_digest: u64,
) -> Vec<WorthUiMountedProductRootEntryReceipt> {
    root_mounts
        .into_iter()
        .map(|mount| WorthUiMountedProductRootEntryReceipt::new(mount, composition_tree_digest))
        .collect()
}
