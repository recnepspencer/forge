use crate::identity::hash_parts;
use crate::query_basis_lifecycle::{
    AdmittedBasisCapability, AdvisoryBasisCapability, BasisCapabilityAdmission,
    BasisLifecyclePosture, BasisOperationLaneRequest, BasisVisibility, NormalizedBasisFamily,
};
use crate::runtime::state_basis_classification::authority_lane_for_basis_family;
use crate::runtime::{ForgeQueryAuthorityLane, ForgeQueryRuntimeStateKind};

use super::ForgeQueryBasisLifecycleInspection;

pub(super) fn from_basis_admission(
    subject_label: &'static str,
    admission: &BasisCapabilityAdmission,
) -> ForgeQueryBasisLifecycleInspection {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => {
            from_admitted_capability(subject_label, admitted, admitted.capability_digest(), None)
        }
        BasisCapabilityAdmission::Advisory(advisory) => {
            from_advisory_capability(subject_label, advisory, advisory.advisory_digest(), None)
        }
    }
}

pub(super) fn from_lower_runtime_bound_basis(
    subject_label: &'static str,
    admission: &BasisCapabilityAdmission,
    authority: &'static str,
    binding_digest: &str,
) -> ForgeQueryBasisLifecycleInspection {
    match admission {
        BasisCapabilityAdmission::Admitted(admitted) => from_admitted_capability(
            subject_label,
            admitted,
            binding_digest,
            Some((authority, binding_digest)),
        ),
        BasisCapabilityAdmission::Advisory(advisory) => from_advisory_capability(
            subject_label,
            advisory,
            binding_digest,
            Some((authority, binding_digest)),
        ),
    }
}

pub(super) fn from_admitted_capability(
    subject_label: &'static str,
    admitted: &AdmittedBasisCapability,
    shape_digest: &str,
    lower_runtime: Option<(&'static str, &str)>,
) -> ForgeQueryBasisLifecycleInspection {
    let authority_lane = authority_lane_for_basis_family(
        admitted.family(),
        admitted.lifecycle_posture(),
        admitted.operation_lane(),
    );
    let explanation = format!(
        "{} is ready for `{}` visibility with `{}` lifecycle posture",
        subject_label,
        admitted.visibility().as_str(),
        admitted.lifecycle_posture().as_str()
    );
    let inspection_digest = build_inspection_digest(
        subject_label,
        ForgeQueryRuntimeStateKind::Ready,
        authority_lane,
        admitted.normalized_basis_intent_digest(),
        shape_digest,
        Some(admitted.family()),
        Some(admitted.operation_lane()),
        Some(admitted.visibility()),
        Some(admitted.lifecycle_posture()),
        lower_runtime,
        None,
    );
    ForgeQueryBasisLifecycleInspection {
        subject_label,
        state_kind: ForgeQueryRuntimeStateKind::Ready,
        authority_lane,
        basis_digest: admitted.normalized_basis_intent_digest().to_string(),
        shape_digest: shape_digest.to_string(),
        family: Some(admitted.family().clone()),
        operation_lane: Some(admitted.operation_lane().clone()),
        visibility: Some(admitted.visibility()),
        lifecycle_posture: Some(admitted.lifecycle_posture()),
        lower_runtime_authority: lower_runtime.map(|value| value.0),
        lower_runtime_binding_digest: lower_runtime.map(|value| value.1.to_string()),
        support_digest: None,
        denial_digest: None,
        explanation,
        inspection_digest,
    }
}

fn from_advisory_capability(
    subject_label: &'static str,
    advisory: &AdvisoryBasisCapability,
    shape_digest: &str,
    lower_runtime: Option<(&'static str, &str)>,
) -> ForgeQueryBasisLifecycleInspection {
    let authority_lane = authority_lane_for_basis_family(
        advisory.family(),
        advisory.lifecycle_posture(),
        advisory.operation_lane(),
    );
    let explanation = format!(
        "{} remains advisory for `{}` visibility with `{}` lifecycle posture",
        subject_label,
        advisory.visibility().as_str(),
        advisory.lifecycle_posture().as_str()
    );
    let inspection_digest = build_inspection_digest(
        subject_label,
        ForgeQueryRuntimeStateKind::Pending,
        authority_lane,
        advisory.normalized_basis_intent_digest(),
        shape_digest,
        Some(advisory.family()),
        Some(advisory.operation_lane()),
        Some(advisory.visibility()),
        Some(advisory.lifecycle_posture()),
        lower_runtime,
        None,
    );
    ForgeQueryBasisLifecycleInspection {
        subject_label,
        state_kind: ForgeQueryRuntimeStateKind::Pending,
        authority_lane,
        basis_digest: advisory.normalized_basis_intent_digest().to_string(),
        shape_digest: shape_digest.to_string(),
        family: Some(advisory.family().clone()),
        operation_lane: Some(advisory.operation_lane().clone()),
        visibility: Some(advisory.visibility()),
        lifecycle_posture: Some(advisory.lifecycle_posture()),
        lower_runtime_authority: lower_runtime.map(|value| value.0),
        lower_runtime_binding_digest: lower_runtime.map(|value| value.1.to_string()),
        support_digest: None,
        denial_digest: None,
        explanation,
        inspection_digest,
    }
}

fn build_inspection_digest(
    subject_label: &str,
    state_kind: ForgeQueryRuntimeStateKind,
    authority_lane: ForgeQueryAuthorityLane,
    basis_digest: &str,
    shape_digest: &str,
    family: Option<&NormalizedBasisFamily>,
    operation_lane: Option<&BasisOperationLaneRequest>,
    visibility: Option<BasisVisibility>,
    lifecycle_posture: Option<BasisLifecyclePosture>,
    lower_runtime: Option<(&'static str, &str)>,
    denial_digest: Option<&str>,
) -> String {
    let mut parts = vec![
        "forge_query_basis_lifecycle_inspection_v1".to_string(),
        format!("subject:{subject_label}"),
        format!("state:{}", state_kind.as_str()),
        format!("authority_lane:{}", authority_lane.as_str()),
        format!("basis:{basis_digest}"),
        format!("shape:{shape_digest}"),
    ];
    if let Some(family) = family {
        parts.push(format!("family:{}", family.as_str()));
    }
    if let Some(operation_lane) = operation_lane {
        parts.push(format!("operation_lane:{}", operation_lane.as_str()));
    }
    if let Some(visibility) = visibility {
        parts.push(format!("visibility:{}", visibility.as_str()));
    }
    if let Some(lifecycle_posture) = lifecycle_posture {
        parts.push(format!("lifecycle_posture:{}", lifecycle_posture.as_str()));
    }
    if let Some((authority, binding_digest)) = lower_runtime {
        parts.push(format!("lower_runtime_authority:{authority}"));
        parts.push(format!("lower_runtime_binding:{binding_digest}"));
    }
    if let Some(denial_digest) = denial_digest {
        parts.push(format!("denial:{denial_digest}"));
    }
    hash_parts(&parts)
}
