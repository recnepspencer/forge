use bank_domain::{proposals::BankProposalDenial, schema::AccountStatus};
use bank_server::{
    BankEstateDisbursementProjectionDenial, BankEstateProgressionDenial, BankMutationCommitOutcome,
};
use worth_foundational::facade::{
    AdmissionReadinessProfile, CertificationPostureProfile, CompatibilityPostureProfile,
    DiagnosticRichnessProfile, FoundationalBoundaryArtifactCategory,
    FoundationalBoundaryEvidenceCloseoutDisposition, FoundationalBoundaryEvidenceExecutionPosture,
    FoundationalBoundaryEvidenceFreshnessPosture, FoundationalBoundaryEvidenceLocality,
    FoundationalBoundaryEvidenceReceiptKind, FoundationalDiagnosticDenialClass,
    FoundationalDiagnosticLocalityClaim, FoundationalDiagnosticOutcomeKind,
    FoundationalDiagnosticRow, FoundationalDiagnosticWidenedFalloutPosture,
    FoundationalProfileAttachmentTargetKind, FoundationalProfileSet, FoundationalProfileSetInput,
    RetentionDeliveryProfile, SupportPostureProfile,
};
use worth_query_host::facade::{
    primary_graph::{
        WorthQueryApplicationAuthorizationExplanationCause, WorthQueryOperationAuthorizationDenial,
        WorthQueryOperationAuthorizationDenialKind,
    },
    publication::domain_computation::{
        publish_application_authorization_denial,
        WorthQueryApplicationAuthorizationPublicationProfile,
        WorthQueryPublishedApplicationAuthorizationDenial,
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
        assert_authorization_publication(&denial, ExpectedAuthorizationPublication { cause, code });
        assert_no_disbursement_effects(&fixture);
    }
}

struct ExpectedAuthorizationPublication {
    cause: WorthQueryApplicationAuthorizationExplanationCause,
    code: &'static str,
}

fn assert_authorization_publication(
    denial: &WorthQueryOperationAuthorizationDenial,
    expected: ExpectedAuthorizationPublication,
) {
    let profile = publication_profile();
    let published = publish_application_authorization_denial(
        denial,
        WorthQueryApplicationAuthorizationPublicationProfile::exact(profile),
    )
    .unwrap();

    assert_eq!(published.artifact().denial(), denial);
    assert_eq!(published.artifact().cause(), expected.cause);
    assert_boundary_profile(&published, profile);
    assert_violation_diagnostic(&published, expected.code);
    assert_denied_and_publication_receipts(&published);
    assert_exact_provenance(&published, denial);
}

fn assert_boundary_profile(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    expected_profile: FoundationalProfileSet,
) {
    assert_eq!(
        published.boundary_category(),
        FoundationalBoundaryArtifactCategory::Artifact
    );
    assert_eq!(
        published.boundary().payload().target_kind(),
        FoundationalProfileAttachmentTargetKind::BoundaryArtifact
    );
    let progression = published.boundary().payload().profile();
    assert_eq!(progression.requested(), &expected_profile);
    assert_eq!(progression.admitted(), &expected_profile);
    assert_eq!(progression.materialized(), &expected_profile);
}

fn assert_violation_diagnostic(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    expected_code: &str,
) {
    assert_eq!(
        published.explanation().outcome_kind(),
        FoundationalDiagnosticOutcomeKind::Violation
    );
    assert_eq!(
        published.explanation().rows()[0].code().as_str(),
        expected_code
    );
    let FoundationalDiagnosticRow::Decision(row) = &published.explanation().rows()[0] else {
        panic!("conflict publication must contain one decision row");
    };
    assert_eq!(
        row.denial_class(),
        Some(FoundationalDiagnosticDenialClass::PolicyDenied)
    );
    assert_eq!(
        row.locality_claim(),
        FoundationalDiagnosticLocalityClaim::ExactSubject
    );
    assert_eq!(
        row.widened_fallout_posture(),
        FoundationalDiagnosticWidenedFalloutPosture::NotWidened
    );
}

fn assert_denied_and_publication_receipts(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
) {
    let closeout = published.denied_closeout_receipt();
    assert_eq!(
        closeout.closeout_disposition(),
        Some(FoundationalBoundaryEvidenceCloseoutDisposition::Denied)
    );
    assert_eq!(
        closeout.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::NotExecuted
    );
    assert_eq!(
        closeout.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Closeout
    );
    assert!(!closeout.did_execute());
    let publication = published.publication_receipt();
    assert_eq!(
        publication.execution_posture(),
        FoundationalBoundaryEvidenceExecutionPosture::Executed
    );
    assert_eq!(
        publication.receipt_kind(),
        FoundationalBoundaryEvidenceReceiptKind::Publication
    );
    assert!(publication.did_execute());
}

fn assert_exact_provenance(
    published: &WorthQueryPublishedApplicationAuthorizationDenial,
    denial: &WorthQueryOperationAuthorizationDenial,
) {
    assert_eq!(
        published.provenance().locality(),
        FoundationalBoundaryEvidenceLocality::Current
    );
    assert_eq!(
        published.provenance().freshness_posture(),
        FoundationalBoundaryEvidenceFreshnessPosture::FreshRetained
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
