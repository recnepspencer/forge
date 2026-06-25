use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiCompositionGraphChildAccessRow, WorthUiLiveViewCompositionChildSubjectKind,
    WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewCompositionSubjectReconciliationReceipt {
    rows: Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
    counters: WorthUiMountedGraphChildSelectionCounters,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewCompositionSubjectReconciliationRow {
    subject_kind: WorthUiLiveViewCompositionChildSubjectKind,
    subject_id: String,
    composition_node_id: Option<String>,
    parent_id: Option<String>,
    posture: WorthUiLiveViewCompositionSubjectReconciliationPosture,
    row_digest: u64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiMountedGraphChildSelectionCounters {
    pub(in crate::runtime::live_view::mounted_view) graph_child_row_count: usize,
    pub(in crate::runtime::live_view::mounted_view) control_payload_lookup_count: usize,
    pub(in crate::runtime::live_view::mounted_view) interaction_payload_lookup_count: usize,
    pub(in crate::runtime::live_view::mounted_view) projection_control_scan_count: usize,
    pub(in crate::runtime::live_view::mounted_view) projection_interaction_scan_count: usize,
    pub(in crate::runtime::live_view::mounted_view) mounted_subject_count: usize,
    pub(in crate::runtime::live_view::mounted_view) declared_unmounted_count: usize,
    pub(in crate::runtime::live_view::mounted_view) missing_payload_count: usize,
    pub(in crate::runtime::live_view::mounted_view) duplicate_subject_count: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewCompositionSubjectReconciliationPosture {
    Mounted,
    DeclaredButUnmounted,
    GraphChildMissingProjectionPayload,
    DuplicateGraphSubject,
}

impl WorthUiLiveViewCompositionSubjectReconciliationReceipt {
    pub(in crate::runtime::live_view::mounted_view) fn new(
        mut rows: Vec<WorthUiLiveViewCompositionSubjectReconciliationRow>,
        counters: WorthUiMountedGraphChildSelectionCounters,
        child_rows: &[WorthUiCompositionGraphChildAccessRow],
    ) -> Self {
        rows.sort_by(|left, right| {
            (
                left.subject_kind.token(),
                left.subject_id.as_str(),
                left.parent_id.as_deref().unwrap_or_default(),
                left.composition_node_id.as_deref().unwrap_or_default(),
            )
                .cmp(&(
                    right.subject_kind.token(),
                    right.subject_id.as_str(),
                    right.parent_id.as_deref().unwrap_or_default(),
                    right.composition_node_id.as_deref().unwrap_or_default(),
                ))
        });
        let mut consumed_facts = child_rows
            .iter()
            .flat_map(|row| [row.node().fact_id().clone(), row.edge().fact_id().clone()])
            .collect::<Vec<_>>();
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            ["live_view_composition_subject_reconciliation".to_owned()]
                .into_iter()
                .chain(rows.iter().map(|row| row.row_digest().to_string()))
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned()))
                .chain([
                    counters.graph_child_row_count().to_string(),
                    counters.mounted_subject_count().to_string(),
                    counters.declared_unmounted_count().to_string(),
                    counters.missing_payload_count().to_string(),
                    counters.duplicate_subject_count().to_string(),
                ]),
        );
        Self {
            rows,
            counters,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn rows(&self) -> &[WorthUiLiveViewCompositionSubjectReconciliationRow] {
        &self.rows
    }

    pub fn counters(&self) -> WorthUiMountedGraphChildSelectionCounters {
        self.counters
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiLiveViewCompositionSubjectReconciliationRow {
    pub(in crate::runtime::live_view::mounted_view) fn new(
        subject_kind: WorthUiLiveViewCompositionChildSubjectKind,
        subject_id: &str,
        composition_node_id: Option<&str>,
        parent_id: Option<&str>,
        posture: WorthUiLiveViewCompositionSubjectReconciliationPosture,
    ) -> Self {
        let row_digest = digest_parts([
            subject_kind.token(),
            subject_id,
            composition_node_id.unwrap_or_default(),
            parent_id.unwrap_or_default(),
            posture.token(),
        ]);
        Self {
            subject_kind,
            subject_id: subject_id.to_owned(),
            composition_node_id: composition_node_id.map(str::to_owned),
            parent_id: parent_id.map(str::to_owned),
            posture,
            row_digest,
        }
    }

    pub fn subject_kind(&self) -> WorthUiLiveViewCompositionChildSubjectKind {
        self.subject_kind
    }

    pub fn subject_id(&self) -> &str {
        &self.subject_id
    }

    pub fn composition_node_id(&self) -> Option<&str> {
        self.composition_node_id.as_deref()
    }

    pub fn parent_id(&self) -> Option<&str> {
        self.parent_id.as_deref()
    }

    pub fn posture(&self) -> WorthUiLiveViewCompositionSubjectReconciliationPosture {
        self.posture
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiMountedGraphChildSelectionCounters {
    pub fn graph_child_row_count(self) -> usize {
        self.graph_child_row_count
    }

    pub fn control_payload_lookup_count(self) -> usize {
        self.control_payload_lookup_count
    }

    pub fn interaction_payload_lookup_count(self) -> usize {
        self.interaction_payload_lookup_count
    }

    pub fn projection_control_scan_count(self) -> usize {
        self.projection_control_scan_count
    }

    pub fn projection_interaction_scan_count(self) -> usize {
        self.projection_interaction_scan_count
    }

    pub fn mounted_subject_count(self) -> usize {
        self.mounted_subject_count
    }

    pub fn declared_unmounted_count(self) -> usize {
        self.declared_unmounted_count
    }

    pub fn missing_payload_count(self) -> usize {
        self.missing_payload_count
    }

    pub fn duplicate_subject_count(self) -> usize {
        self.duplicate_subject_count
    }
}

impl WorthUiLiveViewCompositionSubjectReconciliationPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Mounted => "mounted",
            Self::DeclaredButUnmounted => "declared_but_unmounted",
            Self::GraphChildMissingProjectionPayload => "graph_child_missing_projection_payload",
            Self::DuplicateGraphSubject => "duplicate_graph_subject",
        }
    }
}
