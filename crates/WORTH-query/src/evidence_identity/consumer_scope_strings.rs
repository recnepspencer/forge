use super::scope::WorthQueryEvidenceScope;

pub(crate) fn consumer_kit_evidence_scope_as_str(scope: WorthQueryEvidenceScope) -> &'static str {
    match scope {
        WorthQueryEvidenceScope::ConsumerEvidenceReportField => "consumer-evidence-report-field",
        WorthQueryEvidenceScope::ConsumerEvidenceReport => "consumer-evidence-report",
        WorthQueryEvidenceScope::ConsumerEvidenceReportFieldInventory => {
            "consumer-evidence-report-field-inventory"
        }
        WorthQueryEvidenceScope::ConsumerEvidenceReportDigestParticipation => {
            "consumer-evidence-report-digest-participation"
        }
        WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionFinding => {
            "consumer-evidence-report-adoption-finding"
        }
        WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue => {
            "consumer-evidence-report-adoption-residue"
        }
        WorthQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport => {
            "consumer-evidence-report-adoption-report"
        }
        WorthQueryEvidenceScope::ConsumerBoundaryAuditFinding => "consumer-boundary-audit-finding",
        WorthQueryEvidenceScope::ConsumerBoundaryAuditReport => "consumer-boundary-audit-report",
        WorthQueryEvidenceScope::ConsumerBoundaryAuditCoverage => {
            "consumer-boundary-audit-coverage"
        }
        WorthQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory => {
            "consumer-boundary-audit-source-inventory"
        }
        WorthQueryEvidenceScope::ConsumerSupportSnapshotSchema => {
            "consumer-support-snapshot-schema"
        }
        WorthQueryEvidenceScope::ConsumerSupportSnapshotRow => "consumer-support-snapshot-row",
        WorthQueryEvidenceScope::ConsumerSupportSnapshotDocument => {
            "consumer-support-snapshot-document"
        }
        WorthQueryEvidenceScope::ConsumerSupportPinContractSchema => {
            "consumer-support-pin-contract-schema"
        }
        WorthQueryEvidenceScope::ConsumerSupportPinVocabulary => "consumer-support-pin-vocabulary",
        WorthQueryEvidenceScope::ConsumerSupportPinRequirement => {
            "consumer-support-pin-requirement"
        }
        WorthQueryEvidenceScope::ConsumerSupportPinObservedRow => {
            "consumer-support-pin-observed-row"
        }
        WorthQueryEvidenceScope::ConsumerSupportPinContract => "consumer-support-pin-contract",
        WorthQueryEvidenceScope::ConsumerSupportPinContractDocument => {
            "consumer-support-pin-contract-document"
        }
        WorthQueryEvidenceScope::ConsumerSupportPinFinding => "consumer-support-pin-finding",
        WorthQueryEvidenceScope::ConsumerSupportPinReport => "consumer-support-pin-report",
        WorthQueryEvidenceScope::ConsumerResidueFinding => "consumer-residue-finding",
        WorthQueryEvidenceScope::ConsumerResidueReport => "consumer-residue-report",
        WorthQueryEvidenceScope::ConsumerTestBackendResidueFinding => {
            "consumer-test-backend-residue-finding"
        }
        WorthQueryEvidenceScope::ConsumerTestBackendResidueReport => {
            "consumer-test-backend-residue-report"
        }
        WorthQueryEvidenceScope::ConsumerGraphReadBypassFinding => {
            "consumer-graph-read-bypass-finding"
        }
        WorthQueryEvidenceScope::ConsumerGraphReadBypassReport => {
            "consumer-graph-read-bypass-report"
        }
        WorthQueryEvidenceScope::ConsumerGraphReadBypassResidue => {
            "consumer-graph-read-bypass-residue"
        }
        _ => unreachable!("consumer kit scope helper called with non-consumer scope"),
    }
}
