mod case_fixtures;
mod delta;
mod grazing_cases;
mod host_face_cases;
mod registry;
mod report;

pub use delta::{
    prepare_primitive_construction_policy_pressure_delta_report,
    PrimitiveConstructionPolicyPressureDeltaCase, PrimitiveConstructionPolicyPressureDeltaReport,
    PrimitiveConstructionPolicyPressureDeltaReportError,
    PrimitiveConstructionPolicyPressureDeltaRow,
};
#[cfg(test)]
pub(crate) use registry::{
    required_policy_pressure_delta_cases, required_policy_pressure_direct_cases,
};
pub use report::{
    prepare_primitive_construction_policy_pressure_report, PrimitiveConstructionPolicyPressureCase,
    PrimitiveConstructionPolicyPressureRow, PrimitiveConstructionPolicyPressureSetup,
    PrimitiveConstructionPolicyPressureSurfaceReport,
    PrimitiveConstructionPolicyPressureSurfaceReportError,
};
