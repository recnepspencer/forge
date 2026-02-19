//! Tests for the error taxonomy.

use super::*;
use crate::policy::{PolicyKind, PolicyQuery, PolicyResult};

#[test]
fn success_is_success() {
    let r: PolicyResult<i32> = PolicyResult::Success(42);
    assert!(r.is_success());
    assert!(!r.is_ambiguous());
    assert!(!r.is_hard_error());
}

#[test]
fn ambiguous_carries_query_and_value() {
    let r: PolicyResult<f64> = PolicyResult::Ambiguous {
        query: PolicyQuery {
            kind: PolicyKind::CoincidentGeometry,
            location: [1.0, 2.0, 3.0],
            margin: 1e-9,
            overridable: true,
        },
        potential_value: 0.5,
    };
    assert!(r.is_ambiguous());
}

#[test]
fn hard_error_is_hard_error() {
    let r: PolicyResult<i32> = PolicyResult::HardError(
        KernelError::InvalidInput {
            message: "bad".to_string(),
            context: None,
        },
    );
    assert!(r.is_hard_error());
}

#[test]
fn into_result_strict_rejects_ambiguity() {
    let r: PolicyResult<i32> = PolicyResult::Ambiguous {
        query: PolicyQuery {
            kind: PolicyKind::NearTangency,
            location: [0.0; 3],
            margin: 1e-8,
            overridable: true,
        },
        potential_value: 99,
    };
    assert!(r.into_result_strict().is_err());
}

#[test]
fn into_result_accepting_uses_potential_value() {
    let r: PolicyResult<i32> = PolicyResult::Ambiguous {
        query: PolicyQuery {
            kind: PolicyKind::NearTangency,
            location: [0.0; 3],
            margin: 1e-8,
            overridable: true,
        },
        potential_value: 99,
    };
    assert_eq!(r.into_result_accepting().unwrap(), 99);
}

#[test]
fn from_impl_wraps_in_success() {
    let r: PolicyResult<i32> = 42.into();
    assert!(r.is_success());
}
