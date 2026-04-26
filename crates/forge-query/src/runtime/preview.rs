use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{
    admit_authority_requirements, validate_inputs, ForgeQueryAuthorityLane,
    ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewRuntime, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectHandle, ForgeQueryEffectInspectionEvidence,
    ForgeQueryEffectPolicy, ForgeQueryEffectTriggerSourceKind, ForgeQueryInstalledOperation,
    ForgeQueryLiveView, ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryPatchBatch,
    ForgeQueryPreviewBasisAdmission, ForgeQueryProgramEffect, ForgeQueryProgramTrace,
    ForgeQueryRunReceipt, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt,
};
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationKind};

pub struct ForgeQueryPreviewSession<'a> {
    label: String,
    runtime: &'a mut ForgeQueryRuntime,
    effect_policy: ForgeQueryEffectPolicy,
    basis_admission: ForgeQueryPreviewBasisAdmission,
    basis_snapshot_token: String,
    pending_commands: Vec<ForgeQueryWriteCommand>,
    writes: Vec<ForgeQueryWriteReceipt>,
    handle_bindings: Vec<ForgeQueryPreviewHandleBindingEvidence>,
    execution_evidence: Vec<ForgeQueryPreviewExecutionEvidence>,
    promoted: bool,
    discarded: bool,
}

impl<'a> ForgeQueryPreviewSession<'a> {
    pub(super) fn new(
        label: impl Into<String>,
        runtime: &'a mut ForgeQueryRuntime,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: ForgeQueryPreviewBasisAdmission,
    ) -> Self {
        let basis_snapshot_token = runtime.snapshot_token();
        Self {
            label: label.into(),
            runtime,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            pending_commands: Vec::new(),
            writes: Vec::new(),
            handle_bindings: Vec::new(),
            execution_evidence: Vec::new(),
            promoted: false,
            discarded: false,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_admission(&self) -> &ForgeQueryPreviewBasisAdmission {
        &self.basis_admission
    }

    pub fn admit_effect_action(
        &self,
        action: ForgeQueryEffectAction,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Result<ForgeQueryEffectAdmission, ForgeQueryRuntimeError> {
        self.effect_policy
            .admit(action, target_lane)
            .map_err(ForgeQueryRuntimeError::EffectPolicyDenied)
    }

    pub fn write(
        &mut self,
        command: ForgeQueryWriteCommand,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let receipt = ForgeQueryWriteReceipt::preview(
            &self.label,
            self.pending_commands.len() + 1,
            &command,
            self.runtime.snapshot_token(),
        );
        self.pending_commands.push(command);
        self.writes.push(receipt.clone());
        self.route_preview_execution(&receipt);
        Ok(receipt)
    }

    pub fn use_view<T>(
        &mut self,
        view: &ForgeQueryLiveView<T>,
    ) -> ForgeQueryPreviewHandleBindingEvidence {
        let evidence = ForgeQueryPreviewHandleBindingEvidence::live_view(
            &self.label,
            view.name(),
            self.effect_policy,
            self.basis_admission.evidence(),
        );
        self.handle_bindings.push(evidence.clone());
        evidence
    }

    pub fn use_computed<T>(
        &mut self,
        computed: &ForgeQueryDerivedViewHandle<T>,
    ) -> ForgeQueryPreviewHandleBindingEvidence {
        let evidence = ForgeQueryPreviewHandleBindingEvidence::computed(
            &self.label,
            computed.name(),
            self.effect_policy,
            self.basis_admission.evidence(),
        );
        self.handle_bindings.push(evidence.clone());
        evidence
    }

    pub fn use_effect<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<ForgeQueryPreviewHandleBindingEvidence, ForgeQueryRuntimeError> {
        let inspected = self.runtime.inspect_effect(effect)?;
        let disposition = ForgeQueryPreviewEffectBindingDisposition::from_policy(
            self.effect_policy,
            inspected.action(),
            inspected.target_lane(),
        )?;
        let evidence = ForgeQueryPreviewHandleBindingEvidence::effect(
            &self.label,
            effect.name(),
            inspected.target_lane(),
            self.effect_policy,
            disposition,
            self.basis_admission.evidence(),
        );
        self.handle_bindings.push(evidence.clone());
        Ok(evidence)
    }

    pub fn run_operation(
        &mut self,
        operation: ForgeQueryInstalledOperation,
        inputs: Vec<ForgeQueryOperationInput>,
    ) -> Result<ForgeQueryRunReceipt, ForgeQueryRuntimeError> {
        let query_operation = self.runtime.installed_query_operation(&operation)?;
        admit_authority_requirements(query_operation.authority_requirements())?;
        let bound_inputs = validate_inputs(&query_operation, &inputs)?;
        let mut trace = ForgeQueryProgramTrace::new(
            operation.program_id.clone(),
            operation.operation_id.clone(),
            &bound_inputs,
            query_operation
                .authority_requirements()
                .iter()
                .cloned()
                .collect(),
        );
        trace.record_replay_or_parity(format!("preview-session:{}", self.label));
        let mut outputs = Vec::new();
        let mut write_receipts = Vec::new();
        let mut patch_batches = Vec::new();

        for effect in query_operation.effects() {
            match effect.clone() {
                ForgeQueryProgramEffect::DeclareLiveView {
                    name,
                    request,
                    schema_view,
                } => {
                    let _: ForgeQueryLiveView<Value> =
                        self.runtime
                            .declare_live_view(name.clone(), request, schema_view)?;
                    trace.record_declaration(format!("preview-live:{name}"));
                }
                ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                    let name = view.name().to_string();
                    self.runtime.declare_derived_view(view)?;
                    trace.record_declaration(format!("preview-derived:{name}"));
                }
                ForgeQueryProgramEffect::Write(command) => {
                    self.admit_preview_write_intent()?;
                    let receipt = self.stage_command(command);
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::WriteTemplate(template) => {
                    self.admit_preview_write_intent()?;
                    let command = template.bind(&bound_inputs)?;
                    let receipt = self.stage_command(command);
                    trace.record_write_receipt(receipt.commit_identity().to_string());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::ReadLive { view_name } => {
                    let rows = self.runtime.backend.live_entities(&view_name);
                    outputs.push(ForgeQueryOperationOutput::new(
                        format!("preview-live:{view_name}"),
                        Value::Array(rows.into_iter().map(|row| row.payload).collect()),
                    ));
                    trace.record_replay_or_parity(format!("preview-read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    patch_batches.push(ForgeQueryPatchBatch {
                        view_name,
                        live_patches: Vec::new(),
                        query_delivery_batches: Vec::new(),
                        derived_patch_notes: vec![format!(
                            "preview:{}:patch-drain-deferred",
                            self.label
                        )],
                        derived_patches: Vec::new(),
                    });
                }
            }
        }

        let run_id = self.runtime.next_run_identity(&operation);
        self.runtime.run_traces.insert(run_id.clone(), trace);
        self.writes.extend(write_receipts.iter().cloned());
        Ok(ForgeQueryRunReceipt {
            run_id,
            operation,
            outputs,
            write_receipts,
            patch_batches,
        })
    }

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
        let preview_binding_count = self.handle_bindings.len();
        let effect_binding_count = self.effect_binding_count();
        let effect_delivery_residue_count = self.effect_delivery_residue_count();
        let pending_write_intent_residue_count = self.pending_write_intent_residue_count();
        let authoritative_residue_count = self.authoritative_residue_count();
        let closeout_evidence = self.closeout_evidence(
            ForgeQueryPreviewCloseoutKind::Discarded,
            staged_preview_write_count,
            0,
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

    fn admit_preview_write_intent(&self) -> Result<(), ForgeQueryRuntimeError> {
        self.admit_effect_action(
            ForgeQueryEffectAction::WriteIntent,
            ForgeQueryAuthorityLane::PreviewTruth,
        )
        .map(|_| ())
    }

    fn stage_command(&mut self, command: ForgeQueryWriteCommand) -> ForgeQueryWriteReceipt {
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

    fn route_preview_execution(&mut self, receipt: &ForgeQueryWriteReceipt) {
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
                    &self.label,
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
                    &self.label,
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
                &self.label,
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

    fn effect_binding_count(&self) -> usize {
        self.handle_bindings
            .iter()
            .filter(|binding| binding.family == ForgeQueryPreviewHandleBindingFamily::Effect)
            .count()
    }

    fn effect_delivery_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::EffectDelivery)
    }

    fn pending_write_intent_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::PendingWriteIntent)
    }

    fn subscription_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::LivePatch)
    }

    fn derived_runtime_residue_count(&self) -> usize {
        self.execution_kind_count(ForgeQueryPreviewExecutionKind::ComputedPatch)
    }

    fn authoritative_residue_count(&self) -> usize {
        0
    }

    fn execution_kind_count(&self, kind: ForgeQueryPreviewExecutionKind) -> usize {
        self.execution_evidence
            .iter()
            .filter(|evidence| evidence.kind == kind)
            .count()
    }

    fn closeout_evidence(
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

    fn handle_binding_count(&self, family: ForgeQueryPreviewHandleBindingFamily) -> usize {
        self.handle_bindings
            .iter()
            .filter(|binding| binding.family == family)
            .count()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewCloseoutKind {
    Discarded,
    Promoted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewPromotionDenialKind {
    StaleBasis,
    WriteFailed,
    AtomicBatchUnsupported,
}

impl ForgeQueryPreviewPromotionDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::StaleBasis => "stale-basis",
            Self::WriteFailed => "write-failed",
            Self::AtomicBatchUnsupported => "atomic-batch-unsupported",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewExecutionKind {
    LivePatch,
    ComputedPatch,
    EffectDelivery,
    PendingWriteIntent,
    MutedEffect,
}

impl ForgeQueryPreviewExecutionKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LivePatch => "live-patch",
            Self::ComputedPatch => "computed-patch",
            Self::EffectDelivery => "effect-delivery",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::MutedEffect => "muted-effect",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewExecutionEvidence {
    label: String,
    kind: ForgeQueryPreviewExecutionKind,
    handle_name: String,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    commit_identity: String,
    aspect_paths: Vec<String>,
    execution_digest: String,
}

impl ForgeQueryPreviewExecutionEvidence {
    fn new(
        label: &str,
        kind: ForgeQueryPreviewExecutionKind,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        preview_lane: ForgeQueryAuthorityLane,
        commit_identity: &str,
        aspect_paths: Vec<String>,
    ) -> Self {
        let execution_digest = hash_parts(&[
            "forge_query_preview_execution_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("handle:{handle_name}"),
            format!("source_lane:{source_lane}"),
            format!("preview_lane:{preview_lane}"),
            format!("commit:{commit_identity}"),
            format!("aspects:{}", aspect_paths.join("|")),
        ]);
        Self {
            label: label.to_string(),
            kind,
            handle_name: handle_name.to_string(),
            source_lane,
            preview_lane,
            commit_identity: commit_identity.to_string(),
            aspect_paths,
            execution_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> ForgeQueryPreviewExecutionKind {
        self.kind
    }

    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn preview_lane(&self) -> ForgeQueryAuthorityLane {
        self.preview_lane
    }

    pub fn commit_identity(&self) -> &str {
        &self.commit_identity
    }

    pub fn aspect_paths(&self) -> &[String] {
        &self.aspect_paths
    }

    pub fn execution_digest(&self) -> &str {
        &self.execution_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewPromotionDenialEvidence {
    label: String,
    kind: ForgeQueryPreviewPromotionDenialKind,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    basis_snapshot_token: String,
    promotion_snapshot_token: String,
    staged_preview_write_count: usize,
    promoted_write_count: usize,
    failed_write_sequence: Option<usize>,
    preview_binding_count: usize,
    reason: String,
    denial_digest: String,
}

impl ForgeQueryPreviewPromotionDenialEvidence {
    #[allow(clippy::too_many_arguments)]
    fn new(
        label: &str,
        kind: ForgeQueryPreviewPromotionDenialKind,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: Option<usize>,
        preview_binding_count: usize,
        reason: String,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let denial_digest = hash_parts(&[
            "forge_query_preview_promotion_denial_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("policy:{}", effect_policy.as_str()),
            format!("basis_label:{}", basis_admission.label()),
            format!("basis_lane:{}", basis_admission.authority_lane()),
            format!("basis_snapshot:{basis_snapshot_token}"),
            format!("promotion_snapshot:{promotion_snapshot_token}"),
            format!("basis_evidence:{}", basis_evidence.join("|")),
            format!("staged_preview_writes:{staged_preview_write_count}"),
            format!("promoted_writes:{promoted_write_count}"),
            format!(
                "failed_write_sequence:{}",
                failed_write_sequence
                    .map(|sequence| sequence.to_string())
                    .unwrap_or_else(|| "none".to_string())
            ),
            format!("preview_bindings:{preview_binding_count}"),
            format!("reason:{reason}"),
        ]);
        Self {
            label: label.to_string(),
            kind,
            effect_policy,
            basis_evidence,
            basis_snapshot_token: basis_snapshot_token.to_string(),
            promotion_snapshot_token: promotion_snapshot_token.to_string(),
            staged_preview_write_count,
            promoted_write_count,
            failed_write_sequence,
            preview_binding_count,
            reason,
            denial_digest,
        }
    }

    fn stale_basis(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::StaleBasis,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            "preview promotion rejected because authoritative basis changed before promotion"
                .to_string(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    fn write_failed(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        promoted_write_count: usize,
        failed_write_sequence: usize,
        preview_binding_count: usize,
        reason: String,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::WriteFailed,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            promoted_write_count,
            Some(failed_write_sequence),
            preview_binding_count,
            reason,
        )
    }

    fn atomic_batch_unsupported(
        label: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        basis_snapshot_token: &str,
        promotion_snapshot_token: &str,
        staged_preview_write_count: usize,
        preview_binding_count: usize,
    ) -> Self {
        Self::new(
            label,
            ForgeQueryPreviewPromotionDenialKind::AtomicBatchUnsupported,
            effect_policy,
            basis_admission,
            basis_snapshot_token,
            promotion_snapshot_token,
            staged_preview_write_count,
            0,
            None,
            preview_binding_count,
            "preview promotion rejected because multiple staged writes require atomic promotion support"
                .to_string(),
        )
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> ForgeQueryPreviewPromotionDenialKind {
        self.kind
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn basis_snapshot_token(&self) -> &str {
        &self.basis_snapshot_token
    }

    pub fn promotion_snapshot_token(&self) -> &str {
        &self.promotion_snapshot_token
    }

    pub fn staged_preview_write_count(&self) -> usize {
        self.staged_preview_write_count
    }

    pub fn promoted_write_count(&self) -> usize {
        self.promoted_write_count
    }

    pub fn failed_write_sequence(&self) -> Option<usize> {
        self.failed_write_sequence
    }

    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl ForgeQueryPreviewCloseoutKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discarded => "discarded",
            Self::Promoted => "promoted",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewResidueClass {
    SubscriptionState,
    DerivedRuntimeState,
    EffectDeliveryState,
    PendingWriteIntent,
    PreviewWriteStaging,
    AuthoritativeResidue,
}

impl ForgeQueryPreviewResidueClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SubscriptionState => "subscription-state",
            Self::DerivedRuntimeState => "derived-runtime-state",
            Self::EffectDeliveryState => "effect-delivery-state",
            Self::PendingWriteIntent => "pending-write-intent",
            Self::PreviewWriteStaging => "preview-write-staging",
            Self::AuthoritativeResidue => "authoritative-residue",
        }
    }

    pub fn is_authoritative(self) -> bool {
        self == Self::AuthoritativeResidue
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewCloseoutEvidence {
    label: String,
    kind: ForgeQueryPreviewCloseoutKind,
    effect_policy: ForgeQueryEffectPolicy,
    basis_evidence: Vec<String>,
    preview_binding_count: usize,
    live_binding_count: usize,
    computed_binding_count: usize,
    effect_binding_count: usize,
    subscription_residue_count: usize,
    derived_runtime_residue_count: usize,
    effect_delivery_residue_count: usize,
    pending_write_intent_residue_count: usize,
    preview_write_staging_count: usize,
    promoted_write_count: usize,
    authoritative_residue_count: usize,
    closeout_digest: String,
}

impl ForgeQueryPreviewCloseoutEvidence {
    #[allow(clippy::too_many_arguments)]
    fn new(
        label: &str,
        kind: ForgeQueryPreviewCloseoutKind,
        effect_policy: ForgeQueryEffectPolicy,
        basis_admission: &ForgeQueryPreviewBasisAdmission,
        preview_binding_count: usize,
        live_binding_count: usize,
        computed_binding_count: usize,
        effect_binding_count: usize,
        subscription_residue_count: usize,
        derived_runtime_residue_count: usize,
        preview_write_staging_count: usize,
        promoted_write_count: usize,
        effect_delivery_residue_count: usize,
        pending_write_intent_residue_count: usize,
        authoritative_residue_count: usize,
    ) -> Self {
        let basis_evidence = basis_admission.evidence().to_vec();
        let closeout_digest = hash_parts(&[
            "forge_query_preview_closeout_evidence_v1".to_string(),
            format!("label:{label}"),
            format!("kind:{}", kind.as_str()),
            format!("policy:{}", effect_policy.as_str()),
            format!("basis_label:{}", basis_admission.label()),
            format!("basis_lane:{}", basis_admission.authority_lane()),
            format!("basis_evidence:{}", basis_evidence.join("|")),
            format!("preview_bindings:{preview_binding_count}"),
            format!("live_bindings:{live_binding_count}"),
            format!("computed_bindings:{computed_binding_count}"),
            format!("effect_bindings:{effect_binding_count}"),
            format!("subscription_residue:{subscription_residue_count}"),
            format!("derived_residue:{derived_runtime_residue_count}"),
            format!("effect_delivery_residue:{effect_delivery_residue_count}"),
            format!("pending_write_intent_residue:{pending_write_intent_residue_count}"),
            format!("preview_write_staging:{preview_write_staging_count}"),
            format!("promoted_writes:{promoted_write_count}"),
            format!("authoritative_residue:{authoritative_residue_count}"),
        ]);
        Self {
            label: label.to_string(),
            kind,
            effect_policy,
            basis_evidence,
            preview_binding_count,
            live_binding_count,
            computed_binding_count,
            effect_binding_count,
            subscription_residue_count,
            derived_runtime_residue_count,
            effect_delivery_residue_count,
            pending_write_intent_residue_count,
            preview_write_staging_count,
            promoted_write_count,
            authoritative_residue_count,
            closeout_digest,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn kind(&self) -> ForgeQueryPreviewCloseoutKind {
        self.kind
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }

    pub fn live_binding_count(&self) -> usize {
        self.live_binding_count
    }

    pub fn computed_binding_count(&self) -> usize {
        self.computed_binding_count
    }

    pub fn effect_binding_count(&self) -> usize {
        self.effect_binding_count
    }

    pub fn subscription_residue_count(&self) -> usize {
        self.subscription_residue_count
    }

    pub fn derived_runtime_residue_count(&self) -> usize {
        self.derived_runtime_residue_count
    }

    pub fn effect_delivery_residue_count(&self) -> usize {
        self.effect_delivery_residue_count
    }

    pub fn pending_write_intent_residue_count(&self) -> usize {
        self.pending_write_intent_residue_count
    }

    pub fn preview_write_staging_count(&self) -> usize {
        self.preview_write_staging_count
    }

    pub fn promoted_write_count(&self) -> usize {
        self.promoted_write_count
    }

    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }

    pub fn class_count(&self, residue_class: ForgeQueryPreviewResidueClass) -> usize {
        match residue_class {
            ForgeQueryPreviewResidueClass::SubscriptionState => self.subscription_residue_count,
            ForgeQueryPreviewResidueClass::DerivedRuntimeState => {
                self.derived_runtime_residue_count
            }
            ForgeQueryPreviewResidueClass::EffectDeliveryState => {
                self.effect_delivery_residue_count
            }
            ForgeQueryPreviewResidueClass::PendingWriteIntent => {
                self.pending_write_intent_residue_count
            }
            ForgeQueryPreviewResidueClass::PreviewWriteStaging => self.preview_write_staging_count,
            ForgeQueryPreviewResidueClass::AuthoritativeResidue => self.authoritative_residue_count,
        }
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewHandleBindingFamily {
    LiveView,
    ComputedView,
    Effect,
}

impl ForgeQueryPreviewHandleBindingFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::LiveView => "live-view",
            Self::ComputedView => "computed-view",
            Self::Effect => "effect",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPreviewEffectBindingDisposition {
    MutedByDeriveOnly,
    Muted,
    RedirectedDelivery,
    SandboxedWriteIntent,
    AuthoritativeAllowed,
}

impl ForgeQueryPreviewEffectBindingDisposition {
    fn from_policy(
        policy: ForgeQueryEffectPolicy,
        action: ForgeQueryEffectAction,
        target_lane: ForgeQueryAuthorityLane,
    ) -> Result<Self, ForgeQueryRuntimeError> {
        match policy {
            ForgeQueryEffectPolicy::DeriveOnly => Ok(Self::MutedByDeriveOnly),
            ForgeQueryEffectPolicy::Muted => Ok(Self::Muted),
            ForgeQueryEffectPolicy::Redirected => policy
                .admit(action, ForgeQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::RedirectedDelivery)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
            ForgeQueryEffectPolicy::SandboxedWriteIntent => policy
                .admit(action, ForgeQueryAuthorityLane::PreviewTruth)
                .map(|_| Self::SandboxedWriteIntent)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
            ForgeQueryEffectPolicy::AuthoritativeAllowed => policy
                .admit(action, target_lane)
                .map(|_| Self::AuthoritativeAllowed)
                .map_err(ForgeQueryRuntimeError::EffectPolicyDenied),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::MutedByDeriveOnly => "muted-by-derive-only",
            Self::Muted => "muted",
            Self::RedirectedDelivery => "redirected-delivery",
            Self::SandboxedWriteIntent => "sandboxed-write-intent",
            Self::AuthoritativeAllowed => "authoritative-allowed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewHandleBindingEvidence {
    label: String,
    handle_name: String,
    family: ForgeQueryPreviewHandleBindingFamily,
    source_lane: ForgeQueryAuthorityLane,
    preview_lane: ForgeQueryAuthorityLane,
    effect_policy: ForgeQueryEffectPolicy,
    effect_disposition: Option<ForgeQueryPreviewEffectBindingDisposition>,
    basis_evidence: Vec<String>,
    effect_delivery_admitted: bool,
    pending_write_intent_admitted: bool,
    authoritative_side_effect_admitted: bool,
}

impl ForgeQueryPreviewHandleBindingEvidence {
    fn live_view(
        label: &str,
        handle_name: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::LiveView,
            source_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    fn computed(
        label: &str,
        handle_name: &str,
        effect_policy: ForgeQueryEffectPolicy,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::ComputedView,
            source_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: None,
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: false,
            pending_write_intent_admitted: false,
            authoritative_side_effect_admitted: false,
        }
    }

    fn effect(
        label: &str,
        handle_name: &str,
        source_lane: ForgeQueryAuthorityLane,
        effect_policy: ForgeQueryEffectPolicy,
        disposition: ForgeQueryPreviewEffectBindingDisposition,
        basis_evidence: &[String],
    ) -> Self {
        Self {
            label: label.to_string(),
            handle_name: handle_name.to_string(),
            family: ForgeQueryPreviewHandleBindingFamily::Effect,
            source_lane,
            preview_lane: ForgeQueryAuthorityLane::PreviewTruth,
            effect_policy,
            effect_disposition: Some(disposition),
            basis_evidence: basis_evidence.to_vec(),
            effect_delivery_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::RedirectedDelivery,
            pending_write_intent_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::SandboxedWriteIntent,
            authoritative_side_effect_admitted: disposition
                == ForgeQueryPreviewEffectBindingDisposition::AuthoritativeAllowed,
        }
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn handle_name(&self) -> &str {
        &self.handle_name
    }

    pub fn family(&self) -> ForgeQueryPreviewHandleBindingFamily {
        self.family
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn preview_lane(&self) -> ForgeQueryAuthorityLane {
        self.preview_lane
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn effect_disposition(&self) -> Option<ForgeQueryPreviewEffectBindingDisposition> {
        self.effect_disposition
    }

    pub fn basis_evidence(&self) -> &[String] {
        &self.basis_evidence
    }

    pub fn effect_delivery_admitted(&self) -> bool {
        self.effect_delivery_admitted
    }

    pub fn pending_write_intent_admitted(&self) -> bool {
        self.pending_write_intent_admitted
    }

    pub fn authoritative_side_effect_admitted(&self) -> bool {
        self.authoritative_side_effect_admitted
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewDiff {
    label: String,
    write_count: usize,
    changed_entity_count: usize,
}

impl ForgeQueryPreviewDiff {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn write_count(&self) -> usize {
        self.write_count
    }

    pub fn changed_entity_count(&self) -> usize {
        self.changed_entity_count
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPreviewOutcome {
    label: String,
    effect_policy: ForgeQueryEffectPolicy,
    promoted: bool,
    discarded: bool,
    write_count: usize,
    preview_binding_count: usize,
    effect_binding_count: usize,
    effect_delivery_residue_count: usize,
    pending_write_intent_residue_count: usize,
    authoritative_residue_count: usize,
    closeout_evidence: ForgeQueryPreviewCloseoutEvidence,
    source_lane: ForgeQueryAuthorityLane,
    target_lane: ForgeQueryAuthorityLane,
}

impl ForgeQueryPreviewOutcome {
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn effect_policy(&self) -> ForgeQueryEffectPolicy {
        self.effect_policy
    }

    pub fn promoted(&self) -> bool {
        self.promoted
    }

    pub fn discarded(&self) -> bool {
        self.discarded
    }

    pub fn write_count(&self) -> usize {
        self.write_count
    }

    pub fn preview_binding_count(&self) -> usize {
        self.preview_binding_count
    }

    pub fn effect_binding_count(&self) -> usize {
        self.effect_binding_count
    }

    pub fn effect_delivery_residue_count(&self) -> usize {
        self.effect_delivery_residue_count
    }

    pub fn pending_write_intent_residue_count(&self) -> usize {
        self.pending_write_intent_residue_count
    }

    pub fn authoritative_residue_count(&self) -> usize {
        self.authoritative_residue_count
    }

    pub fn closeout_evidence(&self) -> &ForgeQueryPreviewCloseoutEvidence {
        &self.closeout_evidence
    }

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
}

fn relevant_live_aspects(
    request: &DeclarativeLiveQueryRequest,
    deltas: &[ForgeQueryMutationDelta],
) -> Vec<String> {
    let mut aspects = BTreeSet::new();
    for delta in deltas {
        if delta.collection != request.target() {
            continue;
        }
        if delta.aspect_paths.is_empty()
            || matches!(
                delta.kind,
                ForgeQueryMutationKind::Created | ForgeQueryMutationKind::Deleted
            )
        {
            for field in request.projection() {
                aspects.insert(format!("{}.{}", field.aspect(), field.field()));
            }
            continue;
        }
        for changed in &delta.aspect_paths {
            if request.projection().iter().any(|field| {
                let projected = format!("{}.{}", field.aspect(), field.field());
                changed == &projected
                    || changed.starts_with(&format!("{}.", field.aspect()))
                    || projected.starts_with(&format!("{changed}."))
            }) {
                aspects.insert(changed.clone());
            }
        }
    }
    aspects.into_iter().collect()
}

fn relevant_computed_aspects(
    runtime: &ForgeQueryDerivedViewRuntime,
    live_affected: &BTreeMap<String, Vec<String>>,
    computed_affected: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let mut matched = BTreeSet::new();
    for upstream in runtime.declaration.upstream_live_views() {
        if let Some(aspects) = live_affected.get(upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    for upstream in runtime.declaration.upstream_derived_views() {
        if let Some(aspects) = computed_affected.get(upstream) {
            matched.extend(aspects.iter().cloned());
        }
    }
    if matched.is_empty() {
        return Vec::new();
    }
    if !runtime.declaration.produced_aspects().is_empty() {
        runtime.declaration.produced_aspects().to_vec()
    } else {
        matched.into_iter().collect()
    }
}

fn relevant_effect_aspects(
    inspected: &ForgeQueryEffectInspectionEvidence,
    live_affected: &BTreeMap<String, Vec<String>>,
    computed_affected: &BTreeMap<String, Vec<String>>,
) -> Vec<String> {
    let source_aspects = match inspected.trigger_source_kind() {
        ForgeQueryEffectTriggerSourceKind::LiveView => {
            live_affected.get(inspected.trigger_source())
        }
        ForgeQueryEffectTriggerSourceKind::ComputedView => {
            computed_affected.get(inspected.trigger_source())
        }
    };
    let Some(source_aspects) = source_aspects else {
        return Vec::new();
    };
    source_aspects
        .iter()
        .filter(|aspect| {
            inspected.trigger_aspects().iter().any(|declared| {
                aspect == &declared
                    || aspect.starts_with(&format!("{declared}."))
                    || declared.starts_with(&format!("{aspect}."))
            })
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
