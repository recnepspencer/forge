use worth_foundational::{
    AspectMask, AspectValue, AuthoritativePatchConstructionDenial, AuthoritativeRecordAspectPatch,
    CanonicalFieldPath, MutationMask,
};
use worth_proof::TransitionOutcome;

use super::patch_fixtures::{task_summary_contract, validated_task_summary};
use crate::foundational_vocabulary::{field, key};

#[test]
fn whole_and_field_patch_overlap_is_rejected_as_ambiguous() {
    let contract = task_summary_contract();
    let TransitionOutcome::Success(whole) = AuthoritativeRecordAspectPatch::whole_aspect(
        [validated_task_summary(&contract, "Ship it", false, None)],
        [],
    ) else {
        panic!("expected whole-aspect patch construction to succeed");
    };
    let TransitionOutcome::Success(field_patch) = AuthoritativeRecordAspectPatch::field_level(
        &contract,
        &AspectMask::<MutationMask>::new([CanonicalFieldPath::single(field("done"))]),
        [(field("done"), AspectValue::Bool(true))],
        [],
    ) else {
        panic!("expected field patch construction to succeed");
    };

    let combined = AuthoritativeRecordAspectPatch::combine(whole, field_patch);

    assert_eq!(
        combined,
        TransitionOutcome::Denied(
            AuthoritativePatchConstructionDenial::AmbiguousWholeAndFieldPatch(key("task.summary"))
        )
    );
}

#[test]
fn duplicate_field_patch_combination_is_rejected() {
    let contract = task_summary_contract();
    let mask = AspectMask::<MutationMask>::new([CanonicalFieldPath::single(field("done"))]);
    let TransitionOutcome::Success(first) = AuthoritativeRecordAspectPatch::field_level(
        &contract,
        &mask,
        [(field("done"), AspectValue::Bool(true))],
        [],
    ) else {
        panic!("expected first field patch construction to succeed");
    };
    let TransitionOutcome::Success(second) = AuthoritativeRecordAspectPatch::field_level(
        &contract,
        &mask,
        [(field("done"), AspectValue::Bool(false))],
        [],
    ) else {
        panic!("expected second field patch construction to succeed");
    };

    let combined = AuthoritativeRecordAspectPatch::combine(first, second);

    assert_eq!(
        combined,
        TransitionOutcome::Denied(AuthoritativePatchConstructionDenial::DuplicateFieldPatch(
            key("task.summary")
        ))
    );
}
