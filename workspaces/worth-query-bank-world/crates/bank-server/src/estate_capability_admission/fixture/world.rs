use bank_domain::{
    estate::{
        BankEstateWorld, CapabilityGrantId, DelegationLimit, EmergencyAccessReason,
        EmergencyAccessStatus, EstateEmergencyAccess, EstateEmployeeAssignment, EstateMoment,
        EstateWorkflowStage, MandatoryEstateReview, MandatoryReviewKind, MandatoryReviewStatus,
    },
    model::EmployeeRole,
    proposals::BankSnapshot,
};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use super::{
    base_estate, external_identity, extra_principal, grant, GrantSpec, ALTERNATE_BRANCH,
    ALTERNATE_EMERGENCY_BOUND_GRANT, ALTERNATE_INSTITUTION, APPROVAL_GRANT, APPROVER,
    APPROVER_ASSIGNMENT, APPROVER_DELEGATION_GRANT, APPROVER_REQUEST_GRANT,
    APPROVER_UPPER_BOUND_GRANT, ASSIGNMENT, CLOSED_ACCESS, CLOSE_GRANT, COMMAND_GRANT,
    COMPLETED_REVIEW, DECEASED, DELEGATED_GRANT, DELEGATION_EXECUTOR_ASSIGNMENT,
    DELEGATION_REVIEWER_ASSIGNMENT, DISBURSEMENT_GRANT, EMERGENCY_BOUND_GRANT, EXECUTOR, GRANT,
    INSTITUTION, LIFECYCLE_OBSERVER_GRANT, REQUESTED_ACCESS, REQUESTED_REVIEW, REVIEWER,
    REVIEWER_ASSIGNMENT, REVIEW_GRANT, REVOKE_CAPABILITY_GRANT, SELF_APPROVAL_GRANT, SPECIALIST,
    UNRELATED_GOVERNANCE_GRANT,
};
use crate::{BankEmployeeAssignmentSeed, BankIdentityRuntime, BankPrincipalSeed, BankWorldSeed};

#[path = "world/disbursement.rs"]
mod disbursement;
#[path = "world/foreign_estate.rs"]
mod foreign_estate;
#[path = "world/snapshot.rs"]
mod snapshot_fixture;

use foreign_estate::install_foreign_estate_revocation;
pub(crate) use foreign_estate::{foreign_estate_revocation_world, FOREIGN_ESTATE, FOREIGN_GRANT};
use snapshot_fixture::snapshot;

pub(super) struct FixtureWorldSpec<'a> {
    pub(super) scenario: &'a str,
    pub(super) spec: GrantSpec,
    pub(super) case_stage: EstateWorkflowStage,
    pub(super) specialist_holds_authority: bool,
    pub(super) unrelated_grants: usize,
    pub(super) composition: FixtureWorldComposition,
    pub(super) alternate_emergency_bound: Option<GrantSpec>,
}

#[derive(Clone, Copy)]
pub(super) enum FixtureWorldComposition {
    Admission,
    Lifecycle,
    GovernanceProjection,
    Delegation {
        command_authority: bool,
        parent_context: DelegationParentContext,
    },
    ForeignEstateRevocation,
    Release,
    Disbursement,
}

#[derive(Clone, Copy)]
pub(super) enum DelegationParentContext {
    Exact,
    Branch,
    Institution,
}

pub(super) struct InstalledFixtureWorld {
    pub(super) runtime: BankIdentityRuntime,
    pub(super) estate_world: BankEstateWorld,
    pub(super) identities: [WorthQueryExternalPrincipalIdentity; 5],
}

pub(super) fn install_fixture_world(spec: FixtureWorldSpec<'_>) -> InstalledFixtureWorld {
    let delegation_world = matches!(spec.composition, FixtureWorldComposition::Delegation { .. });
    let identities = identities(spec.scenario);
    let snapshot = if matches!(spec.composition, FixtureWorldComposition::Disbursement) {
        disbursement::snapshot()
    } else {
        snapshot(spec.unrelated_grants)
    };
    let estate = estate(&spec);
    let estate_world = estate.clone();
    let seed = seed(
        snapshot,
        estate,
        &identities,
        spec.scenario,
        spec.unrelated_grants,
        delegation_world,
    );
    let runtime = BankIdentityRuntime::install_world(seed)
        .expect("capability fixture runtime should install");
    InstalledFixtureWorld {
        runtime,
        estate_world,
        identities,
    }
}

fn identities(scenario: &str) -> [WorthQueryExternalPrincipalIdentity; 5] {
    [
        external_identity(scenario, "deceased"),
        external_identity(scenario, "specialist"),
        external_identity(scenario, "executor"),
        external_identity(scenario, "approver"),
        external_identity(scenario, "reviewer"),
    ]
}

fn estate(spec: &FixtureWorldSpec<'_>) -> BankEstateWorld {
    let holder = if spec.specialist_holds_authority {
        SPECIALIST
    } else {
        EXECUTOR
    };
    let estate =
        base_estate(spec.case_stage, holder).with_grant(grant(GRANT, SPECIALIST, spec.spec));
    let mut estate = match spec.composition {
        FixtureWorldComposition::Admission => estate,
        FixtureWorldComposition::Lifecycle => install_lifecycle_grants(estate),
        FixtureWorldComposition::GovernanceProjection => install_governance_projection(estate),
        FixtureWorldComposition::Delegation {
            command_authority,
            parent_context,
        } => install_delegation_grants(estate, command_authority, parent_context, spec.spec),
        FixtureWorldComposition::ForeignEstateRevocation => {
            install_foreign_estate_revocation(estate)
        }
        FixtureWorldComposition::Release => install_release_truth(estate),
        FixtureWorldComposition::Disbursement => disbursement::install_truth(estate),
    };
    if let Some(alternate) = spec.alternate_emergency_bound {
        estate = estate.with_grant(grant(
            ALTERNATE_EMERGENCY_BOUND_GRANT,
            SPECIALIST,
            alternate,
        ));
    }
    for ordinal in 0..spec.unrelated_grants {
        estate = estate.with_grant(grant(
            CapabilityGrantId::new(2_000 + ordinal as u64).unwrap(),
            extra_principal(ordinal),
            GrantSpec::view(),
        ));
    }
    estate
}

fn install_release_truth(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_legal_authority(bank_domain::estate::EstateLegalAuthority {
            id: super::AUTHORITY,
            estate: super::ESTATE,
            holder: super::EXECUTOR,
            kind: bank_domain::estate::LegalAuthorityKind::CourtAppointment,
            recognized: true,
        })
        .with_executor(super::ESTATE, super::EXECUTOR)
        .with_review(MandatoryEstateReview {
            id: super::COMPLETED_REVIEW,
            estate: super::ESTATE,
            kind: MandatoryReviewKind::EstateRelease,
            reviewer: Some(super::REVIEWER),
            status: MandatoryReviewStatus::Completed,
        })
}

fn install_delegation_grants(
    estate: BankEstateWorld,
    command_authority: bool,
    parent_context: DelegationParentContext,
    parent_spec: GrantSpec,
) -> BankEstateWorld {
    let mut parent = grant(super::GRANT, super::SPECIALIST, parent_spec);
    parent.scope.delegation = DelegationLimit::generations(2);
    match parent_context {
        DelegationParentContext::Exact => {}
        DelegationParentContext::Branch => parent.scope.branch = ALTERNATE_BRANCH,
        DelegationParentContext::Institution => {
            parent.scope.institution = ALTERNATE_INSTITUTION;
        }
    }
    let estate = estate
        .with_assignment(EstateEmployeeAssignment {
            id: DELEGATION_EXECUTOR_ASSIGNMENT,
            principal: EXECUTOR,
            institution: super::INSTITUTION,
            branch: super::BRANCH,
            role: EmployeeRole::EstateSpecialist,
        })
        .with_estate_assignment(super::ESTATE, DELEGATION_EXECUTOR_ASSIGNMENT)
        .with_assignment(EstateEmployeeAssignment {
            id: DELEGATION_REVIEWER_ASSIGNMENT,
            principal: REVIEWER,
            institution: super::INSTITUTION,
            branch: super::BRANCH,
            role: EmployeeRole::EstateSpecialist,
        })
        .with_estate_assignment(super::ESTATE, DELEGATION_REVIEWER_ASSIGNMENT)
        .with_grant(parent)
        .with_grant(grant(
            UNRELATED_GOVERNANCE_GRANT,
            EXECUTOR,
            GrantSpec::governance_view(),
        ));
    if !command_authority {
        return estate;
    }
    estate
        .with_grant(grant(COMMAND_GRANT, SPECIALIST, GrantSpec::delegate()))
        .with_grant(grant(
            APPROVER_DELEGATION_GRANT,
            APPROVER,
            GrantSpec::delegate(),
        ))
        .with_grant(grant(
            REVOKE_CAPABILITY_GRANT,
            SPECIALIST,
            GrantSpec::revoke_capability(),
        ))
}

fn install_governance_projection(estate: BankEstateWorld) -> BankEstateWorld {
    let mut parent = grant(GRANT, SPECIALIST, GrantSpec::governance_view());
    parent.scope.delegation = DelegationLimit::generations(1);
    let mut child = grant(DELEGATED_GRANT, APPROVER, GrantSpec::governance_view());
    child.parent = Some(GRANT);
    estate
        .with_grant(parent)
        .with_grant(child)
        .with_grant(grant(
            DISBURSEMENT_GRANT,
            REVIEWER,
            GrantSpec::disburse(50_000),
        ))
        .with_grant(grant(
            EMERGENCY_BOUND_GRANT,
            SPECIALIST,
            GrantSpec::emergency_view(),
        ))
        .with_review(MandatoryEstateReview {
            id: REQUESTED_REVIEW,
            estate: super::ESTATE,
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: None,
            status: MandatoryReviewStatus::Required,
        })
        .with_review(MandatoryEstateReview {
            id: COMPLETED_REVIEW,
            estate: super::ESTATE,
            kind: MandatoryReviewKind::EmergencyAccess,
            reviewer: Some(REVIEWER),
            status: MandatoryReviewStatus::Completed,
        })
        .with_emergency_access(EstateEmergencyAccess {
            id: REQUESTED_ACCESS,
            requester: SPECIALIST,
            approver: None,
            reviewer: None,
            grant: EMERGENCY_BOUND_GRANT,
            review: REQUESTED_REVIEW,
            reason: EmergencyAccessReason::PreventImmediateLoss,
            status: EmergencyAccessStatus::Requested,
            issued_at: EstateMoment::from_epoch_seconds(100),
            expires_at: EstateMoment::from_epoch_seconds(200),
        })
        .with_emergency_access(EstateEmergencyAccess {
            id: CLOSED_ACCESS,
            requester: SPECIALIST,
            approver: Some(APPROVER),
            reviewer: Some(REVIEWER),
            grant: EMERGENCY_BOUND_GRANT,
            review: COMPLETED_REVIEW,
            reason: EmergencyAccessReason::MeetLegalDeadline,
            status: EmergencyAccessStatus::Revoked,
            issued_at: EstateMoment::from_epoch_seconds(300),
            expires_at: EstateMoment::from_epoch_seconds(400),
        })
}

fn install_lifecycle_grants(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_grant(grant(
            COMMAND_GRANT,
            SPECIALIST,
            GrantSpec::emergency_request(),
        ))
        .with_grant(grant(
            APPROVAL_GRANT,
            APPROVER,
            GrantSpec::emergency_approval(),
        ))
        .with_grant(grant(
            SELF_APPROVAL_GRANT,
            SPECIALIST,
            GrantSpec::emergency_approval(),
        ))
        .with_grant(grant(
            APPROVER_REQUEST_GRANT,
            APPROVER,
            GrantSpec::emergency_request(),
        ))
        .with_grant(grant(
            APPROVER_UPPER_BOUND_GRANT,
            APPROVER,
            GrantSpec::emergency_view(),
        ))
        .with_grant(grant(CLOSE_GRANT, APPROVER, GrantSpec::emergency_close()))
        .with_grant(grant(REVIEW_GRANT, REVIEWER, GrantSpec::mandatory_review()))
        .with_grant(grant(
            LIFECYCLE_OBSERVER_GRANT,
            SPECIALIST,
            GrantSpec::governance_view(),
        ))
        .with_grant(grant(
            REVOKE_CAPABILITY_GRANT,
            SPECIALIST,
            GrantSpec::revoke_capability(),
        ))
}

fn seed(
    snapshot: BankSnapshot,
    estate: BankEstateWorld,
    identities: &[WorthQueryExternalPrincipalIdentity; 5],
    scenario: &str,
    unrelated_grants: usize,
    delegation_world: bool,
) -> BankWorldSeed {
    let mut seed = BankWorldSeed::new(snapshot)
        .principal(BankPrincipalSeed::enabled(DECEASED, identities[0].clone()))
        .principal(BankPrincipalSeed::enabled(
            SPECIALIST,
            identities[1].clone(),
        ))
        .principal(BankPrincipalSeed::enabled(EXECUTOR, identities[2].clone()))
        .principal(BankPrincipalSeed::enabled(APPROVER, identities[3].clone()))
        .principal(BankPrincipalSeed::enabled(REVIEWER, identities[4].clone()))
        .employee(BankEmployeeAssignmentSeed::new(
            ASSIGNMENT,
            INSTITUTION,
            SPECIALIST,
            EmployeeRole::EstateSpecialist,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            APPROVER_ASSIGNMENT,
            INSTITUTION,
            APPROVER,
            EmployeeRole::EstateSpecialist,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            REVIEWER_ASSIGNMENT,
            INSTITUTION,
            REVIEWER,
            EmployeeRole::Compliance,
        ))
        .estate(estate);
    if delegation_world {
        seed = seed
            .employee(BankEmployeeAssignmentSeed::new(
                DELEGATION_EXECUTOR_ASSIGNMENT,
                INSTITUTION,
                EXECUTOR,
                EmployeeRole::EstateSpecialist,
            ))
            .employee(BankEmployeeAssignmentSeed::new(
                DELEGATION_REVIEWER_ASSIGNMENT,
                INSTITUTION,
                REVIEWER,
                EmployeeRole::EstateSpecialist,
            ));
    }
    for ordinal in 0..unrelated_grants {
        seed = seed.principal(BankPrincipalSeed::enabled(
            extra_principal(ordinal),
            external_identity(scenario, &format!("extra-{ordinal}")),
        ));
    }
    seed
}
