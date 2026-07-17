use crate::{ControlStoreGeneration, StoreCurrentAuthorityIdentity};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OperationalFencingProviderDenial {
    Unsupported,
    Unavailable,
    Rejected,
    Indeterminate,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryServeLeaseRequest {
    authority: StoreCurrentAuthorityIdentity,
    selected_control_generation: ControlStoreGeneration,
    minimum_epoch_exclusive: u64,
    requested_until_tick: u64,
}

impl PrimaryServeLeaseRequest {
    pub const fn authority(self) -> StoreCurrentAuthorityIdentity {
        self.authority
    }

    pub const fn selected_control_generation(self) -> ControlStoreGeneration {
        self.selected_control_generation
    }

    pub const fn minimum_epoch_exclusive(self) -> u64 {
        self.minimum_epoch_exclusive
    }

    pub const fn requested_until_tick(self) -> u64 {
        self.requested_until_tick
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalServeLeaseGrant {
    token: [u8; 32],
    epoch: u64,
    valid_until_tick: u64,
    provider_identity: [u8; 32],
}

impl ExternalServeLeaseGrant {
    pub const fn from_provider(
        token: [u8; 32],
        epoch: u64,
        valid_until_tick: u64,
        provider_identity: [u8; 32],
    ) -> Self {
        Self {
            token,
            epoch,
            valid_until_tick,
            provider_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternalFenceGrant {
    pub(crate) old_lease_token: [u8; 32],
    pub(crate) new_epoch: u64,
    pub(crate) provider_identity: [u8; 32],
    pub(crate) fence_identity: [u8; 32],
    pub(crate) operation_identity: super::PromotionFenceOperationIdentity,
}

impl ExternalFenceGrant {
    pub const fn from_provider(
        old_lease_token: [u8; 32],
        new_epoch: u64,
        provider_identity: [u8; 32],
        fence_identity: [u8; 32],
        operation_identity: super::PromotionFenceOperationIdentity,
    ) -> Self {
        Self {
            old_lease_token,
            new_epoch,
            provider_identity,
            fence_identity,
            operation_identity,
        }
    }
}

pub trait OperationalFencingAuthorityPort: std::fmt::Debug {
    fn acquire_primary_serve_lease(
        &self,
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial>;

    fn renew_primary_serve_lease(
        &self,
        current_token: [u8; 32],
        request: PrimaryServeLeaseRequest,
    ) -> Result<ExternalServeLeaseGrant, OperationalFencingProviderDenial>;

    fn revoke_and_advance_epoch(
        &self,
        old_lease_token: [u8; 32],
        minimum_epoch_exclusive: u64,
        operation_identity: super::PromotionFenceOperationIdentity,
    ) -> Result<ExternalFenceGrant, OperationalFencingProviderDenial>;

    fn recover_fence(
        &self,
        operation_identity: super::PromotionFenceOperationIdentity,
    ) -> Result<Option<ExternalFenceGrant>, OperationalFencingProviderDenial>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryServeLease {
    authority: StoreCurrentAuthorityIdentity,
    selected_control_generation: ControlStoreGeneration,
    token: [u8; 32],
    epoch: u64,
    valid_until_tick: u64,
    provider_identity: [u8; 32],
}

impl PrimaryServeLease {
    pub const fn authority(self) -> StoreCurrentAuthorityIdentity {
        self.authority
    }

    pub const fn epoch(self) -> u64 {
        self.epoch
    }

    pub const fn valid_until_tick(self) -> u64 {
        self.valid_until_tick
    }

    pub const fn selected_control_generation(self) -> ControlStoreGeneration {
        self.selected_control_generation
    }

    pub fn lease_identity(self) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut digest = Sha256::new();
        digest.update(b"worth-store-primary-serve-lease-v1");
        digest.update(self.authority.fingerprint());
        digest.update(self.selected_control_generation.get().to_be_bytes());
        digest.update(self.token);
        digest.update(self.epoch.to_be_bytes());
        digest.update(self.valid_until_tick.to_be_bytes());
        digest.update(self.provider_identity);
        digest.finalize().into()
    }

    pub(crate) const fn token(self) -> [u8; 32] {
        self.token
    }

    pub(crate) const fn provider_identity(self) -> [u8; 32] {
        self.provider_identity
    }

    pub(crate) const fn from_grant(
        request: PrimaryServeLeaseRequest,
        grant: ExternalServeLeaseGrant,
    ) -> Self {
        Self {
            authority: request.authority,
            selected_control_generation: request.selected_control_generation,
            token: grant.token,
            epoch: grant.epoch,
            valid_until_tick: grant.valid_until_tick,
            provider_identity: grant.provider_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryServeOperation {
    ObserveAsCurrent,
    Mutate,
    Acknowledge,
}

impl PrimaryServeLeaseRequest {
    pub(crate) const fn new(
        authority: StoreCurrentAuthorityIdentity,
        selected_control_generation: ControlStoreGeneration,
        minimum_epoch_exclusive: u64,
        requested_until_tick: u64,
    ) -> Self {
        Self {
            authority,
            selected_control_generation,
            minimum_epoch_exclusive,
            requested_until_tick,
        }
    }
}

pub(crate) use ExternalFenceGrant as ProviderFenceGrant;
