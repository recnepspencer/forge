use super::*;

#[test]
fn execute_read_family_delegates_to_canonical_admission_and_execution_handoff() {
    let runtime = read_runtime();
    let mut delegated_workspace =
        ForgeQueryWorkspace::new("delegated-read-family", runtime).expect("workspace should build");
    let family = identity_read_family(&mut delegated_workspace, "tasks");
    let delegated = delegated_workspace
        .execute_read_family(&family)
        .expect("delegated read family should execute");

    let runtime = read_runtime();
    let mut canonical_workspace =
        ForgeQueryWorkspace::new("canonical-read-family", runtime).expect("workspace should build");
    let canonical_family = identity_read_family(&mut canonical_workspace, "tasks");
    let review = canonical_workspace
        .review_read_execution(canonical_family, None)
        .expect("canonical read review should succeed");
    let handoff = canonical_workspace
        .resolve_reviewed_admitted_read_execution_handoff(review)
        .expect("canonical read handoff should admit");
    let binding = canonical_workspace
        .into_runtime_read_execution_binding(handoff)
        .expect("canonical read binding should prepare");
    let canonical = canonical_workspace
        .execute_bound_read_execution(binding)
        .expect("canonical read binding should execute");

    assert_eq!(delegated.rows(), canonical.rows());
    assert_eq!(
        delegated.receipt().query_digest(),
        canonical.receipt().query_digest()
    );
    assert_eq!(
        delegated.receipt().result_digest(),
        canonical.receipt().result_digest()
    );
    assert_eq!(
        delegated.receipt().execution_provenance_chain_digest(),
        canonical.receipt().execution_provenance_chain_digest()
    );
    assert_eq!(
        delegated
            .receipt()
            .decision_trace_envelope()
            .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest),
        canonical
            .receipt()
            .decision_trace_envelope()
            .map(ForgeQueryIntentDecisionTraceEnvelope::trace_digest)
    );
}

#[test]
fn workspace_read_delegates_to_live_read_intent_execution() {
    let runtime = read_runtime();
    let mut delegated_workspace =
        ForgeQueryWorkspace::new("delegated-live-read", runtime).expect("workspace should build");
    let delegated_view: ForgeQueryLiveView<ForgeQueryNativeRow> = delegated_workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("intent-admission-live-read")
        })
        .expect("live view should declare");
    let delegated = delegated_workspace.read(&delegated_view);

    let runtime = read_runtime();
    let mut canonical_workspace =
        ForgeQueryWorkspace::new("canonical-live-read", runtime).expect("workspace should build");
    let canonical_view: ForgeQueryLiveView<ForgeQueryNativeRow> = canonical_workspace
        .live_view("tasks.table", |q| {
            q.from("Task")
                .select([
                    crate::authoring::AspectFieldKey::new("identity", "id").unwrap(),
                    crate::authoring::AspectFieldKey::new("title", "value").unwrap(),
                ])
                .order_by(crate::authoring::AspectFieldKey::new("title", "value").unwrap())
                .schema_basis("intent-admission-live-read")
        })
        .expect("live view should declare");
    let canonical = canonical_workspace
        .read_live_intent(&canonical_view)
        .execute()
        .expect("live read common path should execute");

    assert_eq!(delegated, canonical.rows());
    assert_eq!(canonical.receipt().view_name(), "tasks.table");
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
    );
    assert_eq!(
        canonical
            .receipt()
            .decision_trace_envelope()
            .map(trace_stages),
        Some(vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ])
    );
}

#[test]
fn runtime_read_live_delegates_to_canonical_live_read_execution() {
    let mut runtime = read_runtime();
    let live_view: ForgeQueryLiveView<ForgeQueryNativeRow> = runtime
        .declare_live_view::<ForgeQueryNativeRow>("tasks.table", task_live_request(), task_schema())
        .expect("live view should declare");

    let delegated = runtime.read_live(&live_view);
    let canonical = runtime
        .read_live_result(&live_view)
        .expect("canonical runtime live read should execute");

    assert_eq!(delegated, canonical.rows());
    assert_eq!(canonical.receipt().view_name(), "tasks.table");
    assert_eq!(
        canonical
            .receipt()
            .execution_provenance()
            .map(|provenance| provenance.entrypoint()),
        Some(ForgeQueryIntentAdmissionCoveredEntrypoint::ExecuteLiveRead)
    );
    assert_eq!(
        canonical
            .receipt()
            .decision_trace_envelope()
            .map(trace_stages),
        Some(vec![
            ForgeQueryIntentDecisionTraceStage::RawIntent,
            ForgeQueryIntentDecisionTraceStage::Eligibility,
            ForgeQueryIntentDecisionTraceStage::AdmittedDecision,
            ForgeQueryIntentDecisionTraceStage::ExecutionHandoff,
            ForgeQueryIntentDecisionTraceStage::ExecutionOutcome,
        ])
    );
}
