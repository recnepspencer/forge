mod artifact_locator;
mod aspect_locator;
mod authority;
mod source_locator;

pub use artifact_locator::{BoundaryArtifactField, BoundaryArtifactLocator};
pub use aspect_locator::{
    AspectContractLocator, AspectFieldLocator, AspectLocator, AspectMaskLocator, AspectValueLocator,
};
pub use authority::LocatorAuthority;
pub use source_locator::{BoundaryMismatchLocator, BoundarySourceLocator};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "locators",
        "structural value, aspect, field, and boundary-artifact locator vocabulary",
        "stringly producer-private path interpretation",
    )
}
