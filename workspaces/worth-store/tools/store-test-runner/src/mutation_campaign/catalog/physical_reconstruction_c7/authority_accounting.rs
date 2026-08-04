use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[ControlledMutation {
    id: 128,
    predicate: "phase-ten-handoff-step-accounting-substitution-accepted",
    source: "tools/store-test-runner/src/durable_publication_boundary_gate/authority_trace.rs",
    needle: "        c8_recovery_handoff_steps: C8_RECOVERY_HANDOFF_STEPS.len(),",
    replacement: "        c8_recovery_handoff_steps: WAL_INVENTORY_REOPEN_STEPS.len(),",
    package: "store-test-runner",
    target: MutationTarget::Library,
    selector: "durable_publication_boundary_gate::closure_ledger::current_accounting::current_ledger_accounting_distinguishes_handoff_from_wal_reopen",
}];
