use super::{RetainedPlanarFactsBasis, RetainedPlanarFactsDenial, RetainedPlanarFactsDenialKind};

pub(crate) fn validate_retained_planar_facts_basis(
    basis: &RetainedPlanarFactsBasis,
) -> Result<(), RetainedPlanarFactsDenial> {
    let bundle = basis.boolean_readiness_receipt();
    if bundle.basis().family_rows().is_empty() {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::MissingRetainedFamilyRows,
            "retained planar facts require non-empty retained family rows from the boolean-readiness bundle",
        ));
    }
    if bundle
        .basis()
        .family_rows()
        .iter()
        .any(|row| row.retained_fact_digests().is_empty())
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::TruncatedRetainedBasis,
            "retained planar facts require every bundle family row to carry retained fact digests",
        ));
    }
    if basis
        .structural_identity_receipt()
        .basis()
        .boolean_readiness_receipt()
        .fact_digest()
        != bundle.fact_digest()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::MismatchedBooleanReadinessBasis,
            "structural identity must retain the same boolean-readiness receipt being frozen",
        ));
    }
    if basis
        .motion_posture_receipt()
        .basis()
        .boolean_readiness_receipt()
        .fact_digest()
        != bundle.fact_digest()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::MismatchedBooleanReadinessBasis,
            "motion posture must retain the same boolean-readiness receipt being frozen",
        ));
    }
    if basis.topology_contract_receipt().fact_digest()
        != bundle.basis().topology_contract_receipt().fact_digest()
    {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::MismatchedTopologyContract,
            "retained planar facts require the topology contract receipt consumed by the bundle",
        ));
    }
    let retained_motion = basis
        .structural_identity_receipt()
        .basis()
        .motion_posture_receipt()
        .map(|receipt| receipt.retained_motion_digest());
    if retained_motion != Some(basis.motion_posture_receipt().retained_motion_digest()) {
        return Err(RetainedPlanarFactsDenial::new(
            RetainedPlanarFactsDenialKind::MismatchedMotionPosture,
            "structural identity must retain the same movement and rotation posture being frozen",
        ));
    }
    Ok(())
}
