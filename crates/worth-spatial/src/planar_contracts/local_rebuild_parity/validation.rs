use crate::bindings::query_native_rebinding_neighborhood_replacement::TopologyNeighborhoodReplacementScope;
use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticTruthEffect;
use crate::planar_contracts::planar_recovery::PlanarRecoveryTruthEffect;

use super::{
    PlanarLocalRebuildParityBasis, PlanarLocalRebuildParityDenial,
    PlanarLocalRebuildParityDenialKind, PlanarRebindingContinuityKind,
};

pub(crate) fn validate_planar_local_rebuild_parity_basis(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    validate_local_neighborhood(basis)?;
    validate_rebinding_authority(basis)?;
    validate_retained_projection_parity(basis)?;
    validate_structural_motion_topology_parity(basis)?;
    validate_no_truth_reclassification(basis)?;
    Ok(())
}

fn validate_local_neighborhood(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    if basis.rebuild_scope().scope_identity().trim().is_empty()
        || basis
            .neighborhood()
            .replacement_neighborhood_identity()
            .trim()
            .is_empty()
        || basis.neighborhood().affected_target_identities().is_empty()
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MissingLocalNeighborhood,
            "local planar rebuild parity requires explicit grouped neighborhood replacement facts",
        ));
    }
    if basis.neighborhood().replacement_scope()
        != TopologyNeighborhoodReplacementScope::LocalNeighborhood
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::BroadSearchNotAllowed,
            "local planar rebuild parity denies broad search or unbounded replacement scope",
        ));
    }
    Ok(())
}

fn validate_rebinding_authority(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    if basis.rebinding().continuity_digest().trim().is_empty() {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MissingRebindingContinuity,
            "local planar rebuild parity requires typed rebinding continuity evidence",
        ));
    }
    match basis.rebinding().kind() {
        PlanarRebindingContinuityKind::QueryContinuation => {
            if basis.rebinding().neighborhood_replacement_digest()
                != basis.neighborhood().fact_digest()
            {
                return Err(deny(
                    PlanarLocalRebuildParityDenialKind::MismatchedRebindingNeighborhood,
                    "rebinding continuity must be bound to the supplied local neighborhood replacement facts",
                ));
            }
            Ok(())
        }
        PlanarRebindingContinuityKind::CorrespondenceOnly => Err(deny(
            PlanarLocalRebuildParityDenialKind::CorrespondenceOnlyRebinding,
            "correspondence-only rebinding cannot certify planar rebuild continuity",
        )),
        PlanarRebindingContinuityKind::KernelSummary => Err(deny(
            PlanarLocalRebuildParityDenialKind::KernelSummaryNotAuthority,
            "kernel-local summaries cannot certify planar rebuild continuity",
        )),
    }
}

fn validate_retained_projection_parity(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    if basis.projection_consumed().retained_planar_fact_digest()
        != basis.retained().retained_fact_digest()
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MismatchedRetainedProjectionBasis,
            "projection-consumed planar facts must come from the supplied retained planar facts",
        ));
    }
    Ok(())
}

fn validate_structural_motion_topology_parity(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    if basis.projection_consumed().structural_identity_digest()
        != basis.structural_identity().structural_identity_digest()
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MismatchedStructuralIdentityBasis,
            "local rebuild parity must use the same structural identity as projection consumption",
        ));
    }
    if basis.projection_consumed().motion_posture_digest()
        != basis.motion().retained_motion_digest()
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MismatchedMotionPostureBasis,
            "local rebuild parity must preserve explicit movement and rotation posture",
        ));
    }
    if basis.projection_consumed().topology_contract_digest() != basis.topology().fact_digest() {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::MismatchedTopologyBasis,
            "local rebuild parity must consume the same topology completeness facts as projection consumption",
        ));
    }
    if basis
        .neighborhood()
        .existing_target_identity_basis()
        .trim()
        .is_empty()
    {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::ProjectionConsumedIdentityRecomputed,
            "local rebuild must carry existing target identity from rebinding authority",
        ));
    }
    Ok(())
}

fn validate_no_truth_reclassification(
    basis: &PlanarLocalRebuildParityBasis,
) -> Result<(), PlanarLocalRebuildParityDenial> {
    if basis.recovery().truth_effect() != PlanarRecoveryTruthEffect::DoesNotChangePlanarTruth {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::RecoveryReclassifiedTruth,
            "recovery posture must not reclassify planar truth during rebuild parity",
        ));
    }
    if basis.diagnostics().truth_effect() != PlanarDiagnosticTruthEffect::DoesNotChangePlanarTruth {
        return Err(deny(
            PlanarLocalRebuildParityDenialKind::DiagnosticReclassifiedTruth,
            "diagnostics must localize mismatch without changing planar truth",
        ));
    }
    Ok(())
}

fn deny(
    kind: PlanarLocalRebuildParityDenialKind,
    reason: &'static str,
) -> PlanarLocalRebuildParityDenial {
    PlanarLocalRebuildParityDenial::new(kind, reason)
}
