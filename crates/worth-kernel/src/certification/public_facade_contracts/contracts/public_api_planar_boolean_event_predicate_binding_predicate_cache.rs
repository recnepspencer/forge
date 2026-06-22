use std::collections::BTreeMap;

use worth_spatial::facade::planar_predicates::{
    planar_predicate_authority_entry, planar_predicate_authority_facts,
    PlanarPredicateAuthorityCase, PlanarPredicateAuthorityQueryDomain,
    PlanarPredicateAuthorityQueryWorld, PlanarPredicateFactReceipt, PlanarPredicateInputBasis,
};
use worth_spatial::facade::planar_segment_segment::CertifiedSegmentSegment2DReceipt;

pub(crate) fn unique_orientation_predicates_cached(
    segments: &[CertifiedSegmentSegment2DReceipt],
    predicate_handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PlanarPredicateAuthorityQueryWorld,
    >,
) -> Vec<PlanarPredicateFactReceipt> {
    let mut cache = BTreeMap::new();
    let mut receipts = BTreeMap::new();
    for segment in segments {
        for receipt in segment_orientation_predicates(segment, predicate_handle, &mut cache) {
            receipts.insert(receipt.fact_digest().to_string(), receipt);
        }
    }
    receipts.into_values().collect()
}

fn segment_orientation_predicates(
    segment: &CertifiedSegmentSegment2DReceipt,
    predicate_handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PlanarPredicateAuthorityQueryWorld,
    >,
    cache: &mut BTreeMap<String, PlanarPredicateFactReceipt>,
) -> Vec<PlanarPredicateFactReceipt> {
    let basis = segment.basis();
    [
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_start_point_2d(),
        ],
        [
            basis.first_start_point_2d(),
            basis.first_end_point_2d(),
            basis.second_end_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_start_point_2d(),
        ],
        [
            basis.second_start_point_2d(),
            basis.second_end_point_2d(),
            basis.first_end_point_2d(),
        ],
    ]
    .into_iter()
    .map(|points| {
        predicate_receipt_cached(
            basis.frame_identity(),
            basis.tolerance_policy_identity(),
            points,
            predicate_handle,
            cache,
        )
    })
    .collect()
}

fn predicate_receipt_cached(
    frame_identity: &str,
    precision_identity: &str,
    points: [[f64; 2]; 3],
    predicate_handle: &forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
        PlanarPredicateAuthorityQueryDomain,
        PlanarPredicateAuthorityQueryWorld,
    >,
    cache: &mut BTreeMap<String, PlanarPredicateFactReceipt>,
) -> PlanarPredicateFactReceipt {
    let key = predicate_cache_key(frame_identity, precision_identity, points);
    if let Some(receipt) = cache.get(&key) {
        return receipt.clone();
    }
    let basis = PlanarPredicateInputBasis::from_projected_orient2d_points(
        frame_identity,
        "topology:event-predicate-binding",
        "movement:event-predicate-binding",
        precision_identity,
        points,
    );
    planar_predicate_authority_facts(
        &planar_predicate_authority_entry(PlanarPredicateAuthorityCase::orient2d(basis)),
        predicate_handle,
    )
    .map(|receipt| {
        cache.insert(key, receipt.clone());
        receipt
    })
    .expect("predicate receipt")
}

fn predicate_cache_key(
    frame_identity: &str,
    precision_identity: &str,
    points: [[f64; 2]; 3],
) -> String {
    format!(
        "{}:{}:{:x}:{:x}:{:x}:{:x}:{:x}:{:x}",
        frame_identity,
        precision_identity,
        points[0][0].to_bits(),
        points[0][1].to_bits(),
        points[1][0].to_bits(),
        points[1][1].to_bits(),
        points[2][0].to_bits(),
        points[2][1].to_bits()
    )
}
