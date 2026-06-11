use worth_spatial::facade::planar_overlap::{
    CertifiedCoplanarOverlapFace2D, CoplanarOverlapContractExtractor,
    CoplanarOverlapContractReceipt,
};

use super::scenario::{near_graze_region, StormRegion};
use crate::public_api_planar_overlap::proof_fixture::{
    overlap_contracts, overlap_face, overlap_face_with_containment_candidate, NEIGHBORHOOD,
};

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
        first_face(world, &region),
        second_face(world, &region),
        false,
    )
}

fn first_face(world: &'static str, region: &StormRegion) -> CertifiedCoplanarOverlapFace2D {
    let face_identity = region.first_face_identity();
    let points = region.first_face.as_slice();
    match &region.containment_candidate {
        Some(candidate) => overlap_face_with_containment_candidate(
            world,
            face_identity,
            "motion:coplanar-storm-canonical",
            points,
            candidate,
        ),
        None => overlap_face(
            world,
            face_identity,
            "motion:coplanar-storm-canonical",
            points,
        ),
    }
}

fn second_face(world: &'static str, region: &StormRegion) -> CertifiedCoplanarOverlapFace2D {
    let face_identity = region.second_face_identity();
    overlap_face(
        world,
        face_identity,
        "motion:coplanar-storm-canonical",
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
