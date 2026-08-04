use std::time::{SystemTime, UNIX_EPOCH};

#[path = "fixture/authentication.rs"]
mod authentication;
#[path = "fixture/grant_spec.rs"]
mod grant_spec;
#[path = "fixture/world.rs"]
mod world;

pub(super) use authentication::request_scope;
use authentication::{
    authentication_configuration, block_on, external_identity, TestAuthenticationAdapter,
    TestCredential,
};
use bank_domain::estate::{
    BankEstateOracles, BankEstateWorld, BranchId, CapabilityGrantId, CapabilityValidity,
    DeathNoticeId, DeathNoticeStatus, DelegationLimit, EstateAction, EstateActorContext,
    EstateBranch, EstateCapabilityGrant, EstateCapabilityScope, EstateCapabilityUse, EstateCase,
    EstateCaseId, EstateCaseStatus, EstateDeathNotice, EstateDecision, EstateEmployeeAssignment,
    EstateLegalAuthority, EstateMoment, EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind,
};
use bank_domain::model::{
    AccountId, BankPrincipalId, EmployeeAssignmentId, EmployeeRole, InstitutionId,
};
pub(super) use grant_spec::GrantSpec;
use world::{install_fixture_world, FixtureWorldSpec};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use crate::{BankAuthenticatedPrincipal, BankAuthenticationBoundary, BankIdentityRuntime};

pub(super) const INSTITUTION: InstitutionId = InstitutionId::new(1).unwrap();
pub(super) const BRANCH: BranchId = BranchId::new(2).unwrap();
pub(super) const ESTATE: EstateCaseId = EstateCaseId::new(3).unwrap();
pub(super) const ACCOUNT: AccountId = AccountId::new(4).unwrap();
pub(super) const OTHER_ACCOUNT: AccountId = AccountId::new(5).unwrap();
pub(super) const DECEASED: BankPrincipalId = BankPrincipalId::new(6).unwrap();
pub(super) const SPECIALIST: BankPrincipalId = BankPrincipalId::new(7).unwrap();
pub(super) const EXECUTOR: BankPrincipalId = BankPrincipalId::new(8).unwrap();
pub(super) const ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(9).unwrap();
pub(super) const APPROVER: BankPrincipalId = BankPrincipalId::new(13).unwrap();
pub(super) const APPROVER_ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(14).unwrap();
pub(super) const REVIEWER: BankPrincipalId = BankPrincipalId::new(15).unwrap();
pub(super) const REVIEWER_ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(16).unwrap();
pub(super) const AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(10).unwrap();
pub(super) const OTHER_AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(11).unwrap();
pub(super) const GRANT: CapabilityGrantId = CapabilityGrantId::new(20).unwrap();
pub(super) const COMMAND_GRANT: CapabilityGrantId = CapabilityGrantId::new(21).unwrap();
pub(super) const APPROVAL_GRANT: CapabilityGrantId = CapabilityGrantId::new(22).unwrap();
pub(super) const SELF_APPROVAL_GRANT: CapabilityGrantId = CapabilityGrantId::new(23).unwrap();
pub(super) const APPROVER_REQUEST_GRANT: CapabilityGrantId = CapabilityGrantId::new(24).unwrap();
pub(super) const APPROVER_UPPER_BOUND_GRANT: CapabilityGrantId =
    CapabilityGrantId::new(25).unwrap();
pub(super) const CLOSE_GRANT: CapabilityGrantId = CapabilityGrantId::new(26).unwrap();
pub(super) const REVIEW_GRANT: CapabilityGrantId = CapabilityGrantId::new(27).unwrap();

pub(super) struct CapabilityFixture {
    pub(super) runtime: BankIdentityRuntime,
    estate_world: BankEstateWorld,
    workflow_stage: EstateWorkflowStage,
    authentication: BankAuthenticationBoundary<TestAuthenticationAdapter>,
    specialist_identity: WorthQueryExternalPrincipalIdentity,
    approver_identity: WorthQueryExternalPrincipalIdentity,
    reviewer_identity: WorthQueryExternalPrincipalIdentity,
}

impl CapabilityFixture {
    pub(super) fn authenticate(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.specialist_identity.clone())
    }

    pub(super) fn authenticate_approver(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.approver_identity.clone())
    }

    pub(super) fn authenticate_reviewer(&self) -> BankAuthenticatedPrincipal {
        self.authenticate_identity(self.reviewer_identity.clone())
    }

    fn authenticate_identity(
        &self,
        identity: WorthQueryExternalPrincipalIdentity,
    ) -> BankAuthenticatedPrincipal {
        let request = request_scope();
        block_on(self.runtime.authenticate_with(
            &self.authentication,
            TestCredential(identity),
            &request,
        ))
        .expect("the mapped employee should authenticate")
    }

    pub(super) fn oracle_decision(&self, action: EstateAction) -> EstateDecision {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("the test clock is after the Unix epoch")
            .as_secs();
        BankEstateOracles::evaluate(
            &self.estate_world,
            EstateActorContext {
                principal: SPECIALIST,
                assignment: ASSIGNMENT,
            },
            action,
            EstateCapabilityUse {
                grant: GRANT,
                workflow_stage: self.workflow_stage,
                now: EstateMoment::from_epoch_seconds(now),
                emergency_access: None,
            },
        )
    }
}

pub(super) fn capability_world(
    scenario: &str,
    spec: GrantSpec,
    case_stage: EstateWorkflowStage,
    specialist_holds_authority: bool,
    unrelated_grants: usize,
) -> CapabilityFixture {
    capability_world_with_command_grant(
        scenario,
        spec,
        case_stage,
        specialist_holds_authority,
        unrelated_grants,
        false,
    )
}

pub(super) fn emergency_request_world(
    scenario: &str,
    upper_bound: GrantSpec,
    case_stage: EstateWorkflowStage,
) -> CapabilityFixture {
    capability_world_with_command_grant(scenario, upper_bound, case_stage, false, 0, true)
}

fn capability_world_with_command_grant(
    scenario: &str,
    spec: GrantSpec,
    case_stage: EstateWorkflowStage,
    specialist_holds_authority: bool,
    unrelated_grants: usize,
    install_command_grant: bool,
) -> CapabilityFixture {
    let installed = install_fixture_world(FixtureWorldSpec {
        scenario,
        spec,
        case_stage,
        specialist_holds_authority,
        unrelated_grants,
        install_command_grants: install_command_grant,
    });
    let runtime = installed.runtime;
    let authentication = runtime
        .admit_authentication_adapter(authentication_configuration(), TestAuthenticationAdapter)
        .expect("the causal test adapter should install");
    CapabilityFixture {
        runtime,
        estate_world: installed.estate_world,
        workflow_stage: case_stage,
        authentication,
        specialist_identity: installed.identities[1].clone(),
        approver_identity: installed.identities[3].clone(),
        reviewer_identity: installed.identities[4].clone(),
    }
}

fn base_estate(stage: EstateWorkflowStage, authority_holder: BankPrincipalId) -> BankEstateWorld {
    BankEstateWorld::default()
        .with_branch(EstateBranch {
            id: BRANCH,
            institution: INSTITUTION,
        })
        .with_death_notice(EstateDeathNotice {
            id: DeathNoticeId::new(12).unwrap(),
            subject: DECEASED,
            status: DeathNoticeStatus::Verified,
        })
        .with_case(EstateCase {
            id: ESTATE,
            institution: INSTITUTION,
            branch: BRANCH,
            deceased: DECEASED,
            account: ACCOUNT,
            death_notice: DeathNoticeId::new(12).unwrap(),
            stage,
            status: EstateCaseStatus::Open,
        })
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

fn grant(
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

fn extra_principal(ordinal: usize) -> BankPrincipalId {
    BankPrincipalId::new(1_000 + ordinal as u64).unwrap()
}
