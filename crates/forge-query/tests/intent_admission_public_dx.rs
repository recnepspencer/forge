use forge_foundational::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    forge_query_basis_observation_intent, forge_query_projection_consumption_intent,
    AdmittedQueryBasisContext, ForgeQueryAspectTouch, ForgeQueryBatchWriteReceipt,
    ForgeQueryEffectHandle, ForgeQueryEffectIntentReceipt, ForgeQueryExistingTruthProbeRequest,
    ForgeQueryExistingTruthProbeResult, ForgeQueryExistingTruthTargetBinding,
    ForgeQueryIntentAdmissionDecision, ForgeQueryIntentConsumerOutcomeClass,
    ForgeQueryIntentDeclaration, ForgeQueryIntentReceipt, ForgeQueryNativeRow,
    ForgeQueryReadFamily, ForgeQueryReadResult, ForgeQueryRuntime, ForgeQueryRuntimeError,
    ForgeQueryWorkspace, ForgeQueryWriteCommand, ForgeQueryWriteReceipt,
    ProjectionConsumptionDeclaration, RawBasisIntent,
};

fn authoritative_common_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    declaration: ForgeQueryIntentDeclaration,
) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
    runtime.intent(declaration).execute()
}

fn authoritative_advanced_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    declaration: ForgeQueryIntentDeclaration,
) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError> {
    let review = runtime.intent(declaration).review()?;

    let _ = review.request().family();
    let _ = review.request().entrypoint();
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.eligibility().trace_evidence().eligibility_digest();
    let _ = review.decision_trace_envelope();
    let _ = review.consumer_inspection().terminal_stage_label();
    if let Some(plan) = review.admitted_plan() {
        let _ = plan.family();
        let _ = plan.entrypoint();
        let _ = plan.execution_seam();
        let _ = plan.decision_digest();
    }

    match review.decision() {
        ForgeQueryIntentAdmissionDecision::Admitted(_) => {}
        ForgeQueryIntentAdmissionDecision::Advisory(_) => {}
        ForgeQueryIntentAdmissionDecision::Violation(_) => {}
    }

    let admitted = review.admit()?;
    let _ = admitted.request().request_digest();
    let _ = admitted.eligibility().eligibility_digest();
    let _ = admitted.decision();
    let _ = admitted.handoff().handoff_digest();
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn effect_common_path_compiles<T>(
    runtime: &mut ForgeQueryRuntime,
    effect: &ForgeQueryEffectHandle<T>,
) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
    let receipt = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .execute()?;
    let _ = receipt.execution_binding_digest();
    let _ = receipt.execution_provenance_chain_digest();
    let _ = receipt.decision_trace_envelope().trace_digest();
    let _ = receipt.decision_trace_envelope().rows()[0].evidence_owner();
    let _ = receipt.consumer_inspection().outcome_class();
    Ok(receipt)
}

fn effect_advanced_path_compiles<T>(
    runtime: &mut ForgeQueryRuntime,
    effect: &ForgeQueryEffectHandle<T>,
) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError> {
    let review = runtime
        .next_effect_write_intent(effect, "1.0", "effect.intent.input.v1")
        .review()?;

    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review.consumer_inspection().decision_trace_digest();
    let _ = review.pending_delivery().commit_identity();
    let admitted = review.admit()?;
    let _ = admitted.handoff().handoff_digest();
    let _ = admitted.execution_binding().binding_digest();
    let _ = admitted.pending_delivery().effect_name();
    admitted.execute()
}

fn write_common_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    command: ForgeQueryWriteCommand,
) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
    let receipt = runtime.write_intent(command).execute()?;
    let _ = receipt.covered_entrypoint_label();
    let _ = receipt.execution_provenance_chain_digest();
    let _ = receipt
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = receipt
        .consumer_inspection()
        .map(|consumer| consumer.outcome_class());
    Ok(receipt)
}

fn workspace_write_common_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    command: ForgeQueryWriteCommand,
) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
    let receipt = workspace.write_intent(command).execute()?;
    let _ = receipt.covered_entrypoint_label();
    let _ = receipt.execution_provenance_chain_digest();
    Ok(receipt)
}

fn write_advanced_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    command: ForgeQueryWriteCommand,
) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError> {
    let review = runtime.write_intent(command).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review.consumer_inspection().decision_trace_digest();
    let admitted = review.admit()?;
    let _ = admitted.handoff().handoff_digest();
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn write_batch_common_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    commands: Vec<ForgeQueryWriteCommand>,
) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
    let receipt = runtime.write_batch_intent(commands).execute()?;
    let _ = receipt
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = receipt
        .execution_provenance()
        .map(|provenance| provenance.execution_provenance_chain_digest());
    Ok(receipt)
}

fn workspace_write_batch_common_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    commands: Vec<ForgeQueryWriteCommand>,
) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
    let receipt = workspace.write_batch_intent(commands).execute()?;
    let _ = receipt
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = receipt
        .execution_provenance()
        .map(|provenance| provenance.execution_provenance_chain_digest());
    Ok(receipt)
}

fn write_batch_advanced_path_compiles(
    runtime: &mut ForgeQueryRuntime,
    commands: Vec<ForgeQueryWriteCommand>,
) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError> {
    let review = runtime.write_batch_intent(commands).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let admitted = review.admit()?;
    let _ = admitted.handoff().handoff_digest();
    admitted.execute()
}

fn consumer_lane_typecheck(
    receipt: &ForgeQueryIntentReceipt,
) -> ForgeQueryIntentConsumerOutcomeClass {
    receipt.consumer_inspection().outcome_class()
}

fn basis_observation_common_path_compiles() {
    let admitted = forge_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .unwrap()
        .admit()
        .unwrap();
    let _ = admitted.plan().execution_seam();
    let _ = admitted.scope();
}

fn projection_consumption_common_path_compiles(declaration: ProjectionConsumptionDeclaration) {
    let admitted = forge_query_projection_consumption_intent(declaration)
        .unwrap()
        .admit()
        .unwrap();
    let _ = admitted.plan().execution_seam();
    let _ = admitted.bind_contract();
}

fn read_family_common_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    let result = workspace.read_family_intent(family).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance_chain_digest();
    Ok(result)
}

fn read_family_advanced_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    let review = workspace.read_family_intent(family).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn read_family_in_basis_context_common_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
    context: &AdmittedQueryBasisContext,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    let result = workspace
        .read_family_in_basis_context_intent(family, context)
        .execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance_chain_digest();
    Ok(result)
}

fn read_family_in_basis_context_advanced_path_compiles(
    workspace: &mut ForgeQueryWorkspace,
    family: &ForgeQueryReadFamily,
    context: &AdmittedQueryBasisContext,
) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError> {
    let review = workspace
        .read_family_in_basis_context_intent(family, context)
        .review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn live_read_common_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    live_view: &forge_query::facade::ForgeQueryLiveView<T>,
) -> Result<forge_query::facade::ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
    let result = workspace.read_live_intent(live_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

fn live_read_advanced_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    live_view: &forge_query::facade::ForgeQueryLiveView<T>,
) -> Result<forge_query::facade::ForgeQueryLiveReadResult, ForgeQueryRuntimeError> {
    let review = workspace.read_live_intent(live_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn derived_materialization_common_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    derived_view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
) -> Result<forge_query::facade::ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
    let result = workspace.materialize_intent(derived_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

fn derived_materialization_advanced_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    derived_view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
) -> Result<forge_query::facade::ForgeQueryDerivedMaterializationResult, ForgeQueryRuntimeError> {
    let review = workspace.materialize_intent(derived_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn derived_inspection_common_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    derived_view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
) -> Result<forge_query::facade::ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
    let result = workspace.inspect_derived_intent(derived_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

fn derived_inspection_advanced_path_compiles<T>(
    workspace: &mut ForgeQueryWorkspace,
    derived_view: &forge_query::facade::ForgeQueryDerivedViewHandle<T>,
) -> Result<forge_query::facade::ForgeQueryDerivedInspectionResult, ForgeQueryRuntimeError> {
    let review = workspace.inspect_derived_intent(derived_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn generic_inspection_common_path_compiles<T>(
    workspace: &ForgeQueryWorkspace,
    live_view: &forge_query::facade::ForgeQueryLiveView<T>,
) -> Result<forge_query::facade::ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
    let result = workspace.inspect_intent(live_view).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

fn generic_inspection_advanced_path_compiles<T>(
    workspace: &ForgeQueryWorkspace,
    live_view: &forge_query::facade::ForgeQueryLiveView<T>,
) -> Result<forge_query::facade::ForgeQueryUnifiedInspectionResult, ForgeQueryRuntimeError> {
    let review = workspace.inspect_intent(live_view).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn existing_truth_probe_common_path_compiles(
    runtime: &ForgeQueryRuntime,
    request: ForgeQueryExistingTruthProbeRequest,
) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
    let result = runtime.probe_existing_intent(request).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    let _ = result.receipt().consumer_inspection();
    Ok(result)
}

fn workspace_existing_truth_probe_common_path_compiles(
    workspace: &ForgeQueryWorkspace,
    request: ForgeQueryExistingTruthProbeRequest,
) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
    let result = workspace.probe_existing_intent(request).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    Ok(result)
}

fn existing_truth_probe_advanced_path_compiles(
    runtime: &ForgeQueryRuntime,
    request: ForgeQueryExistingTruthProbeRequest,
) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError> {
    let review = runtime.probe_existing_intent(request).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let _ = review
        .admitted_handoff()
        .map(|handoff| handoff.handoff_digest().to_string());
    let admitted = review.admit()?;
    let _ = admitted.execution_binding().binding_digest();
    admitted.execute()
}

fn existing_truth_probe_request_typecheck(
    binding: ForgeQueryExistingTruthTargetBinding,
) -> Result<ForgeQueryExistingTruthProbeRequest, forge_query::facade::ForgeQueryWorkspaceError> {
    ForgeQueryExistingTruthProbeRequest::new(binding, [touch("identity.id")])
}

fn touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    let mut segments = aspect_path.split('.');
    let aspect = segments
        .next()
        .and_then(|segment| AspectKey::new(segment.to_string()))
        .expect("test aspect path aspect should admit");
    let fields = segments
        .map(|segment| {
            FieldKey::new(segment.to_string()).expect("test aspect path field should admit")
        })
        .collect::<Vec<_>>();
    if fields.is_empty() {
        ForgeQueryAspectTouch::aspect(aspect)
    } else {
        ForgeQueryAspectTouch::field_path(
            aspect,
            CanonicalFieldPath::new(fields).expect("test aspect path should have fields"),
        )
    }
}

#[test]
fn public_dx_signatures_are_referenced() {
    let _ = authoritative_common_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            ForgeQueryIntentDeclaration,
        ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError>;
    let _ = authoritative_advanced_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            ForgeQueryIntentDeclaration,
        ) -> Result<ForgeQueryIntentReceipt, ForgeQueryRuntimeError>;
    let _ = effect_common_path_compiles::<()>
        as fn(
            &mut ForgeQueryRuntime,
            &ForgeQueryEffectHandle<()>,
        ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError>;
    let _ = effect_advanced_path_compiles::<()>
        as fn(
            &mut ForgeQueryRuntime,
            &ForgeQueryEffectHandle<()>,
        ) -> Result<ForgeQueryEffectIntentReceipt, ForgeQueryRuntimeError>;
    let _ = write_common_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            ForgeQueryWriteCommand,
        ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError>;
    let _ = workspace_write_common_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            ForgeQueryWriteCommand,
        ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError>;
    let _ = write_advanced_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            ForgeQueryWriteCommand,
        ) -> Result<ForgeQueryWriteReceipt, ForgeQueryRuntimeError>;
    let _ = write_batch_common_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            Vec<ForgeQueryWriteCommand>,
        ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError>;
    let _ = workspace_write_batch_common_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            Vec<ForgeQueryWriteCommand>,
        ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError>;
    let _ = write_batch_advanced_path_compiles
        as fn(
            &mut ForgeQueryRuntime,
            Vec<ForgeQueryWriteCommand>,
        ) -> Result<ForgeQueryBatchWriteReceipt, ForgeQueryRuntimeError>;
    let _ = consumer_lane_typecheck
        as fn(&ForgeQueryIntentReceipt) -> ForgeQueryIntentConsumerOutcomeClass;
    let _ = basis_observation_common_path_compiles as fn();
    let _ = projection_consumption_common_path_compiles as fn(ProjectionConsumptionDeclaration);
    let _ = read_family_common_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            &ForgeQueryReadFamily,
        ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError>;
    let _ = read_family_advanced_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            &ForgeQueryReadFamily,
        ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError>;
    let _ = read_family_in_basis_context_common_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            &ForgeQueryReadFamily,
            &AdmittedQueryBasisContext,
        ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError>;
    let _ = read_family_in_basis_context_advanced_path_compiles
        as fn(
            &mut ForgeQueryWorkspace,
            &ForgeQueryReadFamily,
            &AdmittedQueryBasisContext,
        ) -> Result<ForgeQueryReadResult, ForgeQueryRuntimeError>;
    let _ = live_read_common_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryLiveView<ForgeQueryNativeRow>,
        )
            -> Result<forge_query::facade::ForgeQueryLiveReadResult, ForgeQueryRuntimeError>;
    let _ = live_read_advanced_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryLiveView<ForgeQueryNativeRow>,
        )
            -> Result<forge_query::facade::ForgeQueryLiveReadResult, ForgeQueryRuntimeError>;
    let _ = derived_materialization_common_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryDerivedMaterializationResult,
            ForgeQueryRuntimeError,
        >;
    let _ = derived_materialization_advanced_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryDerivedMaterializationResult,
            ForgeQueryRuntimeError,
        >;
    let _ = derived_inspection_common_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryDerivedInspectionResult,
            ForgeQueryRuntimeError,
        >;
    let _ = derived_inspection_advanced_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &mut ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryDerivedInspectionResult,
            ForgeQueryRuntimeError,
        >;
    let _ = generic_inspection_common_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryLiveView<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryUnifiedInspectionResult,
            ForgeQueryRuntimeError,
        >;
    let _ = generic_inspection_advanced_path_compiles::<ForgeQueryNativeRow>
        as fn(
            &ForgeQueryWorkspace,
            &forge_query::facade::ForgeQueryLiveView<ForgeQueryNativeRow>,
        ) -> Result<
            forge_query::facade::ForgeQueryUnifiedInspectionResult,
            ForgeQueryRuntimeError,
        >;
    let _ = existing_truth_probe_common_path_compiles
        as fn(
            &ForgeQueryRuntime,
            ForgeQueryExistingTruthProbeRequest,
        ) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError>;
    let _ = workspace_existing_truth_probe_common_path_compiles
        as fn(
            &ForgeQueryWorkspace,
            ForgeQueryExistingTruthProbeRequest,
        ) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError>;
    let _ = existing_truth_probe_advanced_path_compiles
        as fn(
            &ForgeQueryRuntime,
            ForgeQueryExistingTruthProbeRequest,
        ) -> Result<ForgeQueryExistingTruthProbeResult, ForgeQueryRuntimeError>;
    let _ = existing_truth_probe_request_typecheck
        as fn(
            ForgeQueryExistingTruthTargetBinding,
        ) -> Result<
            ForgeQueryExistingTruthProbeRequest,
            forge_query::facade::ForgeQueryWorkspaceError,
        >;
}
