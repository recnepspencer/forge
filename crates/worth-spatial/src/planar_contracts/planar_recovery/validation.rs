use super::{
    PlanarRecoveryBlockerKind, PlanarRecoveryPostureBasis, PlanarRecoveryPostureDenial,
    PlanarRecoveryPostureDenialKind, PlanarRecoverySource, PlanarRecoverySourceKind,
};

pub(crate) fn validate_planar_recovery_source_authority(
    source: &PlanarRecoverySource,
) -> Result<(), PlanarRecoveryPostureDenial> {
    if source.source_digest().trim().is_empty() {
        return Err(PlanarRecoveryPostureDenial::new(
            PlanarRecoveryPostureDenialKind::MissingRecoverySource,
            "planar recovery requires a typed source digest",
        ));
    }
    if source.kind() == PlanarRecoverySourceKind::KernelSummary {
        return Err(PlanarRecoveryPostureDenial::new(
            PlanarRecoveryPostureDenialKind::SummarySourceNotAuthority,
            "kernel summaries are not authority for planar recovery posture",
        ));
    }
    Ok(())
}

pub(crate) fn validate_planar_recovery_posture_basis(
    basis: &PlanarRecoveryPostureBasis,
) -> Result<(), PlanarRecoveryPostureDenial> {
    validate_planar_recovery_source_authority(basis.source())?;
    match basis.blocker_kind() {
        PlanarRecoveryBlockerKind::ProjectionBasis
        | PlanarRecoveryBlockerKind::RetainedOrProjectionBasis => {
            validate_retained_projection_basis_match(basis)?;
        }
        PlanarRecoveryBlockerKind::DirtyInput
        | PlanarRecoveryBlockerKind::UnsupportedPlanarClass => {}
    }
    Ok(())
}

fn validate_retained_projection_basis_match(
    basis: &PlanarRecoveryPostureBasis,
) -> Result<(), PlanarRecoveryPostureDenial> {
    let retained = basis.retained_planar_facts().ok_or_else(|| {
        PlanarRecoveryPostureDenial::new(
            PlanarRecoveryPostureDenialKind::MissingRetainedPlanarFacts,
            "planar recovery over projection or retained blockers requires retained planar facts",
        )
    })?;
    let projection_consumed = basis.projection_consumed_facts().ok_or_else(|| {
        PlanarRecoveryPostureDenial::new(
            PlanarRecoveryPostureDenialKind::MissingProjectionConsumedPlanarFacts,
            "planar recovery over projection or retained blockers requires projection-consumed planar facts",
        )
    })?;
    if projection_consumed.retained_planar_fact_digest() != retained.retained_fact_digest() {
        return Err(PlanarRecoveryPostureDenial::new(
            PlanarRecoveryPostureDenialKind::MismatchedRetainedProjectionBasis,
            "planar recovery requires projection-consumed facts derived from the same retained planar facts",
        ));
    }
    Ok(())
}
