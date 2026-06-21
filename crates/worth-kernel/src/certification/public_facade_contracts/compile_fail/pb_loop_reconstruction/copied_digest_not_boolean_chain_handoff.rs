use worth_kernel::workload_composition::BooleanChainIntegrationHandoff;

fn require_boolean_chain(_: &BooleanChainIntegrationHandoff) {}

fn main() {
    let copied_digest = String::from("copied-loop-ledger-digest");
    require_boolean_chain(&copied_digest);
}
