use std::collections::BTreeSet;

const REPRESENTATIVE_CROSS_FILE_SURFACES: &[&str] = &[
    "RecoveryCompletion::execute_publication_recovery_replay",
    "IntegrityDamageMap::admit_corruption_readmission",
    "RecoveryPhysicsTimelineAuthority::resolve_candidates",
    "layout_projection::BoundedWalTailLayoutReport::lookup_tail_range",
];

pub(super) fn assert_cross_file_surfaces(actual: &BTreeSet<(String, String, String)>) {
    for expected in REPRESENTATIVE_CROSS_FILE_SURFACES {
        assert!(
            actual.iter().any(|(_, surface, _)| surface == expected),
            "cross-file public impl `{expected}` escaped API discovery"
        );
    }
}
