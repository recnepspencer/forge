use super::{
    evaluate_basis_intent_common_path, BasisOperationLaneRequest, BasisScopedAdmissionDenial,
    BasisScopedAdmissionStatus, DeniedBasisCapabilityKind, NormalizedBasisFamily, RawBasisIntent,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::identity::basis_lifecycle_digest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BasisLaneSupportStatus {
    Admitted,
    Advisory,
    Denied,
}

impl BasisLaneSupportStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Advisory => "advisory",
            Self::Denied => "denied",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisLaneSupportRow {
    family: NormalizedBasisFamily,
    operation_lane: BasisOperationLaneRequest,
    status: BasisLaneSupportStatus,
    trace_label: &'static str,
    denial_label: Option<&'static str>,
    support_digest: String,
}

impl BasisLaneSupportRow {
    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn status(&self) -> BasisLaneSupportStatus {
        self.status
    }

    pub fn trace_label(&self) -> &'static str {
        self.trace_label
    }

    pub fn denial_label(&self) -> Option<&'static str> {
        self.denial_label
    }

    pub fn support_digest(&self) -> &str {
        &self.support_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryBasisLifecycleSupportReport {
    rows: Vec<BasisLaneSupportRow>,
    report_digest: String,
}

impl QueryBasisLifecycleSupportReport {
    pub fn rows(&self) -> &[BasisLaneSupportRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn report_identity(&self) -> WorthQueryEvidenceIdentity {
        let row_identities = self
            .rows
            .iter()
            .map(compose_support_row_identity)
            .collect::<Vec<_>>();
        worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
            .field_shape(
                WorthQueryEvidenceTag::new("identity_family"),
                "worth_query_query_basis_lifecycle_support_report_v1",
            )
            .field_evidence_identity_sequence(
                WorthQueryEvidenceTag::new("rows"),
                row_identities.iter(),
            )
            .seal()
    }

    pub fn report_for_reporting(&self) -> String {
        self.report_identity().as_str().to_string()
    }
}

pub fn query_basis_lifecycle_support_report() -> QueryBasisLifecycleSupportReport {
    let mut rows = Vec::new();
    for family in families() {
        for lane in lanes() {
            rows.push(support_row_for(family.clone(), *lane));
        }
    }
    let report_digest = worth_query_evidence_identity(WorthQueryEvidenceScope::BasisDigest)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "query_basis_lifecycle_support_report_v1",
        )
        .field_usize(WorthQueryEvidenceTag::new("row_count"), rows.len())
        .field_value_sequence(
            WorthQueryEvidenceTag::new("support_row"),
            rows.iter().map(|row| row.support_digest()),
        )
        .seal()
        .as_str()
        .to_string();
    QueryBasisLifecycleSupportReport {
        rows,
        report_digest,
    }
}

fn support_row_for(
    family: NormalizedBasisFamily,
    lane: BasisOperationLaneRequest,
) -> BasisLaneSupportRow {
    let intent = raw_intent_for(&family, lane);
    match evaluate_basis_intent_common_path(intent) {
        Ok(fact) => {
            let status = match fact.status() {
                BasisScopedAdmissionStatus::Admitted => BasisLaneSupportStatus::Admitted,
                BasisScopedAdmissionStatus::Advisory => BasisLaneSupportStatus::Advisory,
            };
            let family_label = family.as_str().to_string();
            let lane_label = lane.as_str().to_string();
            BasisLaneSupportRow {
                family,
                operation_lane: lane,
                status,
                trace_label: fact.trace_label(),
                denial_label: None,
                support_digest: basis_lifecycle_digest(
                    "query_basis_lifecycle_support_row_v1",
                    [
                        ("family", family_label),
                        ("lane", lane_label),
                        ("status", status.as_str().to_string()),
                        ("trace", fact.trace_label().to_string()),
                    ],
                ),
            }
        }
        Err(BasisScopedAdmissionDenial::Eligibility(denial)) => BasisLaneSupportRow {
            family,
            operation_lane: lane,
            status: BasisLaneSupportStatus::Denied,
            trace_label: denial.trace().rule_label(),
            denial_label: Some(denial_label(denial.kind())),
            support_digest: basis_lifecycle_digest(
                "query_basis_lifecycle_support_denied_row_v1",
                [
                    ("family", denial.family().as_str().to_string()),
                    ("lane", denial.operation_lane().as_str().to_string()),
                    ("status", "denied".to_string()),
                    ("trace", denial.trace().rule_label().to_string()),
                    ("denial", denial_label(denial.kind()).to_string()),
                ],
            ),
        },
        Err(BasisScopedAdmissionDenial::Intent(denial)) => BasisLaneSupportRow {
            family: family.clone(),
            operation_lane: lane,
            status: BasisLaneSupportStatus::Denied,
            trace_label: "normalization_denied",
            denial_label: Some("basis_intent_denial"),
            support_digest: basis_lifecycle_digest(
                "query_basis_lifecycle_support_intent_denied_row_v1",
                [
                    ("family", family.as_str().to_string()),
                    ("lane", lane.as_str().to_string()),
                    ("status", "denied".to_string()),
                    ("trace", "normalization_denied".to_string()),
                    ("failure", denial.failure_digest().to_string()),
                ],
            ),
        },
    }
}

fn families() -> &'static [NormalizedBasisFamily] {
    &[
        NormalizedBasisFamily::CurrentHead,
        NormalizedBasisFamily::BranchHead,
        NormalizedBasisFamily::BranchSnapshot,
        NormalizedBasisFamily::RuntimeSnapshot,
        NormalizedBasisFamily::HistoricalSnapshot,
        NormalizedBasisFamily::HistoricalCommit,
        NormalizedBasisFamily::Preview,
        NormalizedBasisFamily::PreviewDerivedHistorical,
    ]
}

fn lanes() -> &'static [BasisOperationLaneRequest] {
    &[
        BasisOperationLaneRequest::Observation,
        BasisOperationLaneRequest::MutationPreparation,
        BasisOperationLaneRequest::Replay,
        BasisOperationLaneRequest::Inspection,
        BasisOperationLaneRequest::Materialization,
        BasisOperationLaneRequest::SubscriptionDeclaration,
        BasisOperationLaneRequest::SubscriptionActivation,
        BasisOperationLaneRequest::PreviewCloseout,
        BasisOperationLaneRequest::Certification,
    ]
}

fn raw_intent_for(
    family: &NormalizedBasisFamily,
    lane: BasisOperationLaneRequest,
) -> RawBasisIntent {
    match family {
        NormalizedBasisFamily::CurrentHead => RawBasisIntent::current_head(lane),
        NormalizedBasisFamily::BranchHead => RawBasisIntent::branch_head(branch_identity(), lane),
        NormalizedBasisFamily::BranchSnapshot => {
            RawBasisIntent::branch_snapshot(branch_identity(), snapshot_identity(), lane)
        }
        NormalizedBasisFamily::RuntimeSnapshot => {
            RawBasisIntent::runtime_snapshot(snapshot_identity(), lane)
        }
        NormalizedBasisFamily::HistoricalSnapshot => {
            RawBasisIntent::historical_snapshot(snapshot_identity(), lane)
        }
        NormalizedBasisFamily::HistoricalCommit => {
            RawBasisIntent::historical_commit(commit_identity(), lane)
        }
        NormalizedBasisFamily::Preview => RawBasisIntent::preview(preview_identity(), lane),
        NormalizedBasisFamily::PreviewDerivedHistorical => {
            RawBasisIntent::preview_derived_historical(preview_identity(), lane)
        }
    }
}

fn branch_identity() -> worth_runtime_bridge::facade::BridgeIdentityEvidence {
    worth_runtime_bridge::facade::TruthBranchIdentity::from_relational_branch_id("branch:main")
        .bridge_admission_evidence()
}

fn snapshot_identity() -> worth_runtime_bridge::facade::BridgeIdentityEvidence {
    worth_runtime_bridge::facade::TruthSnapshotIdentity::from_relational_snapshot(
        worth_runtime_bridge::facade::RelationalBridgeSnapshotIdentityParts::new(
            support_fixture_position("snapshot", "snapshot:1"),
            support_fixture_position("snapshot-version", "snapshot:1"),
        ),
    )
    .bridge_admission_evidence()
}

fn commit_identity() -> worth_runtime_bridge::facade::BridgeIdentityEvidence {
    worth_runtime_bridge::facade::TruthCommitIdentity::from_relational_commit_id(
        support_fixture_position("commit", "commit:1"),
    )
    .bridge_admission_evidence()
}

fn preview_identity() -> worth_runtime_bridge::facade::BridgeIdentityEvidence {
    worth_runtime_bridge::facade::BridgePreviewSessionIdentity::from_stable_name(
        "preview:session-1",
    )
    .bridge_admission_evidence()
}

fn support_fixture_position(namespace: &str, evidence: &str) -> u64 {
    let mut acc = 14_695_981_039_346_656_037_u64;
    for byte in namespace.bytes().chain(evidence.bytes()) {
        acc ^= u64::from(byte);
        acc = acc.wrapping_mul(1_099_511_628_211_u64);
    }
    acc
}

fn compose_support_row_identity(row: &BasisLaneSupportRow) -> WorthQueryEvidenceIdentity {
    let mut builder = worth_query_evidence_identity(WorthQueryEvidenceScope::RawBasisIntent)
        .field_shape(
            WorthQueryEvidenceTag::new("identity_family"),
            "worth_query_query_basis_lifecycle_support_row_v1",
        )
        .field_shape(WorthQueryEvidenceTag::new("family"), row.family().as_str())
        .field_shape(
            WorthQueryEvidenceTag::new("operation_lane"),
            row.operation_lane().as_str(),
        )
        .field_shape(WorthQueryEvidenceTag::new("status"), row.status().as_str())
        .field_shape(WorthQueryEvidenceTag::new("trace"), row.trace_label());
    if let Some(denial_label) = row.denial_label() {
        builder = builder.field_shape(WorthQueryEvidenceTag::new("denial"), denial_label);
    }
    builder.seal()
}

fn denial_label(kind: &DeniedBasisCapabilityKind) -> &'static str {
    match kind {
        DeniedBasisCapabilityKind::Stale { .. } => "stale",
        DeniedBasisCapabilityKind::Inaccessible { .. } => "inaccessible",
        DeniedBasisCapabilityKind::PolicyMasked { .. } => "policy_masked",
        DeniedBasisCapabilityKind::TenantMismatched { .. } => "tenant_mismatched",
        DeniedBasisCapabilityKind::SchemaIncompatible { .. } => "schema_incompatible",
        DeniedBasisCapabilityKind::OperationIneligible { .. } => "operation_ineligible",
        DeniedBasisCapabilityKind::LowerRuntimeBindingMissing { .. } => {
            "lower_runtime_binding_missing"
        }
        DeniedBasisCapabilityKind::LowerRuntimeBindingMismatch { .. } => {
            "lower_runtime_binding_mismatch"
        }
        DeniedBasisCapabilityKind::LowerRuntimeCapabilityUnsupported { .. } => {
            "lower_runtime_capability_unsupported"
        }
        DeniedBasisCapabilityKind::HistoricalReplayUnsupported { .. } => {
            "historical_replay_unsupported"
        }
        DeniedBasisCapabilityKind::PreviewDrifted { .. } => "preview_drifted",
        DeniedBasisCapabilityKind::DurableOverclaim { .. } => "durable_overclaim",
    }
}
