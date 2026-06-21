#[test]
fn touched_graph_basis_public_boundary_rejects_forgery() {
    let workspace_temp = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("trybuild-touched-graph-basis");
    std::fs::create_dir_all(&workspace_temp).expect("trybuild temp directory");
    std::env::set_var("HOME", &workspace_temp);
    std::env::set_var("USERPROFILE", &workspace_temp);
    std::env::set_var("TMP", &workspace_temp);
    std::env::set_var("TEMP", &workspace_temp);
    std::env::set_var("CARGO_TARGET_DIR", workspace_temp.join("cargo-target"));

    let t = trybuild::TestCases::new();
    let compile_fail = "tests/ui/touched_graph_basis";
    t.compile_fail(format!("{compile_fail}/basis_struct_literal.rs"));
    t.compile_fail(format!("{compile_fail}/entity_from_raw_id.rs"));
    t.compile_fail(format!(
        "{compile_fail}/geometry_evidence_from_copied_receipt_identity.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/hidden_spatial_admission_module_not_public.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/geometry_evidence_from_raw_digest.rs"
    ));
    t.compile_fail(format!("{compile_fail}/mutation_record_is_not_basis.rs"));
    t.compile_fail(format!("{compile_fail}/operating_world_from_raw_string.rs"));
    t.compile_fail(format!("{compile_fail}/query_descriptor_is_not_basis.rs"));
    t.compile_fail(format!(
        "{compile_fail}/raw_declaration_cannot_mint_basis.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/workflow_helper_fronts_not_public.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/public_query_domain_generic_contribution_bypass_not_public.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/schema_admission_public_facade_cannot_mint_basis.rs"
    ));
    t.compile_fail(format!(
        "{compile_fail}/serde_deserialization_is_not_authority.rs"
    ));
}
