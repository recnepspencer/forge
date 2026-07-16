use super::{OperationalFencingProviderDenial, PrimaryServeLease, PrimaryServingAuthority};
use super::serve_lease::ProviderFenceGrant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PromotionFenceRequest {
    old_primary_lease: PrimaryServeLease,
    minimum_epoch_exclusive: u64,
}

impl PromotionFenceRequest {
    pub const fn for_old_primary(
        old_primary_lease: PrimaryServeLease,
        minimum_epoch_exclusive: u64,
    ) -> Self {
        Self {
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
            )
            .map_err(PromotionFenceDenial::Provider)?;
        validate_fence_grant(request, grant)
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
