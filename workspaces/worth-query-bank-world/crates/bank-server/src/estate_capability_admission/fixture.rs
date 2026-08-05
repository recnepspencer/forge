#[path = "fixture/authentication.rs"]
mod authentication;
#[path = "fixture/capability_fixture.rs"]
mod capability_fixture;
#[path = "fixture/grant_spec.rs"]
mod grant_spec;
#[path = "fixture/world.rs"]
mod world;

pub(crate) use authentication::request_scope;
use authentication::{
    authentication_configuration, block_on, external_identity, TestAuthenticationAdapter,
    TestCredential,
};
use bank_domain::estate::{
    BankEstateWorld, BranchId, CapabilityGrantId, CapabilityValidity, DeathNoticeId,
    DeathNoticeStatus, DelegationLimit, EstateBranch, EstateCapabilityGrant, EstateCapabilityScope,
    EstateCase, EstateCaseId, EstateCaseStatus, EstateDeathNotice, EstateEmployeeAssignment,
    EstateLegalAuthority, EstateMoment, EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind,
};
use bank_domain::model::{
    AccountId, BankPrincipalId, EmployeeAssignmentId, EmployeeRole, InstitutionId,
};
pub(crate) use capability_fixture::CapabilityFixture;
pub(crate) use grant_spec::GrantSpec;
pub(crate) use world::{foreign_estate_revocation_world, FOREIGN_ESTATE, FOREIGN_GRANT};
use world::{install_fixture_world, FixtureWorldComposition, FixtureWorldSpec};
pub(crate) const INSTITUTION: InstitutionId = InstitutionId::new(1).unwrap();
pub(crate) const BRANCH: BranchId = BranchId::new(2).unwrap();
pub(crate) const ESTATE: EstateCaseId = EstateCaseId::new(3).unwrap();
pub(super) const ALTERNATE_INSTITUTION: InstitutionId = InstitutionId::new(101).unwrap();
pub(super) const ALTERNATE_BRANCH: BranchId = BranchId::new(102).unwrap();
pub(crate) const ACCOUNT: AccountId = AccountId::new(4).unwrap();
pub(crate) const OTHER_ACCOUNT: AccountId = AccountId::new(5).unwrap();
pub(crate) const DECEASED: BankPrincipalId = BankPrincipalId::new(6).unwrap();
pub(super) const SPECIALIST: BankPrincipalId = BankPrincipalId::new(7).unwrap();
pub(crate) const EXECUTOR: BankPrincipalId = BankPrincipalId::new(8).unwrap();
pub(super) const ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(9).unwrap();
pub(crate) const APPROVER: BankPrincipalId = BankPrincipalId::new(13).unwrap();
pub(super) const APPROVER_ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(14).unwrap();
pub(crate) const REVIEWER: BankPrincipalId = BankPrincipalId::new(15).unwrap();
pub(super) const REVIEWER_ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(16).unwrap();
pub(super) const DELEGATION_EXECUTOR_ASSIGNMENT: EmployeeAssignmentId =
    EmployeeAssignmentId::new(17).unwrap();
pub(super) const DELEGATION_REVIEWER_ASSIGNMENT: EmployeeAssignmentId =
    EmployeeAssignmentId::new(18).unwrap();
pub(crate) const AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(10).unwrap();
pub(crate) const NOTICE: DeathNoticeId = DeathNoticeId::new(12).unwrap();
pub(super) const OTHER_AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(11).unwrap();
pub(crate) const GRANT: CapabilityGrantId = CapabilityGrantId::new(20).unwrap();
pub(super) const COMMAND_GRANT: CapabilityGrantId = CapabilityGrantId::new(21).unwrap();
pub(super) const APPROVAL_GRANT: CapabilityGrantId = CapabilityGrantId::new(22).unwrap();
pub(super) const SELF_APPROVAL_GRANT: CapabilityGrantId = CapabilityGrantId::new(23).unwrap();
pub(super) const APPROVER_REQUEST_GRANT: CapabilityGrantId = CapabilityGrantId::new(24).unwrap();
pub(super) const APPROVER_UPPER_BOUND_GRANT: CapabilityGrantId =
    CapabilityGrantId::new(25).unwrap();
pub(super) const CLOSE_GRANT: CapabilityGrantId = CapabilityGrantId::new(26).unwrap();
pub(super) const REVIEW_GRANT: CapabilityGrantId = CapabilityGrantId::new(27).unwrap();
pub(super) const LIFECYCLE_OBSERVER_GRANT: CapabilityGrantId = CapabilityGrantId::new(28).unwrap();
pub(super) const ALTERNATE_EMERGENCY_BOUND_GRANT: CapabilityGrantId =
    CapabilityGrantId::new(29).unwrap();
pub(super) const DELEGATED_GRANT: CapabilityGrantId = CapabilityGrantId::new(30).unwrap();
pub(super) const DISBURSEMENT_GRANT: CapabilityGrantId = CapabilityGrantId::new(31).unwrap();
pub(super) const EMERGENCY_BOUND_GRANT: CapabilityGrantId = CapabilityGrantId::new(32).unwrap();
pub(super) const REVOKE_CAPABILITY_GRANT: CapabilityGrantId = CapabilityGrantId::new(33).unwrap();
pub(super) const APPROVER_DELEGATION_GRANT: CapabilityGrantId = CapabilityGrantId::new(34).unwrap();
pub(crate) const UNRELATED_GOVERNANCE_GRANT: CapabilityGrantId =
    CapabilityGrantId::new(35).unwrap();
pub(super) const REQUESTED_ACCESS: bank_domain::estate::EmergencyAccessId =
    bank_domain::estate::EmergencyAccessId::new(40).unwrap();
pub(super) const CLOSED_ACCESS: bank_domain::estate::EmergencyAccessId =
    bank_domain::estate::EmergencyAccessId::new(41).unwrap();
pub(super) const REQUESTED_REVIEW: bank_domain::estate::MandatoryReviewId =
    bank_domain::estate::MandatoryReviewId::new(50).unwrap();
pub(crate) const COMPLETED_REVIEW: bank_domain::estate::MandatoryReviewId =
    bank_domain::estate::MandatoryReviewId::new(51).unwrap();

pub(super) fn capability_world(
    scenario: &str,
    spec: GrantSpec,
    case_stage: EstateWorkflowStage,
    specialist_holds_authority: bool,
    unrelated_grants: usize,
) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec,
        case_stage,
        specialist_holds_authority,
        unrelated_grants,
        composition: FixtureWorldComposition::Admission,
        alternate_emergency_bound: None,
    })
}

pub(crate) fn release_world(scenario: &str) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::release(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Release,
        alternate_emergency_bound: None,
    })
}

pub(crate) fn disbursement_world(scenario: &str) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::disburse(50_000),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Disbursement,
        alternate_emergency_bound: None,
    })
}

pub(super) fn emergency_request_world(
    scenario: &str,
    upper_bound: GrantSpec,
    case_stage: EstateWorkflowStage,
) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: upper_bound,
        case_stage,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Lifecycle,
        alternate_emergency_bound: None,
    })
}

pub(crate) fn emergency_request_world_with_alternate_bound(
    scenario: &str,
    upper_bound: GrantSpec,
    alternate_bound: GrantSpec,
    case_stage: EstateWorkflowStage,
) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: upper_bound,
        case_stage,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Lifecycle,
        alternate_emergency_bound: Some(alternate_bound),
    })
}

pub(super) fn governance_projection_world(scenario: &str) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::governance_view(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::GovernanceProjection,
        alternate_emergency_bound: None,
    })
}

pub(crate) fn delegation_world(scenario: &str) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::governance_view(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Delegation {
            command_authority: true,
            parent_context: world::DelegationParentContext::Exact,
        },
        alternate_emergency_bound: None,
    })
}

pub(super) fn delegation_world_without_command(scenario: &str) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::governance_view(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Delegation {
            command_authority: false,
            parent_context: world::DelegationParentContext::Exact,
        },
        alternate_emergency_bound: None,
    })
}

pub(super) fn delegation_world_with_parent_branch_mismatch(scenario: &str) -> CapabilityFixture {
    delegation_world_with_parent_context(scenario, world::DelegationParentContext::Branch)
}

pub(super) fn delegation_world_with_parent_institution_mismatch(
    scenario: &str,
) -> CapabilityFixture {
    delegation_world_with_parent_context(scenario, world::DelegationParentContext::Institution)
}

fn delegation_world_with_parent_context(
    scenario: &str,
    parent_context: world::DelegationParentContext,
) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: GrantSpec::governance_view(),
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Delegation {
            command_authority: true,
            parent_context,
        },
        alternate_emergency_bound: None,
    })
}

pub(crate) fn delegation_world_with_parent_spec(
    scenario: &str,
    parent: GrantSpec,
) -> CapabilityFixture {
    capability_world_from_spec(FixtureWorldSpec {
        scenario,
        spec: parent,
        case_stage: EstateWorkflowStage::Administration,
        specialist_holds_authority: false,
        unrelated_grants: 0,
        composition: FixtureWorldComposition::Delegation {
            command_authority: true,
            parent_context: world::DelegationParentContext::Exact,
        },
        alternate_emergency_bound: None,
    })
}

fn capability_world_from_spec(spec: FixtureWorldSpec<'_>) -> CapabilityFixture {
    let case_stage = spec.case_stage;
    let installed = install_fixture_world(spec);
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
        executor_identity: installed.identities[2].clone(),
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
        .with_branch(EstateBranch {
            id: ALTERNATE_BRANCH,
            institution: INSTITUTION,
        })
        .with_death_notice(EstateDeathNotice {
            id: NOTICE,
            subject: DECEASED,
            status: DeathNoticeStatus::Verified,
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
