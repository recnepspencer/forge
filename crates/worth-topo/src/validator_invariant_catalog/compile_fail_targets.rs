#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorthTopologyLegalityCatalogCompileFailTarget {
    path: &'static str,
}

impl WorthTopologyLegalityCatalogCompileFailTarget {
    pub const fn path(&self) -> &'static str {
        self.path
    }
}

pub const WORTH_TOPOLOGY_LEGALITY_CATALOG_COMPILE_FAIL_TARGET_COUNT: usize = 35;

pub const fn worth_topology_legality_catalog_compile_fail_targets(
) -> [WorthTopologyLegalityCatalogCompileFailTarget;
       WORTH_TOPOLOGY_LEGALITY_CATALOG_COMPILE_FAIL_TARGET_COUNT] {
    [
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/validator_identity_from_raw_string.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/family_record_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/phase_three_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/invariant_identity_from_raw_string.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/copied_query_registration_name_cannot_mint_invariant_identity.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/declaration_legality_string_cannot_mint_invariant_identity.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/query_projection_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/source_proof_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/no_execution_proof_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/routing_closure_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_plan_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_obligation_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_enforcement_denial_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_enforcement_receipt_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_phase_five_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_loop_wiring_witness_input_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_loop_wiring_loop_witness_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_loop_wiring_half_edge_witness_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_validator_loop_wiring_witness_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/relational_invariant_closeout_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/relational_invariant_phase_six_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/relational_invariant_query_registration_projection_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/raw_query_row_cannot_mint_selected_relational_invariant_row.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/relational_invariant_old_pack_residue_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_graph_obligation_enforcement_receipt_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_graph_obligation_phase_seven_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/operator_certification_cutover_closeout_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/operator_selected_obligation_row_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/operator_certification_phase_eight_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/raw_expectation_residue_cannot_replace_cutover.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/milestone_nine_closeout_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/milestone_ten_seed_struct_literal.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/raw_deletion_row_cannot_mint_closeout.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/raw_residue_row_cannot_mint_closeout.rs",
        },
        WorthTopologyLegalityCatalogCompileFailTarget {
            path: "tests/ui/validator_invariant_catalog/selected_obligation_digest_cannot_mint_milestone_ten_seed.rs",
        },
    ]
}
