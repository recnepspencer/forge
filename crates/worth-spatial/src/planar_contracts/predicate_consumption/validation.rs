use std::collections::{BTreeMap, BTreeSet};

use crate::planar_contracts::predicate_authority::PlanarPredicateFactReceipt;

use super::{
    PredicateCertificateConsumerKind, PredicateCertificateConsumptionBasis,
    PredicateCertificateConsumptionDenial, PredicateCertificateConsumptionDenialKind,
    PredicateCertificateConsumptionRow,
};

pub(crate) fn validate_predicate_certificate_consumption_basis(
    basis: &mut PredicateCertificateConsumptionBasis,
) -> Result<(), PredicateCertificateConsumptionDenial> {
    validate_required_scope(basis)?;
    let supplied_predicates = supplied_predicate_receipt_map(basis)?;
    validate_supplied_predicate_basis(basis, &supplied_predicates)?;
    let consumed_digests = consumed_predicate_digest_rows(basis)?;
    let rows = materialize_consumption_rows(&supplied_predicates, &consumed_digests)?;
    validate_every_supplied_predicate_is_consumed(&supplied_predicates, &rows)?;
    basis.set_consumption_rows(rows);
    Ok(())
}

fn validate_required_scope(
    basis: &PredicateCertificateConsumptionBasis,
) -> Result<(), PredicateCertificateConsumptionDenial> {
    if basis.predicate_receipts().is_empty() {
        return Err(denial(
            PredicateCertificateConsumptionDenialKind::MissingPredicateAuthority,
            "predicate consumption validation requires predicate authority receipts",
        ));
    }
    if basis.segment_receipts().is_empty()
        && basis.winding_receipt().is_none()
        && basis.signed_area_receipt().is_none()
        && basis.overlap_receipt().is_none()
    {
        return Err(denial(
            PredicateCertificateConsumptionDenialKind::MissingPredicateConsumer,
            "predicate consumption validation requires at least one retained predicate consumer",
        ));
    }
    Ok(())
}

fn supplied_predicate_receipt_map<'a>(
    basis: &'a PredicateCertificateConsumptionBasis,
) -> Result<BTreeMap<&'a str, &'a PlanarPredicateFactReceipt>, PredicateCertificateConsumptionDenial>
{
    let mut predicates = BTreeMap::new();
    for receipt in basis.predicate_receipts() {
        if receipt.fact_digest().is_empty()
            || receipt.declaration_digest().is_empty()
            || receipt.envelope_digest().is_empty()
        {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::SubstitutePredicateEvidence,
                "predicate authority receipt must carry fact, declaration, and envelope digests",
            ));
        }
        if predicates.insert(receipt.fact_digest(), receipt).is_some() {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::DuplicatePredicateReceipt,
                "predicate authority receipts must be unique by fact digest",
            ));
        }
    }
    Ok(predicates)
}

fn validate_supplied_predicate_basis(
    basis: &PredicateCertificateConsumptionBasis,
    predicates: &BTreeMap<&str, &PlanarPredicateFactReceipt>,
) -> Result<(), PredicateCertificateConsumptionDenial> {
    for predicate in predicates.values() {
        if predicate.input_basis().topology_basis_identity() != basis.topology_basis_identity() {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::TopologyBasisMismatch,
                "predicate receipt topology basis must match the validator scope",
            ));
        }
        if predicate.input_basis().movement_rotation_posture_identity()
            != basis.movement_rotation_posture_identity()
        {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::MovementRotationPostureMismatch,
                "predicate receipt movement/rotation posture must match the validator scope",
            ));
        }
        if predicate.input_basis().local_frame_identity() != basis.local_frame_identity() {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::LocalFrameMismatch,
                "predicate receipt local frame identity must match the validator scope",
            ));
        }
        if predicate
            .input_basis()
            .tolerance_policy_identity()
            .is_empty()
        {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::SubstitutePredicateEvidence,
                "predicate receipt must carry tolerance policy identity",
            ));
        }
        if format!("{:?}", predicate.precision_escalation()).is_empty() {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::MissingPrecisionMetadata,
                "predicate receipt must carry worth-math precision escalation metadata",
            ));
        }
    }
    Ok(())
}

fn consumed_predicate_digest_rows(
    basis: &PredicateCertificateConsumptionBasis,
) -> Result<
    Vec<(PredicateCertificateConsumerKind, String, String)>,
    PredicateCertificateConsumptionDenial,
> {
    let mut rows = Vec::new();
    for segment in basis.segment_receipts() {
        validate_segment_scope(basis, segment.basis().topology_basis_identity())?;
        for digest in segment.basis().orientation_fact_digests() {
            rows.push((
                PredicateCertificateConsumerKind::SegmentContact,
                segment.fact_digest().to_string(),
                digest.to_string(),
            ));
        }
    }
    if let Some(winding) = basis.winding_receipt() {
        rows.extend(
            winding
                .basis()
                .winding_predicate_fact_digests()
                .iter()
                .map(|digest| {
                    (
                        PredicateCertificateConsumerKind::PolygonWinding,
                        winding.fact_digest().to_string(),
                        digest.clone(),
                    )
                }),
        );
    }
    if let Some(signed_area) = basis.signed_area_receipt() {
        rows.extend(
            signed_area
                .basis()
                .winding_receipt()
                .basis()
                .winding_predicate_fact_digests()
                .iter()
                .map(|digest| {
                    (
                        PredicateCertificateConsumerKind::SignedArea,
                        signed_area.fact_digest().to_string(),
                        digest.clone(),
                    )
                }),
        );
    }
    if let Some(overlap) = basis.overlap_receipt() {
        rows.extend(overlap_face_predicate_rows(
            PredicateCertificateConsumerKind::CoplanarOverlap,
            overlap.fact_digest(),
            overlap
                .basis()
                .first_face()
                .signed_area_receipt()
                .basis()
                .winding_receipt()
                .basis()
                .winding_predicate_fact_digests(),
        ));
        rows.extend(overlap_face_predicate_rows(
            PredicateCertificateConsumerKind::CoplanarOverlap,
            overlap.fact_digest(),
            overlap
                .basis()
                .second_face()
                .signed_area_receipt()
                .basis()
                .winding_receipt()
                .basis()
                .winding_predicate_fact_digests(),
        ));
    }
    if rows.is_empty() {
        return Err(denial(
            PredicateCertificateConsumptionDenialKind::MissingPredicateConsumer,
            "retained consumers must expose predicate fact digest consumption rows",
        ));
    }
    Ok(rows)
}

fn validate_segment_scope(
    basis: &PredicateCertificateConsumptionBasis,
    topology_basis_identity: &str,
) -> Result<(), PredicateCertificateConsumptionDenial> {
    if topology_basis_identity == basis.topology_basis_identity() {
        Ok(())
    } else {
        Err(denial(
            PredicateCertificateConsumptionDenialKind::TopologyBasisMismatch,
            "segment predicate consumer topology basis must match validator scope",
        ))
    }
}

fn overlap_face_predicate_rows(
    kind: PredicateCertificateConsumerKind,
    consumer_fact_digest: &str,
    predicate_digests: &[String],
) -> Vec<(PredicateCertificateConsumerKind, String, String)> {
    predicate_digests
        .iter()
        .map(|digest| (kind, consumer_fact_digest.to_string(), digest.clone()))
        .collect()
}

fn materialize_consumption_rows(
    supplied_predicates: &BTreeMap<&str, &PlanarPredicateFactReceipt>,
    consumed_digests: &[(PredicateCertificateConsumerKind, String, String)],
) -> Result<Vec<PredicateCertificateConsumptionRow>, PredicateCertificateConsumptionDenial> {
    let mut rows = Vec::new();
    for (consumer_kind, consumer_digest, predicate_digest) in consumed_digests {
        let Some(predicate) = supplied_predicates.get(predicate_digest.as_str()) else {
            return Err(denial(
                PredicateCertificateConsumptionDenialKind::MissingConsumedPredicateReceipt,
                "predicate consumer references a predicate fact digest not supplied to the validator",
            ));
        };
        rows.push(PredicateCertificateConsumptionRow::new(
            *consumer_kind,
            consumer_digest,
            predicate,
        ));
    }
    rows.sort_by(|left, right| {
        left.consumer_kind()
            .cmp(&right.consumer_kind())
            .then_with(|| {
                left.consumer_fact_digest()
                    .cmp(right.consumer_fact_digest())
            })
            .then_with(|| {
                left.predicate_fact_digest()
                    .cmp(right.predicate_fact_digest())
            })
    });
    Ok(rows)
}

fn validate_every_supplied_predicate_is_consumed(
    supplied_predicates: &BTreeMap<&str, &PlanarPredicateFactReceipt>,
    rows: &[PredicateCertificateConsumptionRow],
) -> Result<(), PredicateCertificateConsumptionDenial> {
    let consumed = rows
        .iter()
        .map(|row| row.predicate_fact_digest())
        .collect::<BTreeSet<_>>();
    if supplied_predicates
        .keys()
        .all(|digest| consumed.contains(digest))
    {
        Ok(())
    } else {
        Err(denial(
            PredicateCertificateConsumptionDenialKind::UnconsumedPredicateReceipt,
            "every supplied predicate authority receipt must be consumed by retained planar evidence",
        ))
    }
}

fn denial(
    kind: PredicateCertificateConsumptionDenialKind,
    reason: &'static str,
) -> PredicateCertificateConsumptionDenial {
    PredicateCertificateConsumptionDenial::new(kind, reason)
}
