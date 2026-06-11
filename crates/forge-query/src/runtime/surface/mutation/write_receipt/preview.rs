use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::{
    ForgeQueryAuthorityLane, ForgeQueryExistingTruthBindingEvidence, ForgeQueryMutationFamily,
    ForgeQueryWriteReceipt,
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
        snapshot_token: String,
    ) -> Self {
        let preview_identity = preview_write_receipt_identity(label, sequence);
        let delta = preview_receipt_delta(command, &preview_identity);
        let target_entity_identity = preview_target_entity_identity(command, &delta);
        let target_collection = command.declared_collection();
        let naming_mutation_evidence = naming_mutation_evidence(
            None,
            command.naming_intent(),
            target_entity_identity
                .as_deref()
                .or(Some(delta.entity_identity.as_str())),
            target_collection.as_deref(),
        );
        let continuity_mutation_evidence = continuity_mutation_evidence(
            None,
            command.continuity_intent(),
            command.existing_truth_binding(),
            target_entity_identity
                .as_deref()
                .or(Some(delta.entity_identity.as_str())),
            target_collection.as_deref(),
        );
        Self {
            inner: crate::memory_workspace::ForgeQueryMutationReceipt {
                commit_identity: preview_identity,
                snapshot_token,
                deltas: vec![delta.clone()],
                bridge_authority: None,
            },
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
                Some(delta.entity_identity.as_str()),
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
            declared_aspect_value_digest: crate::runtime::command_declared_aspect_value_digest(
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
        }
    }
}

pub(super) fn preview_receipt_delta(
    command: &ForgeQueryWriteCommand,
    preview_identity: &str,
) -> crate::memory_workspace::ForgeQueryMutationDelta {
    match command {
        ForgeQueryWriteCommand::InsertAspects { collection, .. } => {
            crate::memory_workspace::ForgeQueryMutationDelta {
                collection: collection.clone(),
                entity_identity: preview_identity.to_string(),
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
                entity_identity: binding.resolved_entity_identity().to_string(),
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
                entity_identity: binding.resolved_entity_identity().to_string(),
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
            entity_identity: binding.resolved_entity_identity().to_string(),
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
                entity_identity: binding.resolved_entity_identity().to_string(),
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
                entity_identity: format!("preview-symbolic:{}", reference.symbol()),
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
            entity_identity: binding.resolved_entity_identity().to_string(),
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
            entity_identity: format!("preview-symbolic:{}", reference.symbol()),
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

fn preview_write_receipt_identity(label: &ForgeQuerySessionLabel, sequence: usize) -> String {
    forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewWriteReceiptIdentity)
        .field_identity(
            ForgeQueryEvidenceTag::new("session_label_identity"),
            label.identity_digest().as_str(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("sequence"), sequence)
        .seal()
        .as_str()
        .to_string()
}

fn preview_target_entity_identity(
    command: &ForgeQueryWriteCommand,
    delta: &crate::memory_workspace::ForgeQueryMutationDelta,
) -> Option<String> {
    match command.mutation_family() {
        ForgeQueryMutationFamily::Insert => Some(delta.entity_identity.clone()),
        _ => command.declared_entity_identity(),
    }
}
