use super::{
    hard_prohibition_compile_fail_fixtures, hard_prohibition_documentation_rows,
    hard_prohibition_documented_seam_keys, hard_prohibition_registry,
    render_hard_prohibition_reference, WorthQueryProhibitedSeam,
    WorthQueryProhibitionEnforcementTier,
};
use std::collections::BTreeSet;

#[test]
fn registry_names_every_documented_hard_prohibition_once() {
    let registry = hard_prohibition_registry();
    let registry_keys = registry
        .rows()
        .iter()
        .map(|row| row.seam_key())
        .collect::<Vec<_>>();

    assert_eq!(registry_keys, hard_prohibition_documented_seam_keys());
}

#[test]
fn documentation_projection_is_derived_from_registry_rows() {
    let registry = hard_prohibition_registry();
    let documented_rows = hard_prohibition_documentation_rows();

    assert_eq!(documented_rows.len(), registry.rows().len());

    for (documented, registered) in documented_rows.iter().zip(registry.rows()) {
        assert_eq!(documented.seam_key(), registered.seam_key());
        assert_eq!(documented.public_symbol(), registered.public_symbol());
        assert_eq!(documented.enforcement_tier(), registered.enforcement_tier());
        assert_eq!(documented.replacement_lane(), registered.replacement_lane());
        assert_eq!(documented.rationale(), registered.rationale());
    }
}

#[test]
fn rendered_reference_matches_checked_in_consumer_documentation() {
    let checked_in = include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/docs/foundations/hard-prohibitions.md"
    ));

    assert_eq!(
        checked_in.replace("\r\n", "\n"),
        render_hard_prohibition_reference()
    );
}

#[test]
fn compile_fail_manifest_is_derived_from_registered_hard_prohibitions() {
    let registry = hard_prohibition_registry();
    let registered = registry
        .rows()
        .iter()
        .map(|row| row.seam())
        .collect::<Vec<_>>();
    let fixtures = hard_prohibition_compile_fail_fixtures()
        .iter()
        .map(|fixture| fixture.seam())
        .collect::<Vec<_>>();

    assert_eq!(fixtures, registered);

    for fixture in hard_prohibition_compile_fail_fixtures() {
        let row = registry
            .row(fixture.seam())
            .expect("compile-fail fixture seam must be registered");
        assert_eq!(fixture.seam_key(), row.seam_key());
        assert_eq!(fixture.forbidden_symbol(), row.public_symbol());
        assert!(!fixture.fixture_path().is_empty());
    }
}

#[test]
fn compile_fail_manifest_fixture_paths_are_unique() {
    let fixture_paths = hard_prohibition_compile_fail_fixtures()
        .iter()
        .map(|fixture| fixture.fixture_path())
        .collect::<Vec<_>>();
    let unique_paths = fixture_paths.iter().copied().collect::<BTreeSet<_>>();

    assert_eq!(fixture_paths.len(), unique_paths.len());
}

#[test]
fn covered_phase_two_seams_are_sealed_by_visibility() {
    let registry = hard_prohibition_registry();

    for row in registry.rows() {
        assert_eq!(
            row.enforcement_tier(),
            WorthQueryProhibitionEnforcementTier::SealedByVisibility,
            "{} must be sealed before the Phase 3 audit can claim residue coverage",
            row.seam_key()
        );
        assert!(!row.public_symbol().is_empty());
        assert!(!row.replacement_lane().is_empty());
        assert!(!row.rationale().is_empty());
    }
}

#[test]
fn direct_workspace_write_and_existing_truth_bypass_seams_are_registry_owned() {
    let registry = hard_prohibition_registry();

    for seam in [
        WorthQueryProhibitedSeam::WorkspaceDirectWrite,
        WorthQueryProhibitedSeam::WorkspaceDirectBatch,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindEntity,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthBindRelation,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthProbe,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdate,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthAssert,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthVerify,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthUpdateVerified,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDelete,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteWith,
        WorthQueryProhibitedSeam::WorkspaceExistingTruthDeleteVerified,
    ] {
        assert!(
            registry.contains_seam(seam),
            "{} must be present in the hard prohibition registry",
            seam.key()
        );
    }
}
