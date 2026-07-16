use worth_store_formal_models::{
    current_compaction_visibility_owner_cases, require_compaction_visibility_refinement_coverage,
    CompactionLifecycleDenial, CompactionLifecycleModel, CompactionLifecycleState,
    CompactionVisibilityOwnerCaseFamily, CompactionVisibilityRefinementCoverageIssue,
};

use super::{
    adjudicate_compaction_visibility_refinement, mutants::omit_one_mapping,
    scenarios::execute_compaction_visibility_owner_cases,
};

#[test]
fn ordinary_owner_execution_equals_declarations_and_model_mappings() {
    let evidence = adjudicate_compaction_visibility_refinement()
        .expect("ordinary owner cases must have exact model correspondence");
    let coverage = evidence.exact_coverage();

    assert!(coverage.declared_owner_cases() > 0);
    assert_eq!(
        evidence.retained_owner_observation_count(),
        coverage.ordinary_executed_cases()
    );
    assert_eq!(
        coverage.declared_owner_cases(),
        coverage.ordinary_executed_cases()
    );
    assert_eq!(
        coverage.declared_owner_cases(),
        coverage.mapped_model_actions()
    );
    for family in CompactionVisibilityOwnerCaseFamily::all() {
        let family_coverage = coverage.family_coverage(family);
        assert!(family_coverage.declared_owner_cases() > 0);
        assert_eq!(
            family_coverage.declared_owner_cases(),
            family_coverage.ordinary_executed_cases()
        );
        assert_eq!(
            family_coverage.declared_owner_cases(),
            family_coverage.mapped_model_actions()
        );
    }
}

#[test]
fn compaction_crash_orphan_and_reclaim_frontiers_are_explicit() {
    let mut lifecycle = CompactionLifecycleModel::planned();
    lifecycle.begin_write().unwrap();
    lifecycle.complete_durability().unwrap();
    lifecycle.attempt_publication().unwrap();
    lifecycle.classify_crash();
    assert_eq!(
        lifecycle.state(),
        CompactionLifecycleState::OrphanedNewGeneration
    );

    let mut published = CompactionLifecycleModel::planned();
    published.begin_write().unwrap();
    published.complete_durability().unwrap();
    published.attempt_publication().unwrap();
    published.publish().unwrap();
    assert_eq!(
        published.admit_reclaim(false),
        Err(CompactionLifecycleDenial::ReclaimBeforeReadRelease)
    );
    published.admit_reclaim(true).unwrap();
    assert_eq!(published.state(), CompactionLifecycleState::ReclaimEligible);
}

#[test]
fn tombstone_drop_mutant_is_rejected() {
    let mut lifecycle = CompactionLifecycleModel::planned();
    assert_eq!(
        lifecycle.observe_tombstone_preservation(false),
        Err(CompactionLifecycleDenial::TombstoneResurrection)
    );
}

#[test]
fn certification_owner_catalog_cannot_add_a_protocol_binding() {
    let before = current_compaction_visibility_owner_cases().count();
    let courtroom_only_catalog = crate::courtroom::layout::owner_coverage::LayoutOwnerFamily::all();
    assert!(!courtroom_only_catalog.is_empty());
    let after = current_compaction_visibility_owner_cases().count();

    assert_eq!(before, after);
}

#[test]
fn omitted_mapping_mutant_is_rejected_by_owner_correspondence() {
    let execution = execute_compaction_visibility_owner_cases();
    let complete = execution.mapped_cases().collect::<Vec<_>>();
    let omitted = *complete
        .last()
        .expect("ordinary execution maps owner cases");
    let mutant = omit_one_mapping(complete);

    let denial = require_compaction_visibility_refinement_coverage(
        current_compaction_visibility_owner_cases(),
        execution.owner_cases(),
        mutant,
    )
    .expect_err("an omitted mapping must block certification");

    assert!(denial.issues().contains(
        &CompactionVisibilityRefinementCoverageIssue::MissingModelMapping(omitted.owner_case())
    ));
}
