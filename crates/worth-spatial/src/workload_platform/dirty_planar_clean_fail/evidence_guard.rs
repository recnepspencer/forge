use topology::facade::{
    TopologySeedCleanFailClass, TopologySeedCleanFailReceipt, TopologySeedCleanFailStage,
};

use crate::planar_contracts::clean_fail_boundary::{
    PlanarBoundedConversion, PlanarCleanFailBoundaryReceipt, PlanarCleanFailClass,
    PlanarCleanFailTruthEffect, PlanarRepairAttempt,
};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticTruthEffect;
use crate::planar_contracts::planar_recovery::{
    PlanarRecoveryBlockerKind, PlanarRecoverySourcePosture, PlanarRecoveryTruthEffect,
};
use crate::workload_platform::user_response::{
    WorthUserOutcomeCauseKind, WorthUserOutcomeKind, WorthUserResponseReceipt,
};

use super::{case::DirtyPlanarCleanFailCase, failure_policy::DirtyPlanarCleanFailError};

pub(super) fn require_dirty_topology_clean_fail(
    receipt: &TopologySeedCleanFailReceipt,
) -> Result<DirtyPlanarCleanFailCase, DirtyPlanarCleanFailError> {
    if receipt.can_enter_spatial_binding() {
        return Err(DirtyPlanarCleanFailError::TopologyAllowedSpatialBinding);
    }
    if receipt.class() != TopologySeedCleanFailClass::DirtyTopology
        || receipt.stage() != TopologySeedCleanFailStage::SpatialBindingAdmission
    {
        return Err(DirtyPlanarCleanFailError::MissingTopologyCleanFail);
    }
    topology_dirty_case(receipt)
}

pub(super) fn require_clean_fail_boundary(
    receipt: &PlanarCleanFailBoundaryReceipt,
    topology_case: DirtyPlanarCleanFailCase,
) -> Result<DirtyPlanarCleanFailCase, DirtyPlanarCleanFailError> {
    if receipt.class() != PlanarCleanFailClass::DirtyInput {
        return Err(DirtyPlanarCleanFailError::CleanFailDidNotRepresentDirtyInput);
    }
    if receipt.repair_attempt() != PlanarRepairAttempt::NotAttempted {
        return Err(DirtyPlanarCleanFailError::CleanFailAttemptedRepair);
    }
    if receipt.bounded_conversion() != PlanarBoundedConversion::NotAttempted {
        return Err(DirtyPlanarCleanFailError::CleanFailAttemptedBoundedConversion);
    }
    if receipt.truth_effect() != PlanarCleanFailTruthEffect::DoesNotChangePlanarTruth {
        return Err(DirtyPlanarCleanFailError::CleanFailChangedTruth);
    }
    let boundary_case = receipt
        .basis()
        .input()
        .dirty_input_kind()
        .map(DirtyPlanarCleanFailCase::from_dirty_input_kind)
        .ok_or(DirtyPlanarCleanFailError::CleanFailDidNotRepresentDirtyInput)?;
    if topology_case != boundary_case {
        return Err(DirtyPlanarCleanFailError::MismatchedDirtyKind {
            topology: topology_case,
            boundary: boundary_case,
        });
    }
    Ok(boundary_case)
}

pub(super) fn require_recovery_and_diagnostics(
    receipt: &PlanarCleanFailBoundaryReceipt,
) -> Result<(), DirtyPlanarCleanFailError> {
    if receipt.basis().recovery().blocker_kind() != PlanarRecoveryBlockerKind::DirtyInput
        || receipt.basis().recovery().source_posture() != PlanarRecoverySourcePosture::Dirty
    {
        return Err(DirtyPlanarCleanFailError::MissingRecoveryPosture);
    }
    if receipt.basis().recovery().truth_effect()
        != PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth
    {
        return Err(DirtyPlanarCleanFailError::RecoveryAttemptedTruthUpgrade);
    }
    if receipt.basis().diagnostics().truth_effect()
        != PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth
    {
        return Err(DirtyPlanarCleanFailError::CleanFailChangedTruth);
    }
    Ok(())
}

pub(super) fn require_transform_posture(
    receipt: &PlanarCleanFailBoundaryReceipt,
) -> Result<(), DirtyPlanarCleanFailError> {
    if receipt.basis().input().transform_posture_digest().is_none() {
        Err(DirtyPlanarCleanFailError::MissingTransformPosture)
    } else {
        Ok(())
    }
}

pub(super) fn require_stable_identity_does_not_hide_dirty_geometry(
    receipt: &PlanarCleanFailBoundaryReceipt,
    topology_clean_fail_identity: &str,
) -> Result<(), DirtyPlanarCleanFailError> {
    let Some(stable_identity) = receipt.basis().input().stable_topology_identity() else {
        return Ok(());
    };
    let Some(dirty_kind) = receipt.basis().input().dirty_input_kind() else {
        return Ok(());
    };
    if stable_identity == topology_clean_fail_identity {
        return Err(
            DirtyPlanarCleanFailError::StableTopologyIdentityHidDirtyGeometry { dirty_kind },
        );
    }
    Ok(())
}

pub(super) fn require_dirty_user_response(
    receipt: &WorthUserResponseReceipt,
    clean_fail_boundary: &PlanarCleanFailBoundaryReceipt,
) -> Result<(), DirtyPlanarCleanFailError> {
    let outcome = receipt.outcome();
    if outcome.kind() != WorthUserOutcomeKind::NoOptions
        || outcome.cause().map(|cause| cause.kind()) != Some(WorthUserOutcomeCauseKind::DirtyInput)
        || !outcome.choices().is_empty()
    {
        return Err(DirtyPlanarCleanFailError::UserResponseDidNotExplainDirtyNoOptions);
    }
    if outcome.evidence().source_identity() != clean_fail_boundary.clean_fail_boundary_digest() {
        return Err(DirtyPlanarCleanFailError::UserResponseDidNotConsumeCleanFailBoundary);
    }
    Ok(())
}

fn topology_dirty_case(
    receipt: &TopologySeedCleanFailReceipt,
) -> Result<DirtyPlanarCleanFailCase, DirtyPlanarCleanFailError> {
    match receipt.kind() {
        topology::facade::TopologySeedKind::SelfIntersectingLoop => {
            Ok(DirtyPlanarCleanFailCase::SelfIntersectingLoop)
        }
        topology::facade::TopologySeedKind::NonManifoldWire => {
            Ok(DirtyPlanarCleanFailCase::NonManifoldWire)
        }
        topology::facade::TopologySeedKind::ThinWallLocalBasis => {
            Ok(DirtyPlanarCleanFailCase::ThinWallOrInvalidLocalBasis)
        }
        topology::facade::TopologySeedKind::OrientationInconsistency => {
            Ok(DirtyPlanarCleanFailCase::OrientationInconsistency)
        }
        _ => Err(DirtyPlanarCleanFailError::MissingTopologyCleanFail),
    }
}
