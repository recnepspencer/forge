use super::admission::{HistoricalEvaluationAdmission, HistoricalPathAdmitted};
use super::contracts::HistoricalPathComplexityContract;
use super::counters::HistoricalCounterSnapshot;
use super::error::HistoricalEvaluationError;
use super::path_classes::{
    AdmittedHistoricalPathClass, RequestedHistoricalPathClass, ResolvedHistoricalPathClass,
};
use super::request::{
    validate_basis_match, HistoricalCapabilityDescriptor, HistoricalEvaluationRequest,
    HistoricalMaterializationDescriptor, HistoricalPathRequested,
};
use super::resolution::{HistoricalMaterializationPathMetadata, HistoricalPathResolved};

pub fn admit_historical_evaluation_path(
    request: HistoricalEvaluationRequest,
    capability: HistoricalCapabilityDescriptor,
) -> Result<HistoricalEvaluationAdmission, HistoricalEvaluationError> {
    validate_basis_match(&request, capability.basis_identity())?;
    validate_capability_consistency(&capability, request.requested_path_class())?;

    let admitted_path_class = admit_requested_path(&request, &capability)?;
    let requested_path = HistoricalPathRequested::from_request(&request);
    let counters = match admitted_path_class {
        AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath => {
            HistoricalCounterSnapshot::retained_admission(
                request.replay_budget().max_replay_events(),
                request.reconstruction_budget().max_reconstruction_scope(),
            )
        }
        AdmittedHistoricalPathClass::AdmittedDeltaReplayPath => {
            HistoricalCounterSnapshot::replay_admission(
                request.replay_budget().max_replay_events(),
                request.reconstruction_budget().max_reconstruction_scope(),
            )
        }
        AdmittedHistoricalPathClass::AdmittedFullReconstructionPath => {
            HistoricalCounterSnapshot::reconstruction_admission(
                request.replay_budget().max_replay_events(),
                request.reconstruction_budget().max_reconstruction_scope(),
            )
        }
    };

    Ok(HistoricalEvaluationAdmission::admitted(
        requested_path,
        HistoricalPathAdmitted::new(
            request.requested_path_class().clone(),
            admitted_path_class.clone(),
        ),
        request.cost_posture().clone(),
        request.replay_budget().clone(),
        request.reconstruction_budget().clone(),
        capability.reuse_descriptor().clone(),
        complexity_for_admitted_path(&admitted_path_class),
        counters,
    ))
}

pub fn resolve_historical_materialization_path(
    admission: HistoricalEvaluationAdmission,
    materialization: HistoricalMaterializationDescriptor,
) -> Result<HistoricalPathResolved, HistoricalEvaluationError> {
    if admission.requested_path().basis_identity() != materialization.basis_identity() {
        return Err(HistoricalEvaluationError::IncompatibleBasisPathPair {
            requested_basis_identity: admission.requested_path().basis_identity().to_string(),
            descriptor_basis_identity: materialization.basis_identity().to_string(),
            requested_path_class: admission.requested_path().requested_path_class().clone(),
        });
    }

    let expected_resolved = resolved_for_admitted(admission.admitted_path().admitted_path_class());
    if materialization.resolved_path_class() != &expected_resolved {
        return Err(HistoricalEvaluationError::HiddenPathSubstitutionDenied {
            requested_path_class: admission.requested_path().requested_path_class().clone(),
            admitted_path_class: admission.admitted_path().admitted_path_class().clone(),
            attempted_resolved_path_class: materialization.resolved_path_class().clone(),
        });
    }

    let counters = admission
        .counters()
        .clone()
        .with_resolved_metadata()
        .with_historical_replay_span_drift(usize::from(
            materialization.actual_replay_span()
                > admission.counters().predicted_historical_replay_span(),
        ))
        .with_historical_reconstruction_scope_drift(usize::from(
            materialization.actual_reconstruction_scope()
                > admission
                    .counters()
                    .predicted_historical_reconstruction_scope(),
        ));
    Ok(HistoricalPathResolved::new(
        admission.requested_path().requested_path_class().clone(),
        admission.admitted_path().admitted_path_class().clone(),
        materialization.resolved_path_class().clone(),
        admission.cost_posture().clone(),
        admission.complexity_contract().clone(),
        counters,
    ))
}

pub fn materialization_metadata_from_resolved(
    resolved: HistoricalPathResolved,
) -> HistoricalMaterializationPathMetadata {
    HistoricalMaterializationPathMetadata::from_resolved(resolved)
}

fn admit_requested_path(
    request: &HistoricalEvaluationRequest,
    capability: &HistoricalCapabilityDescriptor,
) -> Result<AdmittedHistoricalPathClass, HistoricalEvaluationError> {
    match request.requested_path_class() {
        RequestedHistoricalPathClass::RequestedRetainedSnapshotPath => {
            match capability.admitted_path_class() {
                Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath)
                    if capability.retention_available() =>
                {
                    Ok(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath)
                }
                Some(other) => Err(mismatched_admitted_path_error(
                    request.requested_path_class(),
                    other,
                )),
                None if capability.retention_available() => Err(
                    HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                        requested_path_class: request.requested_path_class().clone(),
                        reason:
                            "retained-snapshot capability omitted the admitted retained path proof",
                    },
                ),
                None => Err(HistoricalEvaluationError::RetentionUnavailable {
                    requested_path_class: request.requested_path_class().clone(),
                }),
            }
        }
        RequestedHistoricalPathClass::RequestedDeltaReplayPath => {
            match capability.admitted_path_class() {
                Some(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath)
                    if capability.replay_permitted() =>
                {
                    Ok(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath)
                }
                Some(other) => Err(mismatched_admitted_path_error(
                    request.requested_path_class(),
                    other,
                )),
                None if capability.replay_permitted() => Err(
                    HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                        requested_path_class: request.requested_path_class().clone(),
                        reason: "replay capability omitted the admitted replay path proof",
                    },
                ),
                None => Err(HistoricalEvaluationError::ReplayNotPermitted {
                    requested_path_class: request.requested_path_class().clone(),
                }),
            }
        }
        RequestedHistoricalPathClass::RequestedFullReconstructionPath => {
            match capability.admitted_path_class() {
                Some(AdmittedHistoricalPathClass::AdmittedFullReconstructionPath)
                    if capability.replay_required() || capability.historical_lookup_available() =>
                {
                    Ok(AdmittedHistoricalPathClass::AdmittedFullReconstructionPath)
                }
                Some(other) => Err(mismatched_admitted_path_error(
                    request.requested_path_class(),
                    other,
                )),
                None if capability.replay_required() || capability.historical_lookup_available() => {
                    Err(HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                        requested_path_class: request.requested_path_class().clone(),
                        reason:
                            "reconstruction capability omitted the admitted reconstruction path proof",
                    })
                }
                None => Err(HistoricalEvaluationError::ReconstructionNotAdmitted {
                    requested_path_class: request.requested_path_class().clone(),
                }),
            }
        }
    }
}

fn validate_capability_consistency(
    capability: &HistoricalCapabilityDescriptor,
    requested_path_class: &RequestedHistoricalPathClass,
) -> Result<(), HistoricalEvaluationError> {
    match capability.admitted_path_class() {
        Some(AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath)
            if !capability.retention_available() =>
        {
            Err(HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                requested_path_class: requested_path_class.clone(),
                reason:
                    "retained-snapshot proof was lowered without retained-snapshot availability",
            })
        }
        Some(AdmittedHistoricalPathClass::AdmittedDeltaReplayPath)
            if !capability.replay_permitted() =>
        {
            Err(HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                requested_path_class: requested_path_class.clone(),
                reason: "delta replay proof was lowered without replay permission",
            })
        }
        Some(AdmittedHistoricalPathClass::AdmittedFullReconstructionPath)
            if !capability.replay_required() && !capability.historical_lookup_available() =>
        {
            Err(HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
                requested_path_class: requested_path_class.clone(),
                reason:
                    "full reconstruction proof was lowered without reconstruction-capable authority",
            })
        }
        _ => Ok(()),
    }
}

fn mismatched_admitted_path_error(
    requested_path_class: &RequestedHistoricalPathClass,
    admitted_path_class: &AdmittedHistoricalPathClass,
) -> HistoricalEvaluationError {
    HistoricalEvaluationError::UnsupportedHistoricalPathRequest {
        requested_path_class: requested_path_class.clone(),
        reason: match admitted_path_class {
            AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath => {
                "lowered capability only proves retained-snapshot admission for this request"
            }
            AdmittedHistoricalPathClass::AdmittedDeltaReplayPath => {
                "lowered capability only proves delta-replay admission for this request"
            }
            AdmittedHistoricalPathClass::AdmittedFullReconstructionPath => {
                "lowered capability only proves full-reconstruction admission for this request"
            }
        },
    }
}

fn complexity_for_admitted_path(
    admitted: &AdmittedHistoricalPathClass,
) -> HistoricalPathComplexityContract {
    match admitted {
        AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath => {
            HistoricalPathComplexityContract::retained_path()
        }
        AdmittedHistoricalPathClass::AdmittedDeltaReplayPath => {
            HistoricalPathComplexityContract::replay_path()
        }
        AdmittedHistoricalPathClass::AdmittedFullReconstructionPath => {
            HistoricalPathComplexityContract::reconstruction_path()
        }
    }
}

fn resolved_for_admitted(admitted: &AdmittedHistoricalPathClass) -> ResolvedHistoricalPathClass {
    match admitted {
        AdmittedHistoricalPathClass::AdmittedRetainedSnapshotPath => {
            ResolvedHistoricalPathClass::ResolvedRetainedSnapshotPath
        }
        AdmittedHistoricalPathClass::AdmittedDeltaReplayPath => {
            ResolvedHistoricalPathClass::ResolvedDeltaReplayPath
        }
        AdmittedHistoricalPathClass::AdmittedFullReconstructionPath => {
            ResolvedHistoricalPathClass::ResolvedFullReconstructionPath
        }
    }
}
