#![allow(dead_code)]

use super::certification_bundle::WorthServerCertificationBundle;

pub fn assert_counter_exact(
    bundle: &WorthServerCertificationBundle,
    counter_name: &str,
    expected_exact_value: u64,
) {
    assert_eq!(
        bundle
            .counter_value(counter_name)
            .unwrap_or_else(|| panic!("missing counter {counter_name}")),
        expected_exact_value,
        "unexpected exact counter value for {counter_name}"
    );
}

pub fn assert_counters_zero(bundle: &WorthServerCertificationBundle, counter_names: &[&str]) {
    for counter_name in counter_names {
        assert_counter_exact(bundle, counter_name, 0);
    }
}
