#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComplexityStatus {
    Verified,
    Debt,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexityContract {
    pub id: &'static str,
    pub function_path: &'static str,
    pub declared_time_complexity: &'static str,
    pub budget_summary: &'static str,
    pub status: ComplexityStatus,
    pub proof_tests: &'static [&'static str],
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct RuntimeComplexityCounters {
    pub full_state_clones: usize,
    pub entity_slots_cloned: usize,
    pub relation_slots_cloned: usize,
    pub visibility_entity_slot_scans: usize,
    pub visibility_relation_slot_scans: usize,
    pub visible_entity_records_materialized: usize,
    pub visible_relation_records_materialized: usize,
    pub invariant_entity_slot_scans: usize,
    pub invariant_relation_slot_scans: usize,
    pub invariant_entity_records_materialized: usize,
    pub invariant_relation_records_materialized: usize,
    pub snapshot_pin_adjustments: usize,
    pub snapshot_pin_full_rebuilds: usize,
    pub retention_entity_slots_scanned: usize,
    pub retention_relation_slots_scanned: usize,
    pub live_entity_history_entries_trimmed: usize,
    pub live_relation_history_entries_trimmed: usize,
    pub forward_adjacency_updates: usize,
    pub reverse_adjacency_updates: usize,
}

pub const COMPLEXITY_CONTRACTS: &[ComplexityContract] = &[
    ComplexityContract {
        id: "runtime.current_state.clone",
        function_path: "logic/runtime/mod.rs::current_state",
        declared_time_complexity: "O(entity_slots + relation_slots + adjacency_edges)",
        budget_summary: "At most one full-state clone per authoritative commit until sparse overlays replace clone staging.",
        status: ComplexityStatus::Debt,
        proof_tests: &["tests::complexity_contracts::complexity_contract_current_state_clone_is_declared_and_measured"],
    },
    ComplexityContract {
        id: "runtime.snapshot_pin_maintenance",
        function_path: "logic/runtime/mod.rs::{snapshot,release_snapshot,pin_entity,unpin_entity,pin_relation,unpin_relation}",
        declared_time_complexity: "O(snapshot_delta_records)",
        budget_summary: "Commit and release paths must not rebuild all pin counters from live snapshots.",
        status: ComplexityStatus::Verified,
        proof_tests: &["tests::complexity_contracts::complexity_budget_snapshot_pin_maintenance_is_incremental"],
    },
    ComplexityContract {
        id: "runtime.visible_entities.scan",
        function_path: "logic/runtime/read.rs::visible_entities_from_state",
        declared_time_complexity: "O(entity_slots)",
        budget_summary: "Visibility scans must report slot-scan cost explicitly; no hidden mutation is allowed.",
        status: ComplexityStatus::Verified,
        proof_tests: &["tests::complexity_contracts::complexity_contract_visibility_scans_are_explicitly_measured"],
    },
    ComplexityContract {
        id: "runtime.visible_relations.scan",
        function_path: "logic/runtime/read.rs::visible_relations_from_state",
        declared_time_complexity: "O(relation_slots)",
        budget_summary: "Relation visibility scans must report slot-scan cost explicitly; no hidden mutation is allowed.",
        status: ComplexityStatus::Verified,
        proof_tests: &["tests::complexity_contracts::complexity_contract_visibility_scans_are_explicitly_measured"],
    },
    ComplexityContract {
        id: "runtime.retention.pass",
        function_path: "logic/runtime/mod.rs::run_retention_pass",
        declared_time_complexity: "O(chunks_with_retained_records + reclaim_batch_size + changed_live_history)",
        budget_summary: "Retention scans should stay chunk-filtered and live-history trimming must remain touched-record bounded.",
        status: ComplexityStatus::Verified,
        proof_tests: &["tests::complexity_contracts::complexity_budget_live_history_trimming_is_touched_record_bounded"],
    },
    ComplexityContract {
        id: "runtime.relation_adjacency.lookup",
        function_path: "logic/runtime/mod.rs::{outgoing_relations_for_entity,incoming_relations_for_entity}",
        declared_time_complexity: "O(out_degree) / O(in_degree)",
        budget_summary: "Forward and reverse relation traversal must not require full relation scans.",
        status: ComplexityStatus::Verified,
        proof_tests: &["tests::complexity_contracts::complexity_budget_bidirectional_adjacency_avoids_relation_scans"],
    },
    ComplexityContract {
        id: "runtime.invariant.materialization",
        function_path: "logic/runtime/invariants.rs::run_invariants_for_state",
        declared_time_complexity: "O(entity_slots + relation_slots) today, moving toward changed-set/index-assisted checks",
        budget_summary: "Invariant materialization cost must be measured explicitly so full-state checks cannot hide.",
        status: ComplexityStatus::Debt,
        proof_tests: &["tests::complexity_contracts::complexity_contract_invariant_materialization_is_declared_and_measured"],
    },
];
