use bank_domain::{
    estate::{
        BankEstateWorld, BranchId, CapabilityGrantId, CapabilityGrantStatus, CapabilityValidity,
        DeathNoticeId, DeathNoticeStatus, DelegationLimit, EstateAction, EstateBranch,
        EstateCapabilityGrant, EstateCapabilityOperation, EstateCapabilityPurpose,
        EstateCapabilityScope, EstateCase, EstateCaseId, EstateCaseStatus, EstateDeathNotice,
        EstateEmployeeAssignment, EstateMoment, EstateWorkflowStage,
    },
    model::{
        AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, EmployeeAssignmentId,
        EmployeeRole, InstitutionId,
    },
    proposals::BankSnapshotBuilder,
    schema::AccountStatus,
};
use bank_server::{
    BankAuthenticatedPrincipal, BankEmployeeAssignmentSeed, BankPrincipalSeed, BankWorldSeed,
};

use crate::support::{block_on, runtime, CausalCredential, DynamicIdentity, TestIdentityWorld};

pub(super) struct NotificationFixture {
    pub(super) world: TestIdentityWorld,
    identities: [DynamicIdentity; 4],
    pub(super) estate: EstateCaseId,
    pub(super) notice: DeathNoticeId,
    pub(super) foreign_notice: DeathNoticeId,
    pub(super) deceased: BankPrincipalId,
    pub(super) foreign_deceased: BankPrincipalId,
    pub(super) other_subject: BankPrincipalId,
}

impl NotificationFixture {
    pub(super) fn authenticate_specialist(&self) -> BankAuthenticatedPrincipal {
        let request = crate::support::request_scope();
        block_on(self.world.runtime.authenticate_with(
            &self.world.authentication,
            CausalCredential::for_identity(&self.identities[1]),
            &request,
        ))
        .expect("the causally mapped estate specialist should authenticate")
    }

    pub(super) const fn action(
        &self,
        notice: DeathNoticeId,
        subject: BankPrincipalId,
    ) -> EstateAction {
        EstateAction::NotifyDeath {
            estate: self.estate,
            notice,
            subject,
        }
    }
}

pub(super) fn notification_world(
    scenario: &str,
    status: DeathNoticeStatus,
) -> NotificationFixture {
    let identities = [
        DynamicIdentity::new(&format!("{scenario}-deceased")),
        DynamicIdentity::new(&format!("{scenario}-specialist")),
        DynamicIdentity::new(&format!("{scenario}-other-subject")),
        DynamicIdentity::new(&format!("{scenario}-foreign-deceased")),
    ];
    let snapshot = snapshot();
    let principals = [DECEASED, SPECIALIST, OTHER_SUBJECT, FOREIGN_DECEASED];
    let seed = identities.iter().enumerate().fold(
        BankWorldSeed::new(snapshot)
            .employee(BankEmployeeAssignmentSeed::new(
                ASSIGNMENT,
                INSTITUTION,
                SPECIALIST,
                EmployeeRole::EstateSpecialist,
            ))
            .estate(estate_world(status)),
        |seed, (ordinal, identity)| {
            seed.principal(BankPrincipalSeed::enabled(
                principals[ordinal],
                identity.external(),
            ))
        },
    );
    NotificationFixture {
        world: runtime(seed),
        identities,
        estate: ESTATE,
        notice: NOTICE,
        foreign_notice: FOREIGN_NOTICE,
        deceased: DECEASED,
        foreign_deceased: FOREIGN_DECEASED,
        other_subject: OTHER_SUBJECT,
    }
}

const INSTITUTION: InstitutionId = InstitutionId::new(1).unwrap();
const BRANCH: BranchId = BranchId::new(2).unwrap();
const ESTATE: EstateCaseId = EstateCaseId::new(3).unwrap();
const FOREIGN_ESTATE: EstateCaseId = EstateCaseId::new(4).unwrap();
const ESTATE_ACCOUNT: AccountId = AccountId::new(5).unwrap();
const FOREIGN_ACCOUNT: AccountId = AccountId::new(6).unwrap();
const DECEASED: BankPrincipalId = BankPrincipalId::new(7).unwrap();
const SPECIALIST: BankPrincipalId = BankPrincipalId::new(8).unwrap();
const OTHER_SUBJECT: BankPrincipalId = BankPrincipalId::new(9).unwrap();
const FOREIGN_DECEASED: BankPrincipalId = BankPrincipalId::new(10).unwrap();
const ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(11).unwrap();
const NOTICE: DeathNoticeId = DeathNoticeId::new(12).unwrap();
const FOREIGN_NOTICE: DeathNoticeId = DeathNoticeId::new(13).unwrap();
const GRANT: CapabilityGrantId = CapabilityGrantId::new(14).unwrap();

fn snapshot() -> bank_domain::proposals::BankSnapshot {
    BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
        .institution(INSTITUTION)
        .principal(DECEASED)
        .principal(SPECIALIST)
        .principal(OTHER_SUBJECT)
        .principal(FOREIGN_DECEASED)
        .personal_account(
            ESTATE_ACCOUNT,
            INSTITUTION,
            DECEASED,
            AccountName::new("Estate Operating").unwrap(),
            AccountStatus::Open,
        )
        .personal_account(
            FOREIGN_ACCOUNT,
            INSTITUTION,
            FOREIGN_DECEASED,
            AccountName::new("Foreign Estate Operating").unwrap(),
            AccountStatus::Open,
        )
        .build()
        .expect("the death-notification snapshot should be valid")
}

fn estate_world(status: DeathNoticeStatus) -> BankEstateWorld {
    BankEstateWorld::default()
        .with_branch(EstateBranch {
            id: BRANCH,
            institution: INSTITUTION,
        })
        .with_death_notice(EstateDeathNotice {
            id: NOTICE,
            subject: DECEASED,
            status,
        })
        .with_death_notice(EstateDeathNotice {
            id: FOREIGN_NOTICE,
            subject: FOREIGN_DECEASED,
            status: DeathNoticeStatus::Reported,
        })
        .with_case(estate_case(ESTATE, ESTATE_ACCOUNT, DECEASED, NOTICE))
        .with_case(estate_case(
            FOREIGN_ESTATE,
            FOREIGN_ACCOUNT,
            FOREIGN_DECEASED,
            FOREIGN_NOTICE,
        ))
        .with_assignment(EstateEmployeeAssignment {
            id: ASSIGNMENT,
            principal: SPECIALIST,
            institution: INSTITUTION,
            branch: BRANCH,
            role: EmployeeRole::EstateSpecialist,
        })
        .with_estate_assignment(ESTATE, ASSIGNMENT)
        .with_grant(notification_grant())
}

fn estate_case(
    id: EstateCaseId,
    account: AccountId,
    deceased: BankPrincipalId,
    notice: DeathNoticeId,
) -> EstateCase {
    EstateCase {
        id,
        institution: INSTITUTION,
        branch: BRANCH,
        deceased,
        account,
        death_notice: notice,
        stage: EstateWorkflowStage::Administration,
        status: EstateCaseStatus::Open,
    }
}

fn notification_grant() -> EstateCapabilityGrant {
    EstateCapabilityGrant {
        id: GRANT,
        grantor: DECEASED,
        grantee: SPECIALIST,
        scope: EstateCapabilityScope {
            account: None,
            estate: ESTATE,
            institution: INSTITUTION,
            branch: BRANCH,
            operation: EstateCapabilityOperation::NotifyDeath,
            purpose: EstateCapabilityPurpose::EstateAdministration,
            field: None,
            amount_ceiling: None,
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
