use worth_ui_host_contract::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationPayload, UiHostObservationReport, UiHostObservationRetentionDenial,
    UiHostObservationSequence, UiHostObservationSequenceRange, UiHostObservationTimeBasis,
    UiHostProtocolContract, UiHostProtocolNegotiation, UiMountedFrameIdentity,
    UiSurfaceBindingGeneration, WorthUiHostCapability, WorthUiHostMechanicsAdapter,
};

use super::WorthUiHostEgui;

#[test]
fn input_capabilities_remain_unadvertised_until_translators_are_installed() {
    let report = WorthUiHostEgui::default().mechanical_capability_report();

    for capability in [
        WorthUiHostCapability::PointerInput,
        WorthUiHostCapability::KeyboardInput,
        WorthUiHostCapability::TextInput,
        WorthUiHostCapability::Ime,
    ] {
        assert!(!report.supports(capability));
    }
}

#[test]
fn releasing_one_session_keeps_the_adapter_observation_port_reusable() {
    let host = WorthUiHostEgui::default();
    host.retain_host_observation(batch(11, 1)).unwrap();
    host.retain_host_observation(batch(12, 1)).unwrap();

    assert_eq!(
        host.drain_mechanical_host_observations(11)
            .unwrap()
            .into_batches()
            .len(),
        1
    );
    let _ = host.release_mechanical_host_session(11);
    assert_eq!(
        host.retain_host_observation(batch(11, 2)),
        Err(UiHostObservationRetentionDenial::ReleasedSession)
    );

    host.retain_host_observation(batch(12, 2))
        .expect("the unreleased session remains usable");
    let retained = host
        .drain_mechanical_host_observations(12)
        .unwrap()
        .into_batches();
    assert_eq!(retained.len(), 2);
    assert!(retained
        .iter()
        .all(|batch| batch.canonical_core().host_session() == 12));
}

fn batch(host_session: u64, sequence: u64) -> UiHostObservationBatch {
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
    };
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol,
        host_session,
        binding: UiSurfaceBindingGeneration::mint_unbound().unwrap(),
        frame: UiMountedFrameIdentity::mint_unbound().unwrap(),
        sequences: UiHostObservationSequenceRange::new(
            UiHostObservationSequence::new(sequence),
            UiHostObservationSequence::new(sequence),
        ),
        loss: UiHostObservationLoss::Complete,
        reports: vec![UiHostObservationReport::new(
            UiHostObservationSequence::new(sequence),
            UiHostObservationTimeBasis::HostMonotonicTick(sequence),
            UiHostObservationPayload::Viewport {
                width_subpixels: 1,
                height_subpixels: 1,
            },
        )],
    })
    .unwrap()
}
