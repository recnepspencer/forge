use crate::facade::{AfterCondition, ClockDomain, ClockTick, IntervalCondition, TemporalCondition};

#[test]
fn temporal_conditions_default_to_monotonic_execution_clock() {
    let after = TemporalCondition::after(25).unwrap();
    let debounce = TemporalCondition::debounce(50).unwrap();
    let interval = TemporalCondition::interval(IntervalCondition::try_new(100).unwrap());

    assert_eq!(after.clock_domain(), ClockDomain::MonotonicExecution);
    assert_eq!(debounce.clock_domain(), ClockDomain::MonotonicExecution);
    assert_eq!(interval.clock_domain(), ClockDomain::MonotonicExecution);
    match after {
        TemporalCondition::After(condition) => assert_eq!(condition.delay().get(), 25),
        other => panic!("expected After condition, got {other:?}"),
    }
    match debounce {
        TemporalCondition::Debounce(condition) => assert_eq!(condition.quiet_period().get(), 50),
        other => panic!("expected Debounce condition, got {other:?}"),
    }
    match interval {
        TemporalCondition::Interval(condition) => assert_eq!(condition.period().get(), 100),
        other => panic!("expected Interval condition, got {other:?}"),
    }
}

#[test]
fn temporal_condition_clock_domain_rejects_metadata_only_domains() {
    let err = AfterCondition::try_new(25)
        .unwrap()
        .with_clock_domain(ClockDomain::WallClock)
        .unwrap_err();
    assert!(format!("{err}").contains("metadata-only"));

    let err = IntervalCondition::try_new(100)
        .unwrap()
        .with_clock_domain(ClockDomain::Presentation)
        .unwrap_err();
    assert!(format!("{err}").contains("metadata-only"));
}

#[test]
fn zero_width_temporal_declarations_are_rejected() {
    let err = TemporalCondition::after(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::debounce(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::throttle(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = TemporalCondition::stale_after(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));

    let err = IntervalCondition::try_new(0).unwrap_err();
    assert!(format!("{err}").contains("greater than zero"));
}

#[test]
fn at_or_after_condition_uses_clock_tick_semantics() {
    let condition = TemporalCondition::at_or_after(ClockTick::new(42));

    match condition {
        TemporalCondition::AtOrAfter(condition) => {
            assert_eq!(condition.tick(), ClockTick::new(42));
            assert_eq!(condition.clock_domain(), ClockDomain::MonotonicExecution);
        }
        other => panic!("expected AtOrAfter condition, got {other:?}"),
    }
}
