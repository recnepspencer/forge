use std::collections::BTreeSet;

use super::{
    PlanarContractBundleDenial, PlanarContractBundleDenialKind, PlanarContractBundleFamily,
    PlanarContractBundleValidationBasis,
};

pub(crate) fn validate_predicate_consumption_closure(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let predicate_consumption = basis.predicate_consumption_receipt();
    let predicate_consumption_basis = predicate_consumption.basis();
    if !predicate_consumption.proves_no_second_predicate_engine() {
        return Err(predicate_consumption_mismatch(
            "predicate-consumption receipt must prove no second predicate engine",
        ));
    }
    if predicate_consumption_basis.topology_basis_identity() != basis.topology_basis_identity() {
        return Err(predicate_consumption_mismatch(
            "predicate-consumption topology basis must match the readiness bundle",
        ));
    }
    if predicate_consumption_basis.movement_rotation_posture_identity()
        != basis.movement_rotation_posture_identity()
    {
        return Err(predicate_consumption_mismatch(
            "predicate-consumption movement/rotation posture must match the readiness bundle",
        ));
    }
    if predicate_consumption_basis.local_frame_identity()
        != basis.local_frame_receipt().basis().frame_identity()
    {
        return Err(predicate_consumption_mismatch(
            "predicate-consumption local frame must match the readiness bundle frame",
        ));
    }
    validate_predicate_authority_closure(basis)?;
    validate_segment_contact_closure(basis)
}

fn validate_predicate_authority_closure(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let mut expected_predicates =
        BTreeSet::from([basis.precision_receipt().predicate_fact_digest()]);
    expected_predicates.extend(
        basis
            .predicate_consumption_receipt()
            .basis()
            .predicate_receipts()
            .iter()
            .map(|receipt| receipt.fact_digest()),
    );
    let supplied_predicates = basis
        .predicate_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect::<BTreeSet<_>>();
    if supplied_predicates == expected_predicates
        && supplied_predicates.len() == basis.predicate_receipts().len()
    {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleFamily::PredicateAuthority,
            "predicate authority rows must be exactly the precision predicate plus predicate-consumption predicates",
        ))
    }
}

fn validate_segment_contact_closure(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let expected_segments = basis
        .predicate_consumption_receipt()
        .basis()
        .segment_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect::<BTreeSet<_>>();
    let supplied_segments = basis
        .segment_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect::<BTreeSet<_>>();
    if supplied_segments == expected_segments
        && supplied_segments.len() == basis.segment_receipts().len()
    {
        Ok(())
    } else {
        Err(predicate_consumption_mismatch(
            "segment-contact rows must be exactly the rows covered by predicate-consumption",
        ))
    }
}

fn predicate_consumption_mismatch(reason: &'static str) -> PlanarContractBundleDenial {
    denial(
        PlanarContractBundleFamily::PredicateCertificateConsumption,
        reason,
    )
}

fn denial(family: PlanarContractBundleFamily, reason: &'static str) -> PlanarContractBundleDenial {
    PlanarContractBundleDenial::new(
        PlanarContractBundleDenialKind::MismatchedCertificateFamily,
        Some(family),
        reason,
    )
}
