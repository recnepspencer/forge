//! The creation cell where the second owner denies after the first has already
//! forked. Creation never advances an owner, so the only way to reach a
//! partial creation is to hold the Signal destination the plan names.

use super::*;

fn create_partial_effects(
    owner: &TestOwner,
    source: &ProductBranchObservation,
    intent: ProductBranchCreationIntent,
) -> crate::recovery::ProductUnpublishedOwnerEffects {
    let cancellation = RuntimeWorldCancellationSource::new();
    let outcome = RuntimeWorldBranchService::create_product_branch(
        owner,
        RuntimeWorldBranchCreationRequest::new(source.clone(), intent, &cancellation.token()),
    )
    .expect("the later sibling denial is a product-unpublished outcome");
    match outcome {
        RuntimeWorldBranchCreationOutcome::Performed(_) => {
            panic!("a held Signal destination must deny the Signal fork")
        }
        RuntimeWorldBranchCreationOutcome::ProductUnpublished(effects) => effects,
    }
}

#[test]
fn forked_relational_effect_is_retained_when_later_signal_sibling_denies() {
    let (_fixture, owner, source) = setup_with_relational_source(3);
    let history_before = owner.state.history.len();
    let custody_before = owner.state.retention.active_component_obligation_count();
    // The Signal owner already holds this exact destination, so the second leg
    // of the creation denies after the Relational fork has really happened.
    let held = owner
        .state
        .signal
        .mutation_port()
        .reserve_fork_exact(
            validate_signal_branch_name("signal-branch-partial").expect("valid Signal name"),
            source.basis().signal_basis(),
        )
        .expect("the first owner-issued reservation holds the destination");
    let effects = create_partial_effects(
        &owner,
        &source,
        fork_intent(
            "branch-fork-partial",
            relational_fork("relational-branch-partial"),
            signal_fork("signal-branch-partial"),
        ),
    );
    assert_eq!(effects.cause(), ProductUnpublishedCause::SiblingOwnerDenied);
    assert_eq!(effects.owner_effect_count(), 1);
    assert_eq!(
        effects.progress().relational_posture(),
        crate::publication::RelationalAttemptProgressPosture::Performed
    );
    assert_eq!(
        effects.progress().signal_posture(),
        crate::publication::SignalAttemptProgressPosture::Untouched
    );
    assert_ne!(effects.successor_commit(), source.selected_commit());
    assert_ne!(
        effects
            .successor_basis()
            .expect("retained fork successor basis")
            .relational_basis()
            .identity(),
        source.basis().relational_basis().identity()
    );
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    assert_eq!(owner.state.history.len(), history_before + 1);
    assert_eq!(owner.recovery_record_count(), 1);
    assert!(
        owner.state.retention.active_component_obligation_count() > custody_before,
        "partial owner effects retain bounded component custody"
    );
    assert_cleanup_releases_custody(&owner, effects, &source, custody_before);
    drop(held);
}

/// Cleaning up the retained record is what releases the custody the partial
/// creation charged, and it leaves the source observation exactly as it was.
fn assert_cleanup_releases_custody(
    owner: &TestOwner,
    effects: crate::recovery::ProductUnpublishedOwnerEffects,
    source: &ProductBranchObservation,
    custody_before: usize,
) {
    let handle = effects.recovery_handle();
    assert!(owner.inspect_recovery(&handle).is_some());
    assert!(owner.cleanup_recovery(effects));
    assert_eq!(owner.recovery_record_count(), 0);
    assert_eq!(
        owner.state.retention.active_component_obligation_count(),
        custody_before
    );
    let source_after =
        RuntimeWorldObservationService::observe_product_branch(owner, source.branch_identity())
            .expect("source remains observable after recovery cleanup");
    assert_eq!(&source_after, source);
    assert_eq!(owner.state.branches.branch_count(), 1);
}

/// The performed leg of a partial creation is a component branch that really
/// exists. Whatever the creation does with the rest of its reservations, that
/// branch must stay named in custody under the occurrence that made it: losing
/// the record is how a component branch becomes unreachable garbage.
#[test]
fn relational_fork_then_signal_owner_loss_retains_exact_fork_custody() {
    let (_fixture, owner, source) = setup_with_relational_source(3);
    let held = owner
        .state
        .signal
        .mutation_port()
        .reserve_fork_exact(
            validate_signal_branch_name("signal-branch-fork-loss").expect("valid Signal name"),
            source.basis().signal_basis(),
        )
        .expect("the Signal destination this creation will ask for is already held");
    let effects = create_partial_effects(
        &owner,
        &source,
        fork_intent(
            "branch-fork-loss",
            relational_fork("relational-branch-fork-loss"),
            signal_fork("signal-branch-fork-loss"),
        ),
    );
    drop(held);

    assert_eq!(effects.cause(), ProductUnpublishedCause::SiblingOwnerDenied);
    assert_eq!(effects.owner_effect_count(), 1);
    assert_eq!(
        effects.progress().relational_posture(),
        crate::publication::RelationalAttemptProgressPosture::Performed
    );
    assert_eq!(
        effects.progress().signal_posture(),
        crate::publication::SignalAttemptProgressPosture::Untouched
    );

    assert_exact_fork_custody(&owner);
    assert_eq!(owner.state.branches.branch_count(), 1);
    assert_eq!(owner.state.branches.reserved_branch_count(), 0);
    drop(effects);
}

/// Exactly one record, naming the destination the performed fork created, keyed
/// to the product-branch occurrence that reserved it.
fn assert_exact_fork_custody(owner: &TestOwner) {
    let records = owner.state.custody.installed_records();
    assert_eq!(
        records.len(),
        1,
        "the denied Signal leg charges nothing and the performed Relational leg charges once"
    );
    assert_eq!(
        records[0].target(),
        &crate::branch::ComponentBranchTarget::Relational(BranchId(
            "relational-branch-fork-loss".to_owned()
        )),
        "custody names the exact destination the performed fork created"
    );
    assert_eq!(
        records[0].product_branch(),
        &crate::identity::ProductBranchIdentity::issued(
            owner.owner_identity(),
            crate::branch::ProductBranchCreationIntent::named("branch-fork-loss")
                .expect("valid product branch name")
                .name()
                .clone(),
        ),
        "the record is keyed to the product-branch occurrence that reserved it"
    );
}
