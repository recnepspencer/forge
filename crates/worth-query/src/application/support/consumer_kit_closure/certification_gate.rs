use super::evidence::consumer_kit_embedded_source_identity;
use super::family::WorthQueryConsumerKitFamilyName;

pub(super) struct WorthQueryConsumerKitCertificationSource {
    pub(super) path: &'static str,
    pub(super) source: WorthQueryConsumerKitCertificationSourceText,
}

pub(super) enum WorthQueryConsumerKitCertificationSourceText {
    StaticSource(&'static str),
}

impl WorthQueryConsumerKitCertificationSourceText {
    pub(super) const fn static_source(source: &'static str) -> Self {
        Self::StaticSource(source)
    }

    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::StaticSource(source) => source,
        }
    }
}

const EVIDENCE_REPORT_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/evidence_report/tests.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/evidence_report/tests.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/evidence_report_compile_fail.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/evidence_report_compile_fail.rs"
        )),
    },
];

const PROHIBITION_REGISTRY_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/prohibition_registry/tests.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/prohibition_registry/tests.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/prohibition_registry_compile_fail.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/prohibition_registry_compile_fail.rs"
        )),
    },
];

const BOUNDARY_AUDIT_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/boundary_audit/tests/detection.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/detection.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/boundary_audit/tests/false_positive.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/false_positive.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/boundary_audit/tests/coverage.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/coverage.rs"
        )),
    },
];

const SUPPORT_SNAPSHOT_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_snapshot/tests/equivalence.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/equivalence.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_snapshot/tests/serialization.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/serialization.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_snapshot/tests/document_load_denial.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/document_load_denial.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/support_snapshot_facade.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/support_snapshot_facade.rs"
        )),
    },
];

const SUPPORT_PINNING_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_pinning/tests/drift_localization.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/drift_localization.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_pinning/tests/evaluation_success.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/evaluation_success.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/support_pinning/tests/rejection.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/rejection.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/support_pinning_facade.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/support_pinning_facade.rs"
        )),
    },
];

const TEST_BACKEND_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/test_backend/workspace_behavior_tests.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/test_backend/workspace_behavior_tests.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/test_backend/support_profile_tests.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/test_backend/support_profile_tests.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/in_memory_test_backend_facade.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/in_memory_test_backend_facade.rs"
        )),
    },
];

const CONSUMER_RESIDUE_AUDIT_SOURCES: &[WorthQueryConsumerKitCertificationSource] = &[
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/src/consumer_kit/consumer_residue/tests.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/consumer_residue/tests.rs"
        )),
    },
    WorthQueryConsumerKitCertificationSource {
        path: "crates/worth-query/tests/consumer_residue_audit.rs",
        source: WorthQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/consumer_residue_audit.rs"
        )),
    },
];

pub(super) fn certification_sources_for_family(
    family: WorthQueryConsumerKitFamilyName,
) -> &'static [WorthQueryConsumerKitCertificationSource] {
    match family {
        WorthQueryConsumerKitFamilyName::EvidenceReportKit => EVIDENCE_REPORT_SOURCES,
        WorthQueryConsumerKitFamilyName::HardProhibitionRegistry => PROHIBITION_REGISTRY_SOURCES,
        WorthQueryConsumerKitFamilyName::BoundaryAudit => BOUNDARY_AUDIT_SOURCES,
        WorthQueryConsumerKitFamilyName::SupportSnapshot => SUPPORT_SNAPSHOT_SOURCES,
        WorthQueryConsumerKitFamilyName::SupportPinning => SUPPORT_PINNING_SOURCES,
        WorthQueryConsumerKitFamilyName::InMemoryTestBackend => TEST_BACKEND_SOURCES,
        WorthQueryConsumerKitFamilyName::ConsumerResidueAudit => CONSUMER_RESIDUE_AUDIT_SOURCES,
    }
}

pub(super) fn certification_source_contains(
    family: WorthQueryConsumerKitFamilyName,
    required_signal: &str,
) -> bool {
    certification_sources_for_family(family)
        .iter()
        .any(|source| source.source.as_str().contains(required_signal))
}

pub(super) fn certification_source_paths_for_family(
    family: WorthQueryConsumerKitFamilyName,
) -> Vec<&'static str> {
    certification_sources_for_family(family)
        .iter()
        .map(|source| source.path)
        .collect()
}

pub(super) fn consumer_kit_family_evidence_digest(
    family: WorthQueryConsumerKitFamilyName,
    evidence_label: &str,
) -> String {
    let sources = certification_sources_for_family(family);
    consumer_kit_embedded_source_identity(
        evidence_label,
        sources.iter().map(|source| source.path),
        sources.iter().map(|source| source.source.as_str()),
    )
    .as_str()
    .to_owned()
}

pub(super) fn consumer_kit_family_certification_gate_certified(
    family: WorthQueryConsumerKitFamilyName,
) -> bool {
    let sources = certification_sources_for_family(family);
    !sources.is_empty()
        && sources
            .iter()
            .all(|source| !source.path.is_empty() && !source.source.as_str().trim().is_empty())
}
