use super::report::certify_lower_runtime_performance_slopes;
use crate::lower_runtime_routing::certification::surface::forge_query_lower_runtime_representative_surface;

#[test]
fn scenario_profiles_are_monotonic_across_width_variants() {
    let surface = forge_query_lower_runtime_representative_surface();
    let report = certify_lower_runtime_performance_slopes(&surface);
    let profiles = report.profiles();

    assert_eq!(profiles.len(), 3);
    assert!(
        profiles[0].counters().crossing_inventory_width()
            < profiles[1].counters().crossing_inventory_width()
    );
    assert!(
        profiles[1].counters().crossing_inventory_width()
            < profiles[2].counters().crossing_inventory_width()
    );
    assert!(
        profiles[0].counters().support_lookup_operations()
            < profiles[1].counters().support_lookup_operations()
    );
    assert!(
        profiles[1].counters().support_lookup_operations()
            < profiles[2].counters().support_lookup_operations()
    );
    assert!(
        profiles[0].counters().debt_registry_lookup_operations()
            < profiles[1].counters().debt_registry_lookup_operations()
    );
    assert!(
        profiles[1].counters().debt_registry_lookup_operations()
            < profiles[2].counters().debt_registry_lookup_operations()
    );
}

#[test]
fn slope_report_emits_all_phase_seven_outputs_from_observed_profiles() {
    let surface = forge_query_lower_runtime_representative_surface();
    let report = certify_lower_runtime_performance_slopes(&surface);

    assert_eq!(report.rows().len(), 6);
    for row in report.rows() {
        assert_eq!(
            report.digest_for_output(row.family().output_name()),
            Some(row.slope_digest())
        );
    }
    assert!(!report
        .full_profile()
        .counters()
        .counter_snapshot_digest()
        .is_empty());
}
