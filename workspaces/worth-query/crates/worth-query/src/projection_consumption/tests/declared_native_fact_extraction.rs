use std::collections::BTreeMap;

use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectValue, CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32,
    CanonicalF64, CanonicalRational, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz,
    ContentRefId, FieldDeclaration, FieldKey, FieldRequirement, InternedString, PartitionId,
    ScalarAspectType, StructAspectShape, StructAspectValue, Symbol,
};

use super::phase_four::support::test_entity_identity;
use crate::memory_workspace::WorthQueryEntity;
use crate::projection_consumption::{
    bind_materialized_projection_contract, declare_projection_consumption,
    evaluate_projection_consumption_eligibility, DeclaredNativeAspectContractBasis,
    DeclaredNativeFactContract, ProjectMaterializedFacts, ProjectionConsumptionBindingContext,
    ProjectionConsumptionEligibility, ProjectionConsumptionSource, ProjectionFactExtractionError,
    ProjectionSourceFamily,
};
use crate::runtime::{WorthQueryReadExecutionEngine, WorthQueryReadReceipt, WorthQueryReadResult};

#[test]
fn native_contract_extraction_preserves_all_foundational_scalar_families_and_absence() {
    let samples = scalar_samples();
    let fields = samples
        .iter()
        .enumerate()
        .map(|(index, value)| {
            field_declaration(
                &format!("f{index:02}"),
                value.value_family(),
                AbsenceLaw::Required,
            )
        })
        .chain([
            field_declaration("optional", ScalarAspectType::String, AbsenceLaw::Optional),
            field_declaration("defaulted", ScalarAspectType::String, AbsenceLaw::Defaulted),
        ])
        .collect::<Vec<_>>();
    let contract = struct_contract("native", 0x9150_0001, fields);
    let basis = DeclaredNativeAspectContractBasis::new(contract.clone());
    let mut requested = ProjectMaterializedFacts::declare();
    let mut visible = Vec::new();
    for field in contract_fields(&contract) {
        let declared = DeclaredNativeFactContract::field(
            std::sync::Arc::clone(&basis),
            &[],
            true,
            field.key(),
        )
        .unwrap();
        visible.push(
            declared
                .field_path()
                .terminal_projection_for_boundary()
                .to_string(),
        );
        requested = requested.display_native(declared).unwrap();
    }
    let struct_values: Vec<_> = samples
        .iter()
        .enumerate()
        .map(|(index, value)| {
            (
                FieldKey::new(format!("f{index:02}")).unwrap(),
                value.clone(),
            )
        })
        .collect();
    let result = read_result(vec![WorthQueryEntity::from_aspect_projection(
        test_entity_identity("native-row"),
        BTreeMap::new(),
        BTreeMap::from([(
            contract.key().clone(),
            StructAspectValue::new(struct_values).unwrap(),
        )]),
        BTreeMap::new(),
    )]);
    let fact_set = extract(requested, &visible, &result).unwrap();

    assert_eq!(fact_set.display_fields().len(), samples.len() + 2);
    let facts_by_path = fact_set
        .display_fields()
        .iter()
        .map(|fact| (fact.field_path().terminal_projection_for_boundary(), fact))
        .collect::<BTreeMap<_, _>>();
    for (index, expected) in samples.iter().enumerate() {
        let path = format!("native.f{index:02}");
        assert_eq!(
            facts_by_path[path.as_str()].native_value().scalar(),
            Some(expected)
        );
    }
    let absences = ["native.defaulted", "native.optional"]
        .map(|path| facts_by_path[path].native_value().absence())
        .to_vec();
    assert_eq!(
        absences,
        vec![Some(AbsenceLaw::Defaulted), Some(AbsenceLaw::Optional)]
    );
}

#[test]
fn native_contract_extraction_preserves_whole_structs_and_denies_required_absence() {
    let contract = struct_contract(
        "profile",
        0x9150_0002,
        [field_declaration(
            "label",
            ScalarAspectType::String,
            AbsenceLaw::Required,
        )],
    );
    let basis = DeclaredNativeAspectContractBasis::new(contract.clone());
    let declared = DeclaredNativeFactContract::whole(std::sync::Arc::clone(&basis), true).unwrap();
    let requested = ProjectMaterializedFacts::declare()
        .display_native(declared.clone())
        .unwrap();
    let value = StructAspectValue::new([(
        FieldKey::new("label").unwrap(),
        AspectValue::String("whole-struct".into()),
    )])
    .unwrap();
    let row = WorthQueryEntity::from_aspect_projection(
        test_entity_identity("struct-row"),
        BTreeMap::new(),
        BTreeMap::from([(contract.key().clone(), value.clone())]),
        BTreeMap::new(),
    );
    let result = read_result(vec![row]);
    let facts = extract(
        requested,
        &[declared
            .field_path()
            .terminal_projection_for_boundary()
            .to_string()],
        &result,
    )
    .unwrap();
    assert_eq!(
        facts.display_fields()[0].native_value().struct_value(),
        Some(&value)
    );

    let required_field = FieldKey::new("label").unwrap();
    let required = DeclaredNativeFactContract::field(basis, &[], true, &required_field).unwrap();
    let missing = read_result(vec![WorthQueryEntity::from_native_field_values(
        test_entity_identity("missing-row"),
        BTreeMap::new(),
    )]);
    let denial = extract(
        ProjectMaterializedFacts::declare()
            .display_native(required.clone())
            .unwrap(),
        &[required
            .field_path()
            .terminal_projection_for_boundary()
            .to_string()],
        &missing,
    )
    .unwrap_err();
    assert!(matches!(
        denial,
        ProjectionFactExtractionError::MissingRequiredNativeFact {
            contract_revision: AspectContractRevision(1),
            ..
        }
    ));
}

#[test]
fn whole_struct_validation_retains_the_exact_foundational_shape_denial() {
    let required_rank = FieldKey::new("rank").unwrap();
    let contract = struct_contract(
        "profile",
        0x9150_0003,
        [
            field_declaration("label", ScalarAspectType::String, AbsenceLaw::Required),
            field_declaration("rank", ScalarAspectType::UInt64, AbsenceLaw::Required),
        ],
    );
    let basis = DeclaredNativeAspectContractBasis::new(contract.clone());
    let declared = DeclaredNativeFactContract::whole(basis, true).unwrap();
    let incomplete = StructAspectValue::new([(
        FieldKey::new("label").unwrap(),
        AspectValue::String("incomplete".into()),
    )])
    .unwrap();
    let result = read_result(vec![WorthQueryEntity::from_aspect_projection(
        test_entity_identity("shape-mismatch"),
        BTreeMap::new(),
        BTreeMap::from([(contract.key().clone(), incomplete)]),
        BTreeMap::new(),
    )]);
    let denial = extract(
        ProjectMaterializedFacts::declare()
            .display_native(declared.clone())
            .unwrap(),
        &[declared
            .field_path()
            .terminal_projection_for_boundary()
            .to_string()],
        &result,
    )
    .unwrap_err();

    assert!(matches!(
        denial,
        ProjectionFactExtractionError::NativeContractValueValidationDenied {
            denial: worth_foundational::facade::ContractValidationDenial::MissingRequiredField(field),
            ..
        } if field == required_rank
    ));
}

#[test]
fn dotted_aspect_keys_remain_one_atomic_foundational_locator() {
    let field = FieldKey::new("x").unwrap();
    let contract = struct_contract(
        "geo.position",
        0x9150_0004,
        [field_declaration(
            "x",
            ScalarAspectType::UInt64,
            AbsenceLaw::Required,
        )],
    );
    let declared = DeclaredNativeFactContract::field(
        DeclaredNativeAspectContractBasis::new(contract.clone()),
        &[],
        true,
        &field,
    )
    .unwrap();
    assert_eq!(
        declared.field_path().native_aspect_key(),
        Some(contract.key())
    );
    assert_eq!(declared.field_path().native_field_key(), Some(&field));

    let row = WorthQueryEntity::from_aspect_projection(
        test_entity_identity("dotted-aspect"),
        BTreeMap::new(),
        BTreeMap::from([(
            contract.key().clone(),
            StructAspectValue::new([(field.clone(), AspectValue::UInt64(17))]).unwrap(),
        )]),
        BTreeMap::new(),
    );
    let facts = extract_authorized(
        ProjectMaterializedFacts::declare()
            .display_native(declared)
            .unwrap(),
        vec![
            crate::authorized_projection::AuthorizedProjectionFieldPath::from_native_keys(
                contract.key().clone(),
                field,
            ),
        ],
        &read_result(vec![row]),
    )
    .unwrap();
    assert_eq!(facts.display_fields()[0].as_uint64(), Ok(&17));
}

fn extract(
    requested: ProjectMaterializedFacts,
    visible: &[String],
    result: &WorthQueryReadResult,
) -> Result<crate::projection_consumption::ConsumedProjectionFactSet, ProjectionFactExtractionError>
{
    let visible_refs = visible.iter().map(String::as_str).collect::<Vec<_>>();
    extract_authorized(
        requested,
        crate::projection_consumption::test_authorized_field_paths(&visible_refs),
        result,
    )
}

fn extract_authorized(
    requested: ProjectMaterializedFacts,
    visible: Vec<crate::authorized_projection::AuthorizedProjectionFieldPath>,
    result: &WorthQueryReadResult,
) -> Result<crate::projection_consumption::ConsumedProjectionFactSet, ProjectionFactExtractionError>
{
    let source = ProjectionConsumptionSource::test_only(
        ProjectionSourceFamily::QueryReadReceipt,
        Some(result.receipt().canonical_query_digest()),
        Some(result.receipt().basis_digest()),
        Some(result.receipt().result_digest()),
        Some("result-shape:test"),
        result.receipt().read_graph_digest(),
    );
    let binding = ProjectionConsumptionBindingContext::test_only(
        "result-shape:test",
        "authorized-projection:test",
        visible,
    );
    let declaration = declare_projection_consumption(source, binding, requested).unwrap();
    let ProjectionConsumptionEligibility::Admitted(admitted) =
        evaluate_projection_consumption_eligibility(&declaration)
    else {
        panic!("native declaration should admit")
    };
    bind_materialized_projection_contract(&admitted).extract_from_read_result(result)
}

fn read_result(rows: Vec<WorthQueryEntity>) -> WorthQueryReadResult {
    WorthQueryReadResult::test_only(
        rows,
        WorthQueryReadReceipt::test_only(
            "read-graph:test",
            "query:test",
            "basis:test",
            "result:test",
            WorthQueryReadExecutionEngine::QueryRuntimeCurrent,
        ),
    )
}

fn struct_contract(
    key: &str,
    identity: u64,
    fields: impl IntoIterator<Item = FieldDeclaration>,
) -> AspectContract {
    AspectContract::struct_aspect(
        AspectKey::new(key).unwrap(),
        AspectIdentity(identity),
        AspectContractRevision(1),
        StructAspectShape::new(fields).unwrap(),
    )
}

fn field_declaration(key: &str, family: ScalarAspectType, absence: AbsenceLaw) -> FieldDeclaration {
    let requirement = match absence {
        AbsenceLaw::Required => FieldRequirement::Required,
        AbsenceLaw::Optional => FieldRequirement::Optional,
        AbsenceLaw::Defaulted => FieldRequirement::Defaulted,
    };
    FieldDeclaration::new(
        FieldKey::new(key).unwrap(),
        family,
        requirement,
        absence,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .unwrap()
}

fn contract_fields(contract: &AspectContract) -> &[FieldDeclaration] {
    let worth_foundational::facade::AspectShape::Struct(shape) = contract.shape() else {
        panic!("test contract is structured")
    };
    shape.fields()
}

fn scalar_samples() -> Vec<AspectValue> {
    vec![
        AspectValue::Null,
        AspectValue::Bool(true),
        AspectValue::Int8(-7),
        AspectValue::Int16(-320),
        AspectValue::Int32(-32_000),
        AspectValue::Int64(-12),
        AspectValue::UInt8(7),
        AspectValue::UInt16(320),
        AspectValue::UInt32(32_000),
        AspectValue::UInt64(12),
        AspectValue::Float32(CanonicalF32::from_f32(1.5)),
        AspectValue::Float64(CanonicalF64::from_f64(2.5)),
        AspectValue::Decimal(CanonicalDecimal::new("12.50")),
        AspectValue::BigInt(CanonicalBigInt::new("-12345678901234567890")),
        AspectValue::Rational(
            CanonicalRational::new(CanonicalBigInt::new("22"), CanonicalBigInt::new("7")).unwrap(),
        ),
        AspectValue::String(InternedString::Raw("alpha".into())),
        AspectValue::Bytes(ContentRefId(41)),
        AspectValue::Uuid([7; 16]),
        AspectValue::Date(CanonicalDate {
            days_from_unix_epoch: 20_000,
        }),
        AspectValue::Time(CanonicalTime::new(1_000).unwrap()),
        AspectValue::Timestamp(CanonicalTimestamp {
            micros_since_unix_epoch: 123_456,
        }),
        AspectValue::TimestampTz(CanonicalTimestampTz {
            utc_micros_since_unix_epoch: 123_456,
            offset_minutes: -360,
        }),
        AspectValue::EntityRef(worth_foundational::facade::EntityId::new(
            PartitionId(9),
            10,
            11,
        )),
        AspectValue::ContentRef(ContentRefId(42)),
        AspectValue::String(InternedString::Symbol(Symbol(17))),
    ]
}
