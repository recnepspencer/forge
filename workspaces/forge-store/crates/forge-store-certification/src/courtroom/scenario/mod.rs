//! Scenario definition, planning, and execution surfaces.

pub mod s8_layout;

pub use crate::evidence::scheduling::scenario_planned_work_evidence::PhysicalScenarioPlannedWorkBoundaryReport;
pub use crate::scenario::scheduling::scenario_definition::{
    PhysicalScenarioDefinition, PhysicalScenarioDefinitionBuilder,
    PhysicalScenarioDefinitionDenial, PhysicalStoryStep, ScenarioLane,
};
pub use crate::scenario::scheduling::scenario_execution::{
    PhysicalScenarioExecution, PhysicalScenarioExecutionReport,
};
pub use crate::scenario::scheduling::scenario_plan::{
    ArtifactPolicy, ExpectedPhysicalFootprint, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioPlan, PhysicalScenarioPlanDenial,
    PhysicalScenarioPlanIdentity, StorageBoundaryCrossing, WorkloadScale,
};
pub use s8_layout::{certify_s8_layout_scenario, S8LayoutScenarioCertificate};
