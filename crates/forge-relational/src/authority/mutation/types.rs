use crate::config::data::MutationConfig;
use crate::diagnostics::data::RelationalDiagnosticsEntry;
use crate::identity::data::{EntityId, RelationId, VersionId};
use crate::publication::data::diff::PatchRecord;
use crate::schema::data::RelationalSchemaRegistry;
use crate::storage::overlay::RelationalDraft;
use crate::symbols::data::StringInterner;
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

    pub(crate) fn record_change(&mut self, record: RecordRef) {
        self.changed_records.push(record);
    }

    pub(crate) fn record_patch(&mut self, patch: PatchRecord) {
        self.patch_records.push(patch);
    }

    pub(crate) fn record_diagnostic(&mut self, diagnostic: RelationalDiagnosticsEntry) {
        self.diagnostics.push(diagnostic);
    }

    pub(crate) fn record_adjacency_delta(&mut self, delta: AdjacencyDelta) {
        self.adjacency_deltas.push(delta);
    }
}

pub(crate) struct MutationWorkspace<'a> {
    pub(crate) draft: &'a mut RelationalDraft,
    pub(crate) symbols: &'a mut StringInterner,
    pub(crate) config: &'a MutationConfig,
    pub(crate) schema: &'a RelationalSchemaRegistry,
    pub(crate) version_id: VersionId,
}

impl<'a> MutationWorkspace<'a> {
    pub(crate) fn as_parts_mut(
        &mut self,
    ) -> (
        &mut RelationalDraft,
        &mut StringInterner,
        &MutationConfig,
        &RelationalSchemaRegistry,
        VersionId,
    ) {
        (
            self.draft,
            self.symbols,
            self.config,
            self.schema,
            self.version_id,
        )
    }
}
