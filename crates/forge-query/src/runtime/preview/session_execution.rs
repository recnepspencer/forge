use super::*;

impl<'a> ForgeQueryPreviewSession<'a> {
    pub(super) fn admit_preview_write_intent(&self) -> Result<(), ForgeQueryRuntimeError> {
        self.admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )
        .map(|_| ())
    }

    pub(super) fn stage_command(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> ForgeQueryWriteReceipt {
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.snapshot_token(),
        );
        self.pending_commands.push(command);
        self.route_preview_execution(&receipt);
        receipt
    }

    pub fn preview_execution_evidence(&self) -> &[ForgeQueryPreviewExecutionEvidence] {
        &self.execution_evidence
    }

    pub fn preview_intent_receipts(&self) -> &[ForgeQueryPreviewIntentReceipt] {
        &self.intent_receipts
    }

    pub(super) fn route_preview_execution(&mut self, receipt: &ForgeQueryWriteReceipt) {
        let mut live_affected: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut computed_affected: BTreeMap<String, Vec<String>> = BTreeMap::new();

        for binding in self
            .handle_bindings
            .iter()
            .filter(|binding| binding.family == ForgeQueryPreviewHandleBindingFamily::LiveView)
        {
            let Some(state) = self.runtime.live_subscriptions.get(binding.handle_name()) else {
                continue;
            };
            let affected_aspects = relevant_live_aspects(&state.request, receipt.deltas());
            if affected_aspects.is_empty() {
                continue;
            }
            live_affected.insert(binding.handle_name().to_string(), affected_aspects.clone());
            self.execution_evidence
                .push(ForgeQueryPreviewExecutionEvidence::new(
                    &self.basis_admission,
                    ForgeQueryPreviewExecutionKind::LivePatch,
                    binding.handle_name(),
                    binding.source_lane(),
                    ForgeQueryAuthorityLane::PreviewTruth,
                    receipt.commit_identity(),
                    affected_aspects,
                ));
        }

        for binding in self
            .handle_bindings
            .iter()
            .filter(|binding| binding.family == ForgeQueryPreviewHandleBindingFamily::ComputedView)
        {
            let Some(runtime) = self.runtime.derived_views.get(binding.handle_name()) else {
                continue;
            };
            let affected_aspects =
                relevant_computed_aspects(runtime, &live_affected, &computed_affected);
            if affected_aspects.is_empty() {
                continue;
            }
            computed_affected.insert(binding.handle_name().to_string(), affected_aspects.clone());
            self.execution_evidence
                .push(ForgeQueryPreviewExecutionEvidence::new(
                    &self.basis_admission,
                    ForgeQueryPreviewExecutionKind::ComputedPatch,
                    binding.handle_name(),
                    binding.source_lane(),
                    ForgeQueryAuthorityLane::PreviewTruth,
                    receipt.commit_identity(),
                    affected_aspects,
                ));
        }

        let mut pending_effect_evidence = Vec::new();
        for binding in self
            .handle_bindings
            .iter()
            .filter(|binding| binding.family == ForgeQueryPreviewHandleBindingFamily::Effect)
        {
            let Some(disposition) = binding.effect_disposition() else {
                continue;
            };
            let Ok(inspected) = self.runtime.inspect_effect_by_name(binding.handle_name()) else {
                continue;
            };
            let affected_aspects =
                relevant_effect_aspects(&inspected, &live_affected, &computed_affected);
            if affected_aspects.is_empty() {
                continue;
            }
            let kind = match disposition {
                ForgeQueryPreviewEffectBindingDisposition::RedirectedDelivery
                | ForgeQueryPreviewEffectBindingDisposition::AuthoritativeAllowed => {
                    ForgeQueryPreviewExecutionKind::EffectDelivery
                }
                ForgeQueryPreviewEffectBindingDisposition::SandboxedWriteIntent => {
                    ForgeQueryPreviewExecutionKind::PendingWriteIntent
                }
                ForgeQueryPreviewEffectBindingDisposition::Muted
                | ForgeQueryPreviewEffectBindingDisposition::MutedByDeriveOnly => {
                    ForgeQueryPreviewExecutionKind::MutedEffect
                }
            };
            pending_effect_evidence.push(ForgeQueryPreviewExecutionEvidence::new(
                &self.basis_admission,
                kind,
                binding.handle_name(),
                binding.source_lane(),
                ForgeQueryAuthorityLane::PreviewTruth,
                receipt.commit_identity(),
                affected_aspects,
            ));
        }
        self.execution_evidence.extend(pending_effect_evidence);
    }
}
