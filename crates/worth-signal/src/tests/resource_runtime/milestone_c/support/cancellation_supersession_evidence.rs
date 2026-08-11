use super::super::*;

pub(super) struct ResourceMilestoneCCancellationSupersessionEvidence {
    pub(super) cancellation_report: ResourceCancellationReport,
    pub(super) overlap_admission: ResourceOverlappingGenerationAdmission,
    pub(super) intent_coalescing: ResourceIntentEquivalenceCoalescing,
}

pub(super) fn resource_milestone_c_cancellation_supersession_evidence(
) -> ResourceMilestoneCCancellationSupersessionEvidence {
    let mut cancellation_graph = SignalGraph::new();
    let cancel_node = cancellation_graph.node().build();
    let overlap_node = cancellation_graph.node().build();
    let coalesce_node = cancellation_graph.node().build();
    let mut cancellation_runtime = TestRuntime::build(cancellation_graph);
    cancellation_runtime
        .declare_resource_node(resource_declaration(cancel_node))
        .expect("cancellation declaration should lower");
    cancellation_runtime
        .declare_resource_node(overlap_cancelled_host_work_resource_declaration(
            overlap_node,
        ))
        .expect("overlap declaration should lower");
    cancellation_runtime
        .declare_resource_node(intent_equivalent_coalescing_resource_declaration(
            coalesce_node,
        ))
        .expect("coalescing declaration should lower");
    let cancelled_request = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            cancel_node,
        )))
        .expect("cancel request should admit")
        .admitted_request();
    let cancellation_report = cancellation_runtime
        .cancel_resource_request(
            cancelled_request.handle(),
            ResourceCancellationReason::HostRequested,
        )
        .expect("cancellation should admit");
    let _first_overlap = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            overlap_node,
        )))
        .expect("first overlap request should admit");
    let second_overlap = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            overlap_node,
        )))
        .expect("second overlap request should admit");
    let overlap_admission = second_overlap
        .supersession_record()
        .and_then(|record| record.overlap_admission().cloned())
        .expect("overlap policy should retain overlap admission evidence");
    let _first_coalesced = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            coalesce_node,
        )))
        .expect("first coalescing request should admit");
    let second_coalesced = cancellation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            coalesce_node,
        )))
        .expect("second coalescing request should coalesce");
    let intent_coalescing = second_coalesced
        .intent_equivalence_coalescing()
        .expect("coalescing policy should retain lineage evidence");

    ResourceMilestoneCCancellationSupersessionEvidence {
        cancellation_report,
        overlap_admission,
        intent_coalescing,
    }
}
