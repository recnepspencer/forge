use topology::derived_invalidation_migrated_products::{
    MaterializedGraphReadEntityRow, MaterializedGraphReadRelationRow,
    MaterializedGraphReadStageReceipt,
};

fn main() {
    let _ = MaterializedGraphReadStageReceipt {
        selected_plan_digest: String::new(),
        touched_closure_digest: String::new(),
        query_support_digest: String::new(),
        legality_support_digest: String::new(),
        materialized_graph_selected_row_digest: String::new(),
        native_query_read_receipt_digest: String::new(),
        selected_legality_receipt_digest: String::new(),
        read_source_digest: String::new(),
        selected_entity_count: 0,
        selected_relation_count: 0,
        available_entity_count: 0,
        available_relation_count: 0,
        topology_entity_count: 0,
        topology_relation_count: 0,
        selected_entity_rows: fake_entity_rows(),
        selected_relation_rows: fake_relation_rows(),
        receipt_digest: String::new(),
    };
}

fn fake_entity_rows() -> Vec<MaterializedGraphReadEntityRow> {
    panic!("compile-fail fixture does not execute")
}

fn fake_relation_rows() -> Vec<MaterializedGraphReadRelationRow> {
    panic!("compile-fail fixture does not execute")
}
