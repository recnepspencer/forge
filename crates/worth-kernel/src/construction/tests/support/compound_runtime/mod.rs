mod builder;
mod cases;
mod parity;
mod row_builder;
mod rows;
mod schema;

pub(crate) use builder::{
    prepare_primitive_construction_compound_adversarial_lanes,
    PrimitiveConstructionCompoundAdversarialLanes,
    PrimitiveConstructionCompoundAdversarialSiegeError,
};
pub(crate) use cases::PrimitiveConstructionCompoundScenario;
pub(crate) use cases::{
    compound_scenarios, PrimitiveConstructionCompoundGrazingPlan,
    PrimitiveConstructionCompoundMotionPlan,
};
pub(crate) use parity::{compound_parity_registry, exhaustion_witness_kind_for};
pub(crate) use rows::{
    PrimitiveConstructionCompoundExhaustionWitnessParityRow,
    PrimitiveConstructionCompoundGrazingBoundaryRow, PrimitiveConstructionCompoundMotionParityRow,
    PrimitiveConstructionCompoundRow,
};
pub(crate) use schema::PrimitiveConstructionCompoundRowClass;
pub(crate) use schema::{
    PrimitiveConstructionCompoundGrazingKind, PrimitiveConstructionCompoundMotionKind,
    PrimitiveConstructionCompoundTopologyClass, PrimitiveConstructionCompoundWorkloadFamily,
};
