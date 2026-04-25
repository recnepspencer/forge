use serde_json::Value;

use super::{
    admit_authority_requirements, validate_inputs, ForgeQueryAuthorityLane, ForgeQueryEffectAction,
    ForgeQueryEffectAdmission, ForgeQueryEffectPolicy, ForgeQueryInstalledOperation,
    ForgeQueryLiveView, ForgeQueryOperationInput, ForgeQueryOperationOutput, ForgeQueryPatchBatch,
    ForgeQueryPreviewBasisAdmission, ForgeQueryProgramEffect, ForgeQueryProgramTrace,
    ForgeQueryRunReceipt, ForgeQueryRuntime, ForgeQueryRuntimeError, ForgeQueryWriteCommand,
    ForgeQueryWriteReceipt,
};
use crate::memory_workspace::ForgeQueryMutationKind;

pub struct ForgeQueryPreviewSession<'a> {
    label: String,
    runtime: &'a mut ForgeQueryRuntime,
    effect_policy: ForgeQueryEffectPolicy,
    basis_admission: ForgeQueryPreviewBasisAdmission,
    pending_commands: Vec<ForgeQueryWriteCommand>,
    writes: Vec<ForgeQueryWriteReceipt>,
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
        Self {
            label: label.into(),
            runtime,
            effect_policy,
            basis_admission,
            pending_commands: Vec::new(),
            writes: Vec::new(),
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
        Ok(receipt)
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

    pub fn promote(mut self) -> ForgeQueryPreviewOutcome {
        let mut promoted_writes = 0;
        for command in std::mem::take(&mut self.pending_commands) {
            if let Ok(receipt) = self.runtime.write(command) {
                self.writes.push(receipt);
                promoted_writes += 1;
            }
        }
        self.promoted = true;
        ForgeQueryPreviewOutcome {
            label: self.label,
            effect_policy: self.effect_policy,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: promoted_writes,
            source_lane: ForgeQueryAuthorityLane::PreviewTruth,
            target_lane: ForgeQueryAuthorityLane::AuthoritativeTruth,
        }
    }

    pub fn discard(mut self) -> ForgeQueryPreviewOutcome {
        self.discarded = true;
        ForgeQueryPreviewOutcome {
            label: self.label,
            effect_policy: self.effect_policy,
            promoted: self.promoted,
            discarded: self.discarded,
            write_count: self.writes.len(),
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
        receipt
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

    pub fn source_lane(&self) -> ForgeQueryAuthorityLane {
        self.source_lane
    }

    pub fn target_lane(&self) -> ForgeQueryAuthorityLane {
        self.target_lane
    }
}
