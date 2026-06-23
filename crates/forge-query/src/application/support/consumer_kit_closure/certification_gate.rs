use crate::ForgeQueryExternalSupportPinContractTerminalJsonDocument;

use super::evidence::consumer_kit_embedded_source_identity;
use super::family::ForgeQueryConsumerKitFamilyName;

pub(super) struct ForgeQueryConsumerKitCertificationSource {
    pub(super) path: &'static str,
    pub(super) source: ForgeQueryConsumerKitCertificationSourceText,
}

pub(super) enum ForgeQueryConsumerKitCertificationSourceText {
    StaticSource(&'static str),
    ExternalSupportPinContractTerminalJsonDocument(
        ForgeQueryExternalSupportPinContractTerminalJsonDocument,
    ),
}

impl ForgeQueryConsumerKitCertificationSourceText {
    pub(super) const fn static_source(source: &'static str) -> Self {
        Self::StaticSource(source)
    }

    pub(super) const fn external_support_pin_contract_terminal_json_document(
        source: &'static str,
    ) -> Self {
        Self::ExternalSupportPinContractTerminalJsonDocument(
            ForgeQueryExternalSupportPinContractTerminalJsonDocument::from_static_external_terminal_json_document(source),
        )
    }

    pub(super) fn as_str(&self) -> &str {
        match self {
            Self::StaticSource(source) => source,
            Self::ExternalSupportPinContractTerminalJsonDocument(document) => document.as_str(),
        }
    }
}

const EVIDENCE_REPORT_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/evidence_report/tests.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/evidence_report/tests.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/evidence_report_compile_fail.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/evidence_report_compile_fail.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/tests/query_evidence_report_adoption.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../../worth-kernel/src/construction/tests/query_evidence_report_adoption.rs"
        )),
    },
];

const PROHIBITION_REGISTRY_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/prohibition_registry/tests.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/prohibition_registry/tests.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/prohibition_registry_compile_fail.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/prohibition_registry_compile_fail.rs"
        )),
    },
];

const BOUNDARY_AUDIT_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/boundary_audit/tests/detection.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/detection.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/boundary_audit/tests/false_positive.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/false_positive.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/boundary_audit/tests/coverage.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/boundary_audit/tests/coverage.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/hard_prohibition_boundary_audit.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/hard_prohibition_boundary_audit.rs"
        )),
    },
];

const SUPPORT_SNAPSHOT_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_snapshot/tests/equivalence.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/equivalence.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_snapshot/tests/serialization.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/serialization.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_snapshot/tests/document_load_denial.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_snapshot/tests/document_load_denial.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/support_snapshot_facade.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/support_snapshot_facade.rs"
        )),
    },
];

const SUPPORT_PINNING_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_pinning/tests/drift_localization.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/drift_localization.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_pinning/tests/evaluation_success.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/evaluation_success.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/support_pinning/tests/rejection.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/support_pinning/tests/rejection.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/support_pinning_facade.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/support_pinning_facade.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/tests/support_pinning_adoption.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../../worth-kernel/src/construction/tests/support_pinning_adoption.rs"
        )),
    },
];

const TEST_BACKEND_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/test_backend/workspace_behavior_tests.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/test_backend/workspace_behavior_tests.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/test_backend/support_profile_tests.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/test_backend/support_profile_tests.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/in_memory_test_backend_residue_audit.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/in_memory_test_backend_residue_audit.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/in_memory_test_backend_facade.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/in_memory_test_backend_facade.rs"
        )),
    },
];

const CONSUMER_RESIDUE_AUDIT_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/src/consumer_kit/consumer_residue/tests.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../consumer_kit/consumer_residue/tests.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/consumer_residue_audit.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/consumer_residue_audit.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/forge-query/tests/consumer_residue_reference_adoption.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../tests/consumer_residue_reference_adoption.rs"
        )),
    },
];

const REFERENCE_CONSUMER_SOURCES: &[ForgeQueryConsumerKitCertificationSource] = &[
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/tests/query_enforcement_adoption.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../../worth-kernel/src/construction/tests/query_enforcement_adoption.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/query_enforcement_adoption/adoption_inventory.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../../worth-kernel/src/construction/query_enforcement_adoption/adoption_inventory.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/query_enforcement_adoption/residue_assertions.rs",
        source: ForgeQueryConsumerKitCertificationSourceText::static_source(include_str!(
            "../../../../../worth-kernel/src/construction/query_enforcement_adoption/residue_assertions.rs"
        )),
    },
    ForgeQueryConsumerKitCertificationSource {
        path: "crates/worth-kernel/src/construction/query_support_pins.json",
        source: ForgeQueryConsumerKitCertificationSourceText::external_support_pin_contract_terminal_json_document(
            include_str!("../../../../../worth-kernel/src/construction/query_support_pins.json"),
        ),
    },
];

pub(super) fn certification_sources_for_family(
    family: ForgeQueryConsumerKitFamilyName,
) -> &'static [ForgeQueryConsumerKitCertificationSource] {
    match family {
        ForgeQueryConsumerKitFamilyName::EvidenceReportKit => EVIDENCE_REPORT_SOURCES,
        ForgeQueryConsumerKitFamilyName::HardProhibitionRegistry => PROHIBITION_REGISTRY_SOURCES,
        ForgeQueryConsumerKitFamilyName::BoundaryAudit => BOUNDARY_AUDIT_SOURCES,
        ForgeQueryConsumerKitFamilyName::SupportSnapshot => SUPPORT_SNAPSHOT_SOURCES,
        ForgeQueryConsumerKitFamilyName::SupportPinning => SUPPORT_PINNING_SOURCES,
        ForgeQueryConsumerKitFamilyName::InMemoryTestBackend => TEST_BACKEND_SOURCES,
        ForgeQueryConsumerKitFamilyName::ConsumerResidueAudit => CONSUMER_RESIDUE_AUDIT_SOURCES,
        ForgeQueryConsumerKitFamilyName::ReferenceConsumerAdoption => REFERENCE_CONSUMER_SOURCES,
    }
}

pub(super) fn certification_source_contains(
    family: ForgeQueryConsumerKitFamilyName,
    required_signal: &str,
) -> bool {
    certification_sources_for_family(family)
        .iter()
        .any(|source| source.source.as_str().contains(required_signal))
}

pub(super) fn certification_source_paths_for_family(
    family: ForgeQueryConsumerKitFamilyName,
) -> Vec<&'static str> {
    certification_sources_for_family(family)
        .iter()
        .map(|source| source.path)
        .collect()
}

pub(super) fn consumer_kit_family_evidence_digest(
    family: ForgeQueryConsumerKitFamilyName,
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
    family: ForgeQueryConsumerKitFamilyName,
) -> bool {
    let sources = certification_sources_for_family(family);
    !sources.is_empty()
        && sources
            .iter()
            .all(|source| !source.path.is_empty() && !source.source.as_str().trim().is_empty())
}
