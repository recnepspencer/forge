use worth_ui::facade::mounted::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, UiMountedRetentionClass,
};
use worth_ui::facade::observation_report::{
    UiHostObservationFamily, UiHostObservationLoss, UiHostObservationPayload,
    UiHostObservationReportDenial, UiHostObservationReportOutcome,
};

use crate::host_observation_fixture::{batch, pointer, report, source};
use crate::mounted_application_lifecycle::published_mounted_world::{
    published_observation_world, PublishedObservationWorld,
};

#[test]
fn deterministic_240_tick_burst_coalesces_then_lossless_work_backpressures_exactly() {
    const FRAME_COUNT: u64 = 1;
    const SURFACE_COUNT: u64 = 1;
    const INSTANCE_COUNT: u64 = 1;
    const BATCH_REPORT_COUNT: u64 = 1;
    const POINTER_TICKS: u64 = 240;
    const LOSSLESS_RETAINED: u64 = 63;

    let mut world = published_observation_world("observation-deterministic-240-tick-arrival");
    drive_pointer_burst(&mut world, POINTER_TICKS);
    drive_lossless_tail(&mut world, POINTER_TICKS + 1, LOSSLESS_RETAINED);
    deny_next_lossless(&mut world, POINTER_TICKS + LOSSLESS_RETAINED + 1);
    assert_arrival_oracle(
        &world,
        ArrivalOracle {
            frame_count: FRAME_COUNT,
            surface_count: SURFACE_COUNT,
            instance_count: INSTANCE_COUNT,
            batch_report_count: BATCH_REPORT_COUNT,
            pointer_ticks: POINTER_TICKS,
            lossless_retained: LOSSLESS_RETAINED,
        },
    );
}

fn drive_pointer_burst(world: &mut PublishedObservationWorld, ticks: u64) {
    for sequence in 1..=ticks {
        assert!(matches!(
            validate(
                world,
                sequence,
                pointer(sequence, i64::try_from(sequence).unwrap())
            ),
            UiHostObservationReportOutcome::Validated(_)
        ));
    }
}

fn drive_lossless_tail(world: &mut PublishedObservationWorld, first: u64, count: u64) {
    for sequence in first..(first + count) {
        assert!(matches!(
            validate(world, sequence, keyboard(sequence)),
            UiHostObservationReportOutcome::Validated(_)
        ));
    }
}

fn deny_next_lossless(world: &mut PublishedObservationWorld, sequence: u64) {
    assert_eq!(
        validate(world, sequence, keyboard(sequence)),
        UiHostObservationReportOutcome::Denied(
            UiHostObservationReportDenial::LocalCapacityExceeded(UiHostObservationFamily::Keyboard)
        )
    );
}

fn validate(
    world: &mut PublishedObservationWorld,
    sequence: u64,
    payload: UiHostObservationPayload,
) -> UiHostObservationReportOutcome {
    let raw = batch(
        source(&world.session, world.binding, &world.current),
        (sequence, sequence),
        UiHostObservationLoss::Complete,
        vec![report(sequence, payload, &world.current)],
    );
    world.session.validate_host_observation_batch(raw)
}

struct ArrivalOracle {
    frame_count: u64,
    surface_count: u64,
    instance_count: u64,
    batch_report_count: u64,
    pointer_ticks: u64,
    lossless_retained: u64,
}

fn assert_arrival_oracle(world: &PublishedObservationWorld, oracle: ArrivalOracle) {
    let work = world.session.host_observation_work_report();
    let validated = oracle.pointer_ticks + oracle.lossless_retained;
    let batches = validated + 1;
    let retained = 1 + oracle.lossless_retained;
    assert_eq!(work.batches_handled(), batches);
    assert_eq!(
        work.raw_entries_handled(),
        batches * oracle.batch_report_count
    );
    assert_eq!(work.validated_entries(), validated);
    assert_eq!(work.retained_entries(), retained);
    assert_eq!(work.coalesced_entries(), oracle.pointer_ticks - 1);
    assert_eq!(work.denied_entries(), 1);
    assert_eq!(work.duplicate_entries(), 0);
    assert_eq!(work.quarantined_entries(), 0);
    assert_eq!(work.overflowed_entries(), 0);
    assert_eq!(
        world.session.retained_host_observation_report_count(),
        usize::try_from(retained).unwrap()
    );
    assert_eq!(
        world
            .session
            .mounted_retention_report()
            .class(UiMountedRetentionClass::ObservationBasis)
            .active_leases(),
        usize::try_from(oracle.frame_count).unwrap()
    );
    let inspected = match world
        .session
        .inspect_mounted_frame(UiMountedInspectionRequest::current())
    {
        UiMountedInspectionReceipt::Available(inspected) => inspected,
        other => panic!("arrival basis remains compactly inspectable: {other:?}"),
    };
    assert_eq!(
        inspected.presented_binding_count() as u64,
        oracle.surface_count
    );
    assert_eq!(
        inspected.mounted_instance_count() as u64,
        oracle.instance_count
    );
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: sequence.is_multiple_of(2),
        repeat: false,
    }
}
