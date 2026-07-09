use worth_store_contracts::{
    BufferPoolAuthorityRecap, PhysicalAuthorityRecap, S2BoundedCounterRecap, S2DenialBehaviorRecap,
    S3PhysicalIntegrityReadinessPayload,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityEntryBasis {
    payload: S3PhysicalIntegrityReadinessPayload,
}

impl IntegrityEntryBasis {
    pub(crate) const fn from_payload(payload: S3PhysicalIntegrityReadinessPayload) -> Self {
        Self { payload }
    }

    pub const fn protected_view_count(self) -> u32 {
        self.payload
            .protected_view_capability()
            .protected_view_count()
    }

    pub const fn verifier_resident_limits(self) -> VerifierResidentLimits {
        VerifierResidentLimits {
            resident_bytes: self.payload.verifier_resident_envelope().resident_bytes(),
            pinned_pages: self.payload.verifier_resident_envelope().pinned_pages(),
        }
    }

    pub const fn scrub_envelope_limits(self) -> ScrubEnvelopeLimits {
        ScrubEnvelopeLimits {
            allocation_bytes: self.payload.scrub_allocation_envelope().allocation_bytes(),
        }
    }

    pub const fn counter_recap(self) -> S2BoundedCounterRecap {
        self.payload.counter_recap()
    }

    pub const fn denial_behavior(self) -> S2DenialBehaviorRecap {
        self.payload.denial_behavior()
    }

    pub const fn physical_authority_recap(self) -> PhysicalAuthorityRecap {
        self.payload.physical_authority_recap()
    }

    pub const fn buffer_pool_authority_recap(self) -> BufferPoolAuthorityRecap {
        self.payload.buffer_pool_authority_recap()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VerifierResidentLimits {
    resident_bytes: u64,
    pinned_pages: u32,
}

impl VerifierResidentLimits {
    pub const fn resident_bytes(self) -> u64 {
        self.resident_bytes
    }

    pub const fn pinned_pages(self) -> u32 {
        self.pinned_pages
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrubEnvelopeLimits {
    allocation_bytes: u64,
}

impl ScrubEnvelopeLimits {
    pub const fn allocation_bytes(self) -> u64 {
        self.allocation_bytes
    }
}
