use forge_store_physical_format::RootPublicationValidationWitness;

use super::PhysicalPublicationDenial;
use crate::CurrentPhysicalRoot;

#[derive(Debug, Clone, Copy)]
pub struct PublicationRootCandidate {
    root: CurrentPhysicalRoot,
    validation: RootPublicationValidationWitness,
}

impl PublicationRootCandidate {
    pub fn admit(
        root: CurrentPhysicalRoot,
        validation: RootPublicationValidationWitness,
    ) -> Result<Self, PhysicalPublicationDenial> {
        let validation_root = validation
            .reference()
            .root_reference()
            .ok_or(PhysicalPublicationDenial::RootPublicationValidationRootMismatch)?;
        if validation_root.get() != root.scope() {
            return Err(PhysicalPublicationDenial::RootPublicationValidationRootMismatch);
        }
        Ok(Self { root, validation })
    }

    pub const fn root(self) -> CurrentPhysicalRoot {
        self.root
    }

    pub const fn validation(self) -> RootPublicationValidationWitness {
        self.validation
    }
}
