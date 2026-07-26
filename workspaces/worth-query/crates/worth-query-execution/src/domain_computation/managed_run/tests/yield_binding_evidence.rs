use super::yield_fixture::YieldProvider;
use super::*;

#[test]
fn direct_yield_exposes_one_cross_owner_authority_chain() {
    let (running, graph) = managed_graph_run_with_provider(
        WorthQueryOperationGraphAccess::Observe,
        YieldProvider::installed(5),
    );
    let active = running
        .begin_graph_execution(
            &graph,
            WorthQueryManagedGraphCallRequest::new(
                WorthQueryGraphProviderCallKind::Observe,
                "direct-yield-binding-evidence",
            ),
        )
        .expect("yield evidence provider should begin");
    let paused = match active.advance() {
        WorthQueryDirectGraphStepOutcome::Continue(paused) => paused,
        _ => panic!("yield evidence provider did not pause"),
    };
    let yielded = match paused.yield_run() {
        crate::domain_computation::WorthQueryDirectYieldOutcome::Yielded(yielded) => yielded,
        _ => panic!("eligible evidence run did not yield"),
    };

    assert!(!yielded.operation_binding_identity().is_empty());
    assert!(!yielded.installed_operation_identity().is_empty());
    assert!(!yielded.semantic_basis_identity().is_empty());
    assert_eq!(
        yielded.installation_generation(),
        worth_query_installation::facade::WorthQueryInstallationGeneration::initial()
    );
    assert_eq!(
        yielded
            .resource_attempt_evidence()
            .provider_session_identity(),
        yielded.provider_work().provider_session_identity()
    );
    assert!(!yielded
        .resource_attempt_evidence()
        .envelope_identity()
        .is_empty());
    assert!(yielded.relational_basis_identity().runtime_instance_id() > 0);
    assert!(yielded.relational_basis_identity().lease_ordinal() > 0);
    super::cost_bound::assert_exact_admission_work(yielded.run_counters());

    let cleanup = complete_direct_yield_cleanup(yielded);
    super::cost_bound::assert_exact_admission_work(cleanup.run_counters());
    assert_eq!(
        cleanup
            .checkpoint_release()
            .expect("yielded cleanup carries checkpoint release")
            .disposition(),
        crate::domain_computation::WorthQueryProviderCheckpointReleaseDisposition::Released
    );
}
