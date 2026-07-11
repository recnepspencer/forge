use crate::{PhysicalScopeAdmission, PhysicalScopeDenial, PhysicalScopeDenialKind};
use forge_store_physical_format::PhysicalScopeFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedPhysicalValidatorInput<'lease> {
    admission: PhysicalScopeAdmission<'lease>,
    family: PhysicalScopeFamily,
}

impl<'lease> ScopedPhysicalValidatorInput<'lease> {
    pub fn page(admission: PhysicalScopeAdmission<'lease>) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::Page)
    }

    pub fn frame(admission: PhysicalScopeAdmission<'lease>) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::Frame)
    }

    pub fn wal_frame(
        admission: PhysicalScopeAdmission<'lease>,
    ) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::WalFrame)
    }

    pub fn manifest(
        admission: PhysicalScopeAdmission<'lease>,
    ) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::Manifest)
    }

    pub fn chunk_like(
        admission: PhysicalScopeAdmission<'lease>,
    ) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::ChunkLike)
    }

    pub fn derived_index(
        admission: PhysicalScopeAdmission<'lease>,
    ) -> Result<Self, PhysicalScopeDenial> {
        Self::for_family(admission, PhysicalScopeFamily::DerivedIndex)
    }

    pub const fn admission(&self) -> &PhysicalScopeAdmission<'lease> {
        &self.admission
    }

    pub const fn family(&self) -> PhysicalScopeFamily {
        self.family
    }

    fn for_family(
        admission: PhysicalScopeAdmission<'lease>,
        family: PhysicalScopeFamily,
    ) -> Result<Self, PhysicalScopeDenial> {
        if admission.scope_family() != family {
            return Err(PhysicalScopeDenial::new(
                PhysicalScopeDenialKind::WrongPhysicalFamily,
            ));
        }
        Ok(Self { admission, family })
    }
}
