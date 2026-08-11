use std::time::{Duration, SystemTime, UNIX_EPOCH};

use bank_domain::estate::{
    CapabilityGrantId, CapabilityValidity, DelegationLimit, EstateAction,
    EstateCapabilityDelegationRequest, EstateCapabilityOperation, EstateCapabilityPurpose,
    EstateCapabilityScope, EstateMoment, EstateWorkflowStage, RestrictedBankField,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationCommitDenialKind, WorthQueryApplicationCommitDenialStage,
    WorthQueryApplicationCommitOutcome, WorthQueryApplicationEffectProgram,
    WorthQueryApplicationIdempotencyBinding,
};

use super::*;
use crate::estate_capability_admission::fixture::{
    delegation_world, delegation_world_with_parent_spec, request_scope, GrantSpec, APPROVER,
    BRANCH, ESTATE, GRANT, INSTITUTION, REVIEWER, UNRELATED_GOVERNANCE_GRANT,
};
use crate::BankReadControls;

const CHILD: CapabilityGrantId = CapabilityGrantId::new(401).unwrap();
const GRANDCHILD: CapabilityGrantId = CapabilityGrantId::new(402).unwrap();

#[test]
fn parent_revocation_after_activation_materialization_denies_final_commit() {
    let fixture = delegation_world("delegation-provider-parent-currentness");
    let specialist = fixture.authenticate();
    let action = delegated_action();
    let command = delegation_command(action).unwrap();
    let admission = fixture
        .runtime
        .admit_delegation(&specialist, action, command.child, &request_scope())
        .unwrap();
    let program = fixture
        .runtime
        .materialize_delegation(admission, command.child)
        .expect("the exact activation program must materialize while its parent is current");

    let revoked = fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            EstateAction::RevokeCapability {
                estate: ESTATE,
                grant: GRANT,
            },
            idempotency(121),
            &request_scope(),
        )
        .expect("the exact parent must revoke before the stale program reaches the provider");
    assert!(matches!(
        revoked,
        crate::BankMutationCommitOutcome::Committed(_)
    ));

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_capability_delegation(program, idempotency(123));
    assert_provider_currentness_denial(outcome);
    assert_child_absent(&fixture);
}

#[test]
fn ancestor_revocation_after_grandchild_materialization_denies_final_commit() {
    let fixture = delegation_world("delegation-provider-ancestor-currentness");
    let specialist = fixture.authenticate();
    let child = delegated_action();
    fixture
        .runtime
        .delegate_estate_capability(&specialist, child, idempotency(129), &request_scope())
        .expect("the root must activate the intermediate child");

    let approver = fixture.authenticate_approver();
    let action = delegated_action_from_child();
    let command = delegation_command(action).unwrap();
    let admission = fixture
        .runtime
        .admit_delegation(&approver, action, command.child, &request_scope())
        .expect("the active child and its lineage must admit the grandchild");
    let program = fixture
        .runtime
        .materialize_delegation(admission, command.child)
        .expect("the grandchild program must retain its exact ancestor lineage");

    fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            EstateAction::RevokeCapability {
                estate: ESTATE,
                grant: GRANT,
            },
            idempotency(131),
            &request_scope(),
        )
        .expect("the root must revoke before the retained grandchild reaches the provider");

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_capability_delegation(program, idempotency(133));
    assert_provider_currentness_denial(outcome);
    assert_grant_absent(&fixture, GRANDCHILD);
}

#[test]
fn generic_provider_entry_cannot_bypass_typed_activation_materialization() {
    let fixture = delegation_world("delegation-generic-provider-bypass");
    let specialist = fixture.authenticate();
    let program = materialize_generic(&fixture.runtime, &specialist)
        .expect("the hostile generic program must reach the public generic provider guard");

    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_application(program, idempotency(125));
    let WorthQueryApplicationCommitOutcome::Denied(denial) = outcome else {
        panic!("the generic provider entry must reject activation admissions");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCommitDenialKind::DelegationActivationRequired
    );
    assert_eq!(
        denial.stage(),
        WorthQueryApplicationCommitDenialStage::DelegationTransition
    );
    assert_child_absent(&fixture);
}

#[test]
fn activation_program_cannot_cross_runtime_session_authority() {
    let source = delegation_world("delegation-provider-source-runtime");
    let source_principal = source.authenticate();
    let action = delegated_action();
    let command = delegation_command(action).unwrap();
    let admission = source
        .runtime
        .admit_delegation(&source_principal, action, command.child, &request_scope())
        .unwrap();
    let program = source
        .runtime
        .materialize_delegation(admission, command.child)
        .expect("the source runtime must materialize its own activation program");

    let foreign = delegation_world("delegation-provider-foreign-runtime");
    let outcome = foreign
        .runtime
        .application_runtime()
        .compare_and_commit_capability_delegation(program, idempotency(137));
    assert!(matches!(
        outcome,
        WorthQueryApplicationCommitOutcome::Denied(_)
    ));
    assert_child_absent(&source);
    assert_child_absent(&foreign);
}

#[test]
fn unrelated_revocation_does_not_stale_the_exact_activation_support() {
    let fixture = delegation_world("delegation-provider-unrelated-currentness");
    let specialist = fixture.authenticate();
    let action = delegated_action();
    let command = delegation_command(action).unwrap();
    let admission = fixture
        .runtime
        .admit_delegation(&specialist, action, command.child, &request_scope())
        .unwrap();
    let program = fixture
        .runtime
        .materialize_delegation(admission, command.child)
        .expect("the exact activation program must retain only relevant support");

    fixture
        .runtime
        .revoke_estate_capability(
            &specialist,
            EstateAction::RevokeCapability {
                estate: ESTATE,
                grant: UNRELATED_GOVERNANCE_GRANT,
            },
            idempotency(139),
            &request_scope(),
        )
        .expect("an unrelated authority should revoke independently");
    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_capability_delegation(program, idempotency(141));
    assert!(matches!(
        outcome,
        WorthQueryApplicationCommitOutcome::Committed(_)
    ));
}

#[test]
fn parent_expiry_after_activation_materialization_denies_final_commit() {
    let expiry = epoch_seconds() + 20;
    let mut parent = GrantSpec::governance_view();
    parent.not_after = expiry;
    let fixture = delegation_world_with_parent_spec("delegation-provider-parent-expiry", parent);
    let specialist = fixture.authenticate();
    let mut action = delegated_action();
    let EstateAction::DelegateCapability { child, .. } = &mut action else {
        unreachable!("the fixture action is delegation")
    };
    child.scope.validity = CapabilityValidity::new(
        EstateMoment::from_epoch_seconds(0),
        EstateMoment::from_epoch_seconds(expiry),
    )
    .unwrap();
    let command = delegation_command(action).unwrap();
    let admission = fixture
        .runtime
        .admit_delegation(&specialist, action, command.child, &request_scope())
        .unwrap();
    let program = fixture
        .runtime
        .materialize_delegation(admission, command.child)
        .expect("the activation program must materialize before parent expiry");

    while epoch_seconds() <= expiry {
        std::thread::sleep(Duration::from_millis(10));
    }
    let outcome = fixture
        .runtime
        .application_runtime()
        .compare_and_commit_capability_delegation(program, idempotency(127));
    assert_provider_currentness_denial(outcome);
    assert_child_absent(&fixture);
}

fn delegated_action() -> EstateAction {
    EstateAction::DelegateCapability {
        estate: ESTATE,
        parent: GRANT,
        child: EstateCapabilityDelegationRequest {
            id: CHILD,
            grantee: APPROVER,
            scope: EstateCapabilityScope {
                account: None,
                estate: ESTATE,
                institution: INSTITUTION,
                branch: BRANCH,
                operation: EstateCapabilityOperation::ViewRestrictedEstate,
                purpose: EstateCapabilityPurpose::EstateAdministration,
                field: Some(RestrictedBankField::GovernanceMetadata),
                amount_ceiling: None,
                validity: CapabilityValidity::new(
                    EstateMoment::from_epoch_seconds(0),
                    EstateMoment::from_epoch_seconds(u64::MAX),
                )
                .unwrap(),
                delegation: DelegationLimit::generations(1),
                workflow_stage: EstateWorkflowStage::Administration,
            },
        },
    }
}

fn delegated_action_from_child() -> EstateAction {
    let EstateAction::DelegateCapability { child, .. } = delegated_action() else {
        unreachable!("the fixture action is delegation")
    };
    EstateAction::DelegateCapability {
        estate: ESTATE,
        parent: CHILD,
        child: EstateCapabilityDelegationRequest {
            id: GRANDCHILD,
            grantee: REVIEWER,
            scope: EstateCapabilityScope {
                delegation: DelegationLimit::generations(0),
                ..child.scope
            },
        },
    }
}

fn materialize_generic(
    runtime: &BankIdentityRuntime,
    principal: &BankAuthenticatedPrincipal,
) -> Result<
    WorthQueryApplicationEffectProgram<
        BankSchema,
        DelegateEstateCapabilityOperation,
        EstateAction,
        EstateCase,
    >,
    BankEstateProgressionDenial,
> {
    let action = delegated_action();
    let command = delegation_command(action)?;
    let admission = runtime.admit_delegation(principal, action, command.child, &request_scope())?;
    let projected = runtime
        .invariant_projection()
        .project_admitted_operation(&admission, |reader, estate| {
            project_delegation(reader, estate, command.child)
        })
        .map_err(BankEstateProgressionDenial::from_projection)?;
    let (result, projection, _) = projected.into_parts();
    result.map_err(BankEstateProgressionDenial::CapabilityDelegationProjection)?;
    let reads = runtime
        .application_runtime()
        .begin_projected_application_read_attempt(admission, projection)
        .map_err(BankEstateProgressionDenial::from_attempt)?;
    reads
        .complete_projected_dependencies()?
        .begin_effect_program()
        .finish()
        .map_err(BankEstateProgressionDenial::from_attempt)
}

fn assert_provider_currentness_denial(outcome: WorthQueryApplicationCommitOutcome) {
    let WorthQueryApplicationCommitOutcome::Denied(denial) = outcome else {
        panic!("stale parent support must deny the materialized activation program");
    };
    assert_eq!(
        denial.kind(),
        WorthQueryApplicationCommitDenialKind::ProviderRejected
    );
    assert_eq!(
        denial.stage(),
        WorthQueryApplicationCommitDenialStage::DecisionReadSet
    );
}

fn assert_child_absent(fixture: &crate::estate_capability_admission::fixture::CapabilityFixture) {
    assert_grant_absent(fixture, CHILD);
}

fn assert_grant_absent(
    fixture: &crate::estate_capability_admission::fixture::CapabilityFixture,
    grant: CapabilityGrantId,
) {
    let result = fixture
        .runtime
        .query(crate::queries::estate_governance_context(ESTATE))
        .as_principal(&fixture.authenticate_executor())
        .controls(BankReadControls::current(request_scope(), 1, 20_000).unwrap())
        .execute()
        .expect("independent governance authority must read authoritative post-denial state");
    assert!(result.rows()[0]
        .capabilities()
        .iter()
        .all(|capability| capability.id() != grant));
}

fn idempotency(seed: u8) -> WorthQueryApplicationIdempotencyBinding {
    WorthQueryApplicationIdempotencyBinding::new([seed; 32], [seed + 1; 32])
}

fn epoch_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("test time follows the Unix epoch")
        .as_secs()
}
