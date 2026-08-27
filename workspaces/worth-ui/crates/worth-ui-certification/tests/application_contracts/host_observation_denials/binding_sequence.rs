use worth_ui::facade::observation_report::WorthUiHostObservationSessionExt;
use worth_ui::facade::observation_report::{
    UiHostObservationLoss, UiHostObservationPayload, UiHostObservationReportDenial,
    UiHostObservationReportOutcome,
};
use worth_ui_runtime::facade::mounted::UiHostSurfacePresentationMode;
use worth_ui_test_support::WorthUiMountedIdentityCertificationExt;

use crate::host_observation_fixture::{batch, report, source};
use crate::mounted_application_lifecycle::known_empty_surface_world::profile;
use crate::mounted_application_lifecycle::published_mounted_world::{
    publish, published_observation_world,
};

#[test]
fn observation_sequence_remains_session_scoped_across_a_binding_successor() {
    let mut world = published_observation_world("observation-binding-successor-sequence");
    let first = batch(
        source(&world.session, world.binding, &world.current),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::Focus { focused: true },
            &world.current,
        )],
    );
    assert!(matches!(
        world.session.validate_host_observation_batch(first),
        UiHostObservationReportOutcome::Validated(_)
    ));

    let successor = world
        .session
        .rebind_host_surface(
            world.binding,
            UiHostSurfacePresentationMode::RecordOnly,
            profile(2),
        )
        .expect("surface rebind issues one successor binding")
        .binding_generation();
    let successor_basis = publish(&mut world.session, &world.host, world.current.instance);

    let next = batch(
        source(&world.session, successor, &successor_basis),
        (2, 2),
        UiHostObservationLoss::Complete,
        vec![report(
            2,
            UiHostObservationPayload::Focus { focused: false },
            &successor_basis,
        )],
    );
    assert!(matches!(
        world.session.validate_host_observation_batch(next),
        UiHostObservationReportOutcome::Validated(_)
    ));

    let reset = batch(
        source(&world.session, successor, &successor_basis),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(
            1,
            UiHostObservationPayload::Tick { tick: 1 },
            &successor_basis,
        )],
    );
    assert_eq!(
        world.session.validate_host_observation_batch(reset),
        UiHostObservationReportOutcome::Denied(UiHostObservationReportDenial::SequenceReordered)
    );
}
