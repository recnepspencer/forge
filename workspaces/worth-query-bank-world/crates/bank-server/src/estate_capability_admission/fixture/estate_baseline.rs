use bank_domain::{
    estate::{
        BankEstateWorld, CapabilityGrantId, CapabilityValidity, DelegationLimit, EstateBranch,
        EstateCapabilityGrant, EstateCapabilityScope, EstateCase, EstateCaseStatus,
        EstateDeathNotice, EstateEmployeeAssignment, EstateLegalAuthority, EstateMoment,
        EstateWorkflowStage, LegalAuthorityKind,
    },
    model::{BankPrincipalId, EmployeeRole},
};

use super::{
    GrantSpec, ACCOUNT, ALTERNATE_BRANCH, APPROVER, APPROVER_ASSIGNMENT, ASSIGNMENT, AUTHORITY,
    BRANCH, DECEASED, ESTATE, EXECUTOR, INSTITUTION, NOTICE, OTHER_AUTHORITY, REVIEWER,
    REVIEWER_ASSIGNMENT, SPECIALIST,
};

pub(super) fn base_estate(
    stage: EstateWorkflowStage,
    authority_holder: BankPrincipalId,
) -> BankEstateWorld {
    let estate = install_case_truth(stage);
    let estate = install_staff_assignments(estate);
    install_legal_authorities(estate, authority_holder)
}

fn install_case_truth(stage: EstateWorkflowStage) -> BankEstateWorld {
    BankEstateWorld::default()
        .with_branch(EstateBranch {
            id: BRANCH,
            institution: INSTITUTION,
        })
        .with_branch(EstateBranch {
            id: ALTERNATE_BRANCH,
            institution: INSTITUTION,
        })
        .with_death_notice(EstateDeathNotice {
            id: NOTICE,
            subject: DECEASED,
            status: bank_domain::estate::DeathNoticeStatus::Verified,
        })
        .with_case(EstateCase {
            id: ESTATE,
            institution: INSTITUTION,
            branch: BRANCH,
            deceased: DECEASED,
            account: ACCOUNT,
            death_notice: NOTICE,
            stage,
            status: EstateCaseStatus::Open,
        })
}

fn install_staff_assignments(estate: BankEstateWorld) -> BankEstateWorld {
    estate
        .with_assignment(EstateEmployeeAssignment {
            id: ASSIGNMENT,
            principal: SPECIALIST,
            institution: INSTITUTION,
            branch: BRANCH,
            role: EmployeeRole::EstateSpecialist,
        })
        .with_estate_assignment(ESTATE, ASSIGNMENT)
        .with_assignment(EstateEmployeeAssignment {
            id: APPROVER_ASSIGNMENT,
            principal: APPROVER,
            institution: INSTITUTION,
            branch: BRANCH,
            role: EmployeeRole::EstateSpecialist,
        })
        .with_estate_assignment(ESTATE, APPROVER_ASSIGNMENT)
        .with_assignment(EstateEmployeeAssignment {
            id: REVIEWER_ASSIGNMENT,
            principal: REVIEWER,
            institution: INSTITUTION,
            branch: BRANCH,
            role: EmployeeRole::Compliance,
        })
        .with_estate_assignment(ESTATE, REVIEWER_ASSIGNMENT)
}

fn install_legal_authorities(
    estate: BankEstateWorld,
    authority_holder: BankPrincipalId,
) -> BankEstateWorld {
    estate
        .with_legal_authority(EstateLegalAuthority {
            id: AUTHORITY,
            estate: ESTATE,
            holder: authority_holder,
            kind: LegalAuthorityKind::CourtAppointment,
            recognized: false,
        })
        .with_legal_authority(EstateLegalAuthority {
            id: OTHER_AUTHORITY,
            estate: ESTATE,
            holder: EXECUTOR,
            kind: LegalAuthorityKind::SmallEstateAffidavit,
            recognized: false,
        })
}

pub(super) fn grant(
    id: CapabilityGrantId,
    grantee: BankPrincipalId,
    spec: GrantSpec,
) -> EstateCapabilityGrant {
    EstateCapabilityGrant {
        id,
        grantor: DECEASED,
        grantee,
        scope: EstateCapabilityScope {
            account: spec.account,
            estate: ESTATE,
            institution: INSTITUTION,
            branch: BRANCH,
            operation: spec.operation,
            purpose: spec.purpose,
            field: spec.field,
            amount_ceiling: spec.amount_ceiling,
            validity: CapabilityValidity::new(
                EstateMoment::from_epoch_seconds(spec.not_before),
                EstateMoment::from_epoch_seconds(spec.not_after),
            )
            .unwrap(),
            delegation: DelegationLimit::none(),
            workflow_stage: spec.workflow,
        },
        parent: None,
        status: spec.status,
    }
}

pub(super) fn extra_principal(ordinal: usize) -> BankPrincipalId {
    BankPrincipalId::new(1_000 + ordinal as u64).unwrap()
}
