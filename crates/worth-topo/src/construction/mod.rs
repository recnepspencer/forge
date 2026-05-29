mod authority;
mod certification;
mod execution;
mod facts;
mod lowering;

pub use authority::{topology_construction_authority, TopologyConstructionAuthority};
pub use certification::{
    prepare_primitive_construction_certification, TopologyConstructionCertificationPlan,
    TopologyConstructionCertificationReadSurface, TopologyConstructionInspectionSurface,
};
pub use execution::{
    prepare_primitive_construction_execution, TopologyConstructionExecutionError,
    TopologyConstructionExecutionPlan,
};
pub use facts::{
    build_topology_construction_fact_report, TopologyConstructionFactKind,
    TopologyConstructionFactProvenance, TopologyConstructionFactReport,
    TopologyConstructionFactRow,
};
pub use lowering::{
    lower_primitive_construction_birth_plan, TopologyConstructionLoweringError,
    TopologyConstructionLoweringPlan, TopologyConstructionMutationSurface,
};
