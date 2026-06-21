use worth_kernel::query_graph_authority_gate::{
    certify_worth_graph_authority_gate, current_worth_graph_authority_deletion_ledger,
    current_worth_graph_authority_discovery_records, current_worth_graph_authority_inventory,
    current_worth_lower_authority_promotion_guard_plan,
};

fn main() {
    let _ = certify_worth_graph_authority_gate(
        current_worth_graph_authority_inventory(),
        current_worth_graph_authority_deletion_ledger(),
        current_worth_graph_authority_discovery_records(),
        current_worth_lower_authority_promotion_guard_plan(),
        &[],
    );
}
