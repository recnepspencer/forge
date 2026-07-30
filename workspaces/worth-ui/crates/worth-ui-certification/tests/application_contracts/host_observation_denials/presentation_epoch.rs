use worth_ui::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationLoss, UiHostObservationReportDenial,
};
use worth_ui_host_contract::UiHostPresentationEpoch;

use super::{
    assert_denial, batch, core_with, pointer, published_observation_world, report, source,
    CanonicalCoreMutation,
};

pub(super) fn assert_wrong_presentation_epoch() {
    let mut world = published_observation_world("observation-wrong-presentation-epoch");
    let mut wrong = world.current;
    wrong.epoch =
        UiHostPresentationEpoch::issued_by_host(wrong.epoch.diagnostic_value().wrapping_add(1));
    let raw = batch(
        source(&world.session, world.binding, &wrong),
        (1, 1),
        UiHostObservationLoss::Complete,
        vec![report(1, pointer(1, 10), &wrong)],
    );
    assert_denial(
        &mut world,
        raw,
        UiHostObservationReportDenial::PresentationEpochMismatch,
    );
}

pub(super) fn corrupt_without_resealing(valid: UiHostObservationBatch) -> UiHostObservationBatch {
    let core = valid.canonical_core();
    let corrupt = core_with(
        core,
        CanonicalCoreMutation {
            presentation_epoch: Some(UiHostPresentationEpoch::issued_by_host(
                core.presentation()
                    .epoch()
                    .diagnostic_value()
                    .wrapping_add(1),
            )),
            ..Default::default()
        },
    );
    UiHostObservationBatch::from_untrusted_parts(
        corrupt,
        valid.reports().to_vec(),
        valid.integrity(),
    )
}
