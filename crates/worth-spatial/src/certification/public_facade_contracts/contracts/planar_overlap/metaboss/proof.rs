use std::collections::BTreeSet;
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractExtractor,
    CoplanarOverlapContractReceipt, CoplanarOverlapDenial, CoplanarOverlapDenialKind,
};

use super::scenario::{near_graze_region, RegionShape, StormRegion, StormTransform};
use crate::public_api_planar_overlap::proof_fixture::{
    overlap_contracts, overlap_face, overlap_face_with_containment_candidate, NEIGHBORHOOD,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StormSignature {
    pub(crate) face_count: usize,
    pub(crate) region_count: usize,
    pub(crate) partial_flush_regions: usize,
    pub(crate) nested_hole_regions: usize,
    pub(crate) boundary_touch_regions: usize,
    pub(crate) collinear_run_regions: usize,
    pub(crate) shared_intervals: usize,
    pub(crate) ambiguous_contacts: usize,
    pub(crate) containment_relations: usize,
    pub(crate) policy_required_exits: usize,
}

#[derive(Clone, Debug)]
pub(crate) struct StormProof {
    pub(crate) signature: StormSignature,
    pub(crate) structural_digest: String,
    pub(crate) retained_replay_digest: String,
    pub(crate) max_candidate_pair_breadth: usize,
    pub(crate) regions: Vec<RegionProof>,
}

#[derive(Clone, Debug)]
pub(crate) struct RegionProof {
    pub(crate) region_index: usize,
    pub(crate) region_identity: String,
    pub(crate) shape: RegionShape,
    pub(crate) live_fact_digest: String,
    pub(crate) retained_replay_fact_digest: String,
    pub(crate) projection_basis_digest: String,
    pub(crate) retained_projection_basis_digest: String,
    pub(crate) shared_intervals: usize,
    pub(crate) ambiguous_contacts: usize,
    pub(crate) containment_relations: usize,
    pub(crate) policy_required_exits: usize,
    pub(crate) candidate_pair_breadth: usize,
}

pub(crate) fn certify_storm(
    world: &'static str,
    transform: StormTransform,
    regions: &[StormRegion],
) -> StormProof {
    certify_storm_with_replay_policy(world, transform, regions, false)
}

pub(crate) fn certify_storm_with_retained_replay(
    world: &'static str,
    transform: StormTransform,
    regions: &[StormRegion],
) -> StormProof {
    certify_storm_with_replay_policy(world, transform, regions, true)
}

fn certify_storm_with_replay_policy(
    world: &'static str,
    transform: StormTransform,
    regions: &[StormRegion],
    replay_regions: bool,
) -> StormProof {
    let mut proofs = regions
        .iter()
        .map(|region| certify_region(world, transform, region, false, replay_regions))
        .collect::<Vec<_>>();
    proofs.sort_by(|left, right| left.region_identity.cmp(&right.region_identity));
    proof_from_regions(proofs)
}

pub(crate) fn certify_storm_reversed_host_order(
    world: &'static str,
    transform: StormTransform,
    regions: &[StormRegion],
) -> StormProof {
    let mut proofs = regions
        .iter()
        .rev()
        .map(|region| certify_region(world, transform, region, true, true))
        .collect::<Vec<_>>();
    proofs.sort_by(|left, right| left.region_identity.cmp(&right.region_identity));
    proof_from_regions(proofs)
}

pub(crate) fn deny_tiny_rotation(
    world: &'static str,
    region: &StormRegion,
) -> CoplanarOverlapDenial {
    let first = first_face(world, StormTransform::Identity, region);
    let mut second_region = region.clone();
    second_region.second_face = second_region
        .second_face
        .iter()
        .map(|point| [point[0], point[1] + 1.0e-15])
        .collect();
    let second = second_face_with_motion(
        world,
        &second_region,
        "motion:tiny-rotation-exits-coplanar-class",
    );

    match CoplanarOverlapContractExtractor::between(first, second)
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts(world))
    {
        Ok(_) => panic!("tiny rotation must deny before overlap extraction"),
        Err(denial) => {
            assert_eq!(
                denial.kind(),
                CoplanarOverlapDenialKind::MismatchedMovementRotationPosture
            );
            denial
        }
    }
}

pub(crate) fn certify_policy_required_overlap(
    world: &'static str,
) -> CoplanarOverlapContractReceipt {
    let first = overlap_face_with_containment_candidate(
        world,
        "face:policy-required:outside-candidate",
        "movement:policy-required",
        &[[0.0, 0.0], [2.0e-9, 0.0], [2.0e-9, 2.0e-9], [0.0, 2.0e-9]],
        &[
            [20.0e-9, 20.0e-9],
            [22.0e-9, 20.0e-9],
            [22.0e-9, 22.0e-9],
            [20.0e-9, 22.0e-9],
        ],
    );
    let second = overlap_face(
        world,
        "face:policy-required:adjacent",
        "movement:policy-required",
        &[
            [0.5e-9, 0.5e-9],
            [1.5e-9, 0.5e-9],
            [1.5e-9, 1.5e-9],
            [0.5e-9, 1.5e-9],
        ],
    );
    extract_pair(world, first, second, false)
}

pub(crate) fn certify_representative_overlap(
    world: &'static str,
) -> CoplanarOverlapContractReceipt {
    let region = near_graze_region();
    extract_pair(
        world,
        first_face(world, StormTransform::Identity, &region),
        second_face(world, StormTransform::Identity, &region),
        false,
    )
}

fn certify_region(
    world: &'static str,
    transform: StormTransform,
    region: &StormRegion,
    reverse_faces: bool,
    replay_region: bool,
) -> RegionProof {
    let first = first_face(world, transform, region);
    let second = second_face(world, transform, region);
    let receipt = extract_pair(world, first.clone(), second.clone(), reverse_faces);
    let replay = if replay_region {
        extract_pair(world, first.clone(), second.clone(), reverse_faces)
    } else {
        receipt.clone()
    };
    RegionProof {
        region_index: region.region_index,
        region_identity: region.region_identity(),
        shape: region.shape,
        live_fact_digest: receipt.fact_digest().to_string(),
        retained_replay_fact_digest: replay.fact_digest().to_string(),
        projection_basis_digest: projection_basis_digest(&receipt),
        retained_projection_basis_digest: projection_basis_digest(&replay),
        shared_intervals: receipt.shared_intervals().len(),
        ambiguous_contacts: receipt.ambiguous_contacts().len(),
        containment_relations: receipt.containment_relations().len(),
        policy_required_exits: receipt.policy_required_exits().len(),
        candidate_pair_breadth: receipt.counters().candidate_pair_breadth(),
    }
}

fn proof_from_regions(regions: Vec<RegionProof>) -> StormProof {
    let signature = StormSignature {
        face_count: regions.len() * 2,
        region_count: regions
            .iter()
            .map(|region| region.region_index)
            .collect::<BTreeSet<_>>()
            .len(),
        partial_flush_regions: regions
            .iter()
            .filter(|region| region.shape == RegionShape::PartialFlush)
            .count(),
        nested_hole_regions: regions
            .iter()
            .filter(|region| region.shape == RegionShape::NestedHole)
            .count(),
        boundary_touch_regions: regions
            .iter()
            .filter(|region| region.shape == RegionShape::BoundaryTouch)
            .count(),
        collinear_run_regions: regions
            .iter()
            .filter(|region| region.shape == RegionShape::CollinearRun)
            .count(),
        shared_intervals: regions.iter().map(|region| region.shared_intervals).sum(),
        ambiguous_contacts: regions.iter().map(|region| region.ambiguous_contacts).sum(),
        containment_relations: regions
            .iter()
            .map(|region| region.containment_relations)
            .sum(),
        policy_required_exits: regions
            .iter()
            .map(|region| region.policy_required_exits)
            .sum(),
    };
    let structural_digest = digest(
        "storm-structural",
        regions.iter().flat_map(structural_digest_parts),
    );
    let retained_replay_digest = digest(
        "storm-retained-replay",
        regions.iter().map(|region| {
            format!(
                "{}:{}:{}",
                region.region_identity, region.live_fact_digest, region.retained_replay_fact_digest
            )
        }),
    );
    let max_candidate_pair_breadth = regions
        .iter()
        .map(|region| region.candidate_pair_breadth)
        .max()
        .unwrap_or(0);
    StormProof {
        signature,
        structural_digest,
        retained_replay_digest,
        max_candidate_pair_breadth,
        regions,
    }
}

fn first_face(
    world: &'static str,
    transform: StormTransform,
    region: &StormRegion,
) -> CertifiedCoplanarOverlapFace2D {
    let face_identity = region.first_face_identity(transform);
    let points = region.first_face.as_slice();
    match &region.containment_candidate {
        Some(candidate) => overlap_face_with_containment_candidate(
            world,
            face_identity,
            transform.canonical_motion(),
            points,
            candidate,
        ),
        None => overlap_face(world, face_identity, transform.canonical_motion(), points),
    }
}

fn second_face(
    world: &'static str,
    transform: StormTransform,
    region: &StormRegion,
) -> CertifiedCoplanarOverlapFace2D {
    let face_identity = region.second_face_identity(transform);
    overlap_face(
        world,
        face_identity,
        transform.canonical_motion(),
        region.second_face.as_slice(),
    )
}

fn second_face_with_motion(
    world: &'static str,
    region: &StormRegion,
    movement: &'static str,
) -> CertifiedCoplanarOverlapFace2D {
    let face_identity = region.second_face_identity(StormTransform::Identity);
    overlap_face(
        world,
        face_identity,
        movement,
        region.second_face.as_slice(),
    )
}

fn extract_pair(
    world: &'static str,
    first: CertifiedCoplanarOverlapFace2D,
    second: CertifiedCoplanarOverlapFace2D,
    reverse_faces: bool,
) -> CoplanarOverlapContractReceipt {
    let extractor = if reverse_faces {
        CoplanarOverlapContractExtractor::between(second, first)
    } else {
        CoplanarOverlapContractExtractor::between(first, second)
    };
    extractor
        .within_planar_neighborhood(NEIGHBORHOOD)
        .compile(&overlap_contracts(world))
        .expect("coplanar storm region plan")
        .extract()
        .expect("coplanar storm region receipt")
}

fn projection_basis_digest(receipt: &CoplanarOverlapContractReceipt) -> String {
    let parts = [
        receipt.basis().first_face().signed_area_receipt(),
        receipt.basis().second_face().signed_area_receipt(),
    ]
    .into_iter()
    .flat_map(|area| {
        [
            format!("area:{}", area.fact_digest()),
            format!("winding:{}", area.basis().winding_receipt().fact_digest()),
            format!(
                "precision:{}",
                area.basis().precision_receipt().fact_digest()
            ),
        ]
    });
    digest("projection-basis", parts)
}

fn structural_digest_parts(region: &RegionProof) -> Vec<String> {
    vec![
        format!("region:{}", region.region_identity),
        format!("shared:{}", region.shared_intervals),
        format!("ambiguous:{}", region.ambiguous_contacts),
        format!("containment:{}", region.containment_relations),
        format!("policy:{}", region.policy_required_exits),
        format!("candidate_pair_breadth:{}", region.candidate_pair_breadth),
    ]
}

fn digest(scope: &'static str, parts: impl IntoIterator<Item = String>) -> String {
    let mut parts = parts.into_iter().collect::<Vec<_>>();
    parts.sort();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[scope.to_string(), parts.join("|")],
    )
}
