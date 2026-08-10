use super::super::*;

pub(super) struct ResourceMilestoneCRevalidationEvidence {
    pub(super) revalidation_report: ResourceRevalidationReport,
}

pub(super) fn resource_milestone_c_revalidation_evidence() -> ResourceMilestoneCRevalidationEvidence
{
    let mut revalidation_graph = SignalGraph::new();
    let revalidation_node = revalidation_graph.node().build();
    let mut revalidation_runtime = TestRuntime::build(revalidation_graph);
    revalidation_runtime
        .declare_resource_node(resource_declaration(revalidation_node))
        .expect("revalidation declaration should lower");
    let revalidation_report = revalidation_runtime
        .revalidate_resource_node(ResourceRevalidationIntent::new(ResourceNodeId::from_node(
            revalidation_node,
        )))
        .expect("explicit revalidation should admit");

    ResourceMilestoneCRevalidationEvidence {
        revalidation_report,
    }
}
