mod attachment;
mod materialized;
mod plan;
mod request;
mod source;
mod targets;

pub use attachment::{
    attach_counter_backed_performance_receipt, attach_performance_bundle,
    attach_policy_admission_receipt, FoundationalAttachedCounterBackedPerformanceReceipt,
    FoundationalAttachedPerformanceBundle, FoundationalAttachedPolicyAdmissionReceipt,
    FoundationalPerformanceAttachmentDenial,
};
pub use materialized::FoundationalMaterializedPerformanceReport;
pub use plan::{
    plan_performance_report, FoundationalPerformanceReportMaterializationBoundary,
    FoundationalPerformanceReportPlan, FoundationalPerformanceReportSection,
    FoundationalPerformanceReportSectionDecision,
    FoundationalPerformanceReportSectionDecisionCause,
};
pub use request::FoundationalPerformanceReportRequest;
pub use targets::{
    foundational_performance_attachment_target_kind_definitions,
    FoundationalPerformanceAttachmentTargetKind,
};
