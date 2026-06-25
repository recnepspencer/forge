use super::capability_catalog::{
    anchor_query_graph_read_access_symbols, query_graph_read_access_capability_rows,
};
use super::capability_report::QueryGraphReadAccessCapabilityReport;
use super::cost_counters::anchor_query_graph_read_cost_counter_accessors;
use super::receipt_fields::anchor_query_read_receipt_accessors;

pub fn current_query_graph_read_access_capabilities() -> QueryGraphReadAccessCapabilityReport {
    anchor_query_graph_read_access_symbols();
    anchor_query_read_receipt_accessors();
    anchor_query_graph_read_cost_counter_accessors();

    QueryGraphReadAccessCapabilityReport::new(query_graph_read_access_capability_rows())
}
