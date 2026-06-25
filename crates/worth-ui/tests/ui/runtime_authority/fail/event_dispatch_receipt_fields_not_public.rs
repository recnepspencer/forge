use worth_ui::facade::{
    WorthUiPrimitiveEventDispatchCounters, WorthUiPrimitiveEventDispatchOutcome,
    WorthUiPrimitiveEventDispatchReceipt, WorthUiQueryGraphExecutionReceipt,
};

fn main() {
    let _forged = WorthUiPrimitiveEventDispatchReceipt {
        outcome: outcome(),
        candidates: Vec::new(),
        counters: counters(),
        query_graph_execution: query_graph_execution(),
        dispatch_digest: 1,
    };
}

fn outcome() -> WorthUiPrimitiveEventDispatchOutcome {
    panic!("fixture only checks event dispatch field privacy")
}

fn counters() -> WorthUiPrimitiveEventDispatchCounters {
    panic!("fixture only checks event dispatch field privacy")
}

fn query_graph_execution() -> WorthUiQueryGraphExecutionReceipt {
    panic!("fixture only checks event dispatch field privacy")
}
