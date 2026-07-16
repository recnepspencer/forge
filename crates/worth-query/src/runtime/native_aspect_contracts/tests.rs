use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, AspectMask, AspectValue, CanonicalFieldPath, FieldDeclaration, FieldKey,
    FieldLevelPatchBuilder, FieldRequirement, MutationMask, ScalarAspectType, StructAspectShape,
    WholeAspectPatchBuilder,
};
use worth_proof::TransitionOutcome;

use super::{
    admit_authored_creation_patch, admit_authored_mutation_patch,
    WorthQueryAspectContractRegistrationDenialKind, WorthQueryMutationContractDenialKind,
    WorthQueryNativeAspectContractRegistry,
};
use crate::runtime::{WorthQueryAspectTouch, WorthQueryAuthoredAspectMutation};

#[test]
fn equivalent_contracts_converge_to_one_runtime_entry() {
    let contract = scalar_contract("task.title", ScalarAspectType::String);
    let registry = WorthQueryNativeAspectContractRegistry::from_contracts([
        contract.clone(),
        contract.clone(),
    ])
    .expect("equivalent contracts should converge");

    assert_eq!(registry.len(), 1);
    assert_eq!(registry.contract(contract.key()), Some(&contract));
}

#[test]
fn conflicting_contracts_deny_the_entire_registry() {
    let string_contract = scalar_contract("task.title", ScalarAspectType::String);
    let bool_contract = scalar_contract("task.title", ScalarAspectType::Bool);

    let denial = WorthQueryNativeAspectContractRegistry::from_contracts([
        string_contract.clone(),
        bool_contract,
    ])
    .expect_err("conflicting contracts must deny runtime construction");

    assert_eq!(
        denial.kind(),
        WorthQueryAspectContractRegistrationDenialKind::ConflictingContract
    );
    assert_eq!(denial.aspect_key(), string_contract.key());
}

#[test]
fn creation_assembles_struct_fields_into_one_whole_aspect_set() {
    let contract = required_struct_contract();
    let registry = WorthQueryNativeAspectContractRegistry::from_contracts([contract.clone()])
        .expect("contract registry should admit");
    let authored = [
        field_set("identity", "id", "task-1"),
        field_set("identity", "tenant", "main"),
    ];

    let patch = admit_authored_creation_patch(&authored, &registry)
        .expect("complete struct creation should admit");

    assert_eq!(patch.whole_aspect_sets().count(), 1);
    assert_eq!(patch.field_patches().count(), 0);
    let (_, value) = patch.whole_aspect_sets().next().unwrap();
    let worth_foundational::facade::ContractValidatedAspectValueView::Struct(value) = value.view()
    else {
        panic!("creation should carry a native struct value");
    };
    assert_eq!(value.get(&field("id")), Some(&text("task-1")));
}

#[test]
fn mutation_preserves_exact_struct_field_patch() {
    let contract = required_struct_contract();
    let registry = WorthQueryNativeAspectContractRegistry::from_contracts([contract])
        .expect("contract registry should admit");
    let authored = [field_set("identity", "id", "task-2")];

    let patch = admit_authored_mutation_patch(&authored, &registry)
        .expect("struct field update should admit");

    assert_eq!(patch.whole_aspect_sets().count(), 0);
    let (_, field_patch) = patch.field_patches().next().expect("field patch");
    assert_eq!(field_patch.field_sets().count(), 1);
}

#[test]
fn query_struct_admission_matches_the_foundational_patch_oracle_exactly() {
    let contract = required_struct_contract();
    let registry = WorthQueryNativeAspectContractRegistry::from_contracts([contract.clone()])
        .expect("contract registry should admit");
    let id = text("task-2");
    let authored =
        [
            WorthQueryAuthoredAspectMutation::new_set(field_touch("identity", "id"), id.clone())
                .unwrap(),
        ];

    let actual = admit_authored_mutation_patch(&authored, &registry)
        .expect("Query mutation admission should succeed");
    let mask = AspectMask::<MutationMask>::new([CanonicalFieldPath::single(field("id"))]);
    let TransitionOutcome::Success(expected) = FieldLevelPatchBuilder::new(&contract, &mask)
        .set_field(field("id"), id)
        .finish()
    else {
        panic!("Foundational field-patch oracle should succeed");
    };
    assert_eq!(actual, expected);

    let whole_value = worth_foundational::facade::StructAspectValue::new([
        (field("id"), text("task-3")),
        (field("tenant"), text("main")),
    ])
    .unwrap();
    let whole_authored = [WorthQueryAuthoredAspectMutation::new_set(
        WorthQueryAspectTouch::whole_aspect(contract.key().clone()),
        whole_value.clone(),
    )
    .unwrap()];
    let actual = admit_authored_mutation_patch(&whole_authored, &registry)
        .expect("Query whole-struct admission should succeed");
    let TransitionOutcome::Success(validated) =
        worth_foundational::facade::validate_aspect_value(&contract, whole_value.into())
    else {
        panic!("Foundational struct validation oracle should succeed");
    };
    let TransitionOutcome::Success(expected) =
        WholeAspectPatchBuilder::default().set(validated).finish()
    else {
        panic!("Foundational whole-patch oracle should succeed");
    };
    assert_eq!(actual, expected);
}

#[test]
fn creation_denies_missing_required_struct_fields_and_explicit_clears() {
    let contract = required_struct_contract();
    let registry = WorthQueryNativeAspectContractRegistry::from_contracts([contract])
        .expect("contract registry should admit");

    let missing = admit_authored_creation_patch(
        &[field_set("identity", "id", "task-without-tenant")],
        &registry,
    )
    .expect_err("an authored struct must contain every required field");
    assert_eq!(
        missing.kind(),
        WorthQueryMutationContractDenialKind::ContractValidationDenied
    );
    let clear = WorthQueryAuthoredAspectMutation::new_clear(field_touch("identity", "id"))
        .expect("clear authoring should parse");
    let denial = admit_authored_creation_patch(&[clear], &registry)
        .expect_err("creation must not turn a clear into an absent field silently");
    assert_eq!(
        denial.kind(),
        WorthQueryMutationContractDenialKind::ClearDuringCreation
    );
}

fn scalar_contract(label: &str, family: ScalarAspectType) -> AspectContract {
    AspectContract::scalar(
        AspectKey::new(label).expect("test aspect key should be valid"),
        AspectIdentity(1),
        AspectContractRevision(1),
        family,
    )
}

fn required_struct_contract() -> AspectContract {
    let shape = StructAspectShape::new(["id", "tenant"].map(|name| {
        FieldDeclaration::new(
            field(name),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("required field declaration should admit")
    }))
    .expect("unique struct fields");
    AspectContract::struct_aspect(
        AspectKey::new("identity").unwrap(),
        AspectIdentity(2),
        AspectContractRevision(1),
        shape,
    )
}

fn field_set(aspect: &str, field_name: &str, value: &str) -> WorthQueryAuthoredAspectMutation {
    WorthQueryAuthoredAspectMutation::new_set(field_touch(aspect, field_name), text(value))
        .expect("field set should admit")
}

fn field_touch(aspect: &str, field_name: &str) -> WorthQueryAspectTouch {
    WorthQueryAspectTouch::aspect_field_path(
        AspectKey::new(aspect).unwrap(),
        CanonicalFieldPath::single(field(field_name)),
    )
}

fn field(name: &str) -> FieldKey {
    FieldKey::new(name).unwrap()
}

fn text(value: &str) -> AspectValue {
    WorthQueryAuthoredAspectMutation::native_string_value(value)
}
