use forge_proof::{ProofOutcomeKind, RecipeStageKind};

use super::fixtures::{
    admitted_handle, legal, progressed, AdmittedFamily, AlternateAspectFamily, Declaration,
    DeferredFamily, DeniedFamily, DescriptiveDeferredSignalFamily, FailedFamily,
    MaskedCoverageFamily, ReceiptFamily, StaleFamily, WorldSensitiveFamily,
};
use crate::application::ForgeQueryDeclarationProgressionChecked;
use crate::target_binding::ForgeQueryBindingTargetWitness;

#[test]
fn progression_exposes_deferred_denied_and_failed_outcomes() {
    let handle = admitted_handle("collaborative");

    match handle.progress_declaration_checked(legal(
        &handle,
        Declaration::<DeferredFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Deferred);
        }
        _ => panic!("expected deferred progression"),
    }

    match handle
        .progress_declaration_checked(legal(&handle, Declaration::<DeniedFamily>::new("edge:42")))
    {
        ForgeQueryDeclarationProgressionChecked::Denied(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Denied);
        }
        _ => panic!("expected denied progression"),
    }

    match handle
        .progress_declaration_checked(legal(&handle, Declaration::<FailedFamily>::new("edge:42")))
    {
        ForgeQueryDeclarationProgressionChecked::Failed(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Failed);
        }
        _ => panic!("expected failed progression"),
    }
}

#[test]
fn checked_recipe_lane_preserves_non_success_outcomes() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    match collaborative.progress_declaration_recipe_checked(
        collaborative.declaration_progression_recipe(legal(
            &collaborative,
            Declaration::<DeferredFamily>::new("edge:42"),
        )),
    ) {
        ForgeQueryDeclarationProgressionChecked::Deferred(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Deferred);
        }
        _ => panic!("expected deferred recipe progression"),
    }

    match collaborative.progress_declaration_recipe_checked(
        collaborative.declaration_progression_recipe(legal(
            &collaborative,
            Declaration::<StaleFamily>::new("edge:42"),
        )),
    ) {
        ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Stale);
            assert_eq!(progress.stage(), RecipeStageKind::Lowered);
        }
        _ => panic!("expected stale recipe progression"),
    }

    match restricted.progress_declaration_recipe_checked(restricted.declaration_progression_recipe(
        legal(
            &restricted,
            Declaration::<WorldSensitiveFamily>::new("edge:42"),
        ),
    )) {
        ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::RebindRequired);
            assert_eq!(progress.stage(), RecipeStageKind::Resolved);
        }
        _ => panic!("expected rebind-required recipe progression"),
    }
}

#[test]
fn progression_preserves_stale_and_rebind_required_separately() {
    let collaborative = admitted_handle("collaborative");
    let restricted = admitted_handle("restricted");

    match collaborative.progress_declaration_checked(legal(
        &collaborative,
        Declaration::<StaleFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::Stale(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::Stale);
            assert_eq!(progress.stage(), RecipeStageKind::Lowered);
        }
        _ => panic!("expected stale progression"),
    }

    let collaborative_world_sensitive = legal(
        &collaborative,
        Declaration::<WorldSensitiveFamily>::new("edge:42"),
    );
    assert!(matches!(
        collaborative.progress_declaration_checked(collaborative_world_sensitive),
        ForgeQueryDeclarationProgressionChecked::Admitted(_)
    ));

    match restricted.progress_declaration_checked(legal(
        &restricted,
        Declaration::<WorldSensitiveFamily>::new("edge:42"),
    )) {
        ForgeQueryDeclarationProgressionChecked::RebindRequired(progress) => {
            assert_eq!(progress.outcome().kind(), ProofOutcomeKind::RebindRequired);
            assert_eq!(progress.stage(), RecipeStageKind::Resolved);
        }
        _ => panic!("expected rebind-required progression"),
    }
}

#[test]
fn descriptive_signal_deferred_families_can_still_progress() {
    let handle = admitted_handle("collaborative");
    let progressed = progressed(
        &handle,
        Declaration::<DescriptiveDeferredSignalFamily>::new("edge:42"),
    );

    assert_eq!(progressed.outcome().kind(), ProofOutcomeKind::Success);
}

#[test]
fn progression_digest_changes_when_legality_truth_changes() {
    let handle = admitted_handle("collaborative");
    let admitted = progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));
    let receipt = progressed(&handle, Declaration::<ReceiptFamily>::new("edge:42"));

    assert_ne!(admitted.progression_digest(), receipt.progression_digest());
}

#[test]
fn progressed_artifacts_expose_aspect_contract_and_reviewed_coverage() {
    let handle = admitted_handle("collaborative");
    let progressed = progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));

    assert_eq!(
        progressed.aspect_contract().required(),
        &["selection.active_edge".to_string()]
    );
    assert_eq!(
        progressed.reviewed_aspect_coverage().present(),
        &["selection.active_edge".to_string()]
    );

    let semantics = progressed.binding_target().semantics().clone();
    let (_, _, _, _, _, contract, coverage) = semantics
        .admitted_declaration_progression()
        .expect("progression target semantics should exist");
    assert_eq!(contract.required(), &["selection.active_edge".to_string()]);
    assert_eq!(coverage.present(), &["selection.active_edge".to_string()]);
}

#[test]
fn progression_binding_digest_changes_when_aspect_contract_changes() {
    let handle = admitted_handle("collaborative");
    let left = progressed(&handle, Declaration::<AdmittedFamily>::new("edge:42"));
    let right = progressed(
        &handle,
        Declaration::<AlternateAspectFamily>::new("edge:42"),
    );

    assert_ne!(
        ForgeQueryBindingTargetWitness::binding_digest(&left.binding_target()),
        ForgeQueryBindingTargetWitness::binding_digest(&right.binding_target())
    );
}

#[test]
fn progression_binding_semantics_preserve_masked_reviewed_coverage() {
    let handle = admitted_handle("collaborative");
    let progressed = progressed(&handle, Declaration::<MaskedCoverageFamily>::new("edge:42"));

    assert_eq!(
        progressed.reviewed_aspect_coverage().masked(),
        &["selection.active_edge".to_string()]
    );

    let semantics = progressed.binding_target().semantics().clone();
    let (_, _, _, _, _, _, coverage) = semantics
        .admitted_declaration_progression()
        .expect("progression target semantics should exist");
    assert_eq!(coverage.masked(), &["selection.active_edge".to_string()]);
}
