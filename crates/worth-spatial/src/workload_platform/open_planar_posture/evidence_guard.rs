use topology::facade::{TopologySeedReceipt, TopologySeedTopologyPosture};

use crate::planar_contracts::clean_fail_boundary::{
    PlanarBoundedConversion, PlanarCleanFailBoundaryReceipt, PlanarCleanFailClass,
    PlanarCleanFailTruthEffect,
};
use crate::workload_platform::surface_support::UnsupportedSurfaceSupport;
use crate::workload_platform::user_response::WorthUserResponseReceipt;

use super::{case::OpenPlanarPostureCase, failure_policy::OpenPlanarPostureError};

pub(super) fn require_open_topology(
    receipt: &TopologySeedReceipt,
) -> Result<OpenPlanarPostureCase, OpenPlanarPostureError> {
    if receipt.topology_posture() != TopologySeedTopologyPosture::OpenValid {
        return Err(OpenPlanarPostureError::TopologyWasNotOpen);
    }
    OpenPlanarPostureCase::from_topology_kind(receipt.kind())
        .ok_or(OpenPlanarPostureError::TopologyWasNotOpen)
}

pub(super) fn require_unsupported_surface(
    support: &UnsupportedSurfaceSupport,
    topology: &TopologySeedReceipt,
) -> Result<(), OpenPlanarPostureError> {
    if support.can_enter_operator_execution()
        || support.can_enter_projection_workload()
        || support.can_enter_local_frame_workload()
        || support.receipt().is_none()
    {
        return Err(OpenPlanarPostureError::SurfaceSupportWasAdmitted);
    }
    if support.topology_query_surface() != Some(topology_query_surface(topology).as_str()) {
        return Err(OpenPlanarPostureError::UnsupportedSurfaceDidNotConsumeOpenTopology);
    }
    Ok(())
}

pub(super) fn require_clean_fail_boundary(
    receipt: &PlanarCleanFailBoundaryReceipt,
    topology: &TopologySeedReceipt,
    posture_case: OpenPlanarPostureCase,
) -> Result<(), OpenPlanarPostureError> {
    if receipt.class() != PlanarCleanFailClass::UnboundedOrOpen {
        return Err(OpenPlanarPostureError::CleanFailDidNotRepresentOpenOrUnbounded);
    }
    if receipt.bounded_conversion() != PlanarBoundedConversion::NotAttempted {
        return Err(OpenPlanarPostureError::CleanFailAttemptedBoundedConversion);
    }
    if receipt.truth_effect() != PlanarCleanFailTruthEffect::DoesNotChangePlanarTruth {
        return Err(OpenPlanarPostureError::CleanFailChangedTruth);
    }
    if receipt.basis().input().source_digest() != topology_identity(topology) {
        return Err(OpenPlanarPostureError::CleanFailDidNotConsumeOpenTopology);
    }
    if receipt.basis().input().transform_posture_digest().is_none() {
        return Err(OpenPlanarPostureError::MissingTransformPosture);
    }
    if receipt.basis().input().open_input_kind() != posture_case.expected_open_input_kind() {
        return Err(OpenPlanarPostureError::MismatchedOpenInputKind);
    }
    if receipt.basis().diagnostics().basis().subject().kind()
        != posture_case.expected_diagnostic_subject_kind()
    {
        return Err(OpenPlanarPostureError::MismatchedDiagnosticSubject);
    }
    Ok(())
}

pub(super) fn require_no_bounded_surrogate(
    attempted_surrogate: Option<&TopologySeedReceipt>,
) -> Result<usize, OpenPlanarPostureError> {
    if attempted_surrogate.is_some() {
        return Err(OpenPlanarPostureError::BoundedSurrogateAttempted);
    }
    Ok(1)
}

pub(super) fn require_user_response(
    receipt: &WorthUserResponseReceipt,
    posture_case: OpenPlanarPostureCase,
    posture_identity: &str,
) -> Result<(), OpenPlanarPostureError> {
    let outcome = receipt.outcome();
    let (expected_kind, expected_cause) = posture_case.expected_user_outcome();
    if outcome.kind() != expected_kind
        || outcome.cause().map(|cause| cause.kind()) != Some(expected_cause)
    {
        return Err(OpenPlanarPostureError::UserResponseDidNotMatchOutcome);
    }
    if outcome.evidence().source_identity() != posture_identity {
        return Err(OpenPlanarPostureError::UserResponseDidNotConsumePosture);
    }
    if posture_case == OpenPlanarPostureCase::PolicyRequiredHalfSpace
        && outcome.choices().is_empty()
    {
        return Err(OpenPlanarPostureError::UserResponseDidNotMatchOutcome);
    }
    if posture_case != OpenPlanarPostureCase::PolicyRequiredHalfSpace
        && !outcome.choices().is_empty()
    {
        return Err(OpenPlanarPostureError::UserResponseDidNotMatchOutcome);
    }
    Ok(())
}

pub(super) fn topology_identity(receipt: &TopologySeedReceipt) -> String {
    receipt
        .query_receipts()
        .declaration_receipt()
        .identity()
        .name()
        .to_string()
}

fn topology_query_surface(receipt: &TopologySeedReceipt) -> String {
    receipt.query_receipts().query_surface().to_string()
}
