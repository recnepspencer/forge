use std::collections::BTreeMap;

use worth_foundational::facade::{AbsenceLaw, AspectValue, ScalarAspectType};

use super::declared_native_fact_extraction::{
    extract, field_declaration, read_result, struct_contract,
};
use super::phase_four::support::{canonical_field_path, test_entity_identity};
use crate::memory_workspace::WorthQueryEntity;
use crate::projection_consumption::{
    DeclaredNativeAspectContractBasis, DeclaredNativeFactContract, ProjectMaterializedFacts,
};

#[test]
fn declared_native_field_extracts_from_runtime_scalar_field_storage() {
    let contract = struct_contract(
        "query_text",
        0x9150_0313,
        [field_declaration(
            "status",
            ScalarAspectType::String,
            AbsenceLaw::Required,
        )],
    );
    let field = worth_foundational::facade::FieldKey::new("status").unwrap();
    let declared = DeclaredNativeFactContract::field(
        DeclaredNativeAspectContractBasis::new(contract),
        &[],
        true,
        &field,
    )
    .unwrap();
    let visible = declared
        .field_path()
        .terminal_projection_for_boundary()
        .to_string();
    let result = read_result(vec![WorthQueryEntity::from_native_field_values(
        test_entity_identity("platform.pulse.status"),
        BTreeMap::from([(
            canonical_field_path("query_text.status"),
            AspectValue::String("ONLINE".into()),
        )]),
    )]);

    let facts = extract(
        ProjectMaterializedFacts::declare()
            .display_native(declared)
            .unwrap(),
        &[visible],
        &result,
    )
    .unwrap();

    assert_eq!(
        facts.display_fields()[0].native_value().scalar(),
        Some(&AspectValue::String("ONLINE".into()))
    );
}
