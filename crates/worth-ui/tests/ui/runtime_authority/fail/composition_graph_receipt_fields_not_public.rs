use worth_ui::facade::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionGraphCounters,
};

fn main() {
    let _counters = WorthUiCompositionGraphCounters {
        node_count: 0,
        edge_count: 0,
        policy_attachment_count: 0,
        selected_graph_obligation_count: 0,
        source_reparse_count: 0,
        renderer_parse_count: 0,
    };

    let _receipt = WorthUiAdmittedCompositionGraphReceipt {
        root: todo!(),
        nodes: Vec::new(),
        edges: Vec::new(),
        policy_attachments: Vec::new(),
        consumed_facts: Vec::new(),
        query_graph_execution: todo!(),
        counters: _counters,
        receipt_digest: 0,
    };
}
