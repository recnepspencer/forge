use worth_spatial::facade::projected_overlap_faces::CertifiedProjectedOverlapCandidatePair;

fn main() {
    let _pair = CertifiedProjectedOverlapCandidatePair::from_raw_coordinates(
        "projection:raw",
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0]],
    );
}
