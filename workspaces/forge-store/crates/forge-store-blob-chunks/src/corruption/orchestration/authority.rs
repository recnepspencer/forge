use forge_store_authority::StoreCurrentAuthorityWitness;

#[derive(Debug)]
pub struct BlobQuarantineAuthority {
    current_authority: StoreCurrentAuthorityWitness,
}

impl BlobQuarantineAuthority {
    pub const fn from_current_store_authority(
        current_authority: StoreCurrentAuthorityWitness,
    ) -> Self {
        Self { current_authority }
    }

    pub(crate) fn into_current_authority(self) -> StoreCurrentAuthorityWitness {
        self.current_authority
    }
}