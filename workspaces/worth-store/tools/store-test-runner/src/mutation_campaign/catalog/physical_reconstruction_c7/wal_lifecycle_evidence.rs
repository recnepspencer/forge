use super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 121,
        predicate: "wal-segment-lifecycle-source-counterfeit-accepted",
        source: "crates/worth-store/src/physical_runtime/durability/wal/inventory/reopen.rs",
        needle: "    let names = tree\n        .list_file_names_bounded(&directory, inventory_limit)\n        .map_err(map_listing_failure)?;",
        replacement: "    // let names = tree.list_file_names_bounded(&directory, inventory_limit)?;\n    let names: Vec<String> = Vec::new();",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::contract::wal_segment_lifecycle::wal_segment_lifecycle_resolves_through_real_rust_syntax",
    },
    ControlledMutation {
        id: 122,
        predicate: "wal-segment-lifecycle-stale-reopen-owner-accepted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_segment_lifecycle/reopen.rs",
        needle: "        \"call:inspect\",",
        replacement: "        \"call:inspect_verified_wal_segment\",",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::contract::wal_segment_lifecycle::delegated_reopen_verification_is_the_current_semantic_owner",
    },
    ControlledMutation {
        id: 123,
        predicate: "wal-source-semantic-order-inverted-accepted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_source_syntax.rs",
        needle: "        visit::visit_expr_method_call(self, call);\n        self.steps.push(format!(\"method:{}\", call.method));",
        replacement: "        self.steps.push(format!(\"method:{}\", call.method));\n        visit::visit_expr_method_call(self, call);",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::contract::wal_source_syntax::tests::nested_calls_follow_rust_evaluation_order",
    },
    ControlledMutation {
        id: 124,
        predicate: "phase-six-wal-evidence-source-closure-omission-accepted",
        source: "tools/store-test-runner/src/durable_publication_boundary_gate/closure_ledger/source_identity.rs",
        needle: "    \"workspaces/worth-store/tools/store-test-runner/src/durable_publication_boundary_gate/contract/wal_source_syntax.rs\",\n",
        replacement: "",
        package: "store-test-runner",
        target: MutationTarget::Library,
        selector: "durable_publication_boundary_gate::closure_ledger::phase_six::proved_phase_six_wal_segment_lifecycle_tracks_exact_source_closure",
    },
];
