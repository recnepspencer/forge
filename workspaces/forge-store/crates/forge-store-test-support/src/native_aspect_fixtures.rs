use forge_foundational::{
    AspectContract, AspectValue, AuthoritativeRecordAspectStateArtifact,
    ContractValidatedAspectArtifact, StructAspectValue,
};
use forge_store_aspect_native::{
    StoreAspectBoundaryFact, StoreAspectBoundaryLocator, StoreAspectFieldBoundaryLocator,
    StoreAspectIdentity, StoreAspectPatchBoundaryFact, StoreAspectValueBoundaryLocator,
    StorePhysicalBoundaryWitness,
};

use crate::native_aspect_fixture_authoring::{
    authored_scalar_string_fixture, authored_segment_header_fixture,
    AuthoredNativeStoreAspectFixture,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NativeStoreAspectFixture {
    authored: AuthoredNativeStoreAspectFixture,
}

impl NativeStoreAspectFixture {
    pub fn segment_header(segment: &str, generation: u64) -> Self {
        Self {
            authored: authored_segment_header_fixture(segment, generation),
        }
    }

    pub fn scalar_string(value: &str) -> Self {
        Self {
            authored: authored_scalar_string_fixture(value),
        }
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.authored.identity
    }

    pub const fn contract(&self) -> &AspectContract {
        &self.authored.contract
    }

    pub const fn scalar_value(&self) -> Option<&AspectValue> {
        self.authored.scalar_value.as_ref()
    }

    pub const fn struct_value(&self) -> Option<&StructAspectValue> {
        self.authored.struct_value.as_ref()
    }

    pub const fn validated_value(&self) -> &ContractValidatedAspectArtifact {
        &self.authored.validated_value
    }

    pub const fn authoritative_state(&self) -> &AuthoritativeRecordAspectStateArtifact {
        &self.authored.authoritative_state
    }

    pub const fn boundary_fact(&self) -> &StoreAspectBoundaryFact {
        &self.authored.boundary_fact
    }

    pub const fn patch_boundary_fact(&self) -> &StoreAspectPatchBoundaryFact {
        &self.authored.patch_fact
    }

    pub const fn aspect_locator(&self) -> &StoreAspectBoundaryLocator {
        &self.authored.aspect_locator
    }

    pub const fn value_locator(&self) -> &StoreAspectValueBoundaryLocator {
        &self.authored.value_locator
    }

    pub const fn field_locator(&self) -> Option<&StoreAspectFieldBoundaryLocator> {
        self.authored.field_locator.as_ref()
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.authored.physical_witness
    }
}

pub const fn require_native_store_aspect_fixture(
    fixture: &NativeStoreAspectFixture,
) -> &StoreAspectBoundaryFact {
    fixture.boundary_fact()
}
