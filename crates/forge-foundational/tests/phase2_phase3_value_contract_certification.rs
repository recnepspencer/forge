use forge_foundational::{
    validate_aspect_value, AbsenceLaw, AspectContract, AspectContractRevision,
    AspectEquivalenceBasis, AspectEvolutionKind, AspectIdentity, AspectKey, AspectMask,
    AspectValue, CanonicalBigInt, CanonicalDecimal, CanonicalF32, CanonicalF64, CanonicalFieldPath,
    CanonicalRational, CanonicalString, CanonicalTime, ContractValidatedAspectValue,
    ContractValidationDenial, FieldDeclaration, FieldKey, FieldRequirement,
    MaskAdmissibilityDenial, MutationMask, ProjectionMask, ScalarAspectType, StructAspectShape,
    StructAspectValue,
};
use forge_proof::TransitionOutcome;

fn key(name: &str) -> AspectKey {
    AspectKey::new(name).expect("valid aspect key")
}

fn field(name: &str) -> FieldKey {
    FieldKey::new(name).expect("valid field key")
}

fn revision(value: u64) -> AspectContractRevision {
    AspectContractRevision(value)
}

fn identity(value: u64) -> AspectIdentity {
    AspectIdentity(value)
}

#[test]
fn canonical_value_families_preserve_width_precision_and_reference_kind() {
    let rational = CanonicalRational::new(CanonicalBigInt::new("22"), CanonicalBigInt::new("7"))
        .expect("non-zero denominator");
    let values = vec![
        AspectValue::Null,
        AspectValue::Bool(true),
        AspectValue::Int8(-8),
        AspectValue::Int16(-16),
        AspectValue::Int32(-32),
        AspectValue::Int64(-64),
        AspectValue::UInt8(8),
        AspectValue::UInt16(16),
        AspectValue::UInt32(32),
        AspectValue::UInt64(64),
        AspectValue::Float32(CanonicalF32::from_f32(1.5)),
        AspectValue::Float64(CanonicalF64::from_f64(1.5)),
        AspectValue::Decimal(CanonicalDecimal::new("12.30")),
        AspectValue::BigInt(CanonicalBigInt::new("12345678901234567890")),
        AspectValue::Rational(rational),
        AspectValue::String(CanonicalString::from("name")),
        AspectValue::Bytes(forge_foundational::ContentRefId(7)),
        AspectValue::Uuid([1; 16]),
        AspectValue::Date(forge_foundational::CanonicalDate {
            days_from_unix_epoch: 20_000,
        }),
        AspectValue::Time(CanonicalTime::new(1_000).expect("time in range")),
        AspectValue::Timestamp(forge_foundational::CanonicalTimestamp {
            micros_since_unix_epoch: 42,
        }),
        AspectValue::TimestampTz(forge_foundational::CanonicalTimestampTz {
            utc_micros_since_unix_epoch: 42,
            offset_minutes: -420,
        }),
        AspectValue::EntityRef(forge_foundational::EntityId(9)),
        AspectValue::ContentRef(forge_foundational::ContentRefId(9)),
    ];

    let families: Vec<_> = values.iter().map(AspectValue::value_family).collect();

    assert_eq!(families.len(), 24);
    assert!(families.contains(&ScalarAspectType::Int8));
    assert!(families.contains(&ScalarAspectType::UInt8));
    assert!(families.contains(&ScalarAspectType::EntityRef));
    assert!(families.contains(&ScalarAspectType::ContentRef));
    assert_ne!(
        AspectValue::Bytes(forge_foundational::ContentRefId(9)),
        AspectValue::ContentRef(forge_foundational::ContentRefId(9))
    );
}

#[test]
fn canonical_wrappers_reject_or_normalize_hostile_scalar_edges() {
    assert!(CanonicalRational::new(CanonicalBigInt::new("1"), CanonicalBigInt::new("0")).is_none());
    assert!(CanonicalTime::new(CanonicalTime::NANOS_PER_DAY).is_none());

    let nan_a = CanonicalF32::from_bits(0x7fc0_0001);
    let nan_b = CanonicalF32::from_bits(0x7fc0_ffff);
    let nan_c = CanonicalF64::from_bits(0x7ff8_0000_0000_0001);
    let nan_d = CanonicalF64::from_bits(0x7ff8_ffff_ffff_ffff);

    assert_eq!(nan_a, nan_b);
    assert_eq!(nan_c, nan_d);
}

#[test]
fn equality_distinguishes_storage_shape_from_semantic_variant() {
    assert_ne!(AspectValue::Int8(1), AspectValue::UInt8(1));
    assert_ne!(
        AspectValue::Bytes(forge_foundational::ContentRefId(1)),
        AspectValue::ContentRef(forge_foundational::ContentRefId(1))
    );
}

#[test]
fn scalar_contract_validation_returns_proof_bearing_artifact() {
    let contract = AspectContract::scalar(
        key("temperature.celsius"),
        identity(1),
        revision(3),
        ScalarAspectType::Float64,
    );

    let outcome = validate_aspect_value(
        &contract,
        AspectValue::Float64(CanonicalF64::from_f64(21.0)).into(),
    );

    let TransitionOutcome::Success(artifact) = outcome else {
        panic!("expected validated scalar artifact");
    };

    match artifact.payload() {
        ContractValidatedAspectValue::Scalar {
            key,
            value,
            contract_revision,
        } => {
            assert_eq!(key.as_str(), "temperature.celsius");
            assert_eq!(value.value_family(), ScalarAspectType::Float64);
            assert_eq!(*contract_revision, revision(3));
        }
        ContractValidatedAspectValue::Struct { .. } => panic!("scalar contract produced struct"),
    }
}

#[test]
fn scalar_contract_validation_denies_wrong_width() {
    let contract = AspectContract::scalar(
        key("count"),
        identity(1),
        revision(1),
        ScalarAspectType::Int64,
    );

    let outcome = validate_aspect_value(&contract, AspectValue::Int32(9).into());

    assert_eq!(
        outcome,
        TransitionOutcome::Denied(ContractValidationDenial::ScalarTypeMismatch {
            expected: ScalarAspectType::Int64,
            found: ScalarAspectType::Int32,
        })
    );
}

#[test]
fn struct_contract_validation_is_canonical_and_hostile_to_unknown_fields() {
    let title = field("title");
    let done = field("done");
    let shape = StructAspectShape::new([
        FieldDeclaration::new(
            done.clone(),
            ScalarAspectType::Bool,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        ),
        FieldDeclaration::new(
            title.clone(),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
        ),
    ])
    .expect("unique fields");
    let contract =
        AspectContract::struct_aspect(key("task.summary"), identity(2), revision(1), shape);

    let value = StructAspectValue::new([
        (
            title.clone(),
            AspectValue::String(CanonicalString::from("Ship it")),
        ),
        (done.clone(), AspectValue::Bool(false)),
    ]);
    let outcome = validate_aspect_value(&contract, value.into());

    assert!(matches!(outcome, TransitionOutcome::Success(_)));

    let unknown = field("surprise");
    let denied = validate_aspect_value(
        &contract,
        StructAspectValue::new([
            (title, AspectValue::String(CanonicalString::from("Ship it"))),
            (done, AspectValue::Bool(false)),
            (unknown.clone(), AspectValue::Bool(true)),
        ])
        .into(),
    );

    assert_eq!(
        denied,
        TransitionOutcome::Denied(ContractValidationDenial::UnknownField(unknown))
    );
}

#[test]
fn struct_field_order_is_canonical_across_construction_paths() {
    let a = field("a");
    let b = field("b");
    let left = StructAspectValue::new([
        (b.clone(), AspectValue::Int32(2)),
        (a.clone(), AspectValue::Int32(1)),
    ]);
    let right = StructAspectValue::new([
        (a.clone(), AspectValue::Int32(1)),
        (b.clone(), AspectValue::Int32(2)),
    ]);

    let left_fields: Vec<_> = left.fields().map(|(key, _)| key.as_str()).collect();

    assert_eq!(left, right);
    assert_eq!(left_fields, vec!["a", "b"]);
}

#[test]
fn masks_are_mode_typed_and_shape_admitted() {
    let title = field("title");
    let shape = StructAspectShape::new([FieldDeclaration::new(
        title.clone(),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
    )])
    .expect("unique fields");
    let struct_contract =
        AspectContract::struct_aspect(key("task.summary"), identity(2), revision(1), shape);
    let field_mask = AspectMask::<ProjectionMask>::new([CanonicalFieldPath::single(title.clone())]);

    assert_eq!(struct_contract.admits_projection_mask(&field_mask), Ok(()));

    let scalar_contract = AspectContract::scalar(
        key("task.title"),
        identity(3),
        revision(1),
        ScalarAspectType::String,
    );
    assert_eq!(
        scalar_contract.admits_projection_mask(&field_mask),
        Err(MaskAdmissibilityDenial::FieldMaskRequiresStruct)
    );

    let mutation_whole = AspectMask::<MutationMask>::whole_aspect();
    assert_eq!(
        scalar_contract.admits_mutation_mask(&mutation_whole),
        Ok(())
    );
}

#[test]
fn absence_null_default_and_clear_are_distinct_surface_states() {
    assert_ne!(AbsenceLaw::Required, AbsenceLaw::Optional);
    assert_ne!(AbsenceLaw::Optional, AbsenceLaw::Defaulted);
    assert_ne!(AspectValue::Null.value_family(), ScalarAspectType::String);
}

#[test]
fn evolution_classification_is_deterministic() {
    let base = AspectContract::scalar(
        key("count"),
        identity(9),
        revision(1),
        ScalarAspectType::Int32,
    );
    let widened = AspectContract::scalar(
        key("count"),
        identity(9),
        revision(2),
        ScalarAspectType::Int64,
    );
    let narrowed = AspectContract::scalar(
        key("count"),
        identity(9),
        revision(3),
        ScalarAspectType::Int8,
    );
    let incompatible = AspectContract::scalar(
        key("other"),
        identity(10),
        revision(1),
        ScalarAspectType::Int32,
    );

    assert_eq!(
        base.classify_evolution_to(&widened).kind(),
        AspectEvolutionKind::Widening
    );
    assert_eq!(
        base.classify_evolution_to(&narrowed).kind(),
        AspectEvolutionKind::Narrowing
    );
    assert_eq!(
        base.classify_evolution_to(&incompatible).kind(),
        AspectEvolutionKind::Incompatible
    );
}

#[test]
fn equivalence_basis_is_declared_before_comparison_claims() {
    let scalar = AspectContract::scalar(
        key("count"),
        identity(1),
        revision(1),
        ScalarAspectType::Int64,
    );
    let shape = StructAspectShape::new([FieldDeclaration::new(
        field("title"),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired,
    )])
    .expect("unique fields");
    let structured = AspectContract::struct_aspect(key("task"), identity(2), revision(1), shape);

    assert_eq!(
        scalar.equivalence(),
        AspectEquivalenceBasis::ExactCanonicalValue
    );
    assert_eq!(
        structured.equivalence(),
        AspectEquivalenceBasis::DeclaredStructFields
    );
}
