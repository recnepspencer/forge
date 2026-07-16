use crate::{ControlStoreGeneration, StoreCurrentAuthorityIdentity, StoreCurrentAuthorityWitness};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ControlStoreFencingProviderDenial {
    Unsupported,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlStoreSelectionCoordinates {
    media_identity_fingerprint: [u8; 32],
    generation: ControlStoreGeneration,
    prefix_digest: [u8; 32],
}

impl ControlStoreSelectionCoordinates {
    pub const fn new(
        media_identity_fingerprint: [u8; 32],
        generation: ControlStoreGeneration,
        prefix_digest: [u8; 32],
    ) -> Self {
        Self {
            media_identity_fingerprint,
            generation,
            prefix_digest,
        }
    }

    pub const fn media_identity_fingerprint(self) -> [u8; 32] {
        self.media_identity_fingerprint
    }

    pub const fn generation(self) -> ControlStoreGeneration {
        self.generation
    }

    pub const fn prefix_digest(self) -> [u8; 32] {
        self.prefix_digest
    }
}

pub trait ControlStoreFencingPort: std::fmt::Debug {
    fn selected_control_store(
        &self,
        current_authority: StoreCurrentAuthorityIdentity,
    ) -> Result<ControlStoreSelectionCoordinates, ControlStoreFencingProviderDenial>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectedControlStoreGeneration {
    coordinates: ControlStoreSelectionCoordinates,
    authority_identity: StoreCurrentAuthorityIdentity,
}

impl SelectedControlStoreGeneration {
    pub const fn generation(self) -> ControlStoreGeneration {
        self.coordinates.generation()
    }

    pub const fn media_identity_fingerprint(self) -> [u8; 32] {
        self.coordinates.media_identity_fingerprint()
    }

    pub const fn authority_identity(self) -> StoreCurrentAuthorityIdentity {
        self.authority_identity
    }

    pub const fn prefix_digest(self) -> [u8; 32] {
        self.coordinates.prefix_digest()
    }
}

#[derive(Debug)]
pub struct ControlStoreFencingAuthority<'a> {
    current_authority: &'a StoreCurrentAuthorityWitness,
    provider: &'a dyn ControlStoreFencingPort,
}

impl<'a> ControlStoreFencingAuthority<'a> {
    pub const fn for_current_store(
        current_authority: &'a StoreCurrentAuthorityWitness,
        provider: &'a dyn ControlStoreFencingPort,
    ) -> Self {
        Self {
            current_authority,
            provider,
        }
    }

    pub fn select_generation(
        &self,
    ) -> Result<SelectedControlStoreGeneration, ControlStoreFencingProviderDenial> {
        let authority_identity = self.current_authority.authority_identity();
        let coordinates = self.provider.selected_control_store(authority_identity)?;
        Ok(SelectedControlStoreGeneration {
            coordinates,
            authority_identity,
        })
    }
}
