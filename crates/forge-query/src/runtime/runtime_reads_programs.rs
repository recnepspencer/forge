use super::*;
use crate::memory_workspace::ForgeQuerySnapshotIdentity;

impl ForgeQueryRuntime {
    pub fn read_live<T>(&mut self, view: &ForgeQueryLiveView<T>) -> Vec<ForgeQueryEntity> {
        self.read_live_result(view)
            .expect("live view declaration admitted before runtime.read_live execution")
            .rows()
            .to_vec()
    }

    pub fn read_live_result<T>(
        &mut self,
        view: &ForgeQueryLiveView<T>,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        self.execute_live_read_for_installation(view.subscription_installation().clone())
    }

    pub fn drain_patches<T>(&mut self, view: &ForgeQueryLiveView<T>) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Live)
            .expect("live support was admitted before patch draining");
        let live_target = ForgeQueryLiveArtifactTarget::from_subscription_installation(
            view.subscription_installation(),
        );
        let _legacy_patch_buffer = self.backend.drain_live_patches_for_target(&live_target);
        ForgeQueryPatchBatch {
            view_name: view.name().to_string(),
            live_patches: Vec::new(),
            query_delivery_batches: self
                .live_subscriptions
                .get_mut(&live_target)
                .map(|state| std::mem::take(&mut state.delivery_batches))
                .unwrap_or_default(),
            derived_patch_notes: Vec::new(),
            derived_patches: Vec::new(),
        }
    }

    pub fn drain_derived_patches<T>(
        &mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> ForgeQueryPatchBatch {
        self.drain_derived_patches_by_name(view.name())
    }

    pub(crate) fn drain_derived_patches_by_name(
        &mut self,
        view_name: &str,
    ) -> ForgeQueryPatchBatch {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Computed)
            .expect("computed support was admitted before derived patch draining");
        let derived_patches = self
            .derived_views
            .get_mut(&ForgeQueryDerivedMaterializationTarget::new(view_name))
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

    pub fn read_derived_result<T>(
        &self,
        view: &ForgeQueryDerivedViewHandle<T>,
    ) -> Result<ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
        self.review_runtime_derived_materialization(view.name().to_string())
            .and_then(|review| {
                self.resolve_reviewed_admitted_derived_materialization_handoff(review)
            })
            .map(|handoff| self.prepare_derived_materialization_execution_binding(handoff))
            .and_then(|binding| self.execute_derived_materialization_execution_binding(binding))
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
        let effect_target = ForgeQueryEffectTarget::from_name(effect.name());
        self.effects
            .get_mut(&effect_target)
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
        let effect_target = ForgeQueryEffectTarget::from_name(effect_name);
        self.effects
            .get(&effect_target)
            .map(ForgeQueryEffectInspectionEvidence::from_runtime)
            .ok_or_else(|| ForgeQueryRuntimeError::MissingEffect(effect_name.to_string()))
    }

    pub(super) fn computed_candidate_live_views(
        &self,
        receipt: &ForgeQueryMutationReceipt,
    ) -> BTreeSet<ForgeQueryLiveArtifactTarget> {
        let mut candidates = BTreeSet::new();
        for delta in &receipt.deltas {
            if let Some(entry) = self.live_subscription_index.iter().find(|entry| {
                delta
                    .target_collection_identity()
                    .same_target_collection_as(entry.target_collection())
            }) {
                candidates.extend(entry.targets().iter().cloned());
            }
        }
        candidates
    }

    pub(super) fn live_artifact_target_collections(
        &self,
    ) -> BTreeMap<ForgeQueryLiveArtifactTarget, ForgeQueryMutationTargetCollectionIdentity> {
        self.live_subscriptions
            .iter()
            .map(|(target, state)| (target.clone(), state.request.target_collection_identity()))
            .collect()
    }

    pub fn current_snapshot_identity(&self) -> ForgeQuerySnapshotIdentity {
        self.backend.current_snapshot_identity()
    }

    pub fn install_program(
        &mut self,
        program: ForgeQueryProgram,
    ) -> Result<ForgeQueryInstalledProgram, ForgeQueryRuntimeError> {
        let program_identity = ForgeQueryProgramInstallationIdentity::from_program_id(program.id());
        self.installed_programs
            .insert(program_identity.clone(), program);
        Ok(ForgeQueryInstalledProgram { program_identity })
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
            operation.program_identity.as_str(),
            operation.operation_identity.as_str(),
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
                    let _: ForgeQueryLiveView<ForgeQueryNativeRow> =
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
                    let installation = self
                        .live_subscriptions
                        .get(&ForgeQueryLiveArtifactTarget::from_view_name(&view_name))
                        .map(|state| state.installation.clone())
                        .ok_or_else(|| {
                            ForgeQueryRuntimeError::MissingLiveView(view_name.clone())
                        })?;
                    let read = self.execute_live_read_for_installation(installation)?;
                    outputs.push(ForgeQueryOperationOutput::from_live_read_entities(
                        format!("live:{view_name}"),
                        read.rows().iter().cloned(),
                    ));
                    trace.record_replay_or_parity(format!("read-live:{view_name}"));
                }
                ForgeQueryProgramEffect::DrainPatches { view_name } => {
                    let live_target = self
                        .live_subscriptions
                        .get(&ForgeQueryLiveArtifactTarget::from_view_name(&view_name))
                        .map(|state| {
                            ForgeQueryLiveArtifactTarget::from_subscription_installation(
                                &state.installation,
                            )
                        })
                        .unwrap_or_else(|| {
                            ForgeQueryLiveArtifactTarget::from_view_name(view_name.clone())
                        });
                    let _legacy_patch_buffer =
                        self.backend.drain_live_patches_for_target(&live_target);
                    let query_delivery_batches = self
                        .live_subscriptions
                        .get_mut(&live_target)
                        .map(|state| std::mem::take(&mut state.delivery_batches))
                        .unwrap_or_default();
                    for batch in &query_delivery_batches {
                        trace.record_patch_artifact(format!(
                            "query-delivery:{}:{}",
                            batch.view_name(),
                            batch.delivery_batch_for_reporting()
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
        let run_identity = self.next_run_identity(&operation);
        self.run_traces.insert(run_identity.clone(), trace);
        Ok(ForgeQueryRunReceipt {
            run_identity,
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
            .get(&operation.program_identity)
            .ok_or_else(|| {
                ForgeQueryRuntimeError::UnknownProgram(
                    operation.program_identity.as_str().to_string(),
                )
            })?;
        program
            .operation(operation.operation_identity.as_str())
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownOperation {
                program_id: operation.program_identity.as_str().to_string(),
                operation_id: operation.operation_identity.as_str().to_string(),
            })
            .cloned()
    }

    pub(super) fn next_run_identity(
        &mut self,
        operation: &ForgeQueryInstalledOperation,
    ) -> ForgeQueryProgramRunIdentity {
        self.next_run_id += 1;
        ForgeQueryProgramRunIdentity::from_run_id(format!(
            "query-run:{}:{}:{}",
            operation.program_identity.as_str(),
            operation.operation_identity.as_str(),
            self.next_run_id
        ))
    }

    pub fn inspect_run(
        &self,
        run: &ForgeQueryRunReceipt,
    ) -> Result<ForgeQueryProgramTrace, ForgeQueryRuntimeError> {
        self.admit_facade_family(ForgeQueryRuntimeFacadeFamily::Inspect)?;
        self.run_traces
            .get(&run.run_identity)
            .cloned()
            .ok_or_else(|| ForgeQueryRuntimeError::UnknownProgram(run.run_id().to_string()))
    }

    pub(crate) fn execute_live_read_for_installation(
        &mut self,
        installation: ForgeQueryRuntimeLiveSubscriptionInstallation,
    ) -> Result<ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
        let review = self.review_runtime_live_read_execution(installation)?;
        let handoff = self.resolve_reviewed_admitted_live_read_execution_handoff(review)?;
        let binding = self.prepare_live_read_execution_binding(handoff)?;
        self.execute_live_read_execution_binding(binding)
    }
}
