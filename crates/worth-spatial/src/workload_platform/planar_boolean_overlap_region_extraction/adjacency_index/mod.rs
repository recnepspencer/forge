mod construction;
mod counters;
mod denial;
mod identity;
mod input;
mod lookup;
mod neighborhood;
mod ordering;
mod product;
mod row;
#[cfg(test)]
mod tests;
mod validation;

pub use counters::PlanarBooleanOverlapAdjacencyIndexCounters;
pub use denial::{
    PlanarBooleanOverlapAdjacencyIndexDenial, PlanarBooleanOverlapAdjacencyIndexDenialKind,
};
pub use input::PlanarBooleanOverlapAdjacencyIndexInput;
pub use neighborhood::PlanarBooleanOverlapNeighborhoodView;
pub use ordering::PlanarBooleanOverlapAdjacencyOrderingBasis;
pub use product::PlanarBooleanOverlapRegionAdjacencyIndex;
pub use row::PlanarBooleanOverlapAdjacencyRow;
