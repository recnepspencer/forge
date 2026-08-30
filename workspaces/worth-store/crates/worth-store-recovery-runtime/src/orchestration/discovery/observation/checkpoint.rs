use super::super::{map_discovery_failure, CheckpointDiscovery, DiscoveryFailure};
use crate::entry::{
    PhysicalRecoveryCheckpointIntegrityDenial, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimits,
};
use crate::integrity_ingress::{
    admit_observed_checkpoint_stream, CheckpointStreamAdmissionFailure,
    RecoveryIntegrityIngressRejection, RecoveryIntegrityIngressTrace,
};
use crate::progression::PhysicalRecoveryDiscoveryCounters;
use worth_store::physical_runtime::BoundedRecoveryFilesystemDiscovery;

pub(super) fn observe_checkpoint(
    discovery: &mut BoundedRecoveryFilesystemDiscovery,
    limits: PhysicalRecoveryLimits,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
    ingress_trace: &mut RecoveryIntegrityIngressTrace,
) -> Result<CheckpointDiscovery, DiscoveryFailure> {
    let declaration = limits.declaration();
    let artifact = discovery
        .read_current_checkpoint(declaration.observation_bytes)
        .map_err(|failure| {
            map_discovery_failure(
                failure,
                PhysicalRecoveryLimitDimension::ObservationBytes,
                PhysicalRecoveryLimitDimension::ObservationBytes,
            )
        })?;
    let mut trace = RecoveryIntegrityIngressTrace::new();
    let checkpoint = match admit_observed_checkpoint_stream(
        &artifact,
        discovery.store_identity(),
        declaration.dirty_frames,
        declaration.operation_bindings,
        &mut trace,
    ) {
        Ok(Some(checkpoint)) => CheckpointDiscovery::Admitted(checkpoint),
        Ok(None) => CheckpointDiscovery::Absent,
        Err(CheckpointStreamAdmissionFailure::AllocationRejected) => CheckpointDiscovery::Rejected(
            PhysicalRecoveryCheckpointIntegrityDenial::AllocationRejected,
        ),
        Err(CheckpointStreamAdmissionFailure::Integrity(rejection)) => {
            CheckpointDiscovery::Rejected(checkpoint_denial(rejection))
        }
        Err(CheckpointStreamAdmissionFailure::DirtyRecordLimit { observed, admitted }) => {
            CheckpointDiscovery::Rejected(
                PhysicalRecoveryCheckpointIntegrityDenial::DirtyRecordLimit { observed, admitted },
            )
        }
        Err(CheckpointStreamAdmissionFailure::BindingRecordLimit { observed, admitted }) => {
            CheckpointDiscovery::Rejected(
                PhysicalRecoveryCheckpointIntegrityDenial::BindingRecordLimit {
                    observed,
                    admitted,
                },
            )
        }
    };
    let ingress = trace.counters();
    ingress_trace.append(trace);
    counters.checkpoint_integrity_attempts = ingress.attempted;
    counters.checkpoint_integrity_admissions = ingress.admitted;
    counters.checkpoint_integrity_rejections = ingress.attempted - ingress.admitted;
    counters.checkpoint_owner_projections = ingress.owner_projection_entries;
    counters.checkpoint_owner_decoder_entries = ingress.owner_decoder_entries;
    Ok(checkpoint)
}

fn checkpoint_denial(
    rejection: RecoveryIntegrityIngressRejection,
) -> PhysicalRecoveryCheckpointIntegrityDenial {
    match rejection {
        RecoveryIntegrityIngressRejection::Integrity(rejection) => {
            PhysicalRecoveryCheckpointIntegrityDenial::Integrity(rejection)
        }
        RecoveryIntegrityIngressRejection::NonCanonicalEncoding => {
            PhysicalRecoveryCheckpointIntegrityDenial::NonCanonicalEncoding
        }
        RecoveryIntegrityIngressRejection::ScopeMismatch
        | RecoveryIntegrityIngressRejection::SourceRangeOutsideObservation => {
            PhysicalRecoveryCheckpointIntegrityDenial::ScopeMismatch
        }
        RecoveryIntegrityIngressRejection::SourceIncarnationMismatch
        | RecoveryIntegrityIngressRejection::Absent
        | RecoveryIntegrityIngressRejection::MissingBoundedArtifact
        | RecoveryIntegrityIngressRejection::ConflictingDuplication { .. } => {
            PhysicalRecoveryCheckpointIntegrityDenial::SourceIncarnationMismatch
        }
    }
}
