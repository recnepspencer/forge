use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::identity::data::{EntityId, RelationId};
use crate::publication::data::diff::PatchRecord;
use crate::transactions::data::RecordRef;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum AdjacencyDeltaKind {
    Created { source: EntityId, target: EntityId },
    Deleted { source: EntityId, target: EntityId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct AdjacencyDelta {
    pub(crate) relation_id: RelationId,
    pub(crate) kind: AdjacencyDeltaKind,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MutationEffect {
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) patch_records: Vec<PatchRecord>,
    pub(crate) diagnostics: Vec<RelationalDiagnosticsEntry>,
    pub(crate) adjacency_deltas: Vec<AdjacencyDelta>,
}

impl MutationEffect {
    pub(crate) fn accumulate(&mut self, child: MutationEffect) {
        self.changed_records.extend(child.changed_records);
        self.patch_records.extend(child.patch_records);
        self.diagnostics.extend(child.diagnostics);
        self.adjacency_deltas.extend(child.adjacency_deltas);
    }
}
