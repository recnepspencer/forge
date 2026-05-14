use crate::identity::hash_parts;

use super::{
    BasisEligibility, BasisEligibilityCounters, BasisEligibilityDisposition, BasisEligibilityTrace,
    BasisOperationLaneRequest, DeniedBasisCapability, DeniedBasisCapabilityKind,
    NormalizedBasisFamily, NormalizedBasisIntent,
};

pub fn evaluate_basis_eligibility(
    intent: NormalizedBasisIntent,
) -> Result<BasisEligibility, DeniedBasisCapability> {
    match classify_eligibility(&intent) {
        EligibilityEvaluation::Admitted {
            disposition,
            rule_label,
            explanation,
        } => Ok(BasisEligibility {
            normalized_basis_intent_digest: intent.canonical_digest().to_string(),
            family: intent.family().clone(),
            authority_posture: intent.authority_posture().clone(),
            normalized_label: intent.normalized_label().to_string(),
            operation_lane: intent.operation_lane().clone(),
            tenant_schema_posture: intent.tenant_schema_posture().clone(),
            disposition,
            trace: BasisEligibilityTrace {
                rule_label,
                explanation,
            },
            counters: BasisEligibilityCounters::for_intent(&intent, 0),
            eligibility_digest: hash_parts(&[
                format!(
                    "normalized_basis_intent_digest:{}",
                    intent.canonical_digest()
                ),
                format!("family:{}", intent.family().as_str()),
                format!("operation_lane:{}", intent.operation_lane().as_str()),
                format!(
                    "disposition:{}",
                    classify_eligibility(&intent).disposition_label()
                ),
            ]),
        }),
        EligibilityEvaluation::Denied {
            kind,
            rule_label,
            explanation,
        } => Err(DeniedBasisCapability {
            normalized_basis_intent_digest: intent.canonical_digest().to_string(),
            family: intent.family().clone(),
            operation_lane: intent.operation_lane().clone(),
            kind,
            trace: BasisEligibilityTrace {
                rule_label,
                explanation,
            },
            counters: BasisEligibilityCounters::for_intent(&intent, 1),
            failure_digest: hash_parts(&[
                format!(
                    "normalized_basis_intent_digest:{}",
                    intent.canonical_digest()
                ),
                format!("family:{}", intent.family().as_str()),
                format!("operation_lane:{}", intent.operation_lane().as_str()),
                format!(
                    "failure:{}",
                    classify_eligibility(&intent).disposition_label()
                ),
            ]),
        }),
    }
}

enum EligibilityEvaluation {
    Admitted {
        disposition: BasisEligibilityDisposition,
        rule_label: &'static str,
        explanation: &'static str,
    },
    Denied {
        kind: DeniedBasisCapabilityKind,
        rule_label: &'static str,
        explanation: &'static str,
    },
}

impl EligibilityEvaluation {
    fn disposition_label(&self) -> &'static str {
        match self {
            Self::Admitted { disposition, .. } => disposition.as_str(),
            Self::Denied { kind, .. } => match kind {
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
            },
        }
    }
}

fn classify_eligibility(intent: &NormalizedBasisIntent) -> EligibilityEvaluation {
    if let Some(policy_scope) = intent.policy_scope() {
        if policy_scope.contains("masked") {
            return EligibilityEvaluation::Denied {
                kind: DeniedBasisCapabilityKind::PolicyMasked {
                    policy_scope: policy_scope.to_string(),
                },
                rule_label: "policy_scope_masked",
                explanation:
                    "policy-masked basis requests deny during eligibility before runtime lowering",
            };
        }
        if policy_scope.contains("durable_overclaim") {
            return EligibilityEvaluation::Denied {
                kind: DeniedBasisCapabilityKind::DurableOverclaim {
                    family: intent.family().clone(),
                    operation_lane: intent.operation_lane().clone(),
                },
                rule_label: "durable_overclaim_denied",
                explanation: "durable-overclaim basis requests must stop at eligibility",
            };
        }
    }
    if let Some(tenant_scope) = intent.tenant_scope() {
        if tenant_scope.contains("mismatch") {
            return EligibilityEvaluation::Denied {
                kind: DeniedBasisCapabilityKind::TenantMismatched {
                    tenant_scope: tenant_scope.to_string(),
                },
                rule_label: "tenant_scope_mismatched",
                explanation: "tenant-mismatched basis requests deny during eligibility",
            };
        }
    }
    if let Some(schema_scope) = intent.schema_scope() {
        if schema_scope.contains("incompatible") {
            return EligibilityEvaluation::Denied {
                kind: DeniedBasisCapabilityKind::SchemaIncompatible {
                    schema_scope: schema_scope.to_string(),
                },
                rule_label: "schema_scope_incompatible",
                explanation: "schema-incompatible basis requests deny during eligibility",
            };
        }
    }
    if intent.normalized_label().contains("inaccessible") {
        return EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::Inaccessible {
                family: intent.family().clone(),
                authority: "query_basis_lifecycle",
            },
            rule_label: "basis_authority_inaccessible",
            explanation:
                "inaccessible basis requests deny before read, replay, or materialization entry",
        };
    }
    if intent.normalized_label().contains("stale") {
        return EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::Stale {
                family: intent.family().clone(),
            },
            rule_label: "basis_state_stale",
            explanation:
                "stale basis requests deny during eligibility before lower-runtime work begins",
        };
    }
    if intent.normalized_label().contains("missing_binding") {
        return EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::LowerRuntimeBindingMissing {
                authority: "forge_runtime_bridge::facade",
                family: intent.family().clone(),
                operation_lane: intent.operation_lane().clone(),
            },
            rule_label: "lower_runtime_binding_missing",
            explanation: "missing lower-runtime binding denies during eligibility rather than during receipt construction",
        };
    }

    match (intent.family(), intent.operation_lane()) {
        (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::SubscriptionDeclaration)
        | (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::SubscriptionActivation)
        | (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::Certification)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::SubscriptionDeclaration)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::SubscriptionActivation)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::Certification)
        | (NormalizedBasisFamily::BranchSnapshot, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::BranchSnapshot, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::BranchSnapshot, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::BranchSnapshot, BasisOperationLaneRequest::Certification)
        | (NormalizedBasisFamily::RuntimeSnapshot, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::RuntimeSnapshot, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::RuntimeSnapshot, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::RuntimeSnapshot, BasisOperationLaneRequest::Certification)
        | (NormalizedBasisFamily::HistoricalSnapshot, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::HistoricalSnapshot, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::HistoricalSnapshot, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::HistoricalSnapshot, BasisOperationLaneRequest::Replay)
        | (NormalizedBasisFamily::HistoricalSnapshot, BasisOperationLaneRequest::Certification)
        | (NormalizedBasisFamily::HistoricalCommit, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::HistoricalCommit, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::HistoricalCommit, BasisOperationLaneRequest::Materialization)
        | (NormalizedBasisFamily::HistoricalCommit, BasisOperationLaneRequest::Replay)
        | (NormalizedBasisFamily::HistoricalCommit, BasisOperationLaneRequest::Certification) => {
            EligibilityEvaluation::Admitted {
                disposition: BasisEligibilityDisposition::Success,
                rule_label: "runtime_backed_basis_lane_admitted",
                explanation: "normalized runtime-backed or historical basis is eligible for the requested lane",
            }
        }
        (NormalizedBasisFamily::CurrentHead, BasisOperationLaneRequest::MutationPreparation)
        | (NormalizedBasisFamily::BranchHead, BasisOperationLaneRequest::MutationPreparation) => {
            EligibilityEvaluation::Admitted {
                disposition: BasisEligibilityDisposition::Success,
                rule_label: "mutable_runtime_basis_lane_admitted",
                explanation: "mutable runtime-backed basis is eligible for mutation preparation",
            }
        }
        (NormalizedBasisFamily::Preview, BasisOperationLaneRequest::Observation)
        | (NormalizedBasisFamily::Preview, BasisOperationLaneRequest::Inspection)
        | (NormalizedBasisFamily::Preview, BasisOperationLaneRequest::PreviewCloseout)
        | (
            NormalizedBasisFamily::PreviewDerivedHistorical,
            BasisOperationLaneRequest::Observation,
        )
        | (
            NormalizedBasisFamily::PreviewDerivedHistorical,
            BasisOperationLaneRequest::Inspection,
        )
        | (
            NormalizedBasisFamily::PreviewDerivedHistorical,
            BasisOperationLaneRequest::Certification,
        ) => EligibilityEvaluation::Admitted {
            disposition: BasisEligibilityDisposition::Advisory,
            rule_label: "preview_basis_lane_advisory",
            explanation: "preview-backed basis remains advisory and cannot silently promote into authoritative execution lanes",
        },
        (
            NormalizedBasisFamily::Preview | NormalizedBasisFamily::PreviewDerivedHistorical,
            BasisOperationLaneRequest::MutationPreparation
                | BasisOperationLaneRequest::Materialization
                | BasisOperationLaneRequest::SubscriptionDeclaration
                | BasisOperationLaneRequest::SubscriptionActivation
                | BasisOperationLaneRequest::PreviewCloseout
                | BasisOperationLaneRequest::Replay,
        ) => EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::PreviewDrifted {
                family: intent.family().clone(),
            },
            rule_label: "preview_authority_lane_denied",
            explanation: "preview-backed basis cannot enter authoritative or replay lanes during phase 2 admission",
        },
        (
            NormalizedBasisFamily::CurrentHead
            | NormalizedBasisFamily::BranchHead
            | NormalizedBasisFamily::BranchSnapshot
            | NormalizedBasisFamily::RuntimeSnapshot,
            BasisOperationLaneRequest::Replay,
        ) => EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::HistoricalReplayUnsupported {
                family: intent.family().clone(),
            },
            rule_label: "runtime_basis_replay_denied",
            explanation: "runtime-backed basis cannot claim historical replay eligibility without historical authority",
        },
        _ => EligibilityEvaluation::Denied {
            kind: DeniedBasisCapabilityKind::OperationIneligible {
                family: intent.family().clone(),
                operation_lane: intent.operation_lane().clone(),
            },
            rule_label: "operation_lane_not_admitted",
            explanation: "normalized basis family is not eligible for the requested operation lane",
        },
    }
}
