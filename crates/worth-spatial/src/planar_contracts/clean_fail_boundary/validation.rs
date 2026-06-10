use crate::planar_contracts::admission::{PlanarAdmissionClass, PlanarAdmissionFamily};
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticTruthEffect;
use crate::planar_contracts::planar_recovery::{
    PlanarRecoveryBlockerKind, PlanarRecoverySourcePosture, PlanarRecoveryTruthEffect,
};

use super::{
    PlanarBoundedConversion, PlanarCleanFailBoundaryBasis, PlanarCleanFailBoundaryDenial,
    PlanarCleanFailBoundaryDenialKind, PlanarCleanFailClass, PlanarRepairAttempt,
};

pub(crate) fn validate_planar_clean_fail_boundary_basis(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Result<(), PlanarCleanFailBoundaryDenial> {
    validate_source_and_admission(basis)?;
    validate_no_repair_or_conversion(basis)?;
    validate_recovery_posture(basis)?;
    validate_diagnostics(basis)?;
    Ok(())
}

fn validate_source_and_admission(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Result<(), PlanarCleanFailBoundaryDenial> {
    if basis.input().source_digest().trim().is_empty() {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MissingSourceDigest,
            "clean-fail boundary requires typed dirty or unbounded source digest",
        ));
    }
    let row = basis.input().admission_row().ok_or_else(|| {
        deny(
            PlanarCleanFailBoundaryDenialKind::MissingAdmissionRow,
            "clean-fail boundary requires the Phase 1 admission row",
        )
    })?;
    if row.family() != expected_admission_family(basis.input().class()) {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MismatchedAdmissionFamily,
            "clean-fail boundary admission row must match dirty or unbounded class",
        ));
    }
    if row.class() == PlanarAdmissionClass::Admitted {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::AdmissionRowAdmitsRuntime,
            "dirty or unbounded clean-fail boundary cannot consume admitted runtime row",
        ));
    }
    if basis.input().transform_posture_digest().is_none() {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MissingTransformPosture,
            "clean-fail boundary requires explicit movement and rotation posture",
        ));
    }
    Ok(())
}

fn validate_no_repair_or_conversion(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Result<(), PlanarCleanFailBoundaryDenial> {
    if basis.repair_attempt() != PlanarRepairAttempt::NotAttempted {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::HeuristicRepairAttempted,
            "dirty planar input may not be healed or repaired in M6",
        ));
    }
    if basis.bounded_conversion() != PlanarBoundedConversion::NotAttempted {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::BoundedConversionAttempted,
            "unbounded/open planar input may not be clipped or converted to bounded truth in M6",
        ));
    }
    Ok(())
}

fn validate_recovery_posture(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Result<(), PlanarCleanFailBoundaryDenial> {
    if basis.recovery().truth_effect() != PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MismatchedRecoveryPosture,
            "clean-fail recovery must not change planar truth",
        ));
    }
    let expected = match basis.input().class() {
        PlanarCleanFailClass::DirtyInput => (
            PlanarRecoveryBlockerKind::DirtyInput,
            PlanarRecoverySourcePosture::Dirty,
        ),
        PlanarCleanFailClass::UnboundedOrOpen => (
            PlanarRecoveryBlockerKind::UnsupportedPlanarClass,
            PlanarRecoverySourcePosture::Unsupported,
        ),
    };
    if (
        basis.recovery().blocker_kind(),
        basis.recovery().source_posture(),
    ) != expected
    {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MismatchedRecoveryPosture,
            "clean-fail boundary recovery posture must match dirty or unbounded class",
        ));
    }
    if basis.recovery().basis().source().source_digest() != basis.input().source_digest() {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MismatchedRecoveryPosture,
            "clean-fail boundary recovery source must match the clean-fail input source",
        ));
    }
    Ok(())
}

fn validate_diagnostics(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Result<(), PlanarCleanFailBoundaryDenial> {
    if basis.diagnostics().truth_effect() != PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::DiagnosticChangedTruth,
            "clean-fail diagnostics must explain without changing planar truth",
        ));
    }
    if basis.diagnostics().basis().subject().source_digest() != basis.input().source_digest() {
        return Err(deny(
            PlanarCleanFailBoundaryDenialKind::MismatchedDiagnostics,
            "clean-fail diagnostics must name the same dirty or unbounded source as the boundary",
        ));
    }
    Ok(())
}

fn expected_admission_family(class: PlanarCleanFailClass) -> PlanarAdmissionFamily {
    match class {
        PlanarCleanFailClass::DirtyInput => PlanarAdmissionFamily::DirtyPlanarInput,
        PlanarCleanFailClass::UnboundedOrOpen => PlanarAdmissionFamily::UnboundedPlanarDomain,
    }
}

fn deny(
    kind: PlanarCleanFailBoundaryDenialKind,
    reason: &'static str,
) -> PlanarCleanFailBoundaryDenial {
    PlanarCleanFailBoundaryDenial::new(kind, reason)
}
