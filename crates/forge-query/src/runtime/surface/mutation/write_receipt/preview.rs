use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::memory_workspace::{
    ForgeQueryCommitIdentity, ForgeQueryEntityIdentity, ForgeQuerySnapshotIdentity,
};
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryExistingTruthBindingEvidence, ForgeQueryJournalPosition,
    ForgeQueryMutationFamily, ForgeQueryWriteReceipt,
};
use crate::session_label::ForgeQuerySessionLabel;

use super::{
    helpers::{
        continuity_mutation_evidence, naming_mutation_evidence, symbolic_target_reference_evidence,
        target_evidence_from_receipt,
    },
    ForgeQueryWriteCommand,
};

impl ForgeQueryWriteReceipt {
    pub(in crate::runtime) fn preview(
        label: &ForgeQuerySessionLabel,
        sequence: usize,
        command: &ForgeQueryWriteCommand,
        snapshot_identity: ForgeQuerySnapshotIdentity,
    ) -> Self {
        let preview_identity = preview_write_receipt_identity(label, sequence);
        let commit_identity = ForgeQueryCommitIdentity::preview(preview_identity.clone());
        let commit_evidence_identity =
            super::write_receipt_commit_evidence_identity(&commit_identity);
        let journal_position =
            ForgeQueryJournalPosition::preview(preview_identity.clone(), sequence as u64);
        let snapshot_evidence_identity =
            super::write_receipt_snapshot_evidence_identity(&snapshot_identity);
        let delta = preview_receipt_delta(command, &preview_identity);
        let inner = crate::memory_workspace::ForgeQueryMutationReceipt {
            commit_identity,
            snapshot_identity,
            deltas: vec![delta.clone()],
            bridge_authority: None,
        };
        let committed_truth_identity = super::write_receipt_committed_truth_identity(&inner);
        let target_entity_identity = preview_target_entity_identity(command, &delta);
        let target_collection = command.declared_collection();
        let naming_mutation_evidence = naming_mutation_evidence(
            None,
            command.naming_intent(),
            target_entity_identity
                .as_ref()
                .or(Some(&delta.entity_identity)),
            target_collection.as_deref(),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            None,
            command.continuity_intent(),
            command.existing_truth_binding(),
            target_entity_identity
                .as_ref()
                .or(Some(&delta.entity_identity)),
            target_collection.as_deref(),
        );
        Self {
            inner,
            commit_evidence_identity,
            committed_truth_identity,
            journal_position,
            snapshot_evidence_identity,
            mutation_family: command.mutation_family(),
            authority_lane: ForgeQueryAuthorityLane::PreviewTruth,
            basis_lane: ForgeQueryAuthorityLane::PreviewTruth,
            target_evidence: target_evidence_from_receipt(
                command.mutation_family(),
                command.declared_collection(),
                command.declared_entity_identity(),
                command.declared_collection(),
                command.declared_entity_identity(),
            ),
            existing_truth_assertion_evidence: None,
            existing_truth_binding_evidence: command
                .existing_truth_binding()
                .map(ForgeQueryExistingTruthBindingEvidence::from_binding),
            symbolic_target_reference_evidence: symbolic_target_reference_evidence(
                command.mutation_family(),
                None,
                command.symbolic_target_reference(),
                Some(&delta.entity_identity),
            ),
            symbolic_aspect_resolution_evidence: Vec::new(),
            naming_mutation_evidence,
            continuity_mutation_evidence,
            causality_evidence: None,
            provenance_evidence: None,
            declared_collection: command.declared_collection(),
            declared_entity_identity: command.declared_entity_identity(),
            target_collection,
            target_entity_identity,
            declared_aspect_operations: command.declared_aspect_operations(),
            declared_aspect_value_digest: crate::runtime::command_declared_aspect_value_identity(
                command,
            ),
            mutation_metadata: command.mutation_metadata(),
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
            decision_trace_envelope: None,
            execution_provenance: None,
            obligation_dispatch: None,
        }
    }
}

pub(super) fn preview_receipt_delta(
    command: &ForgeQueryWriteCommand,
    preview_identity: &ForgeQueryEvidenceIdentity,
) -> crate::memory_workspace::ForgeQueryMutationDelta {
    match command {
        ForgeQueryWriteCommand::InsertAspects { collection, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: ForgeQueryEntityIdentity::preview(preview_identity.clone()),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Created,
                aspect_paths: command.declared_aspect_paths(),
            }
        }
        ForgeQueryWriteCommand::UpdateAspect {
            entity_identity,
            aspect_path,
            value: _,
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: "preview".to_string(),
            entity_identity: entity_identity.clone(),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
            aspect_paths: vec![aspect_path.clone()],
        },
        ForgeQueryWriteCommand::UpdateAspects {
            entity_identity, ..
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: "preview".to_string(),
            entity_identity: entity_identity.clone(),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
            aspect_paths: command.declared_aspect_paths(),
        },
        ForgeQueryWriteCommand::UpdateExistingAspects { binding, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: binding
                    .target_collection()
                    .map(str::to_string)
                    .unwrap_or_else(|| "preview".to_string()),
                entity_identity: binding.resolved_entity_artifact_identity(),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            }
        }
        ForgeQueryWriteCommand::VerifyThenUpdateExistingAspects { binding, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: binding
                    .target_collection()
                    .map(str::to_string)
                    .unwrap_or_else(|| "preview".to_string()),
                entity_identity: binding.resolved_entity_artifact_identity(),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            }
        }
        ForgeQueryWriteCommand::VerifyThenDeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: binding
                .target_collection()
                .map(str::to_string)
                .unwrap_or_else(|| "preview".to_string()),
            entity_identity: binding.resolved_entity_artifact_identity(),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Deleted,
            aspect_paths: touched_aspect_paths.clone(),
        },
        ForgeQueryWriteCommand::AssertExistingAspects { binding, .. }
        | ForgeQueryWriteCommand::VerifyExistingAspects { binding, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: binding
                    .target_collection()
                    .map(str::to_string)
                    .unwrap_or_else(|| "preview".to_string()),
                entity_identity: binding.resolved_entity_artifact_identity(),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            }
        }
        ForgeQueryWriteCommand::UpdateSymbolicAspects { reference, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: reference
                    .target_collection()
                    .map(str::to_string)
                    .unwrap_or_else(|| "preview".to_string()),
                entity_identity: preview_symbolic_entity_identity(reference),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Updated,
                aspect_paths: command.declared_aspect_paths(),
            }
        }
        ForgeQueryWriteCommand::DeleteAspects {
            entity_identity,
            declared_collection,
            touched_aspect_paths,
            ..
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: declared_collection
                .clone()
                .unwrap_or_else(|| "preview".to_string()),
            entity_identity: entity_identity.clone(),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Deleted,
            aspect_paths: touched_aspect_paths.clone(),
        },
        ForgeQueryWriteCommand::DeleteExistingAspects {
            binding,
            touched_aspect_paths,
            ..
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: binding
                .target_collection()
                .map(str::to_string)
                .unwrap_or_else(|| "preview".to_string()),
            entity_identity: binding.resolved_entity_artifact_identity(),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Deleted,
            aspect_paths: touched_aspect_paths.clone(),
        },
        ForgeQueryWriteCommand::DeleteSymbolicAspects {
            reference,
            touched_aspect_paths,
            ..
        } => crate::memory_workspace::ForgeQueryMutationDelta {
            collection: reference
                .target_collection()
                .map(str::to_string)
                .unwrap_or_else(|| "preview".to_string()),
            entity_identity: preview_symbolic_entity_identity(reference),
            kind: crate::memory_workspace::ForgeQueryMutationKind::Deleted,
            aspect_paths: touched_aspect_paths.clone(),
        },
        ForgeQueryWriteCommand::Delete { entity_identity } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: "preview".to_string(),
                entity_identity: entity_identity.clone(),
                kind: crate::memory_workspace::ForgeQueryMutationKind::Deleted,
                aspect_paths: Vec::new(),
            }
        }
    }
}

fn preview_write_receipt_identity(
    label: &ForgeQuerySessionLabel,
    sequence: usize,
) -> ForgeQueryEvidenceIdentity {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewWriteReceiptIdentity)
        .field_value(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("sequence"), sequence)
        .seal()
}

fn preview_target_entity_identity(
    command: &ForgeQueryWriteCommand,
    delta: &crate::memory_workspace::ForgeQueryMutationDelta,
) -> Option<ForgeQueryEntityIdentity> {
    match command.mutation_family() {
        ForgeQueryMutationFamily::Insert => Some(delta.entity_identity.clone()),
        _ => command.declared_entity_identity(),
    }
}

fn preview_symbolic_entity_identity(
    reference: &crate::runtime::ForgeQuerySymbolicTargetReference,
) -> ForgeQueryEntityIdentity {
    ForgeQueryEntityIdentity::preview(
        forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewWriteReceiptIdentity)
            .field_value(ForgeQueryEvidenceTag::new("symbol"), reference.symbol())
            .optional_shape(
                ForgeQueryEvidenceTag::new("target_collection"),
                reference.target_collection(),
            )
            .seal(),
    )
}
