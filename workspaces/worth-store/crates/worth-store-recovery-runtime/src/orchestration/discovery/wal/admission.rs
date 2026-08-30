use worth_store::physical_runtime::{
    recovery_wal::{wal_frame_integrity_scope_identity, WalSegmentArtifactIdentity},
    IntegrityAdmittedRecoveryWalFrame, ObservedWalArtifact,
};
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_physical_integrity::{
    validate_wal_frame_prefix, PhysicalIntegrityRejection, UntrustedPhysicalArtifact,
};

use crate::entry::PhysicalRecoveryWalIntegrityObservation;
use crate::integrity_ingress::{
    IntegrityAdmittedRecoveryArtifact, RecoveryIntegrityIngressCounters,
    RecoveryIntegrityIngressRejection,
};

use super::observation_projection::public_observation;

/// C.9 transcript for one exact C.4-observed WAL artifact.
///
/// This records admission truth only. C.8 decides whether a rejection is a
/// lawful torn tail, corruption, or residue after all canonical sources are
/// known.
pub(super) struct WalSegmentAdmissionTranscript {
    pub identity: WalSegmentArtifactIdentity,
    pub name: String,
    pub artifact: ObservedWalArtifact,
    pub observed_bytes: u64,
    pub frames: Vec<IntegrityAdmittedRecoveryWalFrame>,
    pub observations: Vec<PhysicalRecoveryWalIntegrityObservation>,
    pub rejection: Option<PhysicalIntegrityRejection>,
    pub counters: RecoveryIntegrityIngressCounters,
}

pub(super) enum WalSegmentAdmissionDenial {
    CounterOverflow,
    FrameLimitExceeded { observed: u64, admitted: u64 },
    SourceBinding,
}

pub(super) fn admit_segment(
    owner: &worth_store::physical_runtime::PhysicalRecoveryCoordination,
    identity: WalSegmentArtifactIdentity,
    artifact: ObservedWalArtifact,
    store: StableStoreIdentity,
    maximum_attempts: u64,
) -> Result<WalSegmentAdmissionTranscript, WalSegmentAdmissionDenial> {
    let name = artifact.name().to_string_lossy().into_owned();
    let bytes = artifact.bytes().unwrap_or_default();
    let mut offset = 0_usize;
    let mut frames = Vec::new();
    let mut observations = Vec::new();
    let mut counters = RecoveryIntegrityIngressCounters::default();
    let mut rejection = None;
    while offset < bytes.len() || (bytes.is_empty() && observations.is_empty()) {
        if !bytes.is_empty() {
            let observed = (frames.len() as u64)
                .checked_add(1)
                .ok_or(WalSegmentAdmissionDenial::CounterOverflow)?;
            if observed > maximum_attempts {
                return Err(WalSegmentAdmissionDenial::FrameLimitExceeded {
                    observed,
                    admitted: maximum_attempts,
                });
            }
        }
        let input = UntrustedPhysicalArtifact::from_bounded_bytes(&bytes[offset..]);
        let (validation, _) = validate_wal_frame_prefix(
            input,
            store,
            wal_frame_integrity_scope_identity(identity),
            offset as u64,
        );
        let scope = match &validation {
            worth_store_physical_integrity::WalFrameIntegrityValidation::Intact(value) => {
                value.scope()
            }
            worth_store_physical_integrity::WalFrameIntegrityValidation::Rejected(value) => {
                value.scope()
            }
        };
        let attempt = IntegrityAdmittedRecoveryArtifact::bind_wal_frame(
            owner,
            &artifact,
            scope,
            scope.byte_range(),
            validation,
            &mut counters,
        );
        observations.push(public_observation(attempt.observation()));
        match attempt.into_outcome() {
            Ok(IntegrityAdmittedRecoveryArtifact::WalFrame(frame)) => {
                let projection = frame.project(&mut counters);
                debug_assert_eq!(
                    projection.lsn_start,
                    projection.redo.admitted_frame().lsn_start()
                );
                drop(projection);
                offset = usize::try_from(scope.byte_range().end_exclusive())
                    .map_err(|_| WalSegmentAdmissionDenial::CounterOverflow)?;
                frames.push(frame.into_store_admission());
            }
            Ok(_) => unreachable!("WAL ingress routes only WAL frames"),
            Err(RecoveryIntegrityIngressRejection::Integrity(value)) => {
                rejection = Some(value);
                break;
            }
            Err(_) => return Err(WalSegmentAdmissionDenial::SourceBinding),
        }
    }
    let observed_bytes = bytes.len() as u64;
    Ok(WalSegmentAdmissionTranscript {
        identity,
        name,
        artifact,
        observed_bytes,
        frames,
        observations,
        rejection,
        counters,
    })
}
