use crate::identity::hash_parts;
use serde_json::Value;

use super::super::{
    ForgeQueryAspectMutationOperation, ForgeQueryAspectValue, ForgeQueryAuthorityLane,
    ForgeQueryRuntimeError,
};
use crate::memory_workspace::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub enum ForgeQueryWriteCommand {
    #[deprecated(
        note = "payload-first insert is a compatibility path; prefer workspace.insert(...) or preview.insert(...) with aspect-native authoring"
    )]
    Insert {
        collection: String,
        payload: Value,
    },
    InsertAspects {
        collection: String,
        aspects: Vec<ForgeQueryAspectValue>,
    },
    UpdateAspect {
        entity_identity: String,
        aspect_path: String,
        value: Value,
    },
    UpdateAspects {
        entity_identity: String,
        aspects: Vec<ForgeQueryAspectValue>,
    },
    Delete {
        entity_identity: String,
    },
}

impl ForgeQueryWriteCommand {
    #[allow(deprecated)]
    pub(crate) fn declared_aspect_paths(&self) -> Vec<String> {
        super::super::mutation::command_declared_aspect_paths(self)
    }

    pub(crate) fn declared_aspect_operations(&self) -> Vec<ForgeQueryAspectMutationOperation> {
        super::super::mutation::command_declared_aspect_operations(self)
    }

    #[allow(deprecated)]
    pub(crate) fn mutation_family(&self) -> ForgeQueryMutationFamily {
        match self {
            Self::Insert { .. } | Self::InsertAspects { .. } => ForgeQueryMutationFamily::Insert,
            Self::UpdateAspect { .. } | Self::UpdateAspects { .. } => {
                ForgeQueryMutationFamily::Update
            }
            Self::Delete { .. } => ForgeQueryMutationFamily::Delete,
        }
    }

    #[allow(deprecated)]
    pub(crate) fn declared_collection(&self) -> Option<String> {
        match self {
            Self::Insert { collection, .. } | Self::InsertAspects { collection, .. } => {
                Some(collection.clone())
            }
            Self::UpdateAspect { .. } | Self::UpdateAspects { .. } | Self::Delete { .. } => None,
        }
    }

    #[allow(deprecated)]
    pub(crate) fn declared_entity_identity(&self) -> Option<String> {
        match self {
            Self::UpdateAspect {
                entity_identity, ..
            }
            | Self::UpdateAspects {
                entity_identity, ..
            }
            | Self::Delete { entity_identity } => Some(entity_identity.clone()),
            Self::Insert { .. } | Self::InsertAspects { .. } => None,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryMutationFamily {
    Insert,
    Update,
    Delete,
}

impl ForgeQueryMutationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Insert => "insert",
            Self::Update => "update",
            Self::Delete => "delete",
        }
    }
}

impl std::fmt::Display for ForgeQueryMutationFamily {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWriteReceipt {
    pub(super) inner: ForgeQueryMutationReceipt,
    pub(super) mutation_family: ForgeQueryMutationFamily,
    pub(super) authority_lane: ForgeQueryAuthorityLane,
    pub(super) basis_lane: ForgeQueryAuthorityLane,
    pub(super) declared_collection: Option<String>,
    pub(super) declared_entity_identity: Option<String>,
    pub(super) declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
    pub(super) affected_live_view_ids: Vec<String>,
    pub(super) affected_derived_view_ids: Vec<String>,
    pub(super) considered_computed_view_count: usize,
    pub(super) considered_effect_count: usize,
    pub(super) delivered_effect_count: usize,
    pub(super) pending_write_intent_count: usize,
    pub(super) suppressed_effect_count: usize,
    pub(super) meaningful_effect_suppression_count: usize,
    pub(super) effect_expression_failure_count: usize,
    pub(super) refresh_fallback: bool,
}

impl ForgeQueryWriteReceipt {
    pub(in crate::runtime) fn from_mutation_receipt(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        considered_computed_view_count: usize,
        considered_effect_count: usize,
        delivered_effect_count: usize,
        pending_write_intent_count: usize,
        suppressed_effect_count: usize,
        meaningful_effect_suppression_count: usize,
        effect_expression_failure_count: usize,
        refresh_fallback: bool,
    ) -> Self {
        Self {
            inner,
            mutation_family,
            authority_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            basis_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            declared_collection,
            declared_entity_identity,
            declared_aspect_operations,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        }
    }

    #[allow(deprecated)]
    pub(in crate::runtime) fn preview(
        label: &str,
        sequence: usize,
        command: &ForgeQueryWriteCommand,
        snapshot_token: String,
    ) -> Self {
        let delta = match command {
            ForgeQueryWriteCommand::Insert {
                collection,
                payload: _,
            } => ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: format!("preview:{label}:{sequence}"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: Vec::new(),
            },
            ForgeQueryWriteCommand::InsertAspects { collection, .. } => ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: format!("preview:{label}:{sequence}"),
                kind: ForgeQueryMutationKind::Created,
                aspect_paths: command.declared_aspect_paths(),
            },
            ForgeQueryWriteCommand::UpdateAspect {
                entity_identity,
                aspect_path,
                value: _,
            } => ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: vec![aspect_path.clone()],
            },
            ForgeQueryWriteCommand::UpdateAspects {
                entity_identity, ..
            } => ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            },
            ForgeQueryWriteCommand::Delete { entity_identity } => ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: ForgeQueryMutationKind::Deleted,
                aspect_paths: Vec::new(),
            },
        };
        Self {
            inner: ForgeQueryMutationReceipt {
                commit_identity: format!("preview:{label}:{sequence}"),
                snapshot_token,
                deltas: vec![delta],
            },
            mutation_family: command.mutation_family(),
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            basis_lane: ForgeQueryAuthorityLane::PreviewTruth,
            declared_collection: command.declared_collection(),
            declared_entity_identity: command.declared_entity_identity(),
            declared_aspect_operations: command.declared_aspect_operations(),
            affected_live_view_ids: Vec::new(),
            affected_derived_view_ids: Vec::new(),
            considered_computed_view_count: 0,
            considered_effect_count: 0,
            delivered_effect_count: 0,
            pending_write_intent_count: 0,
            suppressed_effect_count: 0,
            meaningful_effect_suppression_count: 0,
            effect_expression_failure_count: 0,
            refresh_fallback: false,
        }
    }

    pub(in crate::runtime) fn batch_component(
        inner: ForgeQueryMutationReceipt,
        mutation_family: ForgeQueryMutationFamily,
        basis_lane: ForgeQueryAuthorityLane,
        declared_collection: Option<String>,
        declared_entity_identity: Option<String>,
        declared_aspect_operations: Vec<ForgeQueryAspectMutationOperation>,
        affected_live_view_ids: Vec<String>,
        authority_lane: ForgeQueryAuthorityLane,
    ) -> Self {
        Self {
            inner,
            mutation_family,
            authority_lane,
            basis_lane,
            declared_collection,
            declared_entity_identity,
            declared_aspect_operations,
            affected_live_view_ids,
            affected_derived_view_ids: Vec::new(),
            considered_computed_view_count: 0,
            considered_effect_count: 0,
            delivered_effect_count: 0,
            pending_write_intent_count: 0,
            suppressed_effect_count: 0,
            meaningful_effect_suppression_count: 0,
            effect_expression_failure_count: 0,
            refresh_fallback: false,
        }
    }

    pub fn commit_identity(&self) -> &str {
        &self.inner.commit_identity
    }

    pub fn mutation_family(&self) -> ForgeQueryMutationFamily {
        self.mutation_family
    }

    pub fn snapshot_token(&self) -> &str {
        &self.inner.snapshot_token
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn declared_collection(&self) -> Option<&str> {
        self.declared_collection.as_deref()
    }

    pub fn declared_entity_identity(&self) -> Option<&str> {
        self.declared_entity_identity.as_deref()
    }

    pub fn declared_aspect_operations(&self) -> &[ForgeQueryAspectMutationOperation] {
        &self.declared_aspect_operations
    }

    pub fn deltas(&self) -> &[ForgeQueryMutationDelta] {
        &self.inner.deltas
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }

    pub fn into_inner(self) -> ForgeQueryMutationReceipt {
        self.inner
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBatchWriteReceipt {
    write_receipts: Vec<ForgeQueryWriteReceipt>,
    authority_lane: ForgeQueryAuthorityLane,
    basis_lane: ForgeQueryAuthorityLane,
    batch_digest: String,
    touched_aspect_paths: Vec<String>,
    affected_live_view_ids: Vec<String>,
    affected_derived_view_ids: Vec<String>,
    considered_computed_view_count: usize,
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_effect_suppression_count: usize,
    effect_expression_failure_count: usize,
    refresh_fallback: bool,
}

impl ForgeQueryBatchWriteReceipt {
    pub(in crate::runtime) fn new(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
        authority_lane: ForgeQueryAuthorityLane,
        basis_lane: ForgeQueryAuthorityLane,
        touched_aspect_paths: Vec<String>,
        affected_live_view_ids: Vec<String>,
        affected_derived_view_ids: Vec<String>,
        considered_computed_view_count: usize,
        considered_effect_count: usize,
        delivered_effect_count: usize,
        pending_write_intent_count: usize,
        suppressed_effect_count: usize,
        meaningful_effect_suppression_count: usize,
        effect_expression_failure_count: usize,
        refresh_fallback: bool,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }

        let batch_digest = hash_parts(
            &std::iter::once("forge_query_batch_write_receipt_v1".to_string())
                .chain(
                    write_receipts
                        .iter()
                        .map(|receipt| format!("commit:{}", receipt.commit_identity())),
                )
                .chain(
                    touched_aspect_paths
                        .iter()
                        .map(|path| format!("aspect:{path}")),
                )
                .chain(
                    affected_live_view_ids
                        .iter()
                        .map(|view| format!("live:{view}")),
                )
                .chain(
                    affected_derived_view_ids
                        .iter()
                        .map(|view| format!("derived:{view}")),
                )
                .collect::<Vec<_>>(),
        );

        Ok(Self {
            write_receipts,
            authority_lane,
            basis_lane,
            batch_digest,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        })
    }

    pub(in crate::runtime) fn from_write_receipts(
        write_receipts: Vec<ForgeQueryWriteReceipt>,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        if write_receipts.is_empty() {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch must produce at least one write receipt",
                ),
            ));
        }
        let authority_lane = write_receipts[0].authority_lane();
        let basis_lane = write_receipts[0].basis_lane();
        if write_receipts
            .iter()
            .any(|receipt| receipt.authority_lane() != authority_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix authority lanes",
                ),
            ));
        }
        if write_receipts
            .iter()
            .any(|receipt| receipt.basis_lane() != basis_lane)
        {
            return Err(ForgeQueryRuntimeError::Workspace(
                crate::memory_workspace::ForgeQueryWorkspaceError::new(
                    "mutation batch may not mix basis lanes",
                ),
            ));
        }

        let mut touched_aspect_paths = write_receipts
            .iter()
            .flat_map(|receipt| {
                receipt
                    .deltas()
                    .iter()
                    .flat_map(|delta| delta.aspect_paths.iter().cloned())
            })
            .collect::<Vec<_>>();
        touched_aspect_paths.sort();
        touched_aspect_paths.dedup();

        let mut affected_live_view_ids = write_receipts
            .iter()
            .flat_map(|receipt| receipt.affected_live_view_ids().iter().cloned())
            .collect::<Vec<_>>();
        affected_live_view_ids.sort();
        affected_live_view_ids.dedup();

        let mut affected_derived_view_ids = write_receipts
            .iter()
            .flat_map(|receipt| receipt.affected_derived_view_ids().iter().cloned())
            .collect::<Vec<_>>();
        affected_derived_view_ids.sort();
        affected_derived_view_ids.dedup();

        let considered_computed_view_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_computed_view_count)
            .sum();
        let considered_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::considered_effect_count)
            .sum();
        let delivered_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::delivered_effect_count)
            .sum();
        let pending_write_intent_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::pending_write_intent_count)
            .sum();
        let suppressed_effect_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::suppressed_effect_count)
            .sum();
        let meaningful_effect_suppression_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::meaningful_effect_suppression_count)
            .sum();
        let effect_expression_failure_count = write_receipts
            .iter()
            .map(ForgeQueryWriteReceipt::effect_expression_failure_count)
            .sum();
        let refresh_fallback = write_receipts
            .iter()
            .any(ForgeQueryWriteReceipt::refresh_fallback);

        Self::new(
            write_receipts,
            authority_lane,
            basis_lane,
            touched_aspect_paths,
            affected_live_view_ids,
            affected_derived_view_ids,
            considered_computed_view_count,
            considered_effect_count,
            delivered_effect_count,
            pending_write_intent_count,
            suppressed_effect_count,
            meaningful_effect_suppression_count,
            effect_expression_failure_count,
            refresh_fallback,
        )
    }

    pub fn authority_lane(&self) -> ForgeQueryAuthorityLane {
        self.authority_lane
    }

    pub fn basis_lane(&self) -> ForgeQueryAuthorityLane {
        self.basis_lane
    }

    pub fn batch_digest(&self) -> &str {
        &self.batch_digest
    }

    pub fn write_count(&self) -> usize {
        self.write_receipts.len()
    }

    pub fn write_receipts(&self) -> &[ForgeQueryWriteReceipt] {
        &self.write_receipts
    }

    pub fn touched_aspect_paths(&self) -> &[String] {
        &self.touched_aspect_paths
    }

    pub fn affected_live_view_ids(&self) -> &[String] {
        &self.affected_live_view_ids
    }

    pub fn affected_derived_view_ids(&self) -> &[String] {
        &self.affected_derived_view_ids
    }

    pub fn considered_computed_view_count(&self) -> usize {
        self.considered_computed_view_count
    }

    pub fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }

    pub fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }

    pub fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }

    pub fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }

    pub fn meaningful_effect_suppression_count(&self) -> usize {
        self.meaningful_effect_suppression_count
    }

    pub fn effect_expression_failure_count(&self) -> usize {
        self.effect_expression_failure_count
    }

    pub fn refresh_fallback(&self) -> bool {
        self.refresh_fallback
    }
}
