use super::*;

impl<'a> ForgeQueryPreviewSession<'a> {
    pub fn compare_to_authoritative(&self) -> ForgeQueryPreviewDiff {
        ForgeQueryPreviewDiff {
            label: self.label.clone(),
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
        let promotion_snapshot_token = self.runtime.snapshot_token();
        let residue_snapshot = self.residue_snapshot();
        if promotion_snapshot_token != self.basis_snapshot_token {
            return Err(ForgeQueryRuntimeError::PreviewPromotionStaleBasis(
                ForgeQueryPreviewPromotionDenialEvidence::stale_basis(
                    &self.label,
                    self.effect_policy,
                    &self.basis_admission,
                    &self.basis_snapshot_token,
                    &promotion_snapshot_token,
                    staged_preview_write_count,
                    self.handle_bindings.len(),
                ),
            ));
        }
        if staged_preview_write_count > 1 {
            return Err(
                ForgeQueryRuntimeError::PreviewPromotionAtomicBatchUnsupported(
                    ForgeQueryPreviewPromotionDenialEvidence::atomic_batch_unsupported(
                        &self.label,
                        self.effect_policy,
                        &self.basis_admission,
                        &self.basis_snapshot_token,
                        &promotion_snapshot_token,
                        staged_preview_write_count,
                        self.handle_bindings.len(),
                    ),
                ),
            );
        }
        let crossed_authoritative_residue_count =
            residue_snapshot.crossed_authoritative_residue_count;
        let promotion_rebinding_digest = hash_parts(&[
            "forge_query_preview_promotion_rebinding_v1".to_string(),
            format!("label:{}", self.label),
            format!("basis_snapshot:{}", self.basis_snapshot_token),
            format!("promotion_snapshot:{}", promotion_snapshot_token),
            format!(
                "crossed_authoritative_residue:{}",
                crossed_authoritative_residue_count
            ),
            format!("preview_bindings:{}", self.handle_bindings.len()),
        ]);
        if crossed_authoritative_residue_count > 0 {
            return Err(ForgeQueryRuntimeError::PreviewPromotionRebindingRequired(
                ForgeQueryPreviewPromotionDenialEvidence::rebinding_required(
                    &self.label,
                    self.effect_policy,
                    &self.basis_admission,
                    &self.basis_snapshot_token,
                    &promotion_snapshot_token,
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
                    return Err(ForgeQueryRuntimeError::PreviewPromotionWriteFailed {
                        evidence: ForgeQueryPreviewPromotionDenialEvidence::write_failed(
                            &self.label,
                            self.effect_policy,
                            &self.basis_admission,
                            &self.basis_snapshot_token,
                            &promotion_snapshot_token,
                            staged_preview_write_count,
                            promoted_writes,
                            index + 1,
                            self.handle_bindings.len(),
                            error.to_string(),
                        ),
                    });
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
            &promotion_snapshot_token,
            Some(promotion_rebinding_digest),
        );
        Ok(ForgeQueryPreviewOutcome {
            label: self.label,
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
            &self.basis_snapshot_token,
            None,
        );
        ForgeQueryPreviewOutcome {
            label: self.label,
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
        let receipt = ForgeQueryPreviewIntentReceipt::new(
            &declaration,
            self.effect_policy,
            &self.basis_admission,
            admission,
        );
        self.intent_receipts.push(receipt.clone());
        self.execution_evidence
            .push(ForgeQueryPreviewExecutionEvidence::new(
                &self.label,
                ForgeQueryPreviewExecutionKind::PendingWriteIntent,
                declaration.name(),
                ForgeQueryAuthorityLane::PendingWriteIntent,
                ForgeQueryAuthorityLane::PreviewTruth,
                receipt.receipt_digest(),
                vec![declaration.strategy_name().to_string()],
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
