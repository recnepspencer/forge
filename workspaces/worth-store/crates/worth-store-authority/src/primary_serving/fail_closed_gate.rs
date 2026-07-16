use crate::{SelectedControlStoreGeneration, StoreCurrentAuthorityWitness};

use super::{
    OperationalFencingAuthorityPort, OperationalFencingProviderDenial, PrimaryServeLease,
    PrimaryServeLeaseRequest, PrimaryServeOperation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryServeAdmissionDenial {
    ControlGenerationAuthorityMismatch,
    InvalidRequestedLifetime,
    InvalidProviderGrant,
    LeaseExpired,
    AuthorityChanged,
    ControlGenerationChanged,
    Provider(OperationalFencingProviderDenial),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryServeAdmission {
    lease: PrimaryServeLease,
    operation: PrimaryServeOperation,
    admitted_at_tick: u64,
}

impl PrimaryServeAdmission {
    pub const fn lease(self) -> PrimaryServeLease {
        self.lease
    }

    pub const fn operation(self) -> PrimaryServeOperation {
        self.operation
    }

    pub const fn admitted_at_tick(self) -> u64 {
        self.admitted_at_tick
    }
}

#[derive(Debug)]
pub struct PrimaryServingAuthority<'a> {
    current_authority: &'a StoreCurrentAuthorityWitness,
    selected_control_generation: SelectedControlStoreGeneration,
    provider: &'a dyn OperationalFencingAuthorityPort,
}

impl<'a> PrimaryServingAuthority<'a> {
    pub fn for_selected_control_generation(
        current_authority: &'a StoreCurrentAuthorityWitness,
        selected_control_generation: SelectedControlStoreGeneration,
        provider: &'a dyn OperationalFencingAuthorityPort,
    ) -> Result<Self, PrimaryServeAdmissionDenial> {
        if selected_control_generation.authority_identity()
            != current_authority.authority_identity()
        {
            return Err(PrimaryServeAdmissionDenial::ControlGenerationAuthorityMismatch);
        }
        Ok(Self {
            current_authority,
            selected_control_generation,
            provider,
        })
    }

    pub fn acquire(
        &self,
        minimum_epoch_exclusive: u64,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<PrimaryServeLease, PrimaryServeAdmissionDenial> {
        let request = self.request(minimum_epoch_exclusive, now_tick, requested_until_tick)?;
        let grant = self
            .provider
            .acquire_primary_serve_lease(request)
            .map_err(PrimaryServeAdmissionDenial::Provider)?;
        self.validate_grant(request, PrimaryServeLease::from_grant(request, grant), now_tick)
    }

    pub fn renew(
        &self,
        lease: PrimaryServeLease,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<PrimaryServeLease, PrimaryServeAdmissionDenial> {
        self.require_current_binding(lease)?;
        let request = self.request(lease.epoch().saturating_sub(1), now_tick, requested_until_tick)?;
        let grant = self
            .provider
            .renew_primary_serve_lease(lease.token(), request)
            .map_err(PrimaryServeAdmissionDenial::Provider)?;
        let renewed = PrimaryServeLease::from_grant(request, grant);
        if renewed.epoch() != lease.epoch()
            || renewed.provider_identity() != lease.provider_identity()
        {
            return Err(PrimaryServeAdmissionDenial::InvalidProviderGrant);
        }
        self.validate_grant(request, renewed, now_tick)
    }

    pub fn admit(
        &self,
        lease: PrimaryServeLease,
        operation: PrimaryServeOperation,
        now_tick: u64,
    ) -> Result<PrimaryServeAdmission, PrimaryServeAdmissionDenial> {
        self.require_current_binding(lease)?;
        if now_tick >= lease.valid_until_tick() {
            return Err(PrimaryServeAdmissionDenial::LeaseExpired);
        }
        Ok(PrimaryServeAdmission {
            lease,
            operation,
            admitted_at_tick: now_tick,
        })
    }

    fn request(
        &self,
        minimum_epoch_exclusive: u64,
        now_tick: u64,
        requested_until_tick: u64,
    ) -> Result<PrimaryServeLeaseRequest, PrimaryServeAdmissionDenial> {
        if requested_until_tick <= now_tick {
            return Err(PrimaryServeAdmissionDenial::InvalidRequestedLifetime);
        }
        Ok(PrimaryServeLeaseRequest::new(
            self.current_authority.authority_identity(),
            self.selected_control_generation.generation(),
            minimum_epoch_exclusive,
            requested_until_tick,
        ))
    }

    fn validate_grant(
        &self,
        request: PrimaryServeLeaseRequest,
        lease: PrimaryServeLease,
        now_tick: u64,
    ) -> Result<PrimaryServeLease, PrimaryServeAdmissionDenial> {
        if lease.epoch() <= request.minimum_epoch_exclusive()
            || lease.valid_until_tick() <= now_tick
            || lease.valid_until_tick() > request.requested_until_tick()
            || lease.token() == [0; 32]
            || lease.provider_identity() == [0; 32]
        {
            return Err(PrimaryServeAdmissionDenial::InvalidProviderGrant);
        }
        Ok(lease)
    }

    pub(super) fn require_current_binding(
        &self,
        lease: PrimaryServeLease,
    ) -> Result<(), PrimaryServeAdmissionDenial> {
        if lease.authority() != self.current_authority.authority_identity() {
            return Err(PrimaryServeAdmissionDenial::AuthorityChanged);
        }
        if lease.selected_control_generation() != self.selected_control_generation.generation() {
            return Err(PrimaryServeAdmissionDenial::ControlGenerationChanged);
        }
        Ok(())
    }

    pub(crate) const fn provider(&self) -> &'a dyn OperationalFencingAuthorityPort {
        self.provider
    }
}
