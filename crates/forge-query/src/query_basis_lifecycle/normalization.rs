use crate::identity::hash_parts;

use super::intent::{
    BasisOperationLaneRequest, RawBasisIntent, RawBasisSelector, RawBasisSourcePath,
    RawFutureBasisNeighborFamily,
};
use super::{
    BasisAuthorityPosture, BasisNormalizationCounters, BasisTenantSchemaPosture,
    NormalizedBasisFamily, NormalizedBasisSubject,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BasisIntentDenialKind {
    MalformedIdentifier {
        field: &'static str,
    },
    UnsupportedCompatibilityFamily {
        family: &'static str,
        owner: &'static str,
    },
    UnsupportedFutureNeighbor {
        family: RawFutureBasisNeighborFamily,
        owner: &'static str,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasisIntentDenial {
    raw_basis_intent_digest: String,
    source_path: RawBasisSourcePath,
    operation_lane: BasisOperationLaneRequest,
    kind: BasisIntentDenialKind,
    counters: BasisNormalizationCounters,
    failure_digest: String,
}

impl BasisIntentDenial {
    pub fn raw_basis_intent_digest(&self) -> &str {
        &self.raw_basis_intent_digest
    }

    pub fn source_path(&self) -> &RawBasisSourcePath {
        &self.source_path
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn kind(&self) -> &BasisIntentDenialKind {
        &self.kind
    }

    pub fn counters(&self) -> &BasisNormalizationCounters {
        &self.counters
    }

    pub fn failure_digest(&self) -> &str {
        &self.failure_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedBasisIntent {
    raw_basis_intent_digest: String,
    canonical_digest: String,
    family: NormalizedBasisFamily,
    authority_posture: BasisAuthorityPosture,
    operation_lane: BasisOperationLaneRequest,
    tenant_scope: Option<String>,
    policy_scope: Option<String>,
    schema_scope: Option<String>,
    tenant_schema_posture: BasisTenantSchemaPosture,
    source_path: RawBasisSourcePath,
    normalized_subject: NormalizedBasisSubject,
    normalized_label: String,
    counters: BasisNormalizationCounters,
}

impl NormalizedBasisIntent {
    pub fn raw_basis_intent_digest(&self) -> &str {
        &self.raw_basis_intent_digest
    }

    pub fn canonical_digest(&self) -> &str {
        &self.canonical_digest
    }

    pub fn family(&self) -> &NormalizedBasisFamily {
        &self.family
    }

    pub fn authority_posture(&self) -> &BasisAuthorityPosture {
        &self.authority_posture
    }

    pub fn operation_lane(&self) -> &BasisOperationLaneRequest {
        &self.operation_lane
    }

    pub fn tenant_scope(&self) -> Option<&str> {
        self.tenant_scope.as_deref()
    }

    pub fn policy_scope(&self) -> Option<&str> {
        self.policy_scope.as_deref()
    }

    pub fn schema_scope(&self) -> Option<&str> {
        self.schema_scope.as_deref()
    }

    pub fn tenant_schema_posture(&self) -> &BasisTenantSchemaPosture {
        &self.tenant_schema_posture
    }

    pub fn source_path(&self) -> &RawBasisSourcePath {
        &self.source_path
    }

    pub fn normalized_subject(&self) -> &NormalizedBasisSubject {
        &self.normalized_subject
    }

    pub fn normalized_label(&self) -> &str {
        &self.normalized_label
    }

    pub fn counters(&self) -> &BasisNormalizationCounters {
        &self.counters
    }
}

pub fn normalize_raw_basis(
    intent: RawBasisIntent,
) -> Result<NormalizedBasisIntent, BasisIntentDenial> {
    validate_optional_scope(
        intent.raw_digest(),
        intent.source_path(),
        intent.operation_lane(),
        "tenant_scope",
        intent.tenant_scope(),
    )?;
    validate_optional_scope(
        intent.raw_digest(),
        intent.source_path(),
        intent.operation_lane(),
        "policy_scope",
        intent.policy_scope(),
    )?;
    validate_optional_scope(
        intent.raw_digest(),
        intent.source_path(),
        intent.operation_lane(),
        "schema_scope",
        intent.schema_scope(),
    )?;

    let tenant_schema_posture = match (
        intent.tenant_scope(),
        intent.policy_scope(),
        intent.schema_scope(),
    ) {
        (None, None, None) => BasisTenantSchemaPosture::Unscoped,
        (Some(_), None, None) => BasisTenantSchemaPosture::TenantScoped,
        (None, Some(_), None) => BasisTenantSchemaPosture::PolicyScoped,
        (None, None, Some(_)) => BasisTenantSchemaPosture::SchemaScoped,
        (Some(_), Some(_), None) => BasisTenantSchemaPosture::TenantAndPolicyScoped,
        (Some(_), None, Some(_)) => BasisTenantSchemaPosture::TenantAndSchemaScoped,
        (None, Some(_), Some(_)) => BasisTenantSchemaPosture::PolicyAndSchemaScoped,
        (Some(_), Some(_), Some(_)) => BasisTenantSchemaPosture::TenantPolicyAndSchemaScoped,
    };

    let (family, authority_posture, normalized_subject) = match intent.selector() {
        RawBasisSelector::CurrentHead => (
            NormalizedBasisFamily::CurrentHead,
            BasisAuthorityPosture::RuntimeBackedCurrentHead,
            NormalizedBasisSubject::CurrentHead,
        ),
        RawBasisSelector::BranchHead { branch_identity } => (
            NormalizedBasisFamily::BranchHead,
            BasisAuthorityPosture::RuntimeBackedBranch,
            NormalizedBasisSubject::BranchHead {
                branch_identity: branch_identity.clone(),
            },
        ),
        RawBasisSelector::BranchSnapshot {
            branch_identity,
            snapshot_identity,
        } => (
            NormalizedBasisFamily::BranchSnapshot,
            BasisAuthorityPosture::RuntimeBackedBranch,
            NormalizedBasisSubject::BranchSnapshot {
                branch_identity: branch_identity.clone(),
                snapshot_identity: snapshot_identity.clone(),
            },
        ),
        RawBasisSelector::RuntimeSnapshot { snapshot_identity } => (
            NormalizedBasisFamily::RuntimeSnapshot,
            BasisAuthorityPosture::RuntimeBackedBranch,
            NormalizedBasisSubject::RuntimeSnapshot {
                snapshot_identity: snapshot_identity.clone(),
            },
        ),
        RawBasisSelector::HistoricalSnapshot { snapshot_identity } => (
            NormalizedBasisFamily::HistoricalSnapshot,
            BasisAuthorityPosture::RuntimeBackedHistorical,
            NormalizedBasisSubject::HistoricalSnapshot {
                snapshot_identity: snapshot_identity.clone(),
            },
        ),
        RawBasisSelector::HistoricalCommit { commit_identity } => (
            NormalizedBasisFamily::HistoricalCommit,
            BasisAuthorityPosture::RuntimeBackedHistorical,
            NormalizedBasisSubject::HistoricalCommit {
                commit_identity: commit_identity.clone(),
            },
        ),
        RawBasisSelector::Preview { preview_identity } => (
            NormalizedBasisFamily::Preview,
            BasisAuthorityPosture::PreviewScoped,
            NormalizedBasisSubject::Preview {
                preview_identity: preview_identity.clone(),
            },
        ),
        RawBasisSelector::PreviewDerivedHistorical { preview_identity } => (
            NormalizedBasisFamily::PreviewDerivedHistorical,
            BasisAuthorityPosture::PreviewScoped,
            NormalizedBasisSubject::PreviewDerivedHistorical {
                preview_identity: preview_identity.clone(),
            },
        ),
        RawBasisSelector::FutureNeighbor { family } => {
            return Err(unsupported_future_neighbor_denial(&intent, family.clone()));
        }
    };
    let normalized_label = normalized_subject.projection_label();

    if normalized_label.is_empty() {
        return Err(malformed_identifier_denial(&intent, "basis_identifier"));
    }

    let canonical_digest = hash_parts(&[
        format!("family:{}", family.as_str()),
        format!("authority:{}", authority_posture.as_str()),
        format!("operation_lane:{}", intent.operation_lane().as_str()),
        format!("tenant_scope:{}", intent.tenant_scope().unwrap_or("-")),
        format!("policy_scope:{}", intent.policy_scope().unwrap_or("-")),
        format!("schema_scope:{}", intent.schema_scope().unwrap_or("-")),
        format!("tenant_schema_posture:{}", tenant_schema_posture.as_str()),
        format!("normalized_label:{normalized_label}"),
    ]);

    Ok(NormalizedBasisIntent {
        raw_basis_intent_digest: intent.raw_digest().to_string(),
        canonical_digest,
        family,
        authority_posture,
        operation_lane: intent.operation_lane().clone(),
        tenant_scope: intent.tenant_scope().map(str::to_string),
        policy_scope: intent.policy_scope().map(str::to_string),
        schema_scope: intent.schema_scope().map(str::to_string),
        tenant_schema_posture,
        source_path: *intent.source_path(),
        normalized_subject,
        normalized_label,
        counters: BasisNormalizationCounters::admitted(),
    })
}

pub(crate) fn unsupported_compatibility_family_denial(
    raw_basis_intent_digest: String,
    source_path: RawBasisSourcePath,
    operation_lane: BasisOperationLaneRequest,
    family: &'static str,
    owner: &'static str,
) -> BasisIntentDenial {
    BasisIntentDenial {
        raw_basis_intent_digest: raw_basis_intent_digest.clone(),
        source_path,
        operation_lane,
        kind: BasisIntentDenialKind::UnsupportedCompatibilityFamily { family, owner },
        counters: BasisNormalizationCounters::denied(),
        failure_digest: hash_parts(&[
            format!("raw_basis_intent_digest:{raw_basis_intent_digest}"),
            format!("failure:unsupported_compatibility_family:{family}"),
            format!("owner:{owner}"),
        ]),
    }
}

fn validate_optional_scope(
    raw_digest: &str,
    source_path: &RawBasisSourcePath,
    operation_lane: &BasisOperationLaneRequest,
    field: &'static str,
    value: Option<&str>,
) -> Result<(), BasisIntentDenial> {
    match value {
        Some(scope) if scope.trim().is_empty() => Err(BasisIntentDenial {
            raw_basis_intent_digest: raw_digest.to_string(),
            source_path: *source_path,
            operation_lane: operation_lane.clone(),
            kind: BasisIntentDenialKind::MalformedIdentifier { field },
            counters: BasisNormalizationCounters::denied(),
            failure_digest: hash_parts(&[
                format!("raw_basis_intent_digest:{raw_digest}"),
                format!("field:{field}"),
                "failure:malformed_identifier".to_string(),
            ]),
        }),
        _ => Ok(()),
    }
}

fn malformed_identifier_denial(intent: &RawBasisIntent, field: &'static str) -> BasisIntentDenial {
    BasisIntentDenial {
        raw_basis_intent_digest: intent.raw_digest().to_string(),
        source_path: *intent.source_path(),
        operation_lane: intent.operation_lane().clone(),
        kind: BasisIntentDenialKind::MalformedIdentifier { field },
        counters: BasisNormalizationCounters::denied(),
        failure_digest: hash_parts(&[
            format!("raw_basis_intent_digest:{}", intent.raw_digest()),
            format!("field:{field}"),
            "failure:malformed_identifier".to_string(),
        ]),
    }
}

fn unsupported_future_neighbor_denial(
    intent: &RawBasisIntent,
    family: RawFutureBasisNeighborFamily,
) -> BasisIntentDenial {
    let owner = match family {
        RawFutureBasisNeighborFamily::Temporal | RawFutureBasisNeighborFamily::AsyncResource => {
            "forge_signal"
        }
        RawFutureBasisNeighborFamily::StoreBackedParity
        | RawFutureBasisNeighborFamily::DurableReload
        | RawFutureBasisNeighborFamily::RestartStableEnvelope => "forge_store",
    };
    BasisIntentDenial {
        raw_basis_intent_digest: intent.raw_digest().to_string(),
        source_path: *intent.source_path(),
        operation_lane: intent.operation_lane().clone(),
        kind: BasisIntentDenialKind::UnsupportedFutureNeighbor { family, owner },
        counters: BasisNormalizationCounters::denied(),
        failure_digest: hash_parts(&[
            format!("raw_basis_intent_digest:{}", intent.raw_digest()),
            format!("failure:unsupported_future_neighbor:{owner}"),
        ]),
    }
}
