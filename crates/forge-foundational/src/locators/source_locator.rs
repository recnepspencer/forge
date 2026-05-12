use super::{AspectFieldLocator, AspectLocator, BoundaryArtifactLocator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundarySourceLocator {
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    BoundaryArtifact(BoundaryArtifactLocator),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BoundaryMismatchLocator {
    Aspect(AspectLocator),
    AspectField(AspectFieldLocator),
    BoundaryArtifact(BoundaryArtifactLocator),
}
