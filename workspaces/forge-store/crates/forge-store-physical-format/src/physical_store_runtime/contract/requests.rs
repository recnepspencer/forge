use crate::{
    ExtentGenerationCell, PhysicalBinaryEncodingWitness, PhysicalHeaderAuthority,
    SlotGenerationCell,
};
use forge_store_aspect_native::StoreAspectIdentity;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PhysicalStoreIdentity(StoreAspectIdentity);

impl PhysicalStoreIdentity {
    pub const fn from_aspect_identity(identity: StoreAspectIdentity) -> Self {
        Self(identity)
    }

    pub const fn aspect_identity(&self) -> &StoreAspectIdentity {
        &self.0
    }

    pub fn authority_identity(&self) -> forge_store_authority::StoreCurrentAuthorityIdentity {
        forge_store_authority::StoreCurrentAuthorityIdentity::from_aspect_identity(&self.0)
    }

    pub fn physical_format_default() -> Self {
        let key = forge_foundational::aspects()
            .vocabulary()
            .key("store.physical.default_instance")
            .expect("canonical physical Store identity key");
        Self::from_aspect_identity(StoreAspectIdentity::from_aspect_key(key))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformPhysicalOpenRequest {
    headers: PhysicalHeaderAuthority,
    store_identity: PhysicalStoreIdentity,
}

impl PlatformPhysicalOpenRequest {
    pub fn new(headers: PhysicalHeaderAuthority) -> Self {
        Self {
            headers,
            store_identity: PhysicalStoreIdentity::physical_format_default(),
        }
    }

    pub const fn for_store(
        headers: PhysicalHeaderAuthority,
        store_identity: PhysicalStoreIdentity,
    ) -> Self {
        Self {
            headers,
            store_identity,
        }
    }

    pub fn physical_format_canonical() -> Self {
        Self::new(PhysicalHeaderAuthority::for_canonical_physical_format(
            PhysicalBinaryEncodingWitness::physical_format_canonical()
                .expect("canonical S.1 binary format is admitted"),
        ))
    }

    pub fn physical_format_for_store(store_identity: PhysicalStoreIdentity) -> Self {
        Self::for_store(
            PhysicalHeaderAuthority::for_canonical_physical_format(
                PhysicalBinaryEncodingWitness::physical_format_canonical()
                    .expect("canonical physical format is admitted"),
            ),
            store_identity,
        )
    }

    pub const fn headers(&self) -> &PhysicalHeaderAuthority {
        &self.headers
    }

    pub const fn store_identity(&self) -> &PhysicalStoreIdentity {
        &self.store_identity
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlatformPhysicalRecordTarget {
    PageSlot(SlotGenerationCell),
    Extent(ExtentGenerationCell),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PlatformPhysicalAppendRequest<'a> {
    target: PlatformPhysicalRecordTarget,
    payload: &'a [u8],
}

impl<'a> PlatformPhysicalAppendRequest<'a> {
    pub const fn page_slot(slot_cell: SlotGenerationCell, payload: &'a [u8]) -> Self {
        Self {
            target: PlatformPhysicalRecordTarget::PageSlot(slot_cell),
            payload,
        }
    }

    pub const fn extent(extent_cell: ExtentGenerationCell, payload: &'a [u8]) -> Self {
        Self {
            target: PlatformPhysicalRecordTarget::Extent(extent_cell),
            payload,
        }
    }

    pub const fn target(self) -> PlatformPhysicalRecordTarget {
        self.target
    }

    pub const fn payload(self) -> &'a [u8] {
        self.payload
    }
}
