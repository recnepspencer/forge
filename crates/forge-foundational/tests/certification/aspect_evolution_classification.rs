use forge_foundational::{
    AbsenceLaw, AspectContract, AspectEquivalenceBasis, AspectEvolutionKind, AspectValue,
    FieldDeclaration, FieldRequirement, ScalarAspectType, StructAspectShape,
};

use super::support::{field, identity, key, revision};

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
