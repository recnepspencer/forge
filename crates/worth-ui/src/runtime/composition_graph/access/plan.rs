use crate::runtime::WorthUiRuntimeGraphAuthority;

use super::super::WorthUiAdmittedCompositionGraphReceipt;
use super::consumed_facts::consumed_facts_for_request;
use super::denial::WorthUiCompositionGraphAccessReport;
use super::indexes::WorthUiCompositionGraphIndexes;
use super::planned_counters::counters_for_request;
use super::receipt::{
    WorthUiCompositionGraphAccessPlanReceipt, WorthUiCompositionGraphAccessReceipt,
};
use super::request::WorthUiCompositionGraphAccessRequest;
use super::request_validation::validate_access_request;

pub fn admit_composition_graph_access(
    graph: &WorthUiAdmittedCompositionGraphReceipt,
    request: WorthUiCompositionGraphAccessRequest,
) -> Result<WorthUiCompositionGraphAccessReceipt, WorthUiCompositionGraphAccessReport> {
    let indexes = WorthUiCompositionGraphIndexes::from_graph(graph);
    let denials = validate_access_request(graph, &indexes, &request);
    if !denials.is_empty() {
        return Err(WorthUiCompositionGraphAccessReport::from_denials(denials));
    }
    let consumed_facts = consumed_facts_for_request(graph, &indexes, &request);
    let counters = counters_for_request(graph, &indexes, &request);
    let query_graph_execution = WorthUiRuntimeGraphAuthority::new()
        .plan_composition_graph_access_operation(
            graph.root().root_id().as_str(),
            request.token(),
            consumed_facts.clone(),
        )
        .into_execution_receipt();
    let plan = WorthUiCompositionGraphAccessPlanReceipt::new(
        graph,
        request,
        consumed_facts,
        query_graph_execution,
        counters,
    );
    Ok(plan.execute(graph, indexes))
}
