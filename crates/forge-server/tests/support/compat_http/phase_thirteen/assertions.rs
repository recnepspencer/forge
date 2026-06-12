#![allow(dead_code)]

use forge_server::{
    ForgeServerBinaryCounterSet, ForgeServerCompatibilityDenial,
    ForgeServerCompatibilityDenialCode, ForgeServerExternalCounterSet,
    ForgeServerQueryHandoffDenial,
};

use crate::compat_http_phase_thirteen_bundle::ForgeServerPhaseThirteenBundle;

pub(crate) fn assert_bundle_digests_equal(
    left: &ForgeServerPhaseThirteenBundle,
    right: &ForgeServerPhaseThirteenBundle,
    labels: &[&'static str],
) {
    for label in labels {
        assert_eq!(
            left.digest(label),
            right.digest(label),
            "expected digest `{label}` to stay equal"
        );
    }
}

pub(crate) fn assert_bundle_digests_not_equal(
    left: &ForgeServerPhaseThirteenBundle,
    right: &ForgeServerPhaseThirteenBundle,
    labels: &[&'static str],
) {
    for label in labels {
        assert_ne!(
            left.digest(label),
            right.digest(label),
            "expected digest `{label}` to stay distinct"
        );
    }
}

pub(crate) fn assert_external_counter(
    counters: &ForgeServerExternalCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "expected external counter `{name}` to equal `{expected}`",
    );
}

pub(crate) fn assert_binary_counter(
    counters: &ForgeServerBinaryCounterSet,
    name: &str,
    expected: u64,
) {
    assert_eq!(
        counters.counter(name),
        Some(expected),
        "expected binary counter `{name}` to equal `{expected}`",
    );
}

pub(crate) fn assert_external_counters_zero(
    counters: &ForgeServerExternalCounterSet,
    names: &[&str],
) {
    for name in names {
        assert_external_counter(counters, name, 0);
    }
}

pub(crate) fn assert_denial_contains(
    denial: &ForgeServerQueryHandoffDenial,
    expected_code: forge_server::ForgeServerQueryHandoffDenialCode,
    expected_detail_fragment: &str,
) {
    assert_eq!(denial.code(), expected_code);
    assert!(
        denial.detail().contains(expected_detail_fragment),
        "expected denial detail to contain `{expected_detail_fragment}`, got `{}`",
        denial.detail()
    );
}

pub(crate) fn assert_compatibility_denial_contains(
    denial: &ForgeServerCompatibilityDenial,
    expected_code: ForgeServerCompatibilityDenialCode,
    expected_detail_fragment: &str,
) {
    assert_eq!(denial.code(), expected_code);
    assert!(
        denial.detail().contains(expected_detail_fragment),
        "expected denial detail to contain `{expected_detail_fragment}`, got `{}`",
        denial.detail()
    );
}
