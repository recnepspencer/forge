mod employee_record;
mod parity;
mod scale;

pub use employee_record::{
    employee_record_policy_fixture, EmployeeRecordCertificationBundle, EmployeeRecordPolicyFixture,
    EmployeeRecordPolicyScenario, EmployeeRecordQueryFamily, EmployeeRecordTenantVariant,
};
pub use parity::{
    policy_composition_parity_report, policy_identity_aware_inspector_parity_report,
    policy_mask_parity_report, policy_view_shape_parity_report, PolicyCompositionParityReport,
    PolicyIdentityAwareInspectorParityReport, PolicyMaskParityReport, PolicyViewShapeParityReport,
};
pub use scale::{
    employee_record_policy_scale_report, PolicyScaleCounterSnapshot, PolicyScaleFixtureSize,
    PolicyScaleSlopeDigest, PolicyScaleSlopeReport,
};

#[cfg(test)]
mod tests;
