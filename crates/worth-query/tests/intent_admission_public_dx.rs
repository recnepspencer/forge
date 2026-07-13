use worth_query::facade::{
    worth_query_basis_observation_intent, AdmittedQueryBasisContext, RawBasisIntent,
    WorthQueryBatchWriteReceipt, WorthQueryEffectHandle, WorthQueryEffectIntentReceipt,
    WorthQueryExistingTruthProbeRequest, WorthQueryExistingTruthProbeResult,
    WorthQueryExistingTruthTargetBinding, WorthQueryIntentAdmissionDecision,
    WorthQueryIntentConsumerOutcomeClass, WorthQueryIntentDeclaration, WorthQueryIntentReceipt,
    WorthQueryNativeRow, WorthQueryReadFamily, WorthQueryReadResult, WorthQueryRuntime,
    WorthQueryRuntimeError, WorthQueryWorkspace, WorthQueryWriteCommand, WorthQueryWriteReceipt,
};

mod support;

use support::aspect_touch as touch;

fn authoritative_common_path_compiles(
    runtime: &mut WorthQueryRuntime,
    declaration: WorthQueryIntentDeclaration,
) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
    runtime.intent(declaration).execute()
}

fn authoritative_advanced_path_compiles(
    runtime: &mut WorthQueryRuntime,
    declaration: WorthQueryIntentDeclaration,
) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError> {
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
        WorthQueryIntentAdmissionDecision::Admitted(_) => {}
        WorthQueryIntentAdmissionDecision::Advisory(_) => {}
        WorthQueryIntentAdmissionDecision::Violation(_) => {}
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
    runtime: &mut WorthQueryRuntime,
    effect: &WorthQueryEffectHandle<T>,
) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
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
    runtime: &mut WorthQueryRuntime,
    effect: &WorthQueryEffectHandle<T>,
) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError> {
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
    runtime: &mut WorthQueryRuntime,
    command: WorthQueryWriteCommand,
) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    command: WorthQueryWriteCommand,
) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
    let receipt = workspace.write_intent(command).execute()?;
    let _ = receipt.covered_entrypoint_label();
    let _ = receipt.execution_provenance_chain_digest();
    Ok(receipt)
}

fn write_advanced_path_compiles(
    runtime: &mut WorthQueryRuntime,
    command: WorthQueryWriteCommand,
) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError> {
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
    runtime: &mut WorthQueryRuntime,
    commands: Vec<WorthQueryWriteCommand>,
) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    commands: Vec<WorthQueryWriteCommand>,
) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
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
    runtime: &mut WorthQueryRuntime,
    commands: Vec<WorthQueryWriteCommand>,
) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError> {
    let review = runtime.write_batch_intent(commands).review()?;
    let _ = review.request().request_digest();
    let _ = review.eligibility().eligibility_digest();
    let _ = review.decision();
    let admitted = review.admit()?;
    let _ = admitted.handoff().handoff_digest();
    admitted.execute()
}

fn consumer_lane_typecheck(
    receipt: &WorthQueryIntentReceipt,
) -> WorthQueryIntentConsumerOutcomeClass {
    receipt.consumer_inspection().outcome_class()
}

fn basis_observation_common_path_compiles() {
    let admitted = worth_query_basis_observation_intent(RawBasisIntent::CurrentHead)
        .unwrap()
        .admit()
        .unwrap();
    let _ = admitted.plan().execution_seam();
    let _ = admitted.scope();
}

fn read_family_common_path_compiles(
    workspace: &mut WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
    let result = workspace.read_family_intent(family).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance_chain_digest();
    Ok(result)
}

fn read_family_advanced_path_compiles(
    workspace: &mut WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
    context: &AdmittedQueryBasisContext,
) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    family: &WorthQueryReadFamily,
    context: &AdmittedQueryBasisContext,
) -> Result<WorthQueryReadResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    live_view: &worth_query::facade::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::WorthQueryLiveReadResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    live_view: &worth_query::facade::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::WorthQueryLiveReadResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::WorthQueryDerivedMaterializationResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::WorthQueryDerivedMaterializationResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::WorthQueryDerivedInspectionResult, WorthQueryRuntimeError> {
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
    workspace: &mut WorthQueryWorkspace,
    derived_view: &worth_query::facade::WorthQueryDerivedViewHandle<T>,
) -> Result<worth_query::facade::WorthQueryDerivedInspectionResult, WorthQueryRuntimeError> {
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
    workspace: &WorthQueryWorkspace,
    live_view: &worth_query::facade::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
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
    workspace: &WorthQueryWorkspace,
    live_view: &worth_query::facade::WorthQueryLiveView<T>,
) -> Result<worth_query::facade::WorthQueryUnifiedInspectionResult, WorthQueryRuntimeError> {
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
    runtime: &WorthQueryRuntime,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
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
    workspace: &WorthQueryWorkspace,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
    let result = workspace.probe_existing_intent(request).execute()?;
    let _ = result
        .receipt()
        .decision_trace_envelope()
        .map(|trace| trace.trace_digest());
    let _ = result.receipt().execution_provenance();
    Ok(result)
}

fn existing_truth_probe_advanced_path_compiles(
    runtime: &WorthQueryRuntime,
    request: WorthQueryExistingTruthProbeRequest,
) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError> {
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
    binding: WorthQueryExistingTruthTargetBinding,
) -> Result<WorthQueryExistingTruthProbeRequest, worth_query::facade::WorthQueryWorkspaceError> {
    WorthQueryExistingTruthProbeRequest::new(binding, [touch("identity.id")])
}

#[test]
fn public_dx_signatures_are_referenced() {
    let _ = authoritative_common_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            WorthQueryIntentDeclaration,
        ) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError>;
    let _ = authoritative_advanced_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            WorthQueryIntentDeclaration,
        ) -> Result<WorthQueryIntentReceipt, WorthQueryRuntimeError>;
    let _ = effect_common_path_compiles::<()>
        as fn(
            &mut WorthQueryRuntime,
            &WorthQueryEffectHandle<()>,
        ) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError>;
    let _ = effect_advanced_path_compiles::<()>
        as fn(
            &mut WorthQueryRuntime,
            &WorthQueryEffectHandle<()>,
        ) -> Result<WorthQueryEffectIntentReceipt, WorthQueryRuntimeError>;
    let _ = write_common_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            WorthQueryWriteCommand,
        ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError>;
    let _ = workspace_write_common_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            WorthQueryWriteCommand,
        ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError>;
    let _ = write_advanced_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            WorthQueryWriteCommand,
        ) -> Result<WorthQueryWriteReceipt, WorthQueryRuntimeError>;
    let _ = write_batch_common_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            Vec<WorthQueryWriteCommand>,
        ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError>;
    let _ = workspace_write_batch_common_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            Vec<WorthQueryWriteCommand>,
        ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError>;
    let _ = write_batch_advanced_path_compiles
        as fn(
            &mut WorthQueryRuntime,
            Vec<WorthQueryWriteCommand>,
        ) -> Result<WorthQueryBatchWriteReceipt, WorthQueryRuntimeError>;
    let _ = consumer_lane_typecheck
        as fn(&WorthQueryIntentReceipt) -> WorthQueryIntentConsumerOutcomeClass;
    let _ = basis_observation_common_path_compiles as fn();
    let _ = read_family_common_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            &WorthQueryReadFamily,
        ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError>;
    let _ = read_family_advanced_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            &WorthQueryReadFamily,
        ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError>;
    let _ = read_family_in_basis_context_common_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            &WorthQueryReadFamily,
            &AdmittedQueryBasisContext,
        ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError>;
    let _ = read_family_in_basis_context_advanced_path_compiles
        as fn(
            &mut WorthQueryWorkspace,
            &WorthQueryReadFamily,
            &AdmittedQueryBasisContext,
        ) -> Result<WorthQueryReadResult, WorthQueryRuntimeError>;
    let _ = live_read_common_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryLiveView<WorthQueryNativeRow>,
        )
            -> Result<worth_query::facade::WorthQueryLiveReadResult, WorthQueryRuntimeError>;
    let _ = live_read_advanced_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryLiveView<WorthQueryNativeRow>,
        )
            -> Result<worth_query::facade::WorthQueryLiveReadResult, WorthQueryRuntimeError>;
    let _ = derived_materialization_common_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryDerivedViewHandle<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryDerivedMaterializationResult,
            WorthQueryRuntimeError,
        >;
    let _ = derived_materialization_advanced_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryDerivedViewHandle<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryDerivedMaterializationResult,
            WorthQueryRuntimeError,
        >;
    let _ = derived_inspection_common_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryDerivedViewHandle<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryDerivedInspectionResult,
            WorthQueryRuntimeError,
        >;
    let _ = derived_inspection_advanced_path_compiles::<WorthQueryNativeRow>
        as fn(
            &mut WorthQueryWorkspace,
            &worth_query::facade::WorthQueryDerivedViewHandle<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryDerivedInspectionResult,
            WorthQueryRuntimeError,
        >;
    let _ = generic_inspection_common_path_compiles::<WorthQueryNativeRow>
        as fn(
            &WorthQueryWorkspace,
            &worth_query::facade::WorthQueryLiveView<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryUnifiedInspectionResult,
            WorthQueryRuntimeError,
        >;
    let _ = generic_inspection_advanced_path_compiles::<WorthQueryNativeRow>
        as fn(
            &WorthQueryWorkspace,
            &worth_query::facade::WorthQueryLiveView<WorthQueryNativeRow>,
        ) -> Result<
            worth_query::facade::WorthQueryUnifiedInspectionResult,
            WorthQueryRuntimeError,
        >;
    let _ = existing_truth_probe_common_path_compiles
        as fn(
            &WorthQueryRuntime,
            WorthQueryExistingTruthProbeRequest,
        ) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError>;
    let _ = workspace_existing_truth_probe_common_path_compiles
        as fn(
            &WorthQueryWorkspace,
            WorthQueryExistingTruthProbeRequest,
        ) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError>;
    let _ = existing_truth_probe_advanced_path_compiles
        as fn(
            &WorthQueryRuntime,
            WorthQueryExistingTruthProbeRequest,
        ) -> Result<WorthQueryExistingTruthProbeResult, WorthQueryRuntimeError>;
    let _ = existing_truth_probe_request_typecheck
        as fn(
            WorthQueryExistingTruthTargetBinding,
        ) -> Result<
            WorthQueryExistingTruthProbeRequest,
            worth_query::facade::WorthQueryWorkspaceError,
        >;
}
