use bank_domain::model::{
    AccountAuthorizationId, AccountId, AccountName, BankPrincipalId, BankSnapshotVersion,
    BusinessId, CustomerRole, EmployeeAssignmentId, EmployeeRole, InstitutionId, Money, PaymentId,
};
use bank_domain::proposals::{
    BankAccountAuthorization, BankIdempotencyKey, BankOperationScopeBinding, BankProposalEngine,
    BankSnapshot, BankSnapshotBuilder,
};
use bank_domain::schema::{
    AccountStatus, ApplyOpeningFunding, CreateBusinessAccount, CreatePersonalAccount,
    GrantAccountAuthorization, InitiateBusinessPayment, SendMoney,
};
use bank_server::{
    BankAuthenticatedPrincipal, BankBusinessOwnerSeed, BankEmployeeAssignmentSeed,
    BankPrincipalSeed, BankWorldSeed,
};

use crate::support::{block_on, runtime, CausalCredential, DynamicIdentity, TestIdentityWorld};

pub(super) const OWNER: usize = 0;
pub(super) const RECIPIENT: usize = 1;
pub(super) const APPROVER: usize = 2;
pub(super) const VIEWER: usize = 3;
pub(super) const STRANGER: usize = 4;
pub(super) const AUDITOR: usize = 5;
pub(super) const TELLER: usize = 6;

pub(super) struct OrdinaryReadFixture {
    pub world: TestIdentityWorld,
    identities: Vec<DynamicIdentity>,
    pub personal_account: AccountId,
    pub recipient_account: AccountId,
    pub business_account: AccountId,
    pub institution: InstitutionId,
    pub payment: PaymentId,
}

pub(super) struct AccountDiscoveryFixture {
    pub world: TestIdentityWorld,
    actor: DynamicIdentity,
}

impl AccountDiscoveryFixture {
    pub fn authenticate(&self) -> BankAuthenticatedPrincipal {
        let request = crate::support::request_scope();
        block_on(self.world.runtime.authenticate_with(
            &self.world.authentication,
            CausalCredential::for_identity(&self.actor),
            &request,
        ))
        .expect("discovery actor should authenticate")
    }
}

impl OrdinaryReadFixture {
    pub fn authenticate(&self, principal: usize) -> BankAuthenticatedPrincipal {
        let request = crate::support::request_scope();
        block_on(self.world.runtime.authenticate_with(
            &self.world.authentication,
            CausalCredential::for_identity(&self.identities[principal]),
            &request,
        ))
        .expect("fixture principal should authenticate")
    }
}

pub(super) fn ordinary_read_world(
    scenario: &str,
    unrelated_accounts: usize,
) -> OrdinaryReadFixture {
    let mut identities = (0..(7 + unrelated_accounts))
        .map(|ordinal| DynamicIdentity::new(&format!("{scenario}-{ordinal}")))
        .collect::<Vec<_>>();
    let institution = id(InstitutionId::new, 1);
    let business = id(BusinessId::new, 1);
    let mut builder = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(institution)
        .business(business)
        .business(id(BusinessId::new, 2))
        .institution_cash_account(id(AccountId::new, 100), institution);
    for ordinal in 0..identities.len() {
        builder = builder.principal(principal_id(ordinal));
    }
    let mut snapshot = builder.build().expect("base read world should build");
    snapshot = create_personal(snapshot, principal_id(OWNER), "Daily", "personal");
    snapshot = create_personal(snapshot, principal_id(RECIPIENT), "Recipient", "recipient");
    snapshot = create_business(snapshot, institution, business);
    let personal_account = snapshot
        .primary_account(principal_id(OWNER))
        .expect("owner account should exist");
    let recipient_account = snapshot
        .primary_account(principal_id(RECIPIENT))
        .expect("recipient account should exist");
    let business_account = snapshot
        .business_account(business)
        .expect("business account should exist");
    snapshot = fund(
        snapshot,
        institution,
        personal_account,
        10_000,
        "fund-personal",
    );
    snapshot = fund(
        snapshot,
        institution,
        business_account,
        20_000,
        "fund-business",
    );
    snapshot = send(snapshot, personal_account, principal_id(RECIPIENT));
    snapshot = grant(
        snapshot,
        personal_account,
        principal_id(VIEWER),
        CustomerRole::Viewer,
        "viewer",
    );
    snapshot = grant(
        snapshot,
        business_account,
        principal_id(OWNER),
        CustomerRole::Initiator,
        "initiator",
    );
    snapshot = grant(
        snapshot,
        business_account,
        principal_id(APPROVER),
        CustomerRole::Approver,
        "approver",
    );
    let payment_proposal = BankProposalEngine::prepare_initiate_business_payment(
        &snapshot,
        binding(8),
        &key("pending-payment"),
        principal_id(OWNER),
        &InitiateBusinessPayment {
            business,
            from: business_account,
            recipient: principal_id(RECIPIENT),
            amount: Money::from_minor(900).unwrap(),
        },
    )
    .expect("pending payment should prepare");
    snapshot = payment_proposal.proposed_snapshot().clone();
    let payment = snapshot
        .payments()
        .next()
        .expect("pending payment should exist")
        .id();
    for ordinal in 0..unrelated_accounts {
        snapshot = create_personal(
            snapshot,
            principal_id(7 + ordinal),
            &format!("Unrelated {ordinal}"),
            &format!("unrelated-{ordinal}"),
        );
    }

    let mut seed = BankWorldSeed::new(snapshot)
        .business_owner(BankBusinessOwnerSeed::new(business, principal_id(OWNER)))
        .employee(BankEmployeeAssignmentSeed::new(
            id(EmployeeAssignmentId::new, 1),
            institution,
            principal_id(AUDITOR),
            EmployeeRole::Auditor,
        ))
        .employee(BankEmployeeAssignmentSeed::new(
            id(EmployeeAssignmentId::new, 2),
            institution,
            principal_id(TELLER),
            EmployeeRole::Teller,
        ));
    for (ordinal, identity) in identities.iter_mut().enumerate() {
        seed = seed.principal(BankPrincipalSeed::enabled(
            principal_id(ordinal),
            identity.external(),
        ));
    }
    OrdinaryReadFixture {
        world: runtime(seed),
        identities,
        personal_account,
        recipient_account,
        business_account,
        institution,
        payment,
    }
}

pub(super) fn over_budget_discovery_world(
    scenario: &str,
    authorized_accounts: usize,
) -> AccountDiscoveryFixture {
    let actor = DynamicIdentity::new(&format!("{scenario}-actor"));
    let owner_identities = (0..authorized_accounts)
        .map(|ordinal| DynamicIdentity::new(&format!("{scenario}-owner-{ordinal}")))
        .collect::<Vec<_>>();
    let institution = id(InstitutionId::new, 1);
    let actor_id = id(BankPrincipalId::new, 1);
    let mut builder = BankSnapshotBuilder::new(id(BankSnapshotVersion::new, 1))
        .institution(institution)
        .principal(actor_id)
        .institution_cash_account(id(AccountId::new, 100), institution);
    for ordinal in 0..authorized_accounts {
        let owner = id(BankPrincipalId::new, u64::try_from(ordinal).unwrap() + 2);
        let account = id(AccountId::new, u64::try_from(ordinal).unwrap() + 1_000);
        builder = builder
            .principal(owner)
            .personal_account(
                account,
                institution,
                owner,
                AccountName::new(format!("Authorized {ordinal}")).unwrap(),
                AccountStatus::Open,
            )
            .projected_authorization(BankAccountAuthorization::from_projection(
                id(
                    AccountAuthorizationId::new,
                    u64::try_from(ordinal).unwrap() + 1,
                ),
                account,
                actor_id,
                CustomerRole::Viewer,
            ));
    }
    let mut seed = BankWorldSeed::new(builder.build().expect("discovery world should build"))
        .principal(BankPrincipalSeed::enabled(actor_id, actor.external()));
    for (ordinal, identity) in owner_identities.iter().enumerate() {
        seed = seed.principal(BankPrincipalSeed::enabled(
            id(BankPrincipalId::new, u64::try_from(ordinal).unwrap() + 2),
            identity.external(),
        ));
    }
    AccountDiscoveryFixture {
        world: runtime(seed),
        actor,
    }
}

fn create_personal(
    snapshot: BankSnapshot,
    owner: BankPrincipalId,
    name: &str,
    operation: &str,
) -> BankSnapshot {
    BankProposalEngine::prepare_create_personal_account(
        &snapshot,
        binding(1),
        &key(operation),
        &CreatePersonalAccount {
            institution: id(InstitutionId::new, 1),
            owner,
            display_name: AccountName::new(name).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

fn create_business(
    snapshot: BankSnapshot,
    institution: InstitutionId,
    business: BusinessId,
) -> BankSnapshot {
    BankProposalEngine::prepare_create_business_account(
        &snapshot,
        binding(2),
        &key("business"),
        &CreateBusinessAccount {
            institution,
            business,
            display_name: AccountName::new("Operations").unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

fn fund(
    snapshot: BankSnapshot,
    institution: InstitutionId,
    account: AccountId,
    amount: i64,
    operation: &str,
) -> BankSnapshot {
    BankProposalEngine::prepare_opening_funding(
        &snapshot,
        binding(3),
        &key(operation),
        &ApplyOpeningFunding {
            institution,
            account,
            amount: Money::from_minor(amount).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

fn send(snapshot: BankSnapshot, from: AccountId, recipient: BankPrincipalId) -> BankSnapshot {
    BankProposalEngine::prepare_send_money(
        &snapshot,
        binding(4),
        &key("personal-send"),
        &SendMoney {
            from,
            recipient,
            amount: Money::from_minor(2_500).unwrap(),
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

fn grant(
    snapshot: BankSnapshot,
    account: AccountId,
    principal: BankPrincipalId,
    role: CustomerRole,
    operation: &str,
) -> BankSnapshot {
    BankProposalEngine::prepare_grant_account_authorization(
        &snapshot,
        binding(5),
        &key(operation),
        &GrantAccountAuthorization {
            account,
            principal,
            role,
        },
    )
    .unwrap()
    .proposed_snapshot()
    .clone()
}

pub(super) fn principal_id(ordinal: usize) -> BankPrincipalId {
    id(BankPrincipalId::new, u64::try_from(ordinal).unwrap() + 1)
}

fn binding(value: u8) -> BankOperationScopeBinding {
    BankOperationScopeBinding::from_fingerprint_bytes([value; 32])
}

fn key(value: &str) -> BankIdempotencyKey {
    BankIdempotencyKey::new(value).unwrap()
}

fn id<T>(constructor: impl FnOnce(u64) -> Option<T>, value: u64) -> T {
    constructor(value).unwrap()
}
