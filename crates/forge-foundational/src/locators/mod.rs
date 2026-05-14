mod artifact_locator;
mod aspect_locator;
mod authority;
mod source_locator;
mod transition_locator;

pub use artifact_locator::{BoundaryArtifactField, BoundaryArtifactLocator};
pub use aspect_locator::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectMaskLocator, AspectValueLocator,
};
pub use authority::LocatorAuthority;
pub use source_locator::{BoundaryMismatchLocator, BoundarySourceLocator};
pub use transition_locator::{
    FoundationalBranchCandidateLocator, FoundationalCommitParentageLocator,
    FoundationalCommittedDeltaLocator, FoundationalMergeConflictLocator,
    FoundationalTransitionLocator,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "locators",
        "structural value, aspect, field, boundary-artifact, and transition locator vocabulary",
        "stringly producer-private path interpretation",
    )
}
