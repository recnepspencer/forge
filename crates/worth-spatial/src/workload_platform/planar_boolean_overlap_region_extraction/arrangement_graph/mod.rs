pub(crate) mod cell_classification;
mod construction;
mod counters;
mod denial;
mod graph;
mod identity;
mod input;
mod lookup;
mod product;
mod source_only_topology;
#[cfg(test)]
pub(crate) mod tests;
mod topology_ordering;
mod topology_validation;
mod validation;

pub use cell_classification::{
    PlanarBooleanOverlapCellClassificationCounters, PlanarBooleanOverlapCellClassificationDenial,
    PlanarBooleanOverlapCellClassificationDenialKind,
    PlanarBooleanOverlapCellContainmentEvidenceKind, PlanarBooleanOverlapCellContainmentInput,
    PlanarBooleanOverlapCellContainmentMap, PlanarBooleanOverlapCellContainmentRow,
    PlanarBooleanOverlapCellWindingEvidenceKind, PlanarBooleanOverlapCellWindingField,
    PlanarBooleanOverlapCellWindingFieldInput, PlanarBooleanOverlapCellWindingRow,
};
pub use counters::PlanarBooleanOverlapArrangementGraphCounters;
pub use denial::{
    PlanarBooleanOverlapArrangementGraphDenial, PlanarBooleanOverlapArrangementGraphDenialKind,
};
pub use graph::{
    PlanarBooleanCoplanarOverlapArrangementNeighborhoodRow,
    PlanarBooleanOverlapArrangementBoundarySegmentRow, PlanarBooleanOverlapArrangementCellRow,
    PlanarBooleanOverlapArrangementCellSet,
};
pub use input::PlanarBooleanOverlapArrangementGraphInput;
pub use product::PlanarBooleanCoplanarOverlapArrangementGraph;
