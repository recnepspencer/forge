use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{
    admit_authority_requirements, validate_inputs, ForgeQueryAspectMutationBuilder,
    ForgeQueryAuthorityLane, ForgeQueryBatchWriteReceipt, ForgeQueryDerivedViewHandle,
    ForgeQueryDerivedViewRuntime, ForgeQueryEffectAction, ForgeQueryEffectAdmission,
    ForgeQueryEffectHandle, ForgeQueryEffectInspectionEvidence, ForgeQueryEffectPolicy,
    ForgeQueryEffectTriggerSourceKind, ForgeQueryInstalledOperation, ForgeQueryIntentDeclaration,
    ForgeQueryIntentDenialEvidence, ForgeQueryIntentSourceLane, ForgeQueryLiveView,
    ForgeQueryMutationBatchBuilder, ForgeQueryOperationInput, ForgeQueryOperationOutput,
    ForgeQueryPatchBatch, ForgeQueryPreviewBasisAdmission, ForgeQueryPreviewIntentReceipt,
    ForgeQueryProgramEffect, ForgeQueryProgramTrace, ForgeQueryRunReceipt, ForgeQueryRuntime,
    ForgeQueryRuntimeError, ForgeQueryRuntimeFacadeFamily, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt,
};
use crate::declarative_live::DeclarativeLiveQueryRequest;
use crate::identity::hash_parts;
use crate::memory_workspace::{ForgeQueryMutationDelta, ForgeQueryMutationKind};
mod aspects;
mod binding;
mod evidence;
mod outcome;
mod session_closeout;
mod session_execution;

use aspects::{relevant_computed_aspects, relevant_effect_aspects, relevant_live_aspects};
pub use binding::{
    ForgeQueryPreviewEffectBindingDisposition, ForgeQueryPreviewHandleBindingEvidence,
    ForgeQueryPreviewHandleBindingFamily,
};
pub use evidence::{
    ForgeQueryPreviewCloseoutEvidence, ForgeQueryPreviewCloseoutKind,
    ForgeQueryPreviewExecutionEvidence, ForgeQueryPreviewExecutionKind,
    ForgeQueryPreviewPromotionDenialEvidence, ForgeQueryPreviewPromotionDenialKind,
    ForgeQueryPreviewResidueClass,
};
pub use outcome::{ForgeQueryPreviewDiff, ForgeQueryPreviewOutcome};

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
    intent_receipts: Vec<ForgeQueryPreviewIntentReceipt>,
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
            intent_receipts: Vec::new(),
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

    pub fn insert(
        &mut self,
        collection: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_insert(collection)?;
        self.write(command)
    }

    pub fn update(
        &mut self,
        entity_identity: impl Into<String>,
        declaration: impl FnOnce(ForgeQueryAspectMutationBuilder) -> ForgeQueryAspectMutationBuilder,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        let command =
            declaration(ForgeQueryAspectMutationBuilder::new()).build_update(entity_identity)?;
        self.write(command)
    }

    pub fn delete(
        &mut self,
        entity_identity: impl Into<String>,
    ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
        self.write(ForgeQueryWriteCommand::Delete {
            entity_identity: entity_identity.into(),
        })
    }

    pub fn batch(
        &mut self,
        declaration: impl FnOnce(ForgeQueryMutationBatchBuilder) -> ForgeQueryMutationBatchBuilder,
    ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
        let commands = declaration(ForgeQueryMutationBatchBuilder::new()).finish()?;
        let mut receipts = Vec::with_capacity(commands.len());
        for command in commands {
            receipts.push(self.write(command)?);
        }
        ForgeQueryBatchWriteReceipt::from_write_receipts(receipts)
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
            self.admit_operation_effect(effect)?;
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

    fn admit_operation_effect(
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
