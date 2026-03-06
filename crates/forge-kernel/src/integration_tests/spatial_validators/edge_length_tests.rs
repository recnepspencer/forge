//! Edge length validator poison tests.

use super::test_support::*;
use forge_core::{KernelError, TopologyError};
use forge_spatial::validators::edge_length::validate_zero_length_edges;

#[test]
fn valid_edge_passes() {
    let mut draft = empty_test_draft();
    let (_h0, v0, v1) = build_edge(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider::default();

    let result = validate_zero_length_edges(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1.0, 0.0, 0.0])
            } else {
                None
            }
        },
        &tol,
    );
    assert!(result.is_ok());
}

#[test]
fn zero_length_detected() {
    let mut draft = empty_test_draft();
    let (_h0, v0, v1) = build_edge(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-6,
    };

    let result = validate_zero_length_edges(
        arena,
        &|v| {
            if v == v0 || v == v1 {
                Some([5.0, 5.0, 5.0])
            } else {
                None
            }
        },
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroLengthEdge {
                computed_length, ..
            },
            ..
        } => {
            assert_eq!(computed_length, 0.0);
        }
        other => panic!("Expected ZeroLengthEdge, got: {:?}", other),
    }
}

#[test]
fn sub_tolerance_length_detected() {
    let mut draft = empty_test_draft();
    let (_h0, v0, v1) = build_edge(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-4,
    };

    let result = validate_zero_length_edges(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1e-5, 0.0, 0.0])
            } else {
                None
            }
        },
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err:
                TopologyError::ZeroLengthEdge {
                    computed_length,
                    threshold,
                    ..
                },
            ..
        } => {
            assert!(computed_length > 0.0);
            assert!(computed_length < threshold);
        }
        other => panic!("Expected sub-tolerance ZeroLengthEdge, got: {:?}", other),
    }
}

#[test]
fn self_loop_skipped() {
    let mut draft = empty_test_draft();
    let (_h0, v0) = build_self_loop(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider::default();

    let result = validate_zero_length_edges(
        arena,
        &|v| {
            if v == v0 {
                Some([1.0, 2.0, 3.0])
            } else {
                None
            }
        },
        &tol,
    );
    assert!(result.is_ok(), "Self-loops (A→A) should be skipped");
}

#[test]
fn multiple_edges_one_zero() {
    let mut draft = empty_test_draft();
    let (_h0, v0, v1) = build_edge(&mut draft);
    let (_h1, u0, u1) = build_edge(&mut draft);
    let arena = draft.arena();
    let tol = MockToleranceProvider {
        default_tolerance: 1e-6,
    };

    let result = validate_zero_length_edges(
        arena,
        &|v| {
            if v == v0 {
                Some([0.0, 0.0, 0.0])
            } else if v == v1 {
                Some([1.0, 0.0, 0.0])
            } else if v == u0 {
                Some([5.0, 5.0, 5.0])
            } else if v == u1 {
                Some([5.0, 5.0, 5.0])
            } else {
                None
            }
        },
        &tol,
    );
    assert!(result.is_err());
    match result.unwrap_err() {
        KernelError::TopologyViolation {
            err: TopologyError::ZeroLengthEdge { .. },
            ..
        } => {}
        other => panic!("Expected ZeroLengthEdge, got: {:?}", other),
    }
}
