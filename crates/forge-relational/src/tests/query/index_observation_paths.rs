use crate::facade::history::BranchId;
use crate::facade::indexes::{
    DerivedIndexBuildRequest, DerivedIndexDefinition, DerivedIndexId, DerivedIndexKind,
};
use crate::storage::data::AuthoritativeFieldComparisonKey;
use crate::tests::support::*;
use crate::validation::data::{InvariantCatalog, InvariantRegistration, InvariantRule};

#[test]
fn unique_field_index_refresh_rewrites_name_membership_after_entity_update() {
    let mut runtime = runtime_with_declared_aspect_schema_and_invariants(InvariantCatalog {
        registrations: vec![InvariantRegistration::mutation_sensitive_blocking(
            InvariantRule::unique_entity_aspect_field(aspect_key("name"), field_key("name")),
        )],
        ..InvariantCatalog::default()
    });
    let alpha = create_entity(&mut runtime, "alpha");
    let beta = create_entity(&mut runtime, "beta");

    runtime.index_authority().rebuild_unique_field_indexes();
    update_entity(&mut runtime, beta, "gamma");

    let index_access = runtime.index_access();
    let name_field = forge_foundational::facade::FieldKey::new("name").expect("valid field key");
    let entries = index_access
        .entity_unique_field_entries(&name_field)
        .expect("name entries");

    assert_eq!(
        entries
            .get(&field_comparison_key("alpha"))
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![alpha]
    );
    assert!(!entries.contains_key(&field_comparison_key("beta")));
    assert_eq!(
        entries
            .get(&field_comparison_key("gamma"))
            .cloned()
            .unwrap_or_default()
            .into_iter()
            .collect::<Vec<_>>(),
        vec![beta]
    );
}

#[test]
fn derived_index_build_materializes_latest_visible_entity_field_values() {
    let mut runtime = runtime_with_test_schema();
    let alpha = create_entity(&mut runtime, "alpha");
    let create_outcome = update_entity(&mut runtime, alpha, "gamma");
    let index = runtime.index_authority().register(DerivedIndexDefinition {
        index_id: DerivedIndexId(0),
        name: "entity.name".to_string(),
        kind: DerivedIndexKind::EntityField {
            field: field_key("name"),
        },
        branch_scoped: true,
    });

    let build = runtime
        .index_authority()
        .build_for_commit(DerivedIndexBuildRequest {
            source_commit_id: create_outcome.commit.commit_id,
            branch_id: BranchId("main".to_string()),
            index_ids: vec![index.index_id],
        });
    let index_access = runtime.index_access();
    let generation = index_access
        .latest_generation(index.index_id, &BranchId("main".to_string()))
        .expect("latest generation");

    assert!(build.failed_indexes.is_empty());
    match &generation.entries {
        crate::indexes::data::DerivedIndexEntries::EntityField(entries) => {
            assert!(!entries.contains_key(&field_comparison_key("alpha")));
            assert_eq!(
                entries
                    .get(&field_comparison_key("gamma"))
                    .cloned()
                    .unwrap_or_default(),
                vec![alpha]
            );
        }
        other => panic!("expected entity field entries, got {other:?}"),
    }
}

fn field_comparison_key(value: &str) -> AuthoritativeFieldComparisonKey {
    AuthoritativeFieldComparisonKey::from_aspect_value(&string_aspect_value(value))
}
