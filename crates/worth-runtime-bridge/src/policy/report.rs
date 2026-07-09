use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::facade::BridgeRequestKind;
use crate::routing::canonicalization::digest_string;

use super::{
    AdmittedBridgePolicyContract, BridgePolicyProvenanceEntry, BridgePolicyProvenanceRecord,
    BridgePolicyReplayBundle, LoweredBridgeExecutionPolicy,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyProvenanceReportRow {
    label: Arc<str>,
    request_kind: BridgeRequestKind,
    execution_class: super::BridgeExecutionPolicyClass,
    diagnostics_tier: super::BridgeDiagnosticsTier,
    route_artifacts: bool,
    replay_artifacts: bool,
    policy_digest: Arc<str>,
    semantic_policy_digest: Arc<str>,
    lowered_policy_digest: Arc<str>,
    provenance_digest: Arc<str>,
    replay_digest: Arc<str>,
    provenance_entries: Arc<[BridgePolicyProvenanceEntry]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyProvenanceReportRow {
    pub fn from_policy_bundle(
        label: impl Into<Arc<str>>,
        contract: &AdmittedBridgePolicyContract,
        lowered: &LoweredBridgeExecutionPolicy,
        provenance: &BridgePolicyProvenanceRecord,
        replay_bundle: &BridgePolicyReplayBundle,
    ) -> Self {
        let label = label.into();
        let provenance_entries =
            Arc::<[BridgePolicyProvenanceEntry]>::from(provenance.entries().to_vec());
        let semantic_policy_digest = Arc::<str>::from(
            semantic_policy_digest_from_parts(contract, lowered, provenance_entries.as_ref())
                .to_string(),
        );
        let canonical_basis = Arc::<str>::from(format!(
            concat!(
                "bridge-policy-provenance-report-row|label={}|request-kind:{:?}|execution:{:?}|",
                "diagnostics:{:?}|route-artifacts:{}|replay-artifacts:{}|policy-digest:{}|",
                "semantic-policy-digest:{}|lowered-policy-digest:{}|provenance-digest:{}|",
                "replay-digest:{}|entry-count:{}"
            ),
            label.as_ref(),
            contract
                .validated_declaration()
                .declaration()
                .request_kind(),
            lowered.execution_class(),
            lowered.diagnostics_tier(),
            lowered.route_artifacts(),
            lowered.replay_artifacts(),
            contract.digest(),
            semantic_policy_digest.as_ref(),
            lowered.digest(),
            provenance.digest(),
            replay_bundle.digest(),
            provenance_entries.len(),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            label,
            request_kind: contract
                .validated_declaration()
                .declaration()
                .request_kind(),
            execution_class: lowered.execution_class(),
            diagnostics_tier: lowered.diagnostics_tier(),
            route_artifacts: lowered.route_artifacts(),
            replay_artifacts: lowered.replay_artifacts(),
            policy_digest: Arc::from(contract.digest().to_owned()),
            semantic_policy_digest,
            lowered_policy_digest: Arc::from(lowered.digest().to_owned()),
            provenance_digest: Arc::from(provenance.digest().to_owned()),
            replay_digest: Arc::from(replay_bundle.digest().to_owned()),
            provenance_entries,
            canonical_basis,
            digest: Arc::from(format!(
                "bridge-policy-provenance-report-row:sha256:{digest:x}"
            )),
        }
    }

    pub fn label(&self) -> &str {
        self.label.as_ref()
    }
    pub fn request_kind(&self) -> BridgeRequestKind {
        self.request_kind
    }
    pub fn execution_class(&self) -> super::BridgeExecutionPolicyClass {
        self.execution_class
    }
    pub fn diagnostics_tier(&self) -> super::BridgeDiagnosticsTier {
        self.diagnostics_tier
    }
    pub fn route_artifacts(&self) -> bool {
        self.route_artifacts
    }
    pub fn replay_artifacts(&self) -> bool {
        self.replay_artifacts
    }
    pub fn policy_digest(&self) -> &str {
        self.policy_digest.as_ref()
    }
    pub fn semantic_policy_digest(&self) -> &str {
        self.semantic_policy_digest.as_ref()
    }
    pub fn lowered_policy_digest(&self) -> &str {
        self.lowered_policy_digest.as_ref()
    }
    pub fn provenance_digest(&self) -> &str {
        self.provenance_digest.as_ref()
    }
    pub fn replay_digest(&self) -> &str {
        self.replay_digest.as_ref()
    }
    pub fn provenance_entries(&self) -> &[BridgePolicyProvenanceEntry] {
        &self.provenance_entries
    }
    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgePolicyProvenanceReport {
    rows: Arc<[BridgePolicyProvenanceReportRow]>,
    canonical_basis: Arc<str>,
    digest: Arc<str>,
}

impl BridgePolicyProvenanceReport {
    pub fn new(rows: Vec<BridgePolicyProvenanceReportRow>) -> Self {
        let rows = Arc::<[BridgePolicyProvenanceReportRow]>::from(rows);
        let canonical_basis = Arc::<str>::from(format!(
            "bridge-policy-provenance-report|row-count:{}|row-digests:{}",
            rows.len(),
            rows.iter()
                .map(BridgePolicyProvenanceReportRow::digest)
                .collect::<Vec<_>>()
                .join("|"),
        ));
        let digest = Sha256::digest(canonical_basis.as_bytes());
        Self {
            rows,
            canonical_basis,
            digest: Arc::from(format!("bridge-policy-provenance-report:sha256:{digest:x}")),
        }
    }

    pub fn rows(&self) -> &[BridgePolicyProvenanceReportRow] {
        &self.rows
    }
    pub fn canonical_basis(&self) -> &str {
        self.canonical_basis.as_ref()
    }
    pub fn digest(&self) -> &str {
        self.digest.as_ref()
    }
}

fn semantic_policy_digest_from_parts(
    contract: &AdmittedBridgePolicyContract,
    lowered: &LoweredBridgeExecutionPolicy,
    entries: &[BridgePolicyProvenanceEntry],
) -> Arc<str> {
    let mut parts = vec![
        format!(
            "request-kind:{:?}",
            contract
                .validated_declaration()
                .declaration()
                .request_kind()
        ),
        format!("execution:{:?}", lowered.execution_class()),
        format!("diagnostics:{:?}", lowered.diagnostics_tier()),
        format!("route-artifacts:{}", lowered.route_artifacts()),
        format!("replay-artifacts:{}", lowered.replay_artifacts()),
    ];
    for entry in entries {
        parts.push(format!(
            "entry:{:?}|{:?}|{:?}|{:?}",
            entry.field_kind(),
            entry.declared_source(),
            entry.operative_source(),
            entry.resolution(),
        ));
    }
    Arc::from(digest_string("policy-semantic-row", &parts.join("|")).to_string())
}
