use crate::source::WorthUiArtifactHandle;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiReplacementScope {
    kind: WorthUiReplacementScopeKind,
    impacted_handles: Vec<WorthUiArtifactHandle>,
    full_artifact_handle_count: usize,
    impact_lookup_count: usize,
    durable_state_receipts_complete: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum WorthUiReplacementScopeKind {
    LocalSubtree,
    Structural,
    Broad,
}

impl WorthUiReplacementScope {
    pub(crate) fn local_subtree(
        impacted_handles: Vec<WorthUiArtifactHandle>,
        full_artifact_handle_count: usize,
        impact_lookup_count: usize,
    ) -> Self {
        Self::new(
            WorthUiReplacementScopeKind::LocalSubtree,
            impacted_handles,
            full_artifact_handle_count,
            impact_lookup_count,
            true,
        )
    }

    pub(crate) fn structural(
        impacted_handles: Vec<WorthUiArtifactHandle>,
        full_artifact_handle_count: usize,
        impact_lookup_count: usize,
    ) -> Self {
        Self::new(
            WorthUiReplacementScopeKind::Structural,
            impacted_handles,
            full_artifact_handle_count,
            impact_lookup_count,
            true,
        )
    }

    pub(crate) fn broad_without_durable_state_receipts(full_artifact_handle_count: usize) -> Self {
        Self::new(
            WorthUiReplacementScopeKind::Broad,
            Vec::new(),
            full_artifact_handle_count,
            0,
            false,
        )
    }

    fn new(
        kind: WorthUiReplacementScopeKind,
        mut impacted_handles: Vec<WorthUiArtifactHandle>,
        full_artifact_handle_count: usize,
        impact_lookup_count: usize,
        durable_state_receipts_complete: bool,
    ) -> Self {
        impacted_handles.sort();
        impacted_handles.dedup();
        Self {
            kind,
            impacted_handles,
            full_artifact_handle_count,
            impact_lookup_count,
            durable_state_receipts_complete,
        }
    }

    pub fn impacted_handle_count(&self) -> usize {
        self.impacted_handles.len()
    }

    pub(crate) fn impacted_handles(&self) -> &[WorthUiArtifactHandle] {
        &self.impacted_handles
    }

    pub fn full_artifact_handle_count(&self) -> usize {
        self.full_artifact_handle_count
    }

    pub fn impact_lookup_count(&self) -> usize {
        self.impact_lookup_count
    }

    pub fn durable_state_receipts_complete(&self) -> bool {
        self.durable_state_receipts_complete
    }

    pub fn is_local_subtree(&self) -> bool {
        self.kind == WorthUiReplacementScopeKind::LocalSubtree
    }

    pub fn is_structural(&self) -> bool {
        self.kind == WorthUiReplacementScopeKind::Structural
    }

    pub fn is_broad(&self) -> bool {
        self.kind == WorthUiReplacementScopeKind::Broad
    }
}
