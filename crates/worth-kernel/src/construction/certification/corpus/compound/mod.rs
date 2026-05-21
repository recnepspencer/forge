mod builder;
mod cases;
mod lane_report;
mod milestone_closeout;
mod ordering_report;
mod parity;
mod report;
mod row_builder;
mod rows;
mod schema;

pub(super) use cases::PrimitiveConstructionCompoundScenario;
pub use lane_report::{
    PrimitiveConstructionCompoundAuthoringOrderRow, PrimitiveConstructionCompoundOrderLaneReport,
};
pub use milestone_closeout::PrimitiveConstructionCompoundMilestoneCloseoutReport;
pub use ordering_report::{
    PrimitiveConstructionCompoundOrderingParityReport,
    PrimitiveConstructionCompoundOrderingScenarioRow,
};
pub use parity::PrimitiveConstructionCompoundParityCanonicalTruth;
pub use parity::{
    PrimitiveConstructionCompoundParityReport,
    PrimitiveConstructionCompoundParityVerificationFailure,
    PrimitiveConstructionCompoundParityVerificationMismatch,
};
pub use report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundExhaustionWitnessParityReport,
    PrimitiveConstructionCompoundGrazingBoundaryReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
pub use rows::{
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
pub use schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundRowClass, PrimitiveConstructionCompoundTopologyClass,
    PrimitiveConstructionCompoundWorkloadFamily,
};

pub use builder::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_exhaustion_witness_parity_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    prepare_primitive_construction_compound_ordering_parity_report,
    prepare_primitive_construction_compound_parity_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};
pub use milestone_closeout::prepare_primitive_construction_compound_milestone_closeout_report;

#[cfg(test)]
mod tests;
