use crate::authoring::ScalarPredicateValue;
use crate::canonicalization::{
    CanonicalPredicateEntry, CanonicalPredicateOperand, CanonicalPredicateOperand::ScalarSet,
    CanonicalScalarSet,
};

pub(super) fn scalar_operand(predicate: &CanonicalPredicateEntry) -> &ScalarPredicateValue {
    match &predicate.operand {
        CanonicalPredicateOperand::Scalar(value) => value,
        _ => unreachable!("scalar predicate expected scalar operand"),
    }
}

pub(super) fn integer_scalar(predicate: &CanonicalPredicateEntry) -> i64 {
    match scalar_operand(predicate) {
        ScalarPredicateValue::Integer(value) => *value,
        _ => unreachable!("integer predicate expected integer scalar"),
    }
}

pub(super) fn string_scalar(predicate: &CanonicalPredicateEntry) -> &str {
    match scalar_operand(predicate) {
        ScalarPredicateValue::String(value) => value,
        _ => unreachable!("string predicate expected string scalar"),
    }
}

pub(super) fn membership_values_set(predicate: &CanonicalPredicateEntry) -> &CanonicalScalarSet {
    match &predicate.operand {
        ScalarSet(values) => values,
        _ => unreachable!("membership predicate expected scalar set"),
    }
}

pub(super) fn membership_values(predicate: &CanonicalPredicateEntry) -> &[ScalarPredicateValue] {
    membership_values_set(predicate).as_slice()
}
