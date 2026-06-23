use super::*;
use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};

impl<'a> ForgeQueryPreviewSession<'a> {
    pub fn compare_to_authoritative(&self) -> ForgeQueryPreviewDiff {
        ForgeQueryPreviewDiff {
            session_label: self.label.clone(),
            write_count: self.writes.len(),
            changed_entity_count: self
                .writes
                .iter()
                .flat_map(|receipt| receipt.deltas())
                .filter(|delta| {
                    matches!(
                        delta.kind,
                        ForgeQueryMutationKind::Created
                            | ForgeQueryMutationKind::Updated
                            | ForgeQueryMutationKind::Deleted
                    )
                })
                .count(),
        }
    }

    pub fn promote(mut self) -> Result<ForgeQueryPreviewOutcome, ForgeQueryRuntimeError> {
        let staged_preview_write_count = self.pending_commands.len();
        let promotion_snapshot_identity = self.runtime.current_snapshot_identity();
        let residue_snapshot = self.residue_snapshot();
        if promotion_snapshot_identity != self.basis_snapshot_identity {
            return Err(ForgeQueryRuntimeError::PreviewPromotionStaleBasis(
                ForgeQueryPreviewPromotionDenialEvidence::stale_basis(
                    self.effect_policy,
                    &self.basis_admission,
                    &self.basis_snapshot_identity,
                    &promotion_snapshot_identity,
                    staged_preview_write_count,
                    self.handle_bindings.len(),
                ),
            ));
        }
        if staged_preview_write_count > 1 {
            return Err(
                ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(
                    ForgeQueryPreviewPromotionDenialEvidence::atomic_batch_unsupported(
                        self.effect_policy,
                        &self.basis_admission,
                        &self.basis_snapshot_identity,
                        &promotion_snapshot_identity,
                        staged_preview_write_count,
                        self.handle_bindings.len(),
                    ),
                ),
            );
        }
        let crossed_authoritative_residue_count =
            residue_snapshot.crossed_authoritative_residue_count;
        let promotion_rebinding_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::PreviewPromotionRebinding)
                .field_value(
                    ForgeQueryEvidenceTag::new("session_label_identity"),
                    self.basis_admission.label_identity().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_snapshot_identity"),
                    &self.basis_snapshot_identity.evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("promotion_snapshot_identity"),
                    &promotion_snapshot_identity.evidence_identity(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("crossed_authoritative_residue_count"),
                    crossed_authoritative_residue_count,
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("preview_binding_count"),
                    self.handle_bindings.len(),
                )
                .seal();
        if crossed_authoritative_residue_count > 0 {
            return Err(ForgeQueryRuntimeError::PreviewPromotionRebindingRequired(
                ForgeQueryPreviewPromotionDenialEvidence::rebinding_required(
                    self.effect_policy,
                    &self.basis_admission,
                    &self.basis_snapshot_identity,
                    &promotion_snapshot_identity,
                    staged_preview_write_count,
                    self.handle_bindings.len(),
                    crossed_authoritative_residue_count,
                    promotion_rebinding_digest,
                ),
            ));
        }
        let mut promoted_writes = 0;
        for (index, command) in std::mem::take(&mut self.pending_commands)
            .into_iter()
            .enumerate()
        {
            match self.runtime.write(command) {
                Ok(receipt) => {
                    self.writes.push(receipt);
                    promoted_writes += 1;
                }
                Err(error) => {
                    let graph_obligation_denial = match &error {
                        ForgeQueryRuntimeError::GraphObligationDenied(denial) => {
                            Some(denial.projection().clone())
                        }
                        _ => None,
                    };
                    let reason = error.to_string();
                    let evidence = if let Some(denial) = graph_obligation_denial {
                        ForgeQueryPreviewPromotionDenialEvidence::write_failed_with_graph_obligation_denial(
                            self.effect_policy,
                            &self.basis_admission,
                            &self.basis_snapshot_identity,
                            &promotion_snapshot_identity,
                            staged_preview_write_count,
                            promoted_writes,
                            index + 1,
                            self.handle_bindings.len(),
                            denial,
                            reason,
                        )
                    } else {
                        ForgeQueryPreviewPromotionDenialEvidence::write_failed(
                            self.effect_policy,
                            &self.basis_admission,
                            &self.basis_snapshot_identity,
                            &promotion_snapshot_identity,
                            staged_preview_write_count,
                            promoted_writes,
                            index + 1,
                            self.handle_bindings.len(),
                            reason,
                        )
                    };
                    return Err(ForgeQueryRuntimeError::PreviewPromotionWriteFailed { evidence });
                }
            }
        }
        self.promoted = true;
        let preview_binding_count = self.handle_bindings.len();
        let effect_binding_count = self.effect_binding_count();
        let effect_delivery_residue_count = self.effect_delivery_residue_count();
        let pending_write_intent_residue_count = self.pending_write_intent_residue_count();
        let authoritative_residue_count = self.authoritative_residue_count();
        let closeout_evidence = self.closeout_evidence(
            ForgeQueryPreviewCloseoutKind::Promoted,
            staged_preview_write_count,
            promoted_writes,
            residue_snapshot,
            &promotion_snapshot_identity,
            Some(promotion_rebinding_digest),
        );
        Ok(ForgeQueryPreviewOutcome {
            session_label: self.label.clone(),
            effect_policy: self.effect_policy,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: promoted_writes,
            preview_binding_count,
            effect_binding_count,
            effect_delivery_residue_count,
            pending_write_intent_residue_count,
            authoritative_residue_count,
            closeout_evidence,
            source_lane: ForgeQueryAuthorityLane::PreviewTruth,
            target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        })
    }

    pub fn discard(mut self) -> ForgeQueryPreviewOutcome {
        self.discarded = true;
        let staged_preview_write_count = self.pending_commands.len();
        let residue_snapshot = self.residue_snapshot();
        let preview_binding_count = self.handle_bindings.len();
        let effect_binding_count = self.effect_binding_count();
        let effect_delivery_residue_count = self.effect_delivery_residue_count();
        let pending_write_intent_residue_count = self.pending_write_intent_residue_count();
        let authoritative_residue_count = self.authoritative_residue_count();
        let closeout_evidence = self.closeout_evidence(
            ForgeQueryPreviewCloseoutKind::Discarded,
            staged_preview_write_count,
            0,
            residue_snapshot,
            &self.basis_snapshot_identity,
            None,
        );
        ForgeQueryPreviewOutcome {
            session_label: self.label.clone(),
            effect_policy: self.effect_policy,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: self.writes.len(),
            preview_binding_count,
            effect_binding_count,
            effect_delivery_residue_count,
            pending_write_intent_residue_count,
            authoritative_residue_count,
            closeout_evidence,
            source_lane: ForgeQueryAuthorityLane::PreviewTruth,
            target_lane: ForgeQueryAuthorityLane::PreviewTruth,
        }
    }

    pub fn execute_intent(
        &mut self,
        declaration: ForgeQueryIntentDeclaration,
    ) -> Result<ForgeQueryPreviewIntentReceipt, ForgeQueryRuntimeError> {
        self.runtime.admit_facade_family_lane(
            ForgeQueryRuntimeFacadeFamily::Intent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )?;
        let declaration = declaration
            .with_source_lane(ForgeQueryIntentSourceLane::PreviewLocal)
            .with_target_lane(ForgeQueryAuthorityLane::PreviewTruth);
        let admission = crate::runtime::intent::admit_preview_intent_declaration(
            &declaration,
            self.effect_policy,
        )
        .map_err(|denial| ForgeQueryRuntimeError::IntentCommitDenied {
            intent_name: declaration.name().to_string(),
            stage: denial.stage(),
            message: denial.message().to_string(),
            evidence: ForgeQueryIntentDenialEvidence::new(&declaration, &denial, None),
        })?;
        let obligation_dispatch = self
            .runtime
            .preview_intent_obligation_dispatch(&declaration)?;
        let receipt = ForgeQueryPreviewIntentReceipt::new(
            &declaration,
            self.effect_policy,
            &self.basis_admission,
            admission,
            obligation_dispatch,
        );
        self.intent_receipts.push(receipt.clone());
        self.execution_evidence
            .push(ForgeQueryPreviewExecutionEvidence::for_intent_strategy(
                &self.basis_admission,
                declaration.name(),
                receipt.receipt_identity(),
                declaration.strategy_name(),
            ));
        Ok(receipt)
    }

    pub(super) fn admit_operation_effect(
        &self,
        effect: &ForgeQueryProgramEffect,
    ) -> Result<(), ForgeQueryRuntimeError> {
        match effect {
            ForgeQueryProgramEffect::DeclareLiveView { name, .. } => {
                Err(ForgeQueryRuntimeError::PreviewOperationEffectDenied {
                    label: self.label.clone(),
                    stage: "effect-admission",
                    message: format!(
                        "preview operations cannot install live view `{name}` into authoritative runtime state; declare the live surface before entering preview or add preview-scoped declaration support"
                    ),
                })
            }
            ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                Err(ForgeQueryRuntimeError::PreviewOperationEffectDenied {
                    label: self.label.clone(),
                    stage: "effect-admission",
                    message: format!(
                        "preview operations cannot install computed view `{}` into authoritative runtime state; declare the computed surface before entering preview or add preview-scoped declaration support",
                        view.name()
                    ),
                })
            }
            ForgeQueryProgramEffect::Write(_)
            | ForgeQueryProgramEffect::WriteTemplate(_)
            | ForgeQueryProgramEffect::ReadLive { .. }
            | ForgeQueryProgramEffect::DrainPatches { .. } => Ok(()),
        }
    }
}
