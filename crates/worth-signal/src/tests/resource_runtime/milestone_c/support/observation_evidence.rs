use super::super::*;

pub(super) struct ResourceMilestoneCObservationEvidence {
    pub(super) observation_report: ResourceObservationBatchReport,
}

pub(super) fn resource_milestone_c_observation_evidence() -> ResourceMilestoneCObservationEvidence {
    let mut observation_graph = SignalGraph::new();
    let observation_node = observation_graph.node().build();
    let mut observation_runtime = TestRuntime::build(observation_graph);
    observation_runtime
        .declare_resource_node(resource_declaration(observation_node))
        .expect("observation declaration should lower");
    let observation_request = observation_runtime
        .admit_resource_request(ResourceRequestIntent::new(ResourceNodeId::from_node(
            observation_node,
        )))
        .expect("observation request should admit")
        .admitted_request();
    let observation_completion = observation_runtime
        .admit_resource_completion(raw_completion(
            &observation_runtime,
            observation_node,
            observation_request.handle(),
            observation_request.attempt(),
            64,
        ))
        .admitted_completion()
        .expect("observation completion should admit");
    let calls = Arc::new(Mutex::new(Vec::<ResourceObservationRecord>::new()));
    observation_runtime.observe_nodes(
        ObservationPolicy::meaningful_change(),
        [observation_node],
        Box::new(ResourceObservationListener {
            calls: Arc::clone(&calls),
        }),
    );
    let mut ctx = ();
    observation_runtime
        .transaction(&mut ctx, |tx| {
            let staged = tx.stage_admitted_resource_completion(observation_completion)?;
            tx.commit_staged_resource_completion(staged.staged_effect())?;
            Ok(())
        })
        .expect("observation completion should commit");
    let observation_report = observation_runtime
        .latest_resource_observation_batch_report()
        .expect("observation batch report should materialize");

    ResourceMilestoneCObservationEvidence { observation_report }
}
