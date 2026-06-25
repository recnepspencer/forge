use worth_ui::facade::{
    WorthUiCompositionParticipationCounters, WorthUiCompositionParticipationReceipt,
};

fn main() {
    let _receipt = WorthUiCompositionParticipationReceipt {
        root_id: "root".to_owned(),
        accessibility_nodes: Vec::new(),
        focus_nodes: Vec::new(),
        associations: Vec::new(),
        consumed_facts: Vec::new(),
        query_graph_execution: panic!("query graph execution is runtime-admitted"),
        counters: WorthUiCompositionParticipationCounters::default(),
        receipt_digest: 0,
    };
}
