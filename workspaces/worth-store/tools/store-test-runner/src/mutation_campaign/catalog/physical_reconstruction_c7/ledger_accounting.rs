use super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[ControlledMutation {
    id: 120,
    predicate: "phase-ten-ledger-current-accounting-drift-accepted",
    source: "tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/current_accounting.rs",
    needle: "        if !row.current_evidence.contains(&claim.clause) {",
    replacement: "        if false {",
    package: "store-test-runner",
    target: MutationTarget::Library,
    selector: "durable_publication_boundary_gate::closure_ledger::current_accounting::current_ledger_accounting_rejects_each_independent_fact_drift",
}];
