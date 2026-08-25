use super::*;
use crate::{
    UiHostObservationBatchInput, UiHostObservationLoss, UiHostObservationPayload,
    UiHostObservationPresentationBasis, UiHostObservationReport, UiHostObservationSequence,
    UiHostObservationSequenceRange, UiHostObservationTimeBasis, UiHostPresentationEpoch,
    UiHostProtocolContract, UiHostProtocolNegotiation, UiMountedFrameIdentity,
    UiSurfaceBindingGeneration,
};

#[test]
fn adapter_retention_is_bounded_drained_and_terminalized_per_session() {
    let retention = UiHostObservationRetention::default();
    retention.register_session(1).unwrap();
    for sequence in 1..=UI_HOST_OBSERVATION_DRAIN_BATCH_LIMIT {
        retention
            .retain(batch(1, sequence as u64, "x"))
            .expect("canonical adapter retention capacity");
    }
    assert_eq!(
        retention.retain(batch(1, 17, "x")),
        Err(UiHostObservationRetentionDenial::Capacity(
            UiHostObservationDrainDenial::BatchCapacityExceeded
        ))
    );
    assert_eq!(retention.drain(1).into_batches().len(), 16);
    assert_eq!(retention.pending_batch_count(), 0);
    retention.release_session(1);
    assert_eq!(
        retention.retain(batch(1, 18, "x")),
        Err(UiHostObservationRetentionDenial::InactiveSession)
    );
    retention.register_session(2).unwrap();
    retention
        .retain(batch(2, 1, "x"))
        .expect("a distinct host session remains usable");
    assert_eq!(retention.pending_batch_count_for(2), 1);
    assert_eq!(retention.drain(2).into_batches().len(), 1);
}

#[test]
fn active_session_tracking_is_constant_space_across_successive_sessions() {
    let retention = UiHostObservationRetention::default();
    for host_session in 1..=1_024 {
        retention.register_session(host_session).unwrap();
        retention
            .retain(batch(host_session, 1, "x"))
            .expect("the next monotonic host session remains usable");
        retention.release_session(host_session);
    }

    assert_eq!(retention.pending_batch_count(), 0);
    assert_eq!(
        retention.retain(batch(512, 2, "x")),
        Err(UiHostObservationRetentionDenial::InactiveSession)
    );
    retention.register_session(1_025).unwrap();
    retention
        .retain(batch(1_025, 1, "x"))
        .expect("the successor session remains usable");
}

#[test]
fn reverse_release_keeps_older_active_session_usable() {
    let retention = UiHostObservationRetention::default();
    retention.register_session(2).unwrap();
    retention.register_session(1).unwrap();
    retention.retain(batch(1, 1, "x")).unwrap();
    retention.retain(batch(2, 1, "x")).unwrap();

    retention.release_session(2);
    retention
        .retain(batch(1, 2, "x"))
        .expect("an older but still-active session remains valid");
    assert_eq!(
        retention.retain(batch(2, 2, "x")),
        Err(UiHostObservationRetentionDenial::InactiveSession)
    );
}

#[test]
fn active_session_tracking_has_a_typed_capacity() {
    let retention = UiHostObservationRetention::default();
    for host_session in 1..=UI_HOST_OBSERVATION_ACTIVE_SESSION_LIMIT {
        retention
            .register_session(host_session as u64)
            .expect("qualified concurrent session capacity");
    }
    assert_eq!(
        retention.register_session(UI_HOST_OBSERVATION_ACTIVE_SESSION_LIMIT as u64 + 1),
        Err(UiHostObservationSessionRegistrationDenial::ActiveSessionCapacityExceeded)
    );
}

#[test]
fn drain_bounds_measure_actual_reports_not_untrusted_core_claims() {
    let report = report(1, "x".repeat(UI_HOST_OBSERVATION_DRAIN_BYTE_LIMIT + 1));
    let baseline = batch(1, 1, "x");
    let core = baseline.canonical_core();
    let forged = UiHostObservationBatch::from_untrusted_parts(
        crate::UiHostObservationCanonicalCore::from_untrusted(
            crate::UiHostObservationCanonicalCoreInput {
                protocol: core.protocol(),
                host_session: core.host_session(),
                presentation: core.presentation(),
                sequences: core.sequences(),
                report_count: core.report_count(),
                byte_count: 0,
                loss: core.loss(),
            },
        ),
        vec![report],
        baseline.integrity(),
    );
    assert!(matches!(
        UiHostObservationDrain::bounded(vec![forged]),
        Err(UiHostObservationDrainDenial::ByteCapacityExceeded)
    ));
}

fn batch(host_session: u64, sequence: u64, text: &str) -> UiHostObservationBatch {
    batch_from_reports(host_session, sequence, vec![report(sequence, text.into())])
}

fn batch_from_reports(
    host_session: u64,
    sequence: u64,
    reports: Vec<UiHostObservationReport>,
) -> UiHostObservationBatch {
    let protocol = match UiHostProtocolContract::current().negotiate() {
        UiHostProtocolNegotiation::Compatible(agreement) => agreement,
        UiHostProtocolNegotiation::Incompatible(_) => unreachable!(),
    };
    UiHostObservationBatch::new(UiHostObservationBatchInput {
        protocol,
        host_session,
        presentation: UiHostObservationPresentationBasis::new(
            UiMountedFrameIdentity::mint_unbound().unwrap(),
            UiSurfaceBindingGeneration::mint_unbound().unwrap(),
            UiHostPresentationEpoch::issued_by_host(1),
        ),
        sequences: UiHostObservationSequenceRange::new(
            UiHostObservationSequence::new(sequence),
            UiHostObservationSequence::new(sequence),
        ),
        loss: UiHostObservationLoss::Complete,
        reports,
    })
    .expect("focused drain fixture is structurally valid")
}

fn report(sequence: u64, text: String) -> UiHostObservationReport {
    UiHostObservationReport::new(
        UiHostObservationSequence::new(sequence),
        UiHostObservationTimeBasis::HostMonotonicMillis(sequence),
        UiHostObservationPayload::TextInput {
            revision: sequence,
            text: text.into_boxed_str(),
        },
    )
}
