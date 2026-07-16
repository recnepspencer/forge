#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow {
    phase: u8,
    path: &'static str,
    probe: &'static str,
}

impl WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow {
    const fn new(phase: u8, path: &'static str, probe: &'static str) -> Self {
        Self { phase, path, probe }
    }

    pub const fn phase(&self) -> u8 {
        self.phase
    }

    pub const fn path(&self) -> &'static str {
        self.path
    }

    pub const fn probe(&self) -> &'static str {
        self.probe
    }
}

const EVIDENCE: [WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow; 25] = [
    row(
        13,
        "crates/worth-query/src/consumer_kit/domain_authority_inventory/tests.rs",
        "fn current_domain_authority_inventory_is_source_complete()",
    ),
    row(
        14,
        "crates/worth-query/src/domain_installation/package_validation_matrix_tests.rs",
        "fn every_package_semantic_family_is_order_independent()",
    ),
    row(
        14,
        "crates/worth-query/src/application/domain_handle/admitted_world_basis_tests.rs",
        "fn query_seals_structured_context_identity_independently_of_field_order()",
    ),
    row(
        15,
        "crates/worth-query/src/runtime/tests/domain_installation.rs",
        "fn equivalent_packages_mint_semantically_equal_but_runtime_affine_handles()",
    ),
    row(
        15,
        "crates/worth-query/src/domain_installation/pending_installations_tests.rs",
        "fn failed_late_compilation_leaves_every_pending_installation_index_unchanged()",
    ),
    row(
        16,
        "crates/worth-query/src/runtime/tests/domain_installation/substrates.rs",
        "fn installed_package_compiles_every_semantic_family_before_runtime_publication()",
    ),
    row(
        16,
        "crates/worth-query/src/runtime/tests/domain_installation/operation_path_equivalence.rs",
        "fn installed_operation_resolution_is_identical_across_every_runtime_path()",
    ),
    row(
        16,
        "crates/worth-query/src/runtime/tests/domain_installation/substrates.rs",
        "fn rebuilt_execution_index_reproduces_resolution_denial_and_diagnostic_identity()",
    ),
    row(
        16,
        "crates/worth-query/src/runtime/tests/domain_installation/lookup_scaling.rs",
        "fn installed_operation_lookup_width_is_independent_of_unrelated_packages_and_operations()",
    ),
    row(
        17,
        "crates/worth-query/src/runtime/tests/domain_installation/authority.rs",
        "fn handle_bound_contribution_matches_internal_oracle_and_retains_installed_witnesses()",
    ),
    row(
        17,
        "crates/worth-query/src/runtime/tests/domain_installation/authority.rs",
        "fn foreign_handle_and_stale_generation_deny_before_contribution_successors_are_issued()",
    ),
    row(
        18,
        "crates/worth-query/src/runtime/tests/domain_installation/journey.rs",
        "fn installed_read_projection_and_receipts_carry_one_authority_witness()",
    ),
    row(
        18,
        "crates/worth-query/src/runtime/tests/domain_installation/journey.rs",
        "fn installed_operational_and_rich_inspection_share_operational_evidence()",
    ),
    row(
        18,
        "crates/worth-query/src/runtime/tests/domain_installation/live_lifecycle.rs",
        "fn generation_turnover_prevents_live_continuation_revival()",
    ),
    row(
        18,
        "crates/worth-query/src/runtime/tests/domain_installation/rebind.rs",
        "fn rebind_reissues_current_authority_only_for_equivalent_package_meaning()",
    ),
    row(
        19,
        "crates/worth-query/tests/installed_domain_facade_extension.rs",
        "fn downstream_extension_preserves_the_generic_installed_capability_artifact()",
    ),
    row(
        20,
        "crates/hadwiger-research/src/query_entry/ordinary_query_tests.rs",
        "fn hadwiger_candidate_search_uses_the_installed_domain_read_journey()",
    ),
    row(
        20,
        "crates/hadwiger-research/src/query_entry/ordinary_query_tests.rs",
        "fn hadwiger_contribution_lowers_through_the_installed_handle()",
    ),
    row(
        20,
        "crates/hadwiger-research/tests/research_graph_invariants/registration.rs",
        "fn domain_package_installs_research_graph_invariants_once()",
    ),
    row(
        20,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements_tests.rs",
        "fn worth_ui_executes_the_installed_domain_read_projection_and_inspection_journey()",
    ),
    row(
        20,
        "workspaces/worth-ui/crates/worth-ui-query-binding/src/installed_measurements_tests.rs",
        "fn worth_ui_executes_installed_workflow_contribution_and_invariant_journeys()",
    ),
    row(
        20,
        "crates/worth-query/tests/consumer_residue_audit.rs",
        "fn installed_domain_reference_consumers_have_no_competing_authority_residue()",
    ),
    row(
        20,
        "crates/worth-query/tests/declarative_facade_docs.rs",
        "fn installed_domain_discovery_teaches_only_the_current_authority_path()",
    ),
    row(
        20,
        "crates/worth-query/docs/domain-capabilities/runtime-installed-domains.md",
        "# Runtime-Installed Domains",
    ),
    row(
        20,
        "_docs/WORTH-query/test-requirements.md",
        "## Milestone 9.13 Phases 13-20 Required Suite",
    ),
];

const fn row(
    phase: u8,
    path: &'static str,
    probe: &'static str,
) -> WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow {
    WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow::new(phase, path, probe)
}

pub fn worth_query_milestone_nine_thirteen_installed_domain_evidence_rows(
) -> &'static [WorthQueryMilestoneNineThirteenInstalledDomainEvidenceRow] {
    &EVIDENCE
}
