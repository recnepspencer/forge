use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::canonical_order::canonical_planar_coordinate_bits;
use super::{PlanarPredicateInputBasis, PlanarPredicateKind};

pub(crate) fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

pub(crate) fn predicate_basis_digest_parts(
    kind: PlanarPredicateKind,
    basis: &PlanarPredicateInputBasis,
    canonical_points: [[f64; 2]; 3],
) -> Vec<String> {
    let mut parts = vec![
        format!("kind:{}", kind.as_str()),
        format!("local_frame:{}", basis.local_frame_identity()),
        format!("topology_basis:{}", basis.topology_basis_identity()),
        format!(
            "movement_rotation:{}",
            basis.movement_rotation_posture_identity()
        ),
        format!("tolerance_policy:{}", basis.tolerance_policy_identity()),
        format!("coincidence_policy:{}", basis.coincidence_policy().as_str()),
    ];
    for (index, point) in canonical_points.into_iter().enumerate() {
        parts.push(format!(
            "point{index}.x:{}",
            canonical_planar_coordinate_bits(point[0])
        ));
        parts.push(format!(
            "point{index}.y:{}",
            canonical_planar_coordinate_bits(point[1])
        ));
    }
    parts
}
