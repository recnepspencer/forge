use bank_domain::estate::{
    BankEstateWorld, BranchId, CapabilityGrantId, CapabilityGrantStatus, CapabilityValidity,
    DeathNoticeId, DeathNoticeStatus, DelegationLimit, EstateBranch, EstateCapabilityGrant,
    EstateCapabilityOperation, EstateCapabilityPurpose, EstateCapabilityScope, EstateCase,
    EstateCaseId, EstateCaseStatus, EstateDeathNotice, EstateEmployeeAssignment,
    EstateLegalAuthority, EstateMoment, EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind,
    RestrictedBankField,
};
use bank_domain::model::{
    AccountId, BankPrincipalId, EmployeeAssignmentId, EmployeeRole, InstitutionId, Money,
};

pub(super) fn estate_world(
    institution: InstitutionId,
    specialist: BankPrincipalId,
) -> BankEstateWorld {
    let estate = EstateCaseId::new(3).unwrap();
    let alternate_estate = EstateCaseId::new(4).unwrap();
    let branch = BranchId::new(2).unwrap();
    let deceased = BankPrincipalId::new(2).unwrap();
    let mut world = BankEstateWorld::default()
        .with_branch(EstateBranch {
            id: branch,
            institution,
        })
        .with_death_notice(EstateDeathNotice {
            id: DeathNoticeId::new(12).unwrap(),
            subject: deceased,
            status: DeathNoticeStatus::Reported,
        })
        .with_death_notice(EstateDeathNotice {
            id: DeathNoticeId::new(13).unwrap(),
            subject: BankPrincipalId::new(3).unwrap(),
            status: DeathNoticeStatus::Reported,
        })
        .with_case(EstateCase {
            id: estate,
            institution,
            branch,
            deceased,
            account: AccountId::new(101).unwrap(),
            death_notice: DeathNoticeId::new(12).unwrap(),
            stage: EstateWorkflowStage::Administration,
            status: EstateCaseStatus::Open,
        })
        .with_case(EstateCase {
            id: alternate_estate,
            institution,
            branch,
            deceased: BankPrincipalId::new(3).unwrap(),
            account: AccountId::new(102).unwrap(),
            death_notice: DeathNoticeId::new(13).unwrap(),
            stage: EstateWorkflowStage::Administration,
            status: EstateCaseStatus::Open,
        });
    for (id, principal, role) in [
        (2, specialist, EmployeeRole::EstateSpecialist),
        (
            3,
            BankPrincipalId::new(5).unwrap(),
            EmployeeRole::EstateSpecialist,
        ),
        (
            4,
            BankPrincipalId::new(6).unwrap(),
            EmployeeRole::Compliance,
        ),
    ] {
        let assignment = EmployeeAssignmentId::new(id).unwrap();
        world = world
            .with_assignment(EstateEmployeeAssignment {
                id: assignment,
                principal,
                institution,
                branch,
                role,
            })
            .with_estate_assignment(estate, assignment)
            .with_estate_assignment(alternate_estate, assignment);
    }
    world = world
        .with_beneficiary(estate, BankPrincipalId::new(3).unwrap())
        .with_joint_owner(
            AccountId::new(102).unwrap(),
            BankPrincipalId::new(3).unwrap(),
        )
        .with_legal_authority(EstateLegalAuthority {
            id: LegalAuthorityId::new(15).unwrap(),
            estate,
            holder: BankPrincipalId::new(4).unwrap(),
            kind: LegalAuthorityKind::CourtAppointment,
            recognized: true,
        })
        .with_executor(estate, BankPrincipalId::new(4).unwrap());
    let grants = [
        grant(
            14,
            deceased,
            specialist,
            estate,
            institution,
            branch,
            EstateCapabilityOperation::NotifyDeath,
            EstateCapabilityPurpose::EstateAdministration,
            None,
            None,
        ),
        grant(
            25,
            BankPrincipalId::new(3).unwrap(),
            specialist,
            alternate_estate,
            institution,
            branch,
            EstateCapabilityOperation::NotifyDeath,
            EstateCapabilityPurpose::EstateAdministration,
            None,
            None,
        ),
        grant(
            16,
            deceased,
            specialist,
            estate,
            institution,
            branch,
            EstateCapabilityOperation::DisburseEstate,
            EstateCapabilityPurpose::EstateDisbursement,
            None,
            Some(Money::from_minor(10_000).unwrap()),
        ),
        grant(
            19,
            deceased,
            specialist,
            estate,
            institution,
            branch,
            EstateCapabilityOperation::RequestEmergencyAccess,
            EstateCapabilityPurpose::EmergencyProtection,
            None,
            None,
        ),
        grant(
            20,
            deceased,
            specialist,
            estate,
            institution,
            branch,
            EstateCapabilityOperation::ViewRestrictedEstate,
            EstateCapabilityPurpose::EmergencyProtection,
            Some(RestrictedBankField::AccountDetails),
            None,
        ),
        grant(
            21,
            deceased,
            specialist,
            estate,
            institution,
            branch,
            EstateCapabilityOperation::ApproveEmergencyAccess,
            EstateCapabilityPurpose::EmergencyProtection,
            None,
            None,
        ),
        grant(
            22,
            deceased,
            BankPrincipalId::new(5).unwrap(),
            estate,
            institution,
            branch,
            EstateCapabilityOperation::ApproveEmergencyAccess,
            EstateCapabilityPurpose::EmergencyProtection,
            None,
            None,
        ),
        grant(
            23,
            deceased,
            BankPrincipalId::new(5).unwrap(),
            estate,
            institution,
            branch,
            EstateCapabilityOperation::RevokeEmergencyAccess,
            EstateCapabilityPurpose::EmergencyProtection,
            None,
            None,
        ),
        grant(
            24,
            deceased,
            BankPrincipalId::new(6).unwrap(),
            estate,
            institution,
            branch,
            EstateCapabilityOperation::CompleteMandatoryReview,
            EstateCapabilityPurpose::MandatoryReview,
            None,
            None,
        ),
    ];
    grants.into_iter().fold(world, BankEstateWorld::with_grant)
}

fn grant(
    id: u64,
    grantor: BankPrincipalId,
    grantee: BankPrincipalId,
    estate: EstateCaseId,
    institution: InstitutionId,
    branch: BranchId,
    operation: EstateCapabilityOperation,
    purpose: EstateCapabilityPurpose,
    field: Option<RestrictedBankField>,
    amount_ceiling: Option<Money<bank_domain::model::USD>>,
) -> EstateCapabilityGrant {
    EstateCapabilityGrant {
        id: CapabilityGrantId::new(id).unwrap(),
        grantor,
        grantee,
        scope: EstateCapabilityScope {
            account: (operation == EstateCapabilityOperation::DisburseEstate)
                .then(|| AccountId::new(101).unwrap()),
            estate,
            institution,
            branch,
            operation,
            purpose,
            field,
            amount_ceiling,
            validity: CapabilityValidity::new(
                EstateMoment::from_epoch_seconds(0),
                EstateMoment::from_epoch_seconds(u64::MAX),
            )
            .unwrap(),
            delegation: DelegationLimit::none(),
            workflow_stage: EstateWorkflowStage::Administration,
        },
        parent: None,
        status: CapabilityGrantStatus::Active,
    }
}
