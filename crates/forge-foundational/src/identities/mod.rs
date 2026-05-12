mod category_ids;

pub use category_ids::{
    BoundaryArtifactId, BoundaryEpoch, BoundaryHandle, CanonicalDigestId, EquivalenceBasisId,
};

use crate::facade::ResponsibilityArea;

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "identity_categories",
        "typed identity, handle, key, and basis-id boundary categories",
        "producer-private identity allocation or storage indexes",
    )
}
