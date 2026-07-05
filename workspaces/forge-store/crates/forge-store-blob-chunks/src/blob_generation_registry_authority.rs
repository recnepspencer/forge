use forge_store_authority::StoreCurrentAuthorityWitness;

#[derive(Debug)]
pub struct BlobGenerationRegistryAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

#[derive(Debug)]
pub struct DerivedBlobRebuildAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

impl BlobGenerationRegistryAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub(crate) fn into_current_authority(self) -> StoreCurrentAuthorityWitness {
        self.current_authority
    }
}

impl DerivedBlobRebuildAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub(crate) fn into_current_authority(self) -> StoreCurrentAuthorityWitness {
        self.current_authority
    }
}
