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
pub(crate) struct MutationPublicationEffect {
    pub(crate) changed_records: Vec<RecordRef>,
    pub(crate) patch_records: Vec<PatchRecord>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MutationDiagnosticsEffect {
    pub(crate) entries: Vec<RelationalDiagnosticsEntry>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MutationAdjacencyEffect {
    pub(crate) deltas: Vec<AdjacencyDelta>,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MutationEffect {
    pub(crate) publication: MutationPublicationEffect,
    pub(crate) diagnostics: MutationDiagnosticsEffect,
    pub(crate) adjacency: MutationAdjacencyEffect,
}

impl MutationEffect {
    pub(crate) fn accumulate(&mut self, child: MutationEffect) {
        self.publication
            .changed_records
            .extend(child.publication.changed_records);
        self.publication
            .patch_records
            .extend(child.publication.patch_records);
        self.diagnostics.entries.extend(child.diagnostics.entries);
        self.adjacency.deltas.extend(child.adjacency.deltas);
    }
}
