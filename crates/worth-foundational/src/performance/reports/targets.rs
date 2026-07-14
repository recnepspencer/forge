use crate::performance::primitives::FoundationalPerformancePrimitiveDefinition;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FoundationalPerformanceAttachmentTargetKind {
    BoundarySummary,
    BoundaryReceipt,
    BoundaryReport,
    BoundaryArtifact,
    SupportBundle,
    CertificationBundle,
}

pub const fn foundational_performance_attachment_target_kind_definitions(
) -> [FoundationalPerformancePrimitiveDefinition<FoundationalPerformanceAttachmentTargetKind>; 6] {
    [
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::BoundarySummary,
            "boundary_summary",
            "a performance surface attached to a boundary summary artifact family",
            "a boundary receipt, boundary report, artifact, support bundle, or certification bundle target",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::BoundaryReceipt,
            "boundary_receipt",
            "a performance surface attached to a boundary receipt artifact family",
            "a summary, report, artifact, support bundle, or certification bundle target",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::BoundaryReport,
            "boundary_report",
            "a performance surface attached to a boundary report artifact family",
            "a summary, receipt, artifact, support bundle, or certification bundle target",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::BoundaryArtifact,
            "boundary_artifact",
            "a performance surface attached to a boundary artifact family",
            "a summary-only, receipt-only, report-only, support-only, or certification-only target",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::SupportBundle,
            "support_bundle",
            "a performance surface attached to a support-bearing bundle family",
            "a summary, receipt, report, artifact, or certification bundle target",
        ),
        FoundationalPerformancePrimitiveDefinition::new(
            FoundationalPerformanceAttachmentTargetKind::CertificationBundle,
            "certification_bundle",
            "a performance surface attached to a stronger certification-bearing bundle family",
            "a summary, receipt, report, artifact, or support bundle target",
        ),
    ]
}
