use super::scope::ForgeQueryEvidenceScope;

pub(crate) fn consumer_kit_evidence_scope_as_str(scope: ForgeQueryEvidenceScope) -> &'static str {
    match scope {
        ForgeQueryEvidenceScope::ConsumerEvidenceReportField => "consumer-evidence-report-field",
        ForgeQueryEvidenceScope::ConsumerEvidenceReport => "consumer-evidence-report",
        ForgeQueryEvidenceScope::ConsumerEvidenceReportFieldInventory => {
            "consumer-evidence-report-field-inventory"
        }
        ForgeQueryEvidenceScope::ConsumerEvidenceReportDigestParticipation => {
            "consumer-evidence-report-digest-participation"
        }
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionFinding => {
            "consumer-evidence-report-adoption-finding"
        }
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionResidue => {
            "consumer-evidence-report-adoption-residue"
        }
        ForgeQueryEvidenceScope::ConsumerEvidenceReportAdoptionReport => {
            "consumer-evidence-report-adoption-report"
        }
        ForgeQueryEvidenceScope::ConsumerBoundaryAuditFinding => "consumer-boundary-audit-finding",
        ForgeQueryEvidenceScope::ConsumerBoundaryAuditReport => "consumer-boundary-audit-report",
        ForgeQueryEvidenceScope::ConsumerBoundaryAuditCoverage => {
            "consumer-boundary-audit-coverage"
        }
        ForgeQueryEvidenceScope::ConsumerBoundaryAuditSourceInventory => {
            "consumer-boundary-audit-source-inventory"
        }
        ForgeQueryEvidenceScope::ConsumerSupportSnapshotSchema => {
            "consumer-support-snapshot-schema"
        }
        ForgeQueryEvidenceScope::ConsumerSupportSnapshotRow => "consumer-support-snapshot-row",
        ForgeQueryEvidenceScope::ConsumerSupportSnapshotDocument => {
            "consumer-support-snapshot-document"
        }
        ForgeQueryEvidenceScope::ConsumerSupportPinContractSchema => {
            "consumer-support-pin-contract-schema"
        }
        ForgeQueryEvidenceScope::ConsumerSupportPinVocabulary => "consumer-support-pin-vocabulary",
        ForgeQueryEvidenceScope::ConsumerSupportPinRequirement => {
            "consumer-support-pin-requirement"
        }
        ForgeQueryEvidenceScope::ConsumerSupportPinObservedRow => {
            "consumer-support-pin-observed-row"
        }
        ForgeQueryEvidenceScope::ConsumerSupportPinContract => "consumer-support-pin-contract",
        ForgeQueryEvidenceScope::ConsumerSupportPinContractDocument => {
            "consumer-support-pin-contract-document"
        }
        ForgeQueryEvidenceScope::ConsumerSupportPinFinding => "consumer-support-pin-finding",
        ForgeQueryEvidenceScope::ConsumerSupportPinReport => "consumer-support-pin-report",
        ForgeQueryEvidenceScope::ConsumerTestBackendResidueFinding => {
            "consumer-test-backend-residue-finding"
        }
        ForgeQueryEvidenceScope::ConsumerTestBackendResidueReport => {
            "consumer-test-backend-residue-report"
        }
        _ => unreachable!("consumer kit scope helper called with non-consumer scope"),
    }
}
