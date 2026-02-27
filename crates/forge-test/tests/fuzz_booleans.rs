//! Integration tests: corpus fuzzing for planar Booleans.
//!
//! These tests validate that the fuzzing infrastructure works correctly.
//! The Boolean pipeline currently passes for concentric/axis-aligned cases
//! but has known limitations for arbitrary overlapping solids.
//! Pipeline bugs found here are tracked for Milestone 2.5 (Edge-Case Battery).

use forge_test::generators::{random_convex_pair, random_cube_pair};
use forge_test::harness::{run_fuzz_corpus, run_single_case, FuzzOutcome};

/// 100 random cube-cube Booleans — reports pass rate.
///
/// Does NOT assert 100% pass rate; the Boolean pipeline has known
/// limitations for partially overlapping solids. This test ensures
/// the fuzzer infrastructure runs without panics.
#[test]
fn fuzz_100_cube_pairs() {
    let report = run_fuzz_corpus(100, 1000, random_cube_pair);

    eprintln!(
        "Cube fuzz: {}/{} passed, {} errors, {} consistency failures",
        report.passed,
        report.total,
        report.errors,
        report.failures.len()
    );

    let panics = report.total - report.passed - report.errors - report.failures.len();
    assert_eq!(panics, 0, "Fuzz corpus should never panic");
}

/// 100 random convex-convex Booleans — reports pass rate.
#[test]
fn fuzz_100_convex_pairs() {
    let report = run_fuzz_corpus(100, 2000, random_convex_pair);

    eprintln!(
        "Convex fuzz: {}/{} passed, {} errors, {} consistency failures",
        report.passed,
        report.total,
        report.errors,
        report.failures.len()
    );

    let panics = report.total - report.passed - report.errors - report.failures.len();
    assert_eq!(panics, 0, "Fuzz corpus should never panic");
}

/// Specific seed regression test — a known-good cube intersection.
#[test]
fn regression_cube_intersection_seed_42() {
    let input = random_cube_pair(42).expect("Failed to generate cube pair");
    let outcome = run_single_case(input);
    assert!(
        matches!(outcome, FuzzOutcome::Pass | FuzzOutcome::BooleanError(_)),
        "Seed 42 failed: {:?}",
        outcome
    );
}
