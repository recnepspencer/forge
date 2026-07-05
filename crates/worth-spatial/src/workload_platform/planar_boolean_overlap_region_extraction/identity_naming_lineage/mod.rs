mod classification;
mod counters;
mod denial;
mod identity;
mod input;
mod product;
mod rows;
#[cfg(test)]
mod tests;
mod validation;

pub use counters::PlanarBooleanOverlapRegionIdentityLineageCounters;
pub use denial::{
    PlanarBooleanOverlapRegionIdentityLineageDenial,
    PlanarBooleanOverlapRegionIdentityLineageDenialKind,
};
pub use input::PlanarBooleanOverlapRegionIdentityLineageInput;
pub use product::{
    PlanarBooleanOverlapRegionIdentityLineageBundle, PlanarBooleanOverlapRegionIdentityMap,
    PlanarBooleanOverlapRegionPersistentNamePropagationMap,
    PlanarBooleanOverlapRegionSubshapeSignatureMap,
};
pub use rows::{
    PlanarBooleanOverlapRegionIdentityRow, PlanarBooleanOverlapRegionPersistentNamePropagationRow,
    PlanarBooleanOverlapRegionSubshapeSignatureRow,
};
