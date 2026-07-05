use super::test_support::{read_pressure_budget, World};

use crate::{
    admit_background_pacing, BackgroundIoPressureShape, BackgroundPacingOutcome,
    BackgroundPacingProgressionDrift, BackgroundPacingStaleRebindKind,
};

#[test]
fn stale_and_rebind_progression_use_readiness_counter_drift_evidence() {
    let world = World::new();
    let requested = read_pressure_budget();

    let stale = admit_background_pacing(world.request_with(
        BackgroundIoPressureShape::scrub_scan().requesting(requested),
        requested,
        requested,
        crate::BackgroundResourceBudget::new(),
        world.progression_from_counter_drift(
            BackgroundPacingProgressionDrift::StaleReadinessCounters,
        ),
    ));
    let BackgroundPacingOutcome::StaleRebindRequired(stale) = stale else {
        panic!("expected stale outcome");
    };
    assert_eq!(stale.kind(), BackgroundPacingStaleRebindKind::Stale);
    assert_eq!(stale.counters().deferred_events(), 1);

    let rebind = admit_background_pacing(world.request_with(
        BackgroundIoPressureShape::repair_scan().requesting(requested),
        requested,
        requested,
        crate::BackgroundResourceBudget::new(),
        world.progression_from_counter_drift(
            BackgroundPacingProgressionDrift::RebindRequiredReadinessCounters,
        ),
    ));
    let BackgroundPacingOutcome::StaleRebindRequired(rebind) = rebind else {
        panic!("expected rebind-required outcome");
    };
    assert_eq!(
        rebind.kind(),
        BackgroundPacingStaleRebindKind::RebindRequired
    );
    assert_eq!(rebind.counters().deferred_events(), 1);
}
