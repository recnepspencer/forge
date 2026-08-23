use std::collections::BTreeSet;

const REPRESENTATIVE_CROSS_FILE_SURFACES: &[&str] = &[
    "PhysicalSourceSelection::root",
    "PhysicalWalSegmentCandidate::frames",
    "ImmutablePhysicalRedoPlan::decisions",
];

pub(super) fn assert_cross_file_surfaces(actual: &BTreeSet<(String, String, String)>) {
    for expected in REPRESENTATIVE_CROSS_FILE_SURFACES {
        assert!(
            actual.iter().any(|(_, surface, _)| surface == expected),
            "cross-file public impl `{expected}` escaped API discovery"
        );
    }
}
