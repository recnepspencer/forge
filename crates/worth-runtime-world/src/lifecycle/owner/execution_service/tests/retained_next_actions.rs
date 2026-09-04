//! SPEC-P4-013. Every retained owner-effect record derives its continuation
//! from one authority, `next_actions_for_progress`. This module drives that
//! authority over the composite progress shapes reachable from the constructors
//! in `publication/progress`, using evidence minted by the production ports.
//!
//! Two shapes are covered elsewhere rather than here. The fork-only `Performed`
//! shape is proved against a real finalization record by
//! `branch/retirement_tests/fork_finalization_race.rs`
//! (`assert_finalization_record_contract`), and a minted record advertising a
//! settled publication is proved by
//! `missing_signal_sibling_after_relational_movement_retains_exact_progress` in
//! `failures.rs`. `SettlementPending` is not reachable from this crate at all:
//! `DeferredPublicationSettlement::new` is `pub(crate)` to `worth-relational`.
//! It drives the same `requires_settlement()` predicate as the `Performed` and
//! `SettlementRequired` shapes proved below.

use worth_relational::facade::branch::AdmittedRelationalBranchBasis;
use worth_relational::facade::history::RelationalCommitIdentity;
use worth_relational::facade::transactions::CommitResult;

use super::*;

use crate::publication::{
    CompositeAttemptProgress, RelationalAttemptProgress, SignalAttemptProgress,
};
use crate::recovery::{next_actions_for_progress, ProductUnpublishedNextAction};

/// The fixed bound a retained list is held to. `RetainedNextActions` asserts the
/// same bound where a record is minted; no derived shape may reach it.
const RETAINED_NEXT_ACTION_BOUND: usize = 5;

/// A Relational commit that still owes settlement admits only settlement.
const SETTLEMENT_OWED: [ProductUnpublishedNextAction; 3] = [
    ProductUnpublishedNextAction::SettleOwnerEffects,
    ProductUnpublishedNextAction::ReleaseObligations,
    ProductUnpublishedNextAction::Inspect,
];

/// Settled publication evidence admits a fresh composite attempt.
const SETTLED_PUBLICATION: [ProductUnpublishedNextAction; 3] = [
    ProductUnpublishedNextAction::StartFreshCompositePublication,
    ProductUnpublishedNextAction::ReleaseObligations,
    ProductUnpublishedNextAction::Inspect,
];

/// A record with no settled publication evidence and no settlement obligation.
const NO_CONTINUATION: [ProductUnpublishedNextAction; 2] = [
    ProductUnpublishedNextAction::ReleaseObligations,
    ProductUnpublishedNextAction::Inspect,
];

/// One real Relational commit settled by the production settlement port, kept
/// as the parts a settled progress row is built from.
struct SettledRelationalEvidence {
    commit_identity: RelationalCommitIdentity,
    successor_basis: AdmittedRelationalBranchBasis,
    result: CommitResult,
}

fn derivation_owner() -> (RealReferenceFixture, TestOwner) {
    let mut fixture = reference_test_fixture::real_fixture(12, 12);
    let owner = TestOwner::new(
        fixture.owner_inputs(budgets(4), RuntimeWorldClock::from_source(FixedClock)),
    )
    .expect("managed owner construction");
    (fixture, owner)
}

fn settled_relational_evidence(
    fixture: &RealReferenceFixture,
    owner: &TestOwner,
) -> SettledRelationalEvidence {
    let performed = fixture.perform_relational_owner_change();
    let commit_identity = performed.commit_identity();
    let successor_basis = performed.next_basis().clone();
    let result = owner
        .state
        .relational
        .settlement_port()
        .settle_performed_publication(performed)
        .expect("the production settlement port settles the performed commit");
    SettledRelationalEvidence {
        commit_identity,
        successor_basis,
        result,
    }
}

fn settled_progress(
    evidence: &SettledRelationalEvidence,
    signal: SignalAttemptProgress,
) -> CompositeAttemptProgress {
    CompositeAttemptProgress::new(
        RelationalAttemptProgress::settled(
            evidence.commit_identity.clone(),
            evidence.successor_basis.clone(),
            evidence.result.clone(),
        ),
        signal,
    )
}

fn assert_derives(progress: &CompositeAttemptProgress, expected: &[ProductUnpublishedNextAction]) {
    let actions = next_actions_for_progress(progress);
    assert_eq!(
        actions.as_slice(),
        expected,
        "{:?} / {:?} derives an exact continuation",
        progress.relational_posture(),
        progress.signal_posture()
    );
    assert!(
        actions.len() <= RETAINED_NEXT_ACTION_BOUND,
        "a derived continuation never exceeds the retained-record bound"
    );
}

/// Signal movement alone is never publication evidence this world can continue
/// from, and neither is an attempt that moved nothing.
fn assert_shapes_without_publication_evidence(fixture: &mut RealReferenceFixture) {
    assert_derives(&CompositeAttemptProgress::untouched(), &NO_CONTINUATION);
    assert_derives(
        &CompositeAttemptProgress::new(
            RelationalAttemptProgress::untouched(),
            SignalAttemptProgress::advanced(fixture.perform_signal_owner_change()),
        ),
        &NO_CONTINUATION,
    );
    assert_derives(
        &CompositeAttemptProgress::new(
            RelationalAttemptProgress::untouched(),
            SignalAttemptProgress::prepared_for_execution(),
        ),
        &NO_CONTINUATION,
    );
    assert_derives(
        &CompositeAttemptProgress::new(
            RelationalAttemptProgress::untouched(),
            SignalAttemptProgress::summary(SignalAttemptProgressPosture::Performed),
        ),
        &NO_CONTINUATION,
    );
}

/// Every Relational posture that still owes settlement names settlement first.
fn assert_shapes_that_owe_settlement(
    fixture: &RealReferenceFixture,
    evidence: &SettledRelationalEvidence,
) {
    assert_derives(
        &CompositeAttemptProgress::new(
            RelationalAttemptProgress::performed(fixture.perform_relational_owner_change()),
            SignalAttemptProgress::untouched(),
        ),
        &SETTLEMENT_OWED,
    );
    assert_derives(
        &CompositeAttemptProgress::new(
            RelationalAttemptProgress::settlement_required(
                evidence.commit_identity.clone(),
                evidence.successor_basis.clone(),
            ),
            SignalAttemptProgress::untouched(),
        ),
        &SETTLEMENT_OWED,
    );
}

/// The exact shapes a CAS loss and a late cancellation retain: the Relational
/// commit is settled, with or without a Signal advance beside it.
fn assert_settled_publication_shapes(
    fixture: &mut RealReferenceFixture,
    evidence: &SettledRelationalEvidence,
) {
    assert_derives(
        &settled_progress(evidence, SignalAttemptProgress::untouched()),
        &SETTLED_PUBLICATION,
    );
    assert_derives(
        &settled_progress(
            evidence,
            SignalAttemptProgress::advanced(fixture.perform_signal_owner_change()),
        ),
        &SETTLED_PUBLICATION,
    );
}

#[test]
fn every_composite_progress_shape_derives_an_exact_continuation() {
    let (mut fixture, owner) = derivation_owner();
    let evidence = settled_relational_evidence(&fixture, &owner);
    assert_shapes_without_publication_evidence(&mut fixture);
    assert_shapes_that_owe_settlement(&fixture, &evidence);
    assert_settled_publication_shapes(&mut fixture, &evidence);
}
