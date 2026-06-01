use crate::locators::{AspectFieldLocator, AspectLocator, BoundaryArtifactLocator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryMismatchLocator {
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    BoundaryArtifact(BoundaryArtifactLocator),
}

impl BoundaryMismatchLocator {
    pub fn aspect(locator: AspectLocator) -> Self {
        Self::Aspect(locator)
    }

    pub fn aspect_field(locator: AspectFieldLocator) -> Self {
        Self::AspectField(locator)
    }

    pub fn boundary_artifact(locator: BoundaryArtifactLocator) -> Self {
        Self::BoundaryArtifact(locator)
    }
}
