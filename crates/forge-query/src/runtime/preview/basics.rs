use super::*;

impl<'a> ForgeQueryPreviewSession<'a> {
    pub fn label(&self) -> &str {
        self.label.display()
    }

    pub fn session_label(&self) -> &ForgeQuerySessionLabel {
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

    pub fn use_view<T>(
        &mut self,
        view: &ForgeQueryLiveView<T>,
    ) -> ForgeQueryPreviewHandleBindingEvidence {
        let evidence = ForgeQueryPreviewHandleBindingEvidence::live_view(
            self.label.display(),
            view.name(),
            self.effect_policy,
            &self.basis_admission.evidence(),
        );
        self.handle_bindings.push(evidence.clone());
        evidence
    }

    pub fn use_computed<T>(
        &mut self,
        computed: &ForgeQueryDerivedViewHandle<T>,
    ) -> ForgeQueryPreviewHandleBindingEvidence {
        let evidence = ForgeQueryPreviewHandleBindingEvidence::computed(
            self.label.display(),
            computed.name(),
            self.effect_policy,
            &self.basis_admission.evidence(),
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
            self.label.display(),
            effect.name(),
            inspected.target_lane(),
            self.effect_policy,
            disposition,
            &self.basis_admission.evidence(),
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
        trace.record_replay_or_parity(format!("preview-session:{}", self.label.display()));
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
                        Value::Array(
                            rows.into_iter()
                                .map(|row| row.into_external_row())
                                .collect(),
                        ),
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
                            self.label.display()
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
}
