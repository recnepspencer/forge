use worth_ui_certification::topology::{
    audit_complete_executable_equivalence_schema, audit_executable_node_schema_source,
};

#[test]
fn executable_equivalence_schema_covers_every_declared_field_and_shared_contract() {
    let violations =
        audit_complete_executable_equivalence_schema(super::workspace_source_inventory());
    assert!(violations.is_empty(), "{}", violations.join("\n"));
}

#[test]
fn executable_equivalence_schema_audit_detects_an_omitted_seed_field() {
    let source = r#"
        pub struct WorthUiPlanNodeInput { identity: String, support: u64, provenance: u64 }
        // non-executable-schema-fields: provenance
        impl WorthUiPlanNodeInput {
            fn executable_schema_matches(&self, other: &Self) -> bool {
                self.identity == other.identity
            }
            pub(crate) fn from_launch_query_binding() {}
        }
    "#;
    let violations = audit_executable_node_schema_source(source);
    assert!(violations
        .iter()
        .any(|violation| violation.contains("`support`")));
}
