use crate::{
    UiHostObservationBatch, UiHostObservationBatchInput, UiHostObservationLoss,
    UiHostObservationPayload, UiHostObservationPresentationBasis, UiHostObservationReport,
    UiHostObservationSchemaVersion, UiHostObservationSequence, UiHostObservationSequenceRange,
    UiHostObservationTimeBasis, UiHostPresentationEpoch, UiHostProtocolContract,
    UiHostProtocolDenial, UiHostProtocolNegotiation, UiHostProtocolSchemaFamily,
    UiMountedFrameIdentity, UiSurfaceBindingGeneration,
};

#[test]
fn observation_schema_v5_is_required_before_batch_authority_exists() {
    let current = UiHostProtocolContract::current();
    assert_eq!(current.observation().revision(), 5);
    assert!(matches!(
        current.negotiate(),
        UiHostProtocolNegotiation::Compatible(_)
    ));
    assert_eq!(
        with_observation_revision(4).negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooOld(
            UiHostProtocolSchemaFamily::Observation,
        ))
    );
    assert_eq!(
        with_observation_revision(6).negotiate(),
        UiHostProtocolNegotiation::Incompatible(UiHostProtocolDenial::SchemaTooNew(
            UiHostProtocolSchemaFamily::Observation,
        ))
    );
}

#[test]
fn exact_host_presentation_epoch_participates_in_batch_identity() {
    let frame = UiMountedFrameIdentity::mint_unbound().expect("frame identity capacity");
    let binding = UiSurfaceBindingGeneration::mint_unbound().expect("binding identity capacity");
    let first = batch_for_epoch(frame, binding, 11);
    let second = batch_for_epoch(frame, binding, 12);

    assert_ne!(first.integrity(), second.integrity());
    assert_eq!(
        first.canonical_core().presentation().epoch(),
        UiHostPresentationEpoch::issued_by_host(11)
    );
}

fn batch_for_epoch(
    frame: UiMountedFrameIdentity,
    binding: UiSurfaceBindingGeneration,
    epoch: u64,
) -> UiHostObservationBatch {
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
    };
    let sequence = UiHostObservationSequence::new(1);
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol,
        host_session: 1,
        presentation: UiHostObservationPresentationBasis::new(
            frame,
            binding,
            UiHostPresentationEpoch::issued_by_host(epoch),
        ),
        sequences: UiHostObservationSequenceRange::new(sequence, sequence),
        loss: UiHostObservationLoss::Complete,
        reports: vec![UiHostObservationReport::new(
            sequence,
            UiHostObservationTimeBasis::HostMonotonicTick(1),
            UiHostObservationPayload::Focus { focused: true },
        )],
    })
    .expect("single focused observation batch is valid")
}

fn with_observation_revision(revision: u16) -> UiHostProtocolContract {
    let current = UiHostProtocolContract::current();
    UiHostProtocolContract::new(
        current.identity(),
        current.protocol(),
        current.mounted_frame(),
        current.mounted_presentation(),
        UiHostObservationSchemaVersion::new(revision),
        current.measurement(),
    )
}
