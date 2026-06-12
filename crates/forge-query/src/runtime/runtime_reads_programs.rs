use super::*;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;

impl ForgeQueryRuntime {
    pub fn read_live<T>(&mut self, view: &ForgeQueryLiveView<T>) -> Vec<ForgeQueryEntity> {
        self.execute_live_read_by_name(view.name())
            .expect("live view declaration admitted before runtime.read_live execution")
            .rows()
            .to_vec()
    }

    pub fn drain_patches<T>(&mut self, view: &ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)
            .expect("live support was admitted before patch draining");
        let _legacy_patch_buffer = self.backend.drain_live_patches(view.name());
        ForgeQueryPatchBatch {
            view_name: view.name().to_string(),
            live_patches: Vec::new(),
            query_delivery_batches: self
                .live_subscriptions
                .get_mut(view.name())
                .map(|state| std::mem::take(&mut state.delivery_batches))
                .unwrap_or_default(),
            derived_patch_notes: Vec::new(),
            derived_patches: Vec::new(),
        }
    }

    pub fn drain_derived_patches(&mut self, view_name: &str) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)
            .expect("computed support was admitted before derived patch draining");
        let derived_patches = self
            .derived_views
            .get_mut(view_name)
            .map(|view| std::mem::take(&mut view.patches))
            .unwrap_or_default();
        ForgeQueryPatchBatch {
            view_name: view_name.to_string(),
            live_patches: Vec::new(),
            query_delivery_batches: Vec::new(),
            derived_patch_notes: derived_patches
                .iter()
                .map(ForgeQueryDerivedPatch::note)
                .collect(),
            derived_patches,
        }
    }

    pub fn read_derived<T>(&self, view: &ForgeQueryDerivedViewHandle<T>) -> Vec<Value> {
        self.review_runtime_derived_materialization(view.name().to_string())
            .and_then(|review| {
                self.resolve_reviewed_admitted_derived_materialization_handoff(review)
            })
            .map(|handoff| self.prepare_derived_materialization_execution_binding(handoff))
            .and_then(|binding| self.execute_derived_materialization_execution_binding(binding))
            .map(|result| result.rows().to_vec())
            .expect("derived view declaration admitted before runtime.read_derived execution")
    }

    pub fn inspect_derived_view<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryComputedInspectionEvidence, ForgeQueryRuntimeError> {
        let review = self.review_runtime_derived_inspection(view.name().to_string())?;
        let handoff = self.resolve_reviewed_admitted_derived_inspection_handoff(review)?;
        let binding = self.prepare_derived_inspection_execution_binding(handoff);
        self.execute_derived_inspection_execution_binding(binding)
            .map(|result| result.evidence().clone())
    }

    pub fn drain_effect_deliveries<T>(
        &mut self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<Vec<ForgeQueryEffectDelivery>, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Effect)?;
        self.effects
            .get_mut(effect.name())
            .map(|runtime| std::mem::take(&mut runtime.deliveries))
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect.name().to_string()))
    }

    pub fn inspect_effect<T>(
        &self,
        effect: &ForgeQueryEffectHandle<T>,
    ) -> Result<ForgeQueryEffectInspectionEvidence, ForgeQueryRuntimeError> {
        match self.inspect(effect)? {
            ForgeQueryInspection::Effect(inspection) => Ok(inspection),
            other => panic!("expected effect inspection, got {other:?}"),
        }
    }

    pub(super) fn inspect_effect_by_name(
        &self,
        effect_name: &str,
    ) -> Result<ForgeQueryEffectInspectionEvidence, ForgeQueryRuntimeError> {
        self.effects
            .get(effect_name)
            .map(ForgeQueryEffectInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect_name.to_string()))
    }

    pub(super) fn computed_candidate_live_views(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> BTreeSet<String> {
        let mut candidates = BTreeSet::new();
        for delta in &receipt.deltas {
            if let Some(view_names) = self.live_subscription_index.get(&delta.collection) {
                candidates.extend(view_names.iter().cloned());
            }
        }
        candidates
    }

    pub(super) fn live_view_targets(&self) -> BTreeMap<String, String> {
        self.live_subscriptions
            .iter()
            .map(|(view_name, state)| (view_name.clone(), state.request.target().to_string()))
            .collect()
    }

    pub fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.backend.current_snapshot_identity()
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        let program_id = program.id().to_string();
        self.installed_programs.insert(program_id.clone(), program);
        Ok(ForgeQueryInstalledProgram { program_id })
    }

    pub fn run_operation(
        &mut self,
        operation: ForgeQueryInstalledOperation,
        inputs: Vec<ForgeQueryOperationInput>,
    ) -> Result<ForgeQueryRunReceipt, ForgeQueryRuntimeError> {
        let query_operation = self.installed_query_operation(&operation)?;
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
                        self.declare_live_view(name.clone(), request, schema_view)?;
                    trace.record_declaration(format!("live:{name}"));
                }
                ForgeQueryProgramEffect::DeclareDerivedView(view) => {
                    let name = view.name().to_string();
                    self.declare_derived_view(view)?;
                    trace.record_declaration(format!("derived:{name}"));
                }
                ForgeQueryProgramEffect::Write(command) => {
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().clone());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::WriteTemplate(template) => {
                    let command = template.bind(&bound_inputs)?;
                    let receipt = self.write(command)?;
                    trace.record_write_receipt(receipt.commit_identity().clone());
                    write_receipts.push(receipt);
                }
                ForgeQueryProgramEffect::ReadLive { view_name } => {
                    let read = self.execute_live_read_by_name(&view_name)?;
                    outputs.push(ForgeQueryOperationOutput::new(
                        format!("live:{view_name}"),
                        Value::Array(
                            read.rows()
                                .iter()
                                .cloned()
                                .map(ForgeQueryEntity::into_external_row)
                                .collect(),
                        ),
                    ));
                    trace.record_replay_or_parity(format!("read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    let _legacy_patch_buffer = self.backend.drain_live_patches(&view_name);
                    let query_delivery_batches = self
                        .live_subscriptions
                        .get_mut(&view_name)
                        .map(|state| std::mem::take(&mut state.delivery_batches))
                        .unwrap_or_default();
                    for batch in &query_delivery_batches {
                        trace.record_patch_artifact(format!(
                            "query-delivery:{}:{}",
                            batch.view_name(),
                            batch.delivery_batch_digest()
                        ));
                    }
                    patch_batches.push(ForgeQueryPatchBatch {
                        view_name,
                        live_patches: Vec::new(),
                        query_delivery_batches,
                        derived_patch_notes: Vec::new(),
                        derived_patches: Vec::new(),
                    });
                }
            }
        }
        let run_id = self.next_run_identity(&operation);
        self.run_traces.insert(run_id.clone(), trace);
        Ok(ForgeQueryRunReceipt {
            run_id,
            operation,
            outputs,
            write_receipts,
            patch_batches,
        })
    }

    pub(super) fn installed_query_operation(
        &self,
        operation: &ForgeQueryInstalledOperation,
    ) -> Result<crate::program::ForgeQueryOperation, ForgeQueryRuntimeError> {
        let program = self
            .installed_programs
            .get(&operation.program_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(operation.program_id.clone()))?;
        program
            .operation(&operation.operation_id)
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownOperation {
                program_id: operation.program_id.clone(),
                operation_id: operation.operation_id.clone(),
            })
            .cloned()
    }

    pub(super) fn next_run_identity(&mut self, operation: &ForgeQueryInstalledOperation) -> String {
        self.next_run_id += 1;
        format!(
            "query-run:{}:{}:{}",
            operation.program_id, operation.operation_id, self.next_run_id
        )
    }

    pub fn inspect_run(
        &self,
        run: &ForgeQueryRunReceipt,
    ) -> Result<ForgeQueryProgramTrace, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.run_traces
            .get(run.run_id())
            .cloned()
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(run.run_id().to_string()))
    }

    pub(crate) fn execute_live_read_by_name(
        &mut self,
        view_name: &str,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        let installation = self
            .live_subscriptions
            .get(view_name)
            .map(|state| state.installation.clone())
            .ok_or_else(|| ForgeQueryRuntimeError::MissingLiveView(view_name.to_string()))?;
        let review = self.review_runtime_live_read_execution(installation)?;
        let handoff = self.resolve_reviewed_admitted_live_read_execution_handoff(review)?;
        let binding = self.prepare_live_read_execution_binding(handoff);
        self.execute_live_read_execution_binding(binding)
    }
}
