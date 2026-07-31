use std::future::Future;
use std::pin::pin;
use std::task::{Context, Poll, Waker};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use bank_domain::estate::{
    BankEstateOracles, BankEstateWorld, BranchId, CapabilityGrantId, CapabilityGrantStatus,
    CapabilityValidity, DeathNoticeId, DeathNoticeStatus, DelegationLimit, EstateAction,
    EstateActorContext, EstateBranch, EstateCapabilityGrant, EstateCapabilityOperation,
    EstateCapabilityPurpose, EstateCapabilityScope, EstateCapabilityUse, EstateCase, EstateCaseId,
    EstateCaseStatus, EstateDeathNotice, EstateDecision, EstateEmployeeAssignment,
    EstateLegalAuthority, EstateMoment, EstateWorkflowStage, LegalAuthorityId, LegalAuthorityKind,
    RestrictedBankField,
};
use bank_domain::model::{
    AccountId, AccountName, BankPrincipalId, BankSnapshotVersion, EmployeeAssignmentId,
    EmployeeRole, InstitutionId, Money, USD,
};
use bank_domain::proposals::BankSnapshotBuilder;
use bank_domain::schema::AccountStatus;
use worth_query_host::facade::admission::authenticated_principal::{
    WorthQueryAuthenticationAdapter, WorthQueryAuthenticationAdapterFailure,
    WorthQueryAuthenticationAdapterFailureKind, WorthQueryAuthenticationAudience,
    WorthQueryAuthenticationFuture, WorthQueryAuthenticationMethod, WorthQueryCancellationSource,
    WorthQueryRequestScope, WorthQueryValidatedExternalPrincipal,
};
use worth_query_host::facade::declaration::authentication::WorthQueryExternalPrincipalIdentity;

use crate::{
    BankAuthenticatedPrincipal, BankAuthenticationBoundary, BankAuthenticationConfiguration,
    BankEmployeeAssignmentSeed, BankIdentityRuntime, BankPrincipalSeed, BankWorldSeed,
};

pub(super) const INSTITUTION: InstitutionId = InstitutionId::new(1).unwrap();
pub(super) const BRANCH: BranchId = BranchId::new(2).unwrap();
pub(super) const ESTATE: EstateCaseId = EstateCaseId::new(3).unwrap();
pub(super) const ACCOUNT: AccountId = AccountId::new(4).unwrap();
pub(super) const OTHER_ACCOUNT: AccountId = AccountId::new(5).unwrap();
pub(super) const DECEASED: BankPrincipalId = BankPrincipalId::new(6).unwrap();
pub(super) const SPECIALIST: BankPrincipalId = BankPrincipalId::new(7).unwrap();
pub(super) const EXECUTOR: BankPrincipalId = BankPrincipalId::new(8).unwrap();
pub(super) const ASSIGNMENT: EmployeeAssignmentId = EmployeeAssignmentId::new(9).unwrap();
pub(super) const AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(10).unwrap();
pub(super) const OTHER_AUTHORITY: LegalAuthorityId = LegalAuthorityId::new(11).unwrap();
pub(super) const GRANT: CapabilityGrantId = CapabilityGrantId::new(20).unwrap();

pub(super) struct GrantSpec {
    pub(super) operation: EstateCapabilityOperation,
    pub(super) purpose: EstateCapabilityPurpose,
    pub(super) account: Option<AccountId>,
    pub(super) field: Option<RestrictedBankField>,
    pub(super) amount_ceiling: Option<Money<USD>>,
    pub(super) status: CapabilityGrantStatus,
    pub(super) not_before: u64,
    pub(super) not_after: u64,
    pub(super) workflow: EstateWorkflowStage,
}

impl GrantSpec {
    pub(super) fn view() -> Self {
        Self {
            operation: EstateCapabilityOperation::ViewRestrictedEstate,
            purpose: EstateCapabilityPurpose::EstateAdministration,
            account: None,
            field: Some(RestrictedBankField::CustomerIdentity),
            amount_ceiling: None,
            status: CapabilityGrantStatus::Active,
            not_before: 0,
            not_after: u64::MAX,
            workflow: EstateWorkflowStage::Administration,
        }
    }

    pub(super) fn freeze() -> Self {
        Self {
            operation: EstateCapabilityOperation::FreezeAccount,
            purpose: EstateCapabilityPurpose::EstateAdministration,
            account: Some(ACCOUNT),
            field: None,
            ..Self::view()
        }
    }

    pub(super) fn identity_verification() -> Self {
        Self {
            purpose: EstateCapabilityPurpose::IdentityVerification,
            ..Self::view()
        }
    }

    pub(super) fn disburse(maximum_minor_units: i64) -> Self {
        Self {
            operation: EstateCapabilityOperation::DisburseEstate,
            purpose: EstateCapabilityPurpose::EstateDisbursement,
            account: Some(ACCOUNT),
            field: None,
            amount_ceiling: Some(Money::from_minor(maximum_minor_units).unwrap()),
            ..Self::view()
        }
    }

    pub(super) fn recognize() -> Self {
        Self {
            operation: EstateCapabilityOperation::RecognizeExecutor,
            purpose: EstateCapabilityPurpose::LegalCompliance,
            account: None,
            field: None,
            ..Self::view()
        }
    }
}

pub(super) struct CapabilityFixture {
    pub(super) runtime: BankIdentityRuntime,
    estate_world: BankEstateWorld,
    workflow_stage: EstateWorkflowStage,
    authentication: BankAuthenticationBoundary<TestAuthenticationAdapter>,
    specialist_identity: WorthQueryExternalPrincipalIdentity,
}

impl CapabilityFixture {
    pub(super) fn authenticate(&self) -> BankAuthenticatedPrincipal {
        let request = request_scope();
        block_on(self.runtime.authenticate_with(
            &self.authentication,
            TestCredential(self.specialist_identity.clone()),
            &request,
        ))
        .expect("the mapped specialist should authenticate")
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
    let identities = [
        external_identity(scenario, "deceased"),
        external_identity(scenario, "specialist"),
        external_identity(scenario, "executor"),
    ];
    let mut snapshot = BankSnapshotBuilder::new(BankSnapshotVersion::new(1).unwrap())
        .institution(INSTITUTION)
        .principal(DECEASED)
        .principal(SPECIALIST)
        .principal(EXECUTOR)
        .personal_account(
            ACCOUNT,
            INSTITUTION,
            DECEASED,
            AccountName::new("Estate Operating").unwrap(),
            AccountStatus::Frozen,
        )
        .personal_account(
            OTHER_ACCOUNT,
            INSTITUTION,
            EXECUTOR,
            AccountName::new("Executor Settlement").unwrap(),
            AccountStatus::Open,
        );
    for ordinal in 0..unrelated_grants {
        snapshot = snapshot.principal(extra_principal(ordinal));
    }
    let snapshot = snapshot
        .build()
        .expect("capability fixture snapshot should be valid");
    let holder = if specialist_holds_authority {
        SPECIALIST
    } else {
        EXECUTOR
    };
    let mut estate = base_estate(case_stage, holder).with_grant(grant(GRANT, SPECIALIST, spec));
    for ordinal in 0..unrelated_grants {
        estate = estate.with_grant(grant(
            CapabilityGrantId::new(2_000 + ordinal as u64).unwrap(),
            extra_principal(ordinal),
            GrantSpec::view(),
        ));
    }
    let estate_world = estate.clone();
    let mut seed = BankWorldSeed::new(snapshot)
        .principal(BankPrincipalSeed::enabled(DECEASED, identities[0].clone()))
        .principal(BankPrincipalSeed::enabled(
            SPECIALIST,
            identities[1].clone(),
        ))
        .principal(BankPrincipalSeed::enabled(EXECUTOR, identities[2].clone()))
        .employee(BankEmployeeAssignmentSeed::new(
            ASSIGNMENT,
            INSTITUTION,
            SPECIALIST,
            EmployeeRole::EstateSpecialist,
        ))
        .estate(estate);
    for ordinal in 0..unrelated_grants {
        seed = seed.principal(BankPrincipalSeed::enabled(
            extra_principal(ordinal),
            external_identity(scenario, &format!("extra-{ordinal}")),
        ));
    }
    let runtime = BankIdentityRuntime::install_world(seed)
        .expect("capability fixture runtime should install");
    let authentication = runtime
        .admit_authentication_adapter(authentication_configuration(), TestAuthenticationAdapter)
        .expect("the causal test adapter should install");
    CapabilityFixture {
        runtime,
        estate_world,
        workflow_stage: case_stage,
        authentication,
        specialist_identity: identities[1].clone(),
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

fn external_identity(scenario: &str, subject: &str) -> WorthQueryExternalPrincipalIdentity {
    WorthQueryExternalPrincipalIdentity::new(
        format!("https://{scenario}.bank.test.invalid"),
        subject,
    )
    .unwrap()
}

pub(super) fn request_scope() -> WorthQueryRequestScope {
    WorthQueryRequestScope::new(
        Instant::now() + Duration::from_secs(60),
        WorthQueryCancellationSource::new().token(),
    )
}

fn authentication_configuration() -> BankAuthenticationConfiguration {
    BankAuthenticationConfiguration::new(
        WorthQueryAuthenticationAudience::new("bank-phase-7-capability-test").unwrap(),
        WorthQueryAuthenticationMethod::new("causal-phase-7-adapter").unwrap(),
    )
}

struct TestCredential(WorthQueryExternalPrincipalIdentity);
struct TestAuthenticationAdapter;

impl WorthQueryAuthenticationAdapter for TestAuthenticationAdapter {
    type Credential = TestCredential;

    fn configuration_identity(&self) -> &str {
        "bank-phase-7-capability-adapter-v1"
    }

    fn validate<'a>(
        &'a self,
        credential: Self::Credential,
        _scope: &'a WorthQueryRequestScope,
    ) -> WorthQueryAuthenticationFuture<'a> {
        Box::pin(async move {
            let now = SystemTime::now();
            WorthQueryValidatedExternalPrincipal::new(
                credential.0,
                WorthQueryAuthenticationAudience::new("bank-phase-7-capability-test").unwrap(),
                WorthQueryAuthenticationMethod::new("causal-phase-7-adapter").unwrap(),
                now,
                now + Duration::from_secs(60),
                Vec::new(),
            )
            .map_err(|_| {
                WorthQueryAuthenticationAdapterFailure::new(
                    WorthQueryAuthenticationAdapterFailureKind::ProtocolViolation,
                )
            })
        })
    }
}

fn block_on<F: Future>(future: F) -> F::Output {
    let mut future = pin!(future);
    let waker = Waker::noop();
    let mut context = Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(output) => return output,
            Poll::Pending => std::thread::yield_now(),
        }
    }
}
