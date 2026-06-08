use super::{
    evaluate_basis_intent_common_path, BasisOperationLaneRequest, BasisScopedAdmissionDenial,
    BasisScopedAdmissionStatus, DeniedBasisCapabilityKind, NormalizedBasisFamily, RawBasisIntent,
};
use crate::identity::hash_parts;

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
}

pub fn query_basis_lifecycle_support_report() -> QueryBasisLifecycleSupportReport {
    let mut rows = Vec::new();
    for family in families() {
        for lane in lanes() {
            rows.push(support_row_for(family.clone(), *lane));
        }
    }
    let mut parts = vec![
        "forge_query_query_basis_lifecycle_support_report_v1".to_string(),
        format!("row_count:{}", rows.len()),
    ];
    parts.extend(rows.iter().map(|row| row.support_digest().to_string()));
    let report_digest = hash_parts(&parts);
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
                support_digest: hash_parts(&[
                    format!("family:{family_label}"),
                    format!("lane:{lane_label}"),
                    format!("status:{}", status.as_str()),
                    format!("trace:{}", fact.trace_label()),
                ]),
            }
        }
        Err(BasisScopedAdmissionDenial::Eligibility(denial)) => BasisLaneSupportRow {
            family,
            operation_lane: lane,
            status: BasisLaneSupportStatus::Denied,
            trace_label: denial.trace().rule_label(),
            denial_label: Some(denial_label(denial.kind())),
            support_digest: hash_parts(&[
                format!("family:{}", denial.family().as_str()),
                format!("lane:{}", denial.operation_lane().as_str()),
                "status:denied".to_string(),
                format!("trace:{}", denial.trace().rule_label()),
                format!("denial:{}", denial_label(denial.kind())),
            ]),
        },
        Err(BasisScopedAdmissionDenial::Intent(denial)) => BasisLaneSupportRow {
            family: family.clone(),
            operation_lane: lane,
            status: BasisLaneSupportStatus::Denied,
            trace_label: "normalization_denied",
            denial_label: Some("basis_intent_denial"),
            support_digest: hash_parts(&[
                format!("family:{}", family.as_str()),
                format!("lane:{}", lane.as_str()),
                "status:denied".to_string(),
                "trace:normalization_denied".to_string(),
                format!("failure:{}", denial.failure_digest()),
            ]),
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
        NormalizedBasisFamily::BranchHead => RawBasisIntent::branch_head("branch:main", lane),
        NormalizedBasisFamily::BranchSnapshot => {
            RawBasisIntent::branch_snapshot("branch:main", "snapshot:1", lane)
        }
        NormalizedBasisFamily::RuntimeSnapshot => {
            RawBasisIntent::runtime_snapshot("runtime:snapshot:1", lane)
        }
        NormalizedBasisFamily::HistoricalSnapshot => {
            RawBasisIntent::historical_snapshot("history:snapshot:1", lane)
        }
        NormalizedBasisFamily::HistoricalCommit => {
            RawBasisIntent::historical_commit("commit:1", lane)
        }
        NormalizedBasisFamily::Preview => RawBasisIntent::preview("preview:session-1", lane),
        NormalizedBasisFamily::PreviewDerivedHistorical => {
            RawBasisIntent::preview_derived_historical("preview:session-1", lane)
        }
    }
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
