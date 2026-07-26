use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationFamily, UiHostObservationLoss, UiHostObservationPayload,
    UiHostObservationReportDenial, UiHostObservationReportOutcome,
};
use worth_ui_runtime::facade::mounted::{
    UiMountedInspectionReceipt, UiMountedInspectionRequest, UiMountedRetentionClass,
};

use crate::host_observation_fixture::{batch, pointer, report, source};
use crate::mounted_application_lifecycle::published_mounted_world::{
    published_observation_world, PublishedObservationWorld,
};

#[test]
fn deterministic_240_tick_burst_coalesces_then_lossless_work_backpressures_exactly() {
    let workload = ArrivalWorkload {
        tick_rate_hz: 240,
        frame_count: 1,
        surfaces_per_frame: 1,
        instances_per_surface: 1,
        resources_per_frame: 0,
        batches_per_tick: 1,
        reports_per_batch: 1,
        pointer_burst_ticks: 240,
        lossless_tail_ticks: 63,
        service: ValidationServicePosture::Immediate {
            batches_per_tick: 1,
        },
    };

    let mut world = published_observation_world("observation-deterministic-240-tick-arrival");
    drive_pointer_burst(&mut world, workload.pointer_burst_ticks);
    drive_lossless_tail(
        &mut world,
        workload.pointer_burst_ticks + 1,
        workload.lossless_tail_ticks,
    );
    deny_next_lossless(
        &mut world,
        workload.pointer_burst_ticks + workload.lossless_tail_ticks + 1,
    );
    assert_arrival_oracle(&world, workload);
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

#[derive(Clone, Copy)]
struct ArrivalWorkload {
    tick_rate_hz: u64,
    frame_count: u64,
    surfaces_per_frame: u64,
    instances_per_surface: u64,
    resources_per_frame: u64,
    batches_per_tick: u64,
    reports_per_batch: u64,
    pointer_burst_ticks: u64,
    lossless_tail_ticks: u64,
    service: ValidationServicePosture,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ValidationServicePosture {
    Immediate { batches_per_tick: u64 },
}

fn assert_arrival_oracle(world: &PublishedObservationWorld, workload: ArrivalWorkload) {
    assert_eq!(workload.tick_rate_hz, 240);
    assert_eq!(workload.resources_per_frame, 0);
    assert_eq!(workload.batches_per_tick, 1);
    assert_eq!(
        workload.service,
        ValidationServicePosture::Immediate {
            batches_per_tick: workload.batches_per_tick
        }
    );
    let work = world.session.host_observation_work_report();
    let validated = workload.pointer_burst_ticks + workload.lossless_tail_ticks;
    let batches = validated + 1;
    let retained = 1 + workload.lossless_tail_ticks;
    assert_eq!(work.batches_handled(), batches);
    assert_eq!(
        work.raw_entries_handled(),
        batches * workload.reports_per_batch
    );
    assert_eq!(work.validated_entries(), validated);
    assert_eq!(work.retained_entries(), retained);
    assert_eq!(work.coalesced_entries(), workload.pointer_burst_ticks - 1);
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
        usize::try_from(workload.frame_count).unwrap()
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
        workload.surfaces_per_frame
    );
    assert_eq!(
        inspected.mounted_instance_count() as u64,
        workload.instances_per_surface
    );
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: sequence.is_multiple_of(2),
        repeat: false,
    }
}
