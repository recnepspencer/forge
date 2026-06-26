use forge_foundational::{
    AspectFieldLocator, AspectKey, AspectLocator, AspectValueLocator, BoundaryArtifactLocator,
};

use crate::{StoreAspectIdentity, StoreAspectNativeDenial};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectBoundaryLocator {
    identity: StoreAspectIdentity,
    locator: AspectLocator,
}

impl StoreAspectBoundaryLocator {
    pub fn new(
        identity: StoreAspectIdentity,
        locator: AspectLocator,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if identity.aspect_key() != locator.aspect_key() {
            return Err(StoreAspectNativeDenial::LocatorIdentityMismatch);
        }

        Ok(Self { identity, locator })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn locator(&self) -> &AspectLocator {
        &self.locator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectFieldBoundaryLocator {
    identity: StoreAspectIdentity,
    locator: AspectFieldLocator,
}

impl StoreAspectFieldBoundaryLocator {
    pub fn new(
        identity: StoreAspectIdentity,
        locator: AspectFieldLocator,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if identity.aspect_key() != locator.aspect().aspect_key() {
            return Err(StoreAspectNativeDenial::LocatorIdentityMismatch);
        }

        Ok(Self { identity, locator })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn locator(&self) -> &AspectFieldLocator {
        &self.locator
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreAspectValueBoundaryLocator {
    identity: StoreAspectIdentity,
    locator: AspectValueLocator,
}

impl StoreAspectValueBoundaryLocator {
    pub fn new(
        identity: StoreAspectIdentity,
        locator: AspectValueLocator,
    ) -> Result<Self, StoreAspectNativeDenial> {
        if identity.aspect_key() != aspect_value_locator_key(&locator) {
            return Err(StoreAspectNativeDenial::LocatorIdentityMismatch);
        }

        Ok(Self { identity, locator })
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn locator(&self) -> &AspectValueLocator {
        &self.locator
    }
}

fn aspect_value_locator_key(locator: &AspectValueLocator) -> &AspectKey {
    match locator {
        AspectValueLocator::WholeAspect(aspect) => aspect.aspect_key(),
        AspectValueLocator::StructField(field) => field.aspect().aspect_key(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreBoundaryArtifactBoundaryLocator {
    locator: BoundaryArtifactLocator,
}

impl StoreBoundaryArtifactBoundaryLocator {
    pub const fn new(locator: BoundaryArtifactLocator) -> Self {
        Self { locator }
    }

    pub const fn locator(&self) -> &BoundaryArtifactLocator {
        &self.locator
    }
}
