use bank_domain::{
    estate::{
        CapabilityGrantId, CapabilityGrantStatus, EstateAction, EstateCaseId, EstateWorkflowStage,
    },
    queries::EstateGovernanceQuery,
    reads::{EstateCapabilityContext, EstateGovernanceContext},
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationAttemptDenialKind, WorthQueryApplicationCommitDenialKind,
    WorthQueryApplicationCommitDenialStage, WorthQueryApplicationIdempotencyBinding,
    WorthQueryApplicationOneShotResult,
};

use super::fixture::{
    emergency_request_world_with_alternate_bound, foreign_estate_revocation_world, request_scope,
    CapabilityFixture, GrantSpec, ALTERNATE_EMERGENCY_BOUND_GRANT, ESTATE, FOREIGN_ESTATE,
    FOREIGN_GRANT, GRANT,
};
use crate::estate_progression::BankCapabilityRevocationProjectionDenial;
use crate::{queries, BankEstateProgressionDenial, BankMutationCommitOutcome, BankReadControls};

type GovernanceResult =
    WorthQueryApplicationOneShotResult<EstateGovernanceQuery, EstateGovernanceContext>;

#[test]
fn equivalent_revocation_retry_recovers_commit_before_fresh_poststate_denial() {
    let fixture = revocation_world();
    let specialist = fixture.authenticate();
    let before = governance_readback(&fixture);
    assert_eq!(
        capability(&before, GRANT).status(),
        CapabilityGrantStatus::Active
    );
    let alternate_before = capability(&before, ALTERNATE_EMERGENCY_BOUND_GRANT).clone();
    assert_eq!(alternate_before.status(), CapabilityGrantStatus::Active);
    assert_ne!(alternate_before.id(), GRANT);
    let binding = idempotency(131);

    let first = fixture
        .runtime
        .revoke_estate_capability(&specialist, revocation_action(), binding, &request_scope())
        .expect("an active exact grant should admit public revocation");
    let BankMutationCommitOutcome::Committed(committed) = first else {
        panic!("the first revocation must authoritatively commit: {first:?}");
    };

    let retry = fixture
        .runtime
        .revoke_estate_capability(&specialist, revocation_action(), binding, &request_scope())
        .expect("an equivalent retry should resolve idempotency before active-state projection");
    let BankMutationCommitOutcome::AlreadyCommitted(recovered) = retry else {
        panic!("the equivalent retry must recover the prior commit: {retry:?}");
    };
    assert!(committed.is_same_authoritative_commit(&recovered));

    let target_drift = fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            revocation_action_for(ALTERNATE_EMERGENCY_BOUND_GRANT),
            binding,
            &request_scope(),
        )
        .expect("governed target drift is a typed commit outcome");
    assert_eq!(
        target_drift,
        BankMutationCommitOutcome::Denied {
            kind: WorthQueryApplicationCommitDenialKind::IdempotencyIntentDrift,
            stage: WorthQueryApplicationCommitDenialStage::Idempotency,
        }
    );

    let after_commit = governance_readback(&fixture);
    assert_revoked_target_and_unchanged_alternate(&after_commit, &alternate_before);
    let target_after_commit = capability(&after_commit, GRANT).clone();

    let denial = fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            revocation_action(),
            idempotency(133),
            &request_scope(),
        )
        .expect_err("a fresh intent must evaluate the authoritative Revoked poststate");
    assert!(matches!(
        denial,
        BankEstateProgressionDenial::CapabilityRevocationProjection(
            BankCapabilityRevocationProjectionDenial::GrantNotActive(
                CapabilityGrantStatus::Revoked
            )
        )
    ));

    let after_denial = governance_readback(&fixture);
    assert_eq!(after_denial.rows(), after_commit.rows());
    assert_eq!(capability(&after_denial, GRANT), &target_after_commit);
    assert_revoked_target_and_unchanged_alternate(&after_denial, &alternate_before);
}

#[test]
fn revocation_cannot_substitute_a_target_from_another_estate() {
    let fixture = foreign_estate_revocation_world("estate-capability-revocation-foreign-target");
    let specialist = fixture.authenticate();
    let primary_before = governance_readback_for(&fixture, ESTATE);
    let foreign_before = governance_readback_for(&fixture, FOREIGN_ESTATE);
    assert_eq!(
        capability(&primary_before, GRANT).status(),
        CapabilityGrantStatus::Active
    );
    assert_eq!(
        capability(&foreign_before, FOREIGN_GRANT).status(),
        CapabilityGrantStatus::Active
    );

    let denial = fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            EstateAction::RevokeCapability {
                estate: FOREIGN_ESTATE,
                grant: GRANT,
            },
            idempotency(135),
            &request_scope(),
        )
        .expect_err("the foreign command estate must not capture another estate's grant");
    let BankEstateProgressionDenial::Attempt(denial) = denial else {
        panic!("foreign-target denial must come from exact revocation materialization: {denial:?}")
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationAttemptDenialKind::CapabilityRevocationProgramMismatch
    );

    assert_eq!(
        governance_readback_for(&fixture, ESTATE).rows(),
        primary_before.rows()
    );
    assert_eq!(
        governance_readback_for(&fixture, FOREIGN_ESTATE).rows(),
        foreign_before.rows()
    );
}

fn revocation_world() -> CapabilityFixture {
    emergency_request_world_with_alternate_bound(
        "estate-capability-revocation-idempotency",
        GrantSpec::emergency_view(),
        GrantSpec::emergency_view(),
        EstateWorkflowStage::Administration,
    )
}

fn revocation_action() -> EstateAction {
    revocation_action_for(GRANT)
}

fn revocation_action_for(grant: CapabilityGrantId) -> EstateAction {
    EstateAction::RevokeCapability {
        estate: ESTATE,
        grant,
    }
}

fn assert_revoked_target_and_unchanged_alternate(
    result: &GovernanceResult,
    alternate_before: &EstateCapabilityContext,
) {
    let target = capability(result, GRANT);
    assert_eq!(target.id(), GRANT);
    assert_eq!(target.estate(), ESTATE);
    assert_eq!(target.status(), CapabilityGrantStatus::Revoked);
    assert_eq!(
        capability(result, ALTERNATE_EMERGENCY_BOUND_GRANT),
        alternate_before,
        "revoking the exact target must not mutate an equivalent alternate grant"
    );
}

fn governance_readback(fixture: &CapabilityFixture) -> GovernanceResult {
    governance_readback_for(fixture, ESTATE)
}

fn governance_readback_for(fixture: &CapabilityFixture, estate: EstateCaseId) -> GovernanceResult {
    fixture
        .runtime
        .query(queries::estate_governance_context(estate))
        .as_principal(&fixture.authenticate())
        .controls(BankReadControls::current(request_scope(), 1, 20_000).unwrap())
        .execute()
        .expect("governance authority should independently observe current graph truth")
}

fn capability(result: &GovernanceResult, grant: CapabilityGrantId) -> &EstateCapabilityContext {
    result.rows()[0]
        .capabilities()
        .iter()
        .find(|capability| capability.id() == grant)
        .expect("the exact capability grant should be present in governance readback")
}

fn idempotency(seed: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([seed; 32], [seed + 1; 32])
}
