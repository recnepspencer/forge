use super::{
    PlanarStructuralIdentityBasis, PlanarStructuralIdentityDenial,
    PlanarStructuralIdentityDenialKind,
};

pub(crate) fn validate_planar_structural_identity_basis(
    basis: &PlanarStructuralIdentityBasis,
) -> Result<(), PlanarStructuralIdentityDenial> {
    reject_coordinate_only_identity_basis(basis)?;
    validate_contrast_identity_presence(basis)?;
    validate_contrast_identities_are_not_structural_authority(basis)?;
    validate_transform_basis_matches_bundle(basis)
}

fn reject_coordinate_only_identity_basis(
    basis: &PlanarStructuralIdentityBasis,
) -> Result<(), PlanarStructuralIdentityDenial> {
    if basis.final_coordinate_digest().is_some() {
        Err(denial(
            PlanarStructuralIdentityDenialKind::CoordinateOnlyIdentityBasis,
            "final coordinates may be inspected but cannot be the planar structural identity basis",
        ))
    } else {
        Ok(())
    }
}

fn validate_contrast_identity_presence(
    basis: &PlanarStructuralIdentityBasis,
) -> Result<(), PlanarStructuralIdentityDenial> {
    let all_present = !basis.topology_identity().is_empty()
        && !basis.persistent_name().is_empty()
        && !basis.binding_identity().is_empty()
        && !basis.lineage_identity().is_empty();
    if all_present {
        Ok(())
    } else {
        Err(denial(
            PlanarStructuralIdentityDenialKind::MissingContrastIdentity,
            "topology, name, binding, and lineage identities must be present as contrast rows",
        ))
    }
}

fn validate_contrast_identities_are_not_structural_authority(
    basis: &PlanarStructuralIdentityBasis,
) -> Result<(), PlanarStructuralIdentityDenial> {
    let transform = basis.canonical_transform_basis();
    let structural_authorities = [
        basis.boolean_readiness_receipt().fact_digest(),
        transform.transform_chain_digest(),
        transform.movement_rotation_posture_identity(),
        transform.local_frame_identity(),
    ];
    let contrast_rows = [
        basis.topology_identity(),
        basis.persistent_name(),
        basis.binding_identity(),
        basis.lineage_identity(),
    ];
    if contrast_rows
        .iter()
        .any(|contrast| structural_authorities.contains(contrast))
    {
        Err(denial(
            PlanarStructuralIdentityDenialKind::IdentityAuthoritySubstitution,
            "topology, name, binding, and lineage identities are contrast rows, not structural authority",
        ))
    } else {
        Ok(())
    }
}

fn validate_transform_basis_matches_bundle(
    basis: &PlanarStructuralIdentityBasis,
) -> Result<(), PlanarStructuralIdentityDenial> {
    let bundle_basis = basis.boolean_readiness_receipt().basis();
    let transform = basis.canonical_transform_basis();
    let frame = bundle_basis.local_frame_receipt().basis();
    if let Some(motion) = basis.motion_posture_receipt() {
        let motion_matches_bundle = motion.basis().boolean_readiness_receipt().fact_digest()
            == basis.boolean_readiness_receipt().fact_digest()
            && transform.movement_rotation_posture_identity() == motion.retained_motion_digest();
        if !motion_matches_bundle {
            return Err(denial(
                PlanarStructuralIdentityDenialKind::BundleTransformMismatch,
                "typed planar motion posture must match the consumed boolean-readiness bundle and transform basis",
            ));
        }
    }
    if transform.local_frame_identity() == frame.frame_identity()
        && (transform.movement_rotation_posture_identity()
            == bundle_basis.movement_rotation_posture_identity()
            || basis.motion_posture_receipt().is_some())
        && transform.transform_chain_digest() == frame.transform_chain_digest()
    {
        Ok(())
    } else {
        Err(denial(
            PlanarStructuralIdentityDenialKind::BundleTransformMismatch,
            "canonical transform basis must match the consumed boolean-readiness bundle frame and posture",
        ))
    }
}

fn denial(
    kind: PlanarStructuralIdentityDenialKind,
    reason: &'static str,
) -> PlanarStructuralIdentityDenial {
    PlanarStructuralIdentityDenial::new(kind, reason)
}
