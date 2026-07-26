use worth_ui::facade::app::WorthUiActiveApplicationSession;
use worth_ui::facade::observation_report::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationMountedBasis, UiHostObservationPayload, UiHostObservationReport,
    UiHostObservationSequence, UiHostObservationSequenceRange, UiHostObservationTimeBasis,
    UiHostProtocolContract, UiHostProtocolNegotiation,
};
use worth_ui_runtime::facade::mounted::UiSurfaceBindingGeneration;

use super::mounted_application_lifecycle::published_mounted_world::PresentedObservationBasis;

#[derive(Clone, Copy)]
pub(super) struct HostObservationSource<'a> {
    session: &'a WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    basis: &'a PresentedObservationBasis,
}

pub(super) fn report(
    sequence: u64,
    payload: UiHostObservationPayload,
    basis: &PresentedObservationBasis,
) -> UiHostObservationReport {
    UiHostObservationReport::new(
        UiHostObservationSequence::new(sequence),
        UiHostObservationTimeBasis::HostMonotonicTick(sequence),
        payload,
    )
    .with_mounted_basis(UiHostObservationMountedBasis::new(
        basis.instance,
        basis.receipt,
    ))
}

pub(super) fn batch(
    source: HostObservationSource<'_>,
    range: (u64, u64),
    loss: UiHostObservationLoss,
    reports: Vec<UiHostObservationReport>,
) -> UiHostObservationBatch {
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol: protocol(),
        host_session: source.session.host_session_identity().as_u64(),
        binding: source.binding,
        frame: source.basis.frame,
        sequences: UiHostObservationSequenceRange::new(
            UiHostObservationSequence::new(range.0),
            UiHostObservationSequence::new(range.1),
        ),
        loss,
        reports,
    })
    .expect("authored raw batch shape is valid")
}

pub(super) fn source<'a>(
    session: &'a WorthUiActiveApplicationSession,
    binding: UiSurfaceBindingGeneration,
    basis: &'a PresentedObservationBasis,
) -> HostObservationSource<'a> {
    HostObservationSource {
        session,
        binding,
        basis,
    }
}

pub(super) fn pointer(sequence: u64, x: i64) -> UiHostObservationPayload {
    UiHostObservationPayload::PointerMotion {
        pointer: 7,
        capture_epoch: 3,
        pressed_buttons: 0,
        x_subpixels: x,
        y_subpixels: i64::try_from(sequence).unwrap(),
    }
}

pub(super) fn protocol() -> worth_ui::facade::observation_report::UiHostProtocolAgreement {
    match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => {
            panic!("current protocol must negotiate")
        }
    }
}
