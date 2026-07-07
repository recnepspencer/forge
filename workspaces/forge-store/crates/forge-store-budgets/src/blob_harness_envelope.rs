use crate::{
    AllocationBudgetDenial, AllocationByteBudget, AllocationEnvelopeDeclaration,
    AllocationEnvelopeSet, FixedMetadataReservation,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BlobHarnessEnvelopeProfile {
    Local,
    CiMemoryEnvelopeExceeding,
    HeavyMultiGb,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlobHarnessEnvelopeDeclaration {
    profile: BlobHarnessEnvelopeProfile,
    declared_logical_bytes: u64,
    resident_memory_budget_bytes: u64,
    allocation_budget_bytes: u64,
}

impl BlobHarnessEnvelopeDeclaration {
    pub const fn local() -> Self {
        Self::new(
            BlobHarnessEnvelopeProfile::Local,
            8 * 1024 * 1024,
            16 * 1024 * 1024,
            64 * 1024,
        )
    }

    pub const fn ci_memory_envelope_exceeding() -> Self {
        Self::new(
            BlobHarnessEnvelopeProfile::CiMemoryEnvelopeExceeding,
            768 * 1024 * 1024,
            512 * 1024 * 1024,
            512 * 1024,
        )
    }

    pub const fn heavy_multi_gb() -> Self {
        Self::new(
            BlobHarnessEnvelopeProfile::HeavyMultiGb,
            64 * 1024 * 1024 * 1024,
            512 * 1024 * 1024,
            1024 * 1024,
        )
    }

    const fn new(
        profile: BlobHarnessEnvelopeProfile,
        declared_logical_bytes: u64,
        resident_memory_budget_bytes: u64,
        allocation_budget_bytes: u64,
    ) -> Self {
        Self {
            profile,
            declared_logical_bytes,
            resident_memory_budget_bytes,
            allocation_budget_bytes,
        }
    }

    pub const fn profile(self) -> BlobHarnessEnvelopeProfile {
        self.profile
    }

    pub const fn declared_logical_bytes(self) -> u64 {
        self.declared_logical_bytes
    }

    pub const fn resident_memory_budget_bytes(self) -> u64 {
        self.resident_memory_budget_bytes
    }

    pub const fn exceeds_resident_memory_envelope(self) -> bool {
        self.declared_logical_bytes > self.resident_memory_budget_bytes
    }

    pub fn allocation_envelope(self) -> Result<AllocationEnvelopeSet, AllocationBudgetDenial> {
        let budget = AllocationByteBudget::bytes(self.allocation_budget_bytes)?;
        AllocationEnvelopeDeclaration::declare()
            .foreground(budget)
            .maintenance(budget)
            .recovery(budget)
            .scrub(budget)
            .import_export(budget)
            .streaming(budget)
            .fixed_metadata(FixedMetadataReservation::constant_bytes(4096)?)
            .seal()
    }
}
