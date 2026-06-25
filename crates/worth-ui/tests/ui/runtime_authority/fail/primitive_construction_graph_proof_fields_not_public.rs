use worth_ui::facade::{
    WorthUiPrimitiveConstructionGraphProof, WorthUiPrimitiveGraphCounters,
    WorthUiPrimitiveQueryPosture, WorthUiQueryGraphExecutionReceipt,
    WorthUiValidatedProjectionDependencyContract,
};

fn main() {
    let _graph_proof = WorthUiPrimitiveConstructionGraphProof {
        surface_id: "worth.surface.preview.primitive.proof".to_owned(),
        component_id: "worth.component.primitive_proof".to_owned(),
        dependency_contract: dependency_contract(),
        published_facts: Vec::new(),
        obligation_rows: Vec::new(),
        query_posture: query_posture(),
        query_graph_execution: query_graph_execution(),
        counters: graph_counters(),
        graph_proof_digest: 1,
    };
    panic!("compile-fail fixture only checks primitive proof field privacy");
}

fn dependency_contract() -> WorthUiValidatedProjectionDependencyContract {
    panic!("fixture only checks primitive proof field privacy")
}

fn query_posture() -> WorthUiPrimitiveQueryPosture {
    panic!("fixture only checks primitive proof field privacy")
}

fn query_graph_execution() -> WorthUiQueryGraphExecutionReceipt {
    panic!("fixture only checks primitive proof field privacy")
}

fn graph_counters() -> WorthUiPrimitiveGraphCounters {
    panic!("fixture only checks primitive proof field privacy")
}
