use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticTruthEffect;
use crate::planar_contracts::planar_recovery::PlanarRecoveryTruthEffect;

use super::{
    PlanarM7BooleanExecutionSupport, PlanarM7ReadinessBasis, PlanarM7ReadinessDenial,
    PlanarM7ReadinessDenialKind,
};

pub(crate) fn validate_m7_readiness_basis(
    basis: &PlanarM7ReadinessBasis,
) -> Result<(), PlanarM7ReadinessDenial> {
    validate_base_receipt(basis)?;
    validate_boolean_readiness_roots(basis)?;
    validate_structural_motion_closure(basis)?;
    validate_retained_projection_closure(basis)?;
    validate_recovery_diagnostics_closure(basis)?;
    validate_support_posture(basis)?;
    Ok(())
}

fn validate_base_receipt(basis: &PlanarM7ReadinessBasis) -> Result<(), PlanarM7ReadinessDenial> {
    if !basis.boolean_readiness().is_ready_for_m7()
        || basis.boolean_readiness().boolean_result().is_some()
        || basis.boolean_readiness().imprint_action().is_some()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::BooleanExecutionAlreadyPresent,
            "M7 readiness can only freeze a pre-boolean readiness bundle",
        ));
    }
    Ok(())
}

fn validate_boolean_readiness_roots(
    basis: &PlanarM7ReadinessBasis,
) -> Result<(), PlanarM7ReadinessDenial> {
    let root = basis.boolean_readiness().fact_digest();
    require_root(
        basis
            .structural_identity()
            .basis()
            .boolean_readiness_receipt()
            .fact_digest(),
        root,
        "structural identity",
    )?;
    require_root(
        basis
            .motion_posture()
            .basis()
            .boolean_readiness_receipt()
            .fact_digest(),
        root,
        "motion posture",
    )?;
    require_root(
        basis
            .retained_planar_facts()
            .basis()
            .boolean_readiness_receipt()
            .fact_digest(),
        root,
        "retained planar facts",
    )?;
    Ok(())
}

fn validate_structural_motion_closure(
    basis: &PlanarM7ReadinessBasis,
) -> Result<(), PlanarM7ReadinessDenial> {
    let retained_motion = basis.motion_posture().retained_motion_digest();
    let structural_motion = basis
        .structural_identity()
        .basis()
        .motion_posture_receipt()
        .map(|receipt| receipt.retained_motion_digest());
    if structural_motion != Some(retained_motion) {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedMotionPosture,
            "structural identity must consume the same movement and rotation posture as M7 readiness",
        ));
    }
    let transform_motion = basis
        .structural_identity()
        .basis()
        .canonical_transform_basis()
        .movement_rotation_posture_identity();
    if transform_motion != retained_motion {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedMotionPosture,
            "canonical transform basis must match the typed movement and rotation posture receipt",
        ));
    }
    Ok(())
}

fn validate_retained_projection_closure(
    basis: &PlanarM7ReadinessBasis,
) -> Result<(), PlanarM7ReadinessDenial> {
    let retained_basis = basis.retained_planar_facts().basis();
    if retained_basis
        .structural_identity_receipt()
        .structural_identity_digest()
        != basis.structural_identity().structural_identity_digest()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedStructuralIdentity,
            "retained planar facts must freeze the supplied structural identity",
        ));
    }
    if retained_basis
        .motion_posture_receipt()
        .retained_motion_digest()
        != basis.motion_posture().retained_motion_digest()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedMotionPosture,
            "retained planar facts must freeze the supplied movement and rotation posture",
        ));
    }
    if basis
        .projection_consumed_facts()
        .retained_planar_fact_digest()
        != basis.retained_planar_facts().retained_fact_digest()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedRetainedFacts,
            "projection-consumed facts must come from the supplied retained planar facts",
        ));
    }
    if basis
        .projection_consumed_facts()
        .structural_identity_digest()
        != basis.structural_identity().structural_identity_digest()
        || basis.projection_consumed_facts().motion_posture_digest()
            != basis.motion_posture().retained_motion_digest()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedProjectionConsumption,
            "projection consumption must preserve retained structural identity and motion posture",
        ));
    }
    Ok(())
}

fn validate_recovery_diagnostics_closure(
    basis: &PlanarM7ReadinessBasis,
) -> Result<(), PlanarM7ReadinessDenial> {
    if basis.recovery_posture().truth_effect()
        != PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedRecoveryPosture,
            "M7 readiness recovery posture must not mutate planar truth",
        ));
    }
    let recovery_basis = basis.recovery_posture().basis();
    if recovery_basis
        .retained_planar_facts()
        .map(|receipt| receipt.retained_fact_digest())
        != Some(basis.retained_planar_facts().retained_fact_digest())
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedRecoveryPosture,
            "recovery posture must inspect the supplied retained planar facts",
        ));
    }
    if recovery_basis
        .projection_consumed_facts()
        .map(|receipt| receipt.projection_consumption_digest())
        != Some(
            basis
                .projection_consumed_facts()
                .projection_consumption_digest(),
        )
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedRecoveryPosture,
            "recovery posture must inspect the supplied projection-consumed facts",
        ));
    }
    if basis.diagnostics().truth_effect() != PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedDiagnostics,
            "M7 readiness diagnostics must localize without changing planar truth",
        ));
    }
    if !diagnostics_names_closeout_inputs(basis) {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedDiagnostics,
            "diagnostics must name recovery, retained, projection, or motion evidence consumed by readiness",
        ));
    }
    if let Some(clean_fail) = basis.clean_fail_boundary() {
        if clean_fail.basis().recovery().recovery_posture_digest()
            != basis.recovery_posture().recovery_posture_digest()
            || clean_fail.basis().diagnostics().diagnostic_bundle_digest()
                != basis.diagnostics().diagnostic_bundle_digest()
        {
            return Err(denial(
                PlanarM7ReadinessDenialKind::MismatchedRecoveryPosture,
                "clean-fail boundary must consume the supplied recovery and diagnostics receipts",
            ));
        }
    }
    Ok(())
}

fn validate_support_posture(basis: &PlanarM7ReadinessBasis) -> Result<(), PlanarM7ReadinessDenial> {
    if basis.support_posture().boolean_execution() != PlanarM7BooleanExecutionSupport::SupportGated
        || basis.support_posture().reason().trim().is_empty()
    {
        return Err(denial(
            PlanarM7ReadinessDenialKind::MissingSupportPosture,
            "M7 split/classify/assemble lanes must be explicitly support-gated",
        ));
    }
    Ok(())
}

fn diagnostics_names_closeout_inputs(basis: &PlanarM7ReadinessBasis) -> bool {
    let subject = basis.diagnostics().basis().subject();
    let source = subject.source_digest();
    if source == basis.recovery_posture().recovery_posture_digest()
        || source == basis.retained_planar_facts().retained_fact_digest()
        || source
            == basis
                .projection_consumed_facts()
                .projection_consumption_digest()
        || source == basis.motion_posture().retained_motion_digest()
    {
        return true;
    }
    subject.evidence().iter().any(|evidence| {
        let digest = evidence.evidence_digest();
        digest == basis.recovery_posture().recovery_posture_digest()
            || digest == basis.retained_planar_facts().retained_fact_digest()
            || digest
                == basis
                    .projection_consumed_facts()
                    .projection_consumption_digest()
            || digest == basis.motion_posture().retained_motion_digest()
    })
}

fn require_root(
    actual: &str,
    expected: &str,
    label: &'static str,
) -> Result<(), PlanarM7ReadinessDenial> {
    if actual == expected {
        Ok(())
    } else {
        Err(denial(
            PlanarM7ReadinessDenialKind::MismatchedBooleanReadinessRoot,
            format!("{label} must consume the same certified boolean-readiness root"),
        ))
    }
}

fn denial(kind: PlanarM7ReadinessDenialKind, reason: impl Into<String>) -> PlanarM7ReadinessDenial {
    PlanarM7ReadinessDenial::new(kind, reason)
}
