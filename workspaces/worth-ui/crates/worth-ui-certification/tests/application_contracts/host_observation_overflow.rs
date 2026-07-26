use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationBatchDisposition, UiHostObservationDisposition, UiHostObservationFamily,
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome, UiHostObservationSequence, UiHostObservationSequenceRange,
};

use super::host_observation_fixture::{batch, pointer, report, source};
use super::mounted_application_lifecycle::published_mounted_world::published_observation_world;

#[test]
fn coalescible_overflow_retains_one_survivor_with_explicit_loss() {
    let mut world = published_observation_world("observation-coalescible-overflow");
    let outcome = world.session.validate_host_observation_batch(batch(
        source(&world.session, world.binding, &world.current),
        (1, 65),
        UiHostObservationLoss::Overflow {
            family: UiHostObservationFamily::PointerMotion,
            affected: range(1, 64),
        },
        vec![report(65, pointer(65, 650), &world.current)],
    ));
    let validated = match outcome {
        UiHostObservationReportOutcome::Validated(validated) => validated,
        other => panic!("overflow batch must validate: {other:?}"),
    };
    assert_eq!(
        validated.disposition(),
        UiHostObservationBatchDisposition::Overflow {
            family: UiHostObservationFamily::PointerMotion,
            affected: range(1, 64),
        }
    );
    assert_eq!(
        validated.reports()[0].disposition(),
        UiHostObservationDisposition::Retained
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 1);
    let work = world.session.host_observation_work_report();
    assert_eq!(work.raw_entries_handled(), 1);
    assert_eq!(work.validated_entries(), 1);
    assert_eq!(work.retained_entries(), 1);
    assert_eq!(work.overflowed_entries(), 64);
}

#[test]
fn complete_range_overflow_retains_nothing_and_advances_the_source() {
    let mut world = published_observation_world("observation-complete-overflow");
    let overflow = world.session.validate_host_observation_batch(batch(
        source(&world.session, world.binding, &world.current),
        (1, 4),
        UiHostObservationLoss::Overflow {
            family: UiHostObservationFamily::Viewport,
            affected: range(1, 4),
        },
        Vec::new(),
    ));
    let validated = match overflow {
        UiHostObservationReportOutcome::Validated(validated) => validated,
        other => panic!("complete overflow must validate: {other:?}"),
    };
    assert_eq!(
        validated.disposition(),
        UiHostObservationBatchDisposition::Overflow {
            family: UiHostObservationFamily::Viewport,
            affected: range(1, 4),
        }
    );
    assert!(validated.reports().is_empty());
    assert_eq!(world.session.retained_host_observation_report_count(), 0);
    assert!(matches!(
        world.session.validate_host_observation_batch(batch(
            source(&world.session, world.binding, &world.current),
            (5, 5),
            UiHostObservationLoss::Complete,
            vec![report(
                5,
                UiHostObservationPayload::Focus { focused: true },
                &world.current,
            )],
        )),
        UiHostObservationReportOutcome::Validated(_)
    ));
    let work = world.session.host_observation_work_report();
    assert_eq!(work.batches_handled(), 2);
    assert_eq!(work.raw_entries_handled(), 1);
    assert_eq!(work.validated_entries(), 1);
    assert_eq!(work.overflowed_entries(), 4);
}

#[test]
fn lossless_overflow_denies_without_retaining_partial_input() {
    let mut world = published_observation_world("observation-lossless-overflow");
    let raw = batch(
        source(&world.session, world.binding, &world.current),
        (1, 2),
        UiHostObservationLoss::Overflow {
            family: UiHostObservationFamily::Keyboard,
            affected: range(1, 1),
        },
        vec![report(2, keyboard(2), &world.current)],
    );
    assert_eq!(
        world.session.validate_host_observation_batch(raw),
        UiHostObservationReportOutcome::Denied(UiHostObservationReportDenial::LosslessOverflow(
            UiHostObservationFamily::Keyboard
        ))
    );
    assert_eq!(world.session.retained_host_observation_report_count(), 0);
    let work = world.session.host_observation_work_report();
    assert_eq!(work.raw_entries_handled(), 1);
    assert_eq!(work.denied_entries(), 1);
    assert_eq!(work.overflowed_entries(), 1);
}

fn keyboard(sequence: u64) -> UiHostObservationPayload {
    UiHostObservationPayload::Keyboard {
        physical_key: u32::try_from(sequence).unwrap(),
        pressed: true,
        repeat: false,
    }
}

fn range(first: u64, last: u64) -> UiHostObservationSequenceRange {
    UiHostObservationSequenceRange::new(
        UiHostObservationSequence::new(first),
        UiHostObservationSequence::new(last),
    )
}
