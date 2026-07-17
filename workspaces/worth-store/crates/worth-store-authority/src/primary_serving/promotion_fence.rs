use super::serve_lease::ProviderFenceGrant;
use super::{OperationalFencingProviderDenial, PrimaryServeLease, PrimaryServingAuthority};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionFenceRequest {
    old_primary_lease: PrimaryServeLease,
    minimum_epoch_exclusive: u64,
    operation_identity: PromotionFenceOperationIdentity,
}

impl PromotionFenceRequest {
    pub const fn for_old_primary(
        old_primary_lease: PrimaryServeLease,
        minimum_epoch_exclusive: u64,
        operation_identity: PromotionFenceOperationIdentity,
    ) -> Self {
        Self {
            old_primary_lease,
            minimum_epoch_exclusive,
            operation_identity,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PromotionFenceOperationIdentity([u8; 32]);

impl PromotionFenceOperationIdentity {
    pub const fn admit(identity: [u8; 32]) -> Option<Self> {
        if all_zero(identity) {
            None
        } else {
            Some(Self(identity))
        }
    }

    pub const fn get(self) -> [u8; 32] {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionFenceRecoveryRequest {
    operation_identity: PromotionFenceOperationIdentity,
    old_primary_lease: PrimaryServeLease,
    minimum_epoch_exclusive: u64,
}

impl PromotionFenceRecoveryRequest {
    pub const fn new(
        operation_identity: PromotionFenceOperationIdentity,
        old_primary_lease: PrimaryServeLease,
        minimum_epoch_exclusive: u64,
    ) -> Self {
        Self {
            operation_identity,
            old_primary_lease,
            minimum_epoch_exclusive,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromotionFenceDenial {
    LeaseBinding,
    EpochDidNotAdvance,
    ProviderIdentityChanged,
    InvalidFenceIdentity,
    OperationIdentityMismatch,
    FenceNotFound,
    Provider(OperationalFencingProviderDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FenceProof {
    old_primary_authority: crate::StoreCurrentAuthorityIdentity,
    fence_identity: [u8; 32],
    provider_identity: [u8; 32],
    new_epoch: PromotedAuthorityEpoch,
}

impl FenceProof {
    pub const fn old_primary_authority(self) -> crate::StoreCurrentAuthorityIdentity {
        self.old_primary_authority
    }

    pub const fn fence_identity(self) -> [u8; 32] {
        self.fence_identity
    }

    pub const fn promoted_epoch(self) -> PromotedAuthorityEpoch {
        self.new_epoch
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PromotedAuthorityEpoch(u64);

impl PromotedAuthorityEpoch {
    pub const fn get(self) -> u64 {
        self.0
    }
}

impl PrimaryServingAuthority<'_> {
    pub fn fence_old_primary(
        &self,
        request: PromotionFenceRequest,
    ) -> Result<FenceProof, PromotionFenceDenial> {
        self.require_current_binding(request.old_primary_lease)
            .map_err(|_| PromotionFenceDenial::LeaseBinding)?;
        let grant = self
            .provider()
            .revoke_and_advance_epoch(
                request.old_primary_lease.token(),
                request.minimum_epoch_exclusive,
                request.operation_identity,
            )
            .map_err(PromotionFenceDenial::Provider)?;
        validate_fence_grant(request, grant)
    }
}

impl PrimaryServingAuthority<'_> {
    pub fn recover_promotion_fence(
        &self,
        request: PromotionFenceRecoveryRequest,
    ) -> Result<FenceProof, PromotionFenceDenial> {
        self.require_current_binding(request.old_primary_lease)
            .map_err(|_| PromotionFenceDenial::LeaseBinding)?;
        let grant = self
            .provider()
            .recover_fence(request.operation_identity)
            .map_err(PromotionFenceDenial::Provider)?
            .ok_or(PromotionFenceDenial::FenceNotFound)?;
        validate_fence_grant(
            PromotionFenceRequest::for_old_primary(
                request.old_primary_lease,
                request.minimum_epoch_exclusive,
                request.operation_identity,
            ),
            grant,
        )
    }
}

fn validate_fence_grant(
    request: PromotionFenceRequest,
    grant: ProviderFenceGrant,
) -> Result<FenceProof, PromotionFenceDenial> {
    if grant.new_epoch <= request.minimum_epoch_exclusive
        || grant.new_epoch <= request.old_primary_lease.epoch()
        || grant.old_lease_token != request.old_primary_lease.token()
    {
        return Err(PromotionFenceDenial::EpochDidNotAdvance);
    }
    if grant.operation_identity != request.operation_identity {
        return Err(PromotionFenceDenial::OperationIdentityMismatch);
    }
    if grant.provider_identity != request.old_primary_lease.provider_identity() {
        return Err(PromotionFenceDenial::ProviderIdentityChanged);
    }
    if grant.fence_identity == [0; 32] {
        return Err(PromotionFenceDenial::InvalidFenceIdentity);
    }
    Ok(FenceProof {
        old_primary_authority: request.old_primary_lease.authority(),
        fence_identity: grant.fence_identity,
        provider_identity: grant.provider_identity,
        new_epoch: PromotedAuthorityEpoch(grant.new_epoch),
    })
}

const fn all_zero(identity: [u8; 32]) -> bool {
    let mut index = 0;
    while index < identity.len() {
        if identity[index] != 0 {
            return false;
        }
        index += 1;
    }
    true
}
