use std::collections::BTreeSet;

use super::{
    PlanarContractBundleDenial, PlanarContractBundleDenialKind, PlanarContractBundleFamily,
    PlanarContractBundleValidationBasis,
};

pub(crate) fn validate_projection_consumption_closure(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    validate_projections_consume_frame(basis)?;
    validate_winding_consumes_supplied_projections(basis)?;
    validate_segments_consume_supplied_projections(basis)?;
    validate_supplied_projection_rows_are_consumed(basis)
}

fn validate_projections_consume_frame(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    if basis.projection_receipts().iter().all(|receipt| {
        receipt.local_frame_fact_digest() == basis.local_frame_receipt().fact_digest()
    }) {
        Ok(())
    } else {
        Err(mismatched_projection_family(
            "projection receipts must consume the supplied local-frame receipt",
        ))
    }
}

fn validate_winding_consumes_supplied_projections(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let projection_digests = projection_fact_digest_set(basis);
    if basis
        .winding_receipt()
        .basis()
        .projected_vertex_fact_digests()
        .into_iter()
        .all(|digest| projection_digests.contains(digest))
    {
        Ok(())
    } else {
        Err(denial(
            PlanarContractBundleFamily::PolygonWinding,
            "winding receipt must consume only projection receipts supplied by the bundle",
        ))
    }
}

fn validate_segments_consume_supplied_projections(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let projection_digests = projection_fact_digest_set(basis);
    for receipt in basis.segment_receipts() {
        if !receipt
            .basis()
            .endpoint_projection_fact_digests()
            .into_iter()
            .all(|digest| projection_digests.contains(digest))
        {
            return Err(denial(
                PlanarContractBundleFamily::SegmentContact,
                "segment-contact receipts must consume projection receipts supplied by the bundle",
            ));
        }
    }
    Ok(())
}

fn validate_supplied_projection_rows_are_consumed(
    basis: &PlanarContractBundleValidationBasis,
) -> Result<(), PlanarContractBundleDenial> {
    let supplied_projection_digests = projection_fact_digest_set(basis);
    if supplied_projection_digests.len() != basis.projection_receipts().len() {
        return Err(mismatched_projection_family(
            "projection-consumption rows must not contain duplicate projection receipts",
        ));
    }
    let consumed_projection_digests = consumed_projection_fact_digest_set(basis);
    if supplied_projection_digests
        .iter()
        .all(|digest| consumed_projection_digests.contains(*digest))
    {
        Ok(())
    } else {
        Err(mismatched_projection_family(
            "every supplied projection-consumption receipt must be consumed by retained bundle evidence",
        ))
    }
}

fn projection_fact_digest_set(basis: &PlanarContractBundleValidationBasis) -> BTreeSet<&str> {
    basis
        .projection_receipts()
        .iter()
        .map(|receipt| receipt.fact_digest())
        .collect()
}

fn consumed_projection_fact_digest_set(
    basis: &PlanarContractBundleValidationBasis,
) -> BTreeSet<&str> {
    let mut consumed = BTreeSet::new();
    consumed.extend(
        basis
            .winding_receipt()
            .basis()
            .projected_vertex_fact_digests(),
    );
    consumed.extend(
        basis
            .signed_area_receipt()
            .basis()
            .winding_receipt()
            .basis()
            .projected_vertex_fact_digests(),
    );
    consumed.extend(
        basis
            .overlap_receipt()
            .basis()
            .first_face()
            .signed_area_receipt()
            .basis()
            .winding_receipt()
            .basis()
            .projected_vertex_fact_digests(),
    );
    consumed.extend(
        basis
            .overlap_receipt()
            .basis()
            .second_face()
            .signed_area_receipt()
            .basis()
            .winding_receipt()
            .basis()
            .projected_vertex_fact_digests(),
    );
    for receipt in basis.segment_receipts() {
        consumed.extend(receipt.basis().endpoint_projection_fact_digests());
    }
    consumed
}

fn mismatched_projection_family(reason: &'static str) -> PlanarContractBundleDenial {
    denial(PlanarContractBundleFamily::ProjectionConsumption, reason)
}

fn denial(family: PlanarContractBundleFamily, reason: &'static str) -> PlanarContractBundleDenial {
    PlanarContractBundleDenial::new(
        PlanarContractBundleDenialKind::MismatchedCertificateFamily,
        Some(family),
        reason,
    )
}
