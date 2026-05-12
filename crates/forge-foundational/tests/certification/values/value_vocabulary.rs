use forge_foundational::{
    AspectValue, CanonicalBigInt, CanonicalDate, CanonicalDecimal, CanonicalF32, CanonicalF64,
    CanonicalRational, CanonicalString, CanonicalTime, CanonicalTimestamp, CanonicalTimestampTz,
    ContentRefId, EntityId, ScalarAspectType,
};

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
        AspectValue::Bytes(ContentRefId(7)),
        AspectValue::Uuid([1; 16]),
        AspectValue::Date(CanonicalDate {
            days_from_unix_epoch: 20_000,
        }),
        AspectValue::Time(CanonicalTime::new(1_000).expect("time in range")),
        AspectValue::Timestamp(CanonicalTimestamp {
            micros_since_unix_epoch: 42,
        }),
        AspectValue::TimestampTz(CanonicalTimestampTz {
            utc_micros_since_unix_epoch: 42,
            offset_minutes: -420,
        }),
        AspectValue::EntityRef(EntityId(9)),
        AspectValue::ContentRef(ContentRefId(9)),
    ];

    let families: Vec<_> = values.iter().map(AspectValue::value_family).collect();

    assert_eq!(families.len(), 24);
    assert!(families.contains(&ScalarAspectType::Int8));
    assert!(families.contains(&ScalarAspectType::UInt8));
    assert!(families.contains(&ScalarAspectType::EntityRef));
    assert!(families.contains(&ScalarAspectType::ContentRef));
    assert_ne!(
        AspectValue::Bytes(ContentRefId(9)),
        AspectValue::ContentRef(ContentRefId(9))
    );
}

#[test]
fn equality_distinguishes_storage_shape_from_semantic_variant() {
    assert_ne!(AspectValue::Int8(1), AspectValue::UInt8(1));
    assert_ne!(
        AspectValue::Bytes(ContentRefId(1)),
        AspectValue::ContentRef(ContentRefId(1))
    );
}
