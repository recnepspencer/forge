use super::*;

impl<'a> ForgeQueryPreviewSession<'a> {
    pub(super) fn effect_binding_count(&self) -> usize {
        self.handle_bindings
            .iter()
            .filter(|binding| binding.family == ForgeQueryPreviewHandleBindingFamily::Effect)
            .count()
    }

    pub(super) fn effect_delivery_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::EffectDelivery)
    }

    pub(super) fn pending_write_intent_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::PendingWriteIntent)
    }

    pub(super) fn subscription_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::LivePatch)
    }

    pub(super) fn derived_runtime_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::ComputedPatch)
    }

    pub(super) fn authoritative_residue_count(&self) -> usize {
        0
    }

    pub(super) fn execution_kind_count(&self, kind: ForgeQueryPreviewExecutionKind) -> usize {
        self.execution_evidence
            .iter()
            .filter(|evidence| evidence.kind == kind)
            .count()
    }

    pub(super) fn closeout_evidence(
        &self,
        kind: ForgeQueryPreviewCloseoutKind,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
    ) -> ForgeQueryPreviewCloseoutEvidence {
        ForgeQueryPreviewCloseoutEvidence::new(
            &self.label,
            kind,
            self.effect_policy,
            &self.basis_admission,
            self.handle_bindings.len(),
            self.handle_binding_count(ForgeQueryPreviewHandleBindingFamily::LiveView),
            self.handle_binding_count(ForgeQueryPreviewHandleBindingFamily::ComputedView),
            self.effect_binding_count(),
            self.subscription_residue_count(),
            self.derived_runtime_residue_count(),
            staged_preview_write_count,
            promoted_write_count,
            self.effect_delivery_residue_count(),
            self.pending_write_intent_residue_count(),
            self.authoritative_residue_count(),
        )
    }

    pub(super) fn handle_binding_count(
        &self,
        family: ForgeQueryPreviewHandleBindingFamily,
    ) -> usize {
        self.handle_bindings
            .iter()
            .filter(|binding| binding.family == family)
            .count()
    }
}
