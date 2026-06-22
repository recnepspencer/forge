use std::collections::BTreeSet;

use super::{
    ProjectionConsumedPlanarFactsBasis, ProjectionConsumedPlanarFactsDenial,
    ProjectionConsumedPlanarFactsDenialKind,
};

pub(crate) fn validate_projection_consumed_planar_facts_basis(
    basis: &ProjectionConsumedPlanarFactsBasis,
) -> Result<(), ProjectionConsumedPlanarFactsDenial> {
    if basis.projection_receipts().is_empty() {
        return Err(ProjectionConsumedPlanarFactsDenial::new(
            ProjectionConsumedPlanarFactsDenialKind::MissingProjectionReceipts,
            "projection-consumed planar facts require certified projection receipts from the retained boolean-readiness bundle",
        ));
    }
    validate_no_duplicate_projection_receipts(basis)?;
    validate_projection_receipts_match_retained_bundle(basis)
}

fn validate_no_duplicate_projection_receipts(
    basis: &ProjectionConsumedPlanarFactsBasis,
) -> Result<(), ProjectionConsumedPlanarFactsDenial> {
    let supplied = supplied_projection_fact_digests(basis);
    if supplied.len() == basis.projection_receipts().len() {
        Ok(())
    } else {
        Err(ProjectionConsumedPlanarFactsDenial::new(
            ProjectionConsumedPlanarFactsDenialKind::DuplicateProjectionReceipt,
            "projection-consumed planar facts require unique projection receipt facts",
        ))
    }
}

fn validate_projection_receipts_match_retained_bundle(
    basis: &ProjectionConsumedPlanarFactsBasis,
) -> Result<(), ProjectionConsumedPlanarFactsDenial> {
    let supplied = supplied_projection_fact_digests(basis);
    let retained = retained_bundle_projection_fact_digests(basis);
    if supplied == retained {
        Ok(())
    } else {
        Err(ProjectionConsumedPlanarFactsDenial::new(
            ProjectionConsumedPlanarFactsDenialKind::MismatchedProjectionClosure,
            "projection-consumed planar facts require the exact projection receipt set retained by the boolean-readiness bundle",
        ))
    }
}

fn supplied_projection_fact_digests(basis: &ProjectionConsumedPlanarFactsBasis) -> BTreeSet<&str> {
    basis
        .projection_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect()
}

fn retained_bundle_projection_fact_digests(
    basis: &ProjectionConsumedPlanarFactsBasis,
) -> BTreeSet<&str> {
    basis
        .retained_planar_facts_receipt()
        .basis()
        .boolean_readiness_receipt()
        .basis()
        .projection_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect()
}
