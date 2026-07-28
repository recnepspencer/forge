mod geometry;
mod index;
mod interval_index;
mod record;
mod validation;

pub(crate) use index::{UiHitTestRegionIndex, UiVisibleRegionIndex};
pub(crate) use record::{UiVisibleOpacity, UiVisibleRegionRecord};
pub(crate) use validation::{
    validate_and_index, UiSpatialIndexBuildCost, UiSpatialValidationDenial,
};

#[cfg(test)]
mod tests;
