use super::{
    registry_row, WorthQueryConsumerResidueClass, WorthQueryConsumerResidueDetection,
    WorthQueryConsumerResidueRegistryRow,
};

pub(super) const EVIDENCE_ROWS: &[WorthQueryConsumerResidueRegistryRow] = &[
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryReport,
        WorthQueryConsumerResidueDetection::Ast,
        "local-query-report-struct",
        "consumer defines a local Query report instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::LocalQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "local-query-proof-struct",
        "consumer defines a local Query proof instead of sealed evidence",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::RawSupportSnapshotRow,
        WorthQueryConsumerResidueDetection::Ast,
        "WorthQuerySupportSnapshotRow",
        "consumer treats raw support rows as proof",
        "support-pinning",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::SupportMatrixRowSearch,
        WorthQueryConsumerResidueDetection::Ast,
        "row_for_family",
        "consumer searches support matrix rows instead of pinning support",
        "support-pinning",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DebugDerivedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "format-debug-query-proof",
        "consumer derives Query proof from debug text",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DelimiterJoinedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "delimiter-joined-query-proof",
        "consumer derives Query proof from delimiter-joined strings",
        "evidence-report-kit",
    ),
    registry_row(
        WorthQueryConsumerResidueClass::DelimiterFormattedQueryProof,
        WorthQueryConsumerResidueDetection::Ast,
        "delimiter-formatted-query-proof",
        "consumer derives Query proof from delimiter-formatted strings",
        "evidence-report-kit",
    ),
];
