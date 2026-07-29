mod account;
mod accounting_neighborhood;
mod authorization;
mod entities;
mod payment;
mod projection_access;

use std::collections::BTreeMap;

use bank_domain::model::{AccountId, BankSnapshotVersion};
use bank_domain::proposals::BankSnapshotBuilder;
use bank_domain::schema::{
    Account, AccountAuthorization, Approval, BankSchema, Business, Institution, JournalEntry,
    PaymentIntent, Posting, Principal,
};
use worth_query_host::facade::primary_graph::{
    WorthQueryApplicationOperationInvariantProjectionReader, WorthQueryInvariantEntityIdentity,
};

use super::BankProjectionDenial;

pub(super) type AccountEntity = WorthQueryInvariantEntityIdentity<BankSchema, Account>;
pub(super) type AuthorizationEntity =
    WorthQueryInvariantEntityIdentity<BankSchema, AccountAuthorization>;
pub(super) type ApprovalEntity = WorthQueryInvariantEntityIdentity<BankSchema, Approval>;
pub(super) type BusinessEntity = WorthQueryInvariantEntityIdentity<BankSchema, Business>;
pub(super) type InstitutionEntity = WorthQueryInvariantEntityIdentity<BankSchema, Institution>;
pub(super) type JournalEntity = WorthQueryInvariantEntityIdentity<BankSchema, JournalEntry>;
pub(super) type PostingEntity = WorthQueryInvariantEntityIdentity<BankSchema, Posting>;
pub(super) type PaymentEntity = WorthQueryInvariantEntityIdentity<BankSchema, PaymentIntent>;
pub(super) type PrincipalEntity = WorthQueryInvariantEntityIdentity<BankSchema, Principal>;
pub(super) type ProjectionReader<'reader, 'runtime, Operation> =
    WorthQueryApplicationOperationInvariantProjectionReader<
        'reader,
        'runtime,
        BankSchema,
        Operation,
    >;

pub(super) struct BoundedProjectionState {
    builder: Option<BankSnapshotBuilder>,
    accounts: BTreeMap<AccountId, AccountEntity>,
    dependency_mode: ProjectionDependencyMode,
}

#[derive(Clone, Copy)]
enum ProjectionDependencyMode {
    InstalledDecisions,
    CapabilityOnly,
}

impl BoundedProjectionState {
    pub(super) fn new<Operation>(
        reader: &ProjectionReader<'_, '_, Operation>,
    ) -> Result<Self, BankProjectionDenial> {
        let version = BankSnapshotVersion::new(reader.version().as_u64())
            .ok_or(BankProjectionDenial::InvalidSnapshotVersion)?;
        Ok(Self {
            builder: Some(BankSnapshotBuilder::new(version)),
            accounts: BTreeMap::new(),
            dependency_mode: ProjectionDependencyMode::InstalledDecisions,
        })
    }

    pub(super) fn for_capability_projection<Operation>(
        reader: &ProjectionReader<'_, '_, Operation>,
    ) -> Result<Self, BankProjectionDenial> {
        let mut state = Self::new(reader)?;
        state.dependency_mode = ProjectionDependencyMode::CapabilityOnly;
        Ok(state)
    }

    pub(super) fn update_builder(
        &mut self,
        update: impl FnOnce(BankSnapshotBuilder) -> BankSnapshotBuilder,
    ) {
        let builder = self
            .builder
            .take()
            .expect("bounded projection owns one live builder");
        self.builder = Some(update(builder));
    }

    pub(super) fn finish(mut self) -> BankSnapshotBuilder {
        self.builder
            .take()
            .expect("bounded projection owns one live builder")
    }
}
