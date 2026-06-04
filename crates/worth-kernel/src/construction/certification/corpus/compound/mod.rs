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

#[cfg(test)]
pub use builder::{
    prepare_primitive_construction_compound_adversarial_siege_report,
    prepare_primitive_construction_compound_exhaustion_witness_parity_report,
    prepare_primitive_construction_compound_grazing_boundary_report,
    prepare_primitive_construction_compound_motion_parity_report,
    prepare_primitive_construction_compound_ordering_parity_report,
    prepare_primitive_construction_compound_parity_report,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};
pub(super) use cases::PrimitiveConstructionCompoundScenario;
pub use milestone_closeout::prepare_primitive_construction_compound_milestone_closeout_report;
pub use milestone_closeout::PrimitiveConstructionCompoundMilestoneCloseoutReport;
#[cfg(test)]
pub use ordering_report::PrimitiveConstructionCompoundOrderingParityReport;
#[cfg(test)]
pub use parity::PrimitiveConstructionCompoundParityCanonicalTruth;
#[cfg(test)]
pub use report::{
    PrimitiveConstructionCompoundAdversarialSiegeReport,
    PrimitiveConstructionCompoundMotionParityReport,
};
pub use rows::PrimitiveConstructionCompoundRow;
pub use schema::PrimitiveConstructionCompoundRowClass;
#[cfg(test)]
pub use schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundTopologyClass, PrimitiveConstructionCompoundWorkloadFamily,
};

#[cfg(test)]
mod tests;
