use bank_domain::{proposals::BankProposalDenial, schema::AccountStatus};
use bank_server::{
    BankEstateDisbursementProjectionDenial, BankEstateProgressionDenial, BankMutationCommitOutcome,
};
use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryEvidenceCloseoutDisposition,
    FoundationalBoundaryEvidenceExecutionPosture, FoundationalDiagnosticOutcomeKind,
    FoundationalProfileSet, FoundationalProfileSetInput, RetentionDeliveryProfile,
    SupportPostureProfile,
};
use worth_query_host::facade::{
    primary_graph::{
        WorthQueryApplicationAuthorizationExplanationCause, WorthQueryOperationAuthorizationDenial,
        WorthQueryOperationAuthorizationDenialKind,
    },
    publication::domain_computation::{
        publish_application_authorization_denial,
        WorthQueryApplicationAuthorizationPublicationProfile,
    },
};

use super::{assert_no_disbursement_effects, disburse, fixture::*, idempotency};

#[test]
fn exact_beneficiary_and_destination_joint_ownership_are_required() {
    for (ordinal, posture) in [
        BeneficiaryPosture::Missing,
        BeneficiaryPosture::WrongEstate,
        BeneficiaryPosture::JointOwnerMissing,
        BeneficiaryPosture::JointOwnerWrongAccount,
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = hostile_world(
            &format!("estate-disbursement-beneficiary-{posture:?}"),
            DisbursementWorldSpec {
                beneficiary: posture,
                ..DisbursementWorldSpec::ready()
            },
        );
        let specialist = fixture.authenticate_actor();
        let denial = disburse(
            &fixture,
            &specialist,
            fixture.action(250),
            idempotency(61 + ordinal as u8),
        )
        .expect_err("missing or misdirected beneficiary truth must deny");
        match posture {
            BeneficiaryPosture::Missing | BeneficiaryPosture::WrongEstate => assert!(matches!(
                denial,
                BankEstateProgressionDenial::EstateDisbursementProjection(
                    BankEstateDisbursementProjectionDenial::EstateBeneficiaryRelationMissing
                )
            )),
            BeneficiaryPosture::JointOwnerMissing | BeneficiaryPosture::JointOwnerWrongAccount => {
                assert!(matches!(
                    denial,
                    BankEstateProgressionDenial::EstateDisbursementProjection(
                        BankEstateDisbursementProjectionDenial::EstateJointOwnerRelationMissing
                    )
                ))
            }
            BeneficiaryPosture::Ready => unreachable!(),
        }
        if ordinal == 0 {
            assert_no_disbursement_effects(&fixture);
        }
    }
}

#[test]
fn a_recognized_exact_executor_authority_is_required_but_not_artificially_unique() {
    for (ordinal, posture) in [
        ExecutorPosture::Missing,
        ExecutorPosture::Unrecognized,
        ExecutorPosture::WrongHolder,
    ]
    .into_iter()
    .enumerate()
    {
        let fixture = hostile_world(
            &format!("estate-disbursement-executor-{posture:?}"),
            DisbursementWorldSpec {
                executor: posture,
                ..DisbursementWorldSpec::ready()
            },
        );
        let specialist = fixture.authenticate_actor();
        let denial = disburse(
            &fixture,
            &specialist,
            fixture.action(250),
            idempotency(71 + ordinal as u8),
        )
        .expect_err("missing exact recognized executor authority must deny");
        assert!(matches!(
            denial,
            BankEstateProgressionDenial::EstateDisbursementProjection(
                BankEstateDisbursementProjectionDenial::RecognizedExecutorAuthorityMissing
            )
        ));
    }

    let fixture = hostile_world(
        "estate-disbursement-multiple-executors",
        DisbursementWorldSpec {
            executor: ExecutorPosture::MultipleLawful,
            ..DisbursementWorldSpec::ready()
        },
    );
    let specialist = fixture.authenticate_actor();
    let outcome = disburse(&fixture, &specialist, fixture.action(250), idempotency(74))
        .expect("multiple lawful recognized executor authorities must remain admissible");
    let BankMutationCommitOutcome::Committed(receipt) = outcome else {
        panic!("multiple lawful executors must commit: {outcome:?}");
    };
    assert_eq!(receipt.decision_fact_count(), Some(41));
}

#[test]
fn source_and_destination_must_both_be_open() {
    for (ordinal, source_status, destination_status, expected_account) in [
        (0, AccountStatus::Frozen, AccountStatus::Open, 0),
        (1, AccountStatus::Open, AccountStatus::Closed, 1),
    ] {
        let fixture = hostile_world(
            &format!("estate-disbursement-status-{ordinal}"),
            DisbursementWorldSpec {
                source_status,
                destination_status,
                ..DisbursementWorldSpec::ready()
            },
        );
        let specialist = fixture.authenticate_actor();
        let denial = disburse(
            &fixture,
            &specialist,
            fixture.action(250),
            idempotency(81 + ordinal),
        )
        .expect_err("a non-open movement endpoint must deny");
        let expected = if expected_account == 0 {
            fixture.source
        } else {
            fixture.destination
        };
        let status = if expected_account == 0 {
            source_status
        } else {
            destination_status
        };
        assert!(matches!(
            denial,
            BankEstateProgressionDenial::Proposal(BankProposalDenial::AccountStatus {
                account,
                status: observed,
            }) if account == expected && observed == status
        ));
    }
}

#[test]
fn beneficiary_and_executor_actors_deny_at_capability_composition() {
    for (ordinal, actor_conflict) in [ActorConflict::Beneficiary, ActorConflict::Executor]
        .into_iter()
        .enumerate()
    {
        let fixture = hostile_world(
            &format!("estate-disbursement-conflict-{actor_conflict:?}"),
            DisbursementWorldSpec {
                actor_conflict,
                ..DisbursementWorldSpec::ready()
            },
        );
        let specialist = fixture.authenticate_actor();
        let denial = disburse(
            &fixture,
            &specialist,
            fixture.action(250),
            idempotency(91 + ordinal as u8),
        )
        .expect_err("conflicted authority must deny before invariant projection");
        let BankEstateProgressionDenial::Authorization(denial) = denial else {
            panic!("actor conflict must deny at Query authorization")
        };
        let (kind, cause, code) = match actor_conflict {
            ActorConflict::Beneficiary => (
                WorthQueryOperationAuthorizationDenialKind::ConflictRuleMatched,
                WorthQueryApplicationAuthorizationExplanationCause::Conflict,
                "worth.query.authorization.conflict",
            ),
            ActorConflict::Executor => (
                WorthQueryOperationAuthorizationDenialKind::SeparationOfDutyRuleMatched,
                WorthQueryApplicationAuthorizationExplanationCause::SeparationOfDuty,
                "worth.query.authorization.separation-of-duty",
            ),
            ActorConflict::None => unreachable!(),
        };
        assert_eq!(denial.kind(), kind);
        assert_authorization_publication(&denial, cause, code);
        assert_no_disbursement_effects(&fixture);
    }
}

fn assert_authorization_publication(
    denial: &WorthQueryOperationAuthorizationDenial,
    expected_cause: WorthQueryApplicationAuthorizationExplanationCause,
    expected_code: &str,
) {
    let published = publish_application_authorization_denial(
        denial,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(publication_profile()),
    )
    .unwrap();

    assert_eq!(published.artifact().denial(), denial);
    assert_eq!(published.artifact().cause(), expected_cause);
    assert_eq!(
        published.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Violation
    );
    assert_eq!(
        published.explanation().rows()[0].code().as_str(),
        expected_code
    );
    assert_eq!(
        published.denied_closeout_receipt().closeout_disposition(),
        Some(FoundationalBoundaryEvidenceCloseoutDisposition::Denied)
    );
    assert_eq!(
        published.denied_closeout_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::NotExecuted
    );
    assert_eq!(
        published.publication_receipt().execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    let locator = published
        .provenance()
        .source_basis()
        .boundary_artifact_locator()
        .unwrap();
    assert_eq!(
        locator.artifact_id().get(),
        denial.identity().unwrap().get()
    );
}

fn publication_profile() -> FoundationalProfileSet {
    FoundationalProfileSet::new(FoundationalProfileSetInput {
        diagnostic_richness: DiagnosticRichnessProfile::Standard,
        support_posture: SupportPostureProfile::SupportReady,
        compatibility_posture: CompatibilityPostureProfile::CompatibilityLowered,
        admission_readiness: AdmissionReadinessProfile::Admitted,
        retention_delivery: RetentionDeliveryProfile::Retained,
        certification_posture: CertificationPostureProfile::EvidenceBacked,
    })
    .unwrap()
}

#[test]
fn approved_emergency_graph_state_is_not_ordinary_disbursement_authority() {
    let fixture = hostile_world(
        "estate-disbursement-emergency-escalation",
        DisbursementWorldSpec {
            grant: GrantPosture::ApprovedEmergencyOnly,
            ..DisbursementWorldSpec::ready()
        },
    );
    let specialist = fixture.authenticate_actor();
    let denial = disburse(&fixture, &specialist, fixture.action(250), idempotency(101))
        .expect_err("an approved graph record does not replace ordinary command authority");
    assert!(matches!(
        denial,
        BankEstateProgressionDenial::Authorization(_)
    ));
}
