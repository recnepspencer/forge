use crate::basis::SnapshotLineageClass;
use crate::live::LiveQueryPlan;
use crate::query_basis_lifecycle::{
    scope_observation_basis_intent, BasisCapabilityAdmission, BasisOperationLaneRequest,
    BasisScopedAdmissionDenial, NormalizedBasisFamily, NormalizedBasisSubject, RawBasisIdentity,
    RawBasisIntent, ScopedObservationBasis,
};

use super::{
    admit_preview_live_session_plan, execute_preview_live_session_plan, PreviewExecutionError,
    PreviewLiveCounters, PreviewLiveError, PreviewLiveFailureClass, PreviewLiveSessionPlanBinding,
    PreviewSessionPlanBinding,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedPreviewSessionPlanBinding {
    scoped_basis: ScopedObservationBasis,
    preview_binding: PreviewSessionPlanBinding,
}

impl ScopedPreviewSessionPlanBinding {
    pub fn scoped_basis(&self) -> &ScopedObservationBasis {
        &self.scoped_basis
    }

    pub fn preview_binding(&self) -> &PreviewSessionPlanBinding {
        &self.preview_binding
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScopedPreviewLiveSessionPlanBinding {
    scoped_binding: ScopedPreviewSessionPlanBinding,
    preview_live: PreviewLiveSessionPlanBinding,
}

impl ScopedPreviewLiveSessionPlanBinding {
    pub fn scoped_binding(&self) -> &ScopedPreviewSessionPlanBinding {
        &self.scoped_binding
    }

    pub fn preview_live(&self) -> &PreviewLiveSessionPlanBinding {
        &self.preview_live
    }
}

pub fn admit_scoped_preview_session_plan_binding(
    scoped_basis: ScopedObservationBasis,
    preview_binding: PreviewSessionPlanBinding,
) -> Result<ScopedPreviewSessionPlanBinding, PreviewLiveError> {
    ensure_scoped_preview_basis_coherence(&scoped_basis, &preview_binding)?;
    Ok(ScopedPreviewSessionPlanBinding {
        scoped_basis,
        preview_binding,
    })
}

pub fn admit_scoped_preview_session_plan_binding_from_preview_binding(
    preview_binding: PreviewSessionPlanBinding,
) -> Result<ScopedPreviewSessionPlanBinding, PreviewLiveError> {
    let scoped_basis = scoped_observation_basis_for_preview_binding(&preview_binding)?;
    admit_scoped_preview_session_plan_binding(scoped_basis, preview_binding)
}

pub fn admit_scoped_preview_live_session_plan(
    scoped_binding: ScopedPreviewSessionPlanBinding,
    live_plan: LiveQueryPlan,
) -> Result<ScopedPreviewLiveSessionPlanBinding, PreviewLiveError> {
    let preview_live =
        admit_preview_live_session_plan(scoped_binding.preview_binding.clone(), live_plan)?;
    Ok(ScopedPreviewLiveSessionPlanBinding {
        scoped_binding,
        preview_live,
    })
}

pub fn execute_scoped_preview_live_session_plan(
    preview_live: &ScopedPreviewLiveSessionPlanBinding,
) -> Result<super::PreviewLiveExecutionEnvelope, PreviewExecutionError> {
    execute_preview_live_session_plan(preview_live.preview_live())
}

pub fn scoped_observation_basis_for_preview_binding(
    preview_binding: &PreviewSessionPlanBinding,
) -> Result<ScopedObservationBasis, PreviewLiveError> {
    let intent = match preview_binding
        .preflight()
        .basis()
        .identity()
        .lineage_class()
    {
        SnapshotLineageClass::CurrentHead => {
            RawBasisIntent::current_head(BasisOperationLaneRequest::Observation)
        }
        SnapshotLineageClass::ReplayEquivalent | SnapshotLineageClass::FutureExtension => {
            RawBasisIntent::runtime_snapshot(
                preview_binding
                    .preflight()
                    .basis()
                    .identity()
                    .snapshot_identity()
                    .clone(),
                BasisOperationLaneRequest::Observation,
            )
        }
    };

    scope_observation_basis_intent(intent).map_err(scoped_basis_denial)
}

fn ensure_scoped_preview_basis_coherence(
    scoped_basis: &ScopedObservationBasis,
    preview_binding: &PreviewSessionPlanBinding,
) -> Result<(), PreviewLiveError> {
    let (observed_family, observed_scope_subject, observed_lane) = match scoped_basis.admission() {
        BasisCapabilityAdmission::Admitted(capability) => (
            capability.family(),
            capability.scope_subject(),
            capability.operation_lane(),
        ),
        BasisCapabilityAdmission::Advisory(capability) => (
            capability.family(),
            capability.scope_subject(),
            capability.operation_lane(),
        ),
    };

    if observed_lane != &BasisOperationLaneRequest::Observation {
        return Err(scoped_basis_mismatch(
            "scoped preview binding requires an observation-lane scoped basis proof",
        ));
    }

    let expected_family = expected_family(preview_binding);
    if observed_family != expected_family {
        return Err(scoped_basis_mismatch(
            "scoped preview binding requires family parity with the preview preflight basis lineage",
        ));
    }

    let expected_scope_subject = expected_scope_subject(preview_binding);
    if observed_scope_subject != &expected_scope_subject {
        return Err(scoped_basis_mismatch(
            "scoped preview binding requires typed basis-subject parity with the preview preflight basis lineage",
        ));
    }

    Ok(())
}

fn expected_family(preview_binding: &PreviewSessionPlanBinding) -> &NormalizedBasisFamily {
    match preview_binding
        .preflight()
        .basis()
        .identity()
        .lineage_class()
    {
        SnapshotLineageClass::CurrentHead => &NormalizedBasisFamily::CurrentHead,
        SnapshotLineageClass::ReplayEquivalent | SnapshotLineageClass::FutureExtension => {
            &NormalizedBasisFamily::RuntimeSnapshot
        }
    }
}

fn expected_scope_subject(preview_binding: &PreviewSessionPlanBinding) -> NormalizedBasisSubject {
    match preview_binding
        .preflight()
        .basis()
        .identity()
        .lineage_class()
    {
        SnapshotLineageClass::CurrentHead => NormalizedBasisSubject::CurrentHead,
        SnapshotLineageClass::ReplayEquivalent | SnapshotLineageClass::FutureExtension => {
            NormalizedBasisSubject::RuntimeSnapshot {
                snapshot_identity: RawBasisIdentity::from(
                    preview_binding
                        .preflight()
                        .basis()
                        .identity()
                        .snapshot_identity()
                        .clone(),
                ),
            }
        }
    }
}

fn scoped_basis_mismatch(message: &'static str) -> PreviewLiveError {
    PreviewLiveError {
        failure_class: PreviewLiveFailureClass::PreviewLiveScopedBasisMismatch,
        message,
        counters: PreviewLiveCounters {
            preview_live_broad_fallback_denial_count: 1,
            ..PreviewLiveCounters::default()
        },
    }
}

fn scoped_basis_denial(denial: BasisScopedAdmissionDenial) -> PreviewLiveError {
    match denial {
        BasisScopedAdmissionDenial::Intent(_) | BasisScopedAdmissionDenial::Eligibility(_) => {
            scoped_basis_mismatch(
                "preview binding basis lineage could not be restated as scoped observation proof",
            )
        }
    }
}
