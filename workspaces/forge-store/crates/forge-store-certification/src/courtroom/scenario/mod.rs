//! Scenario definition, planning, and execution surfaces.

pub mod s8_layout;

pub use crate::scenario_definition::{
    PhysicalScenarioDefinition, PhysicalScenarioDefinitionBuilder,
    PhysicalScenarioDefinitionDenial, PhysicalStoryStep, ScenarioLane,
};
pub use crate::scenario_execution::{PhysicalScenarioExecution, PhysicalScenarioExecutionReport};
pub use crate::scenario_plan::{
    ArtifactPolicy, ExpectedPhysicalFootprint, PhysicalScenarioCapabilityTier,
    PhysicalScenarioCostClass, PhysicalScenarioPlan, PhysicalScenarioPlanDenial,
    PhysicalScenarioPlanIdentity, StorageBoundaryCrossing, WorkloadScale,
};
pub use crate::scenario_planned_work_evidence::PhysicalScenarioPlannedWorkBoundaryReport;
pub use s8_layout::{certify_s8_layout_scenario, S8LayoutScenarioCertificate};
