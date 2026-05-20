mod builder;
mod cases;
mod report;
mod schema;

pub use report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
pub use schema::{
    PrimitiveConstructionCompoundAuthoringOrderRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundGrazingKind,
    PrimitiveConstructionCompoundMotionKind, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow, PrimitiveConstructionCompoundRowClass,
    PrimitiveConstructionCompoundTopologyClass, PrimitiveConstructionCompoundWorkloadFamily,
};

pub use builder::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};

#[cfg(test)]
mod tests;
