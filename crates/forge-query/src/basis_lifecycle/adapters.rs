use crate::historical::HistoricalMaterializationDescriptor;
use crate::query_context::{QueryBasisContextRequest, QueryContextFamily};
use crate::runtime::{
    ForgeQueryBranchBasisAdmission, ForgeQueryPreviewBasisAdmission,
    ForgeQueryRuntimeInspectionEvidence,
};
use crate::subscription::QuerySubscriptionBasisPosture;

use super::admission::{
    admit_basis_capability, evaluate_basis_inspection_advisory_eligibility,
    evaluate_basis_inspection_eligibility, evaluate_basis_mutation_preparation_eligibility,
    evaluate_basis_preview_closeout_eligibility, evaluate_basis_replay_eligibility,
    evaluate_basis_subscription_declaration_eligibility,
};
use super::intent::{normalize_raw_basis_intent, RawBasisIntent};
use super::migration::BasisLifecycleMigrationSurface;
use super::scoping::{
    scope_basis_for_inspection, scope_basis_for_mutation_preparation,
    scope_basis_for_preview_closeout, scope_basis_for_replay,
    scope_basis_for_subscription_declaration,
};

mod proofs;
use proofs::{advisory_proof, scoped_proof, source_digest, typed_denial_proof};
pub use proofs::{
    BasisLifecycleAdapterError, BasisLifecycleAdapterOutcome, BasisLifecycleAdapterProof,
};

pub fn adapt_branch_admission_to_lifecycle(
    admission: &ForgeQueryBranchBasisAdmission,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let raw = RawBasisIntent::BranchHead {
        branch_identity: admission.label_identity().to_string(),
        accessible: true,
    };
    let normalized = normalize_raw_basis_intent(raw, "mutation_preparation")?;
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)?;
    let scoped = scope_basis_for_mutation_preparation(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::BranchPreviewAdmission,
        "ForgeQueryBranchBasisAdmission",
        "mutation_preparation_scoped_basis",
        "mutation_preparation",
        source_digest(
            "branch_admission",
            admission.label_identity().as_str(),
            admission.evidence(),
        ),
        &scoped,
    ))
}

pub fn adapt_preview_admission_to_lifecycle(
    admission: &ForgeQueryPreviewBasisAdmission,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let raw = RawBasisIntent::Preview {
        preview_identity: admission.label_identity().to_string(),
        stale: false,
    };
    let normalized = normalize_raw_basis_intent(raw, "preview_closeout")?;
    let eligibility = evaluate_basis_preview_closeout_eligibility(normalized)?;
    let scoped = scope_basis_for_preview_closeout(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::BranchPreviewAdmission,
        "ForgeQueryPreviewBasisAdmission",
        "preview_closeout_scoped_basis",
        "preview_closeout",
        source_digest(
            "preview_admission",
            admission.label_identity().as_str(),
            admission.evidence(),
        ),
        &scoped,
    ))
}

pub fn adapt_query_basis_context_to_lifecycle(
    request: &QueryBasisContextRequest,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    match request.family() {
        QueryContextFamily::CurrentBranchHead => read_context_inspection_proof(
            request,
            RawBasisIntent::CurrentHead,
            "current_branch_head_context",
        ),
        QueryContextFamily::BranchHead => read_context_mutation_proof(
            request,
            RawBasisIntent::BranchHead {
                branch_identity: request.declared_basis_label().to_string(),
                accessible: true,
            },
            "branch_head_context",
        ),
        QueryContextFamily::HistoricalSnapshot | QueryContextFamily::HistoricalCommit => {
            read_context_replay_proof(
                request,
                RawBasisIntent::HistoricalSnapshot {
                    snapshot_identity: request.declared_basis_label().to_string(),
                    replay_supported: true,
                },
                "historical_context",
            )
        }
        QueryContextFamily::PreviewDerivedHistorical => read_context_advisory_proof(request),
        QueryContextFamily::DiffComparison => Ok(typed_denial_proof(
            BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
            "QueryBasisContextRequest",
            "read_composition_lifecycle_denial",
            "inspection",
            source_digest(
                "query_basis_context",
                request.declared_basis_label(),
                [request.family().as_str()],
            ),
            "diff comparison requires pairwise context admission before lifecycle use",
        )),
    }
}

pub fn adapt_subscription_basis_posture_to_lifecycle(
    posture: &QuerySubscriptionBasisPosture,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let source = source_digest(
        "subscription_basis_posture",
        posture.as_str(),
        [posture.as_str()],
    );
    match posture {
        QuerySubscriptionBasisPosture::CurrentHead => {
            subscription_proof(RawBasisIntent::CurrentHead, source)
        }
        QuerySubscriptionBasisPosture::BranchHead => subscription_proof(
            RawBasisIntent::BranchHead {
                branch_identity: "subscription-branch-head".to_string(),
                accessible: true,
            },
            source,
        ),
        QuerySubscriptionBasisPosture::RuntimeHistoricalSnapshot
        | QuerySubscriptionBasisPosture::PreviewScoped
        | QuerySubscriptionBasisPosture::DeniedUnsupportedBasis => Ok(typed_denial_proof(
            BasisLifecycleMigrationSurface::SubscriptionBasisPosture,
            "QuerySubscriptionBasisPosture",
            "subscription_lifecycle_denial",
            "subscription_declaration",
            source,
            "subscription posture is unsupported until lifecycle admission grants the lane",
        )),
    }
}

pub fn adapt_causal_inspection_evidence_to_lifecycle(
    evidence: &ForgeQueryRuntimeInspectionEvidence,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let raw = RawBasisIntent::PreviewDerived {
        preview_identity: evidence.artifact_family().to_string(),
        source_basis_identity: evidence.authority_lane().as_str().to_string(),
    };
    let normalized = normalize_raw_basis_intent(raw, "inspection")?;
    let advisory = evaluate_basis_inspection_advisory_eligibility(normalized)?;
    Ok(advisory_proof(
        BasisLifecycleMigrationSurface::CausalInspectionBasisEvidence,
        "ForgeQueryRuntimeInspectionEvidence",
        "inspection_advisory_lifecycle",
        "inspection",
        source_digest(
            "runtime_inspection_evidence",
            evidence.artifact_family(),
            evidence.evidence(),
        ),
        advisory.decision_trace().trace_digest(),
    ))
}

pub fn adapt_historical_materialization_to_lifecycle(
    descriptor: &HistoricalMaterializationDescriptor,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let raw = RawBasisIntent::HistoricalSnapshot {
        snapshot_identity: descriptor.basis_identity().to_string(),
        replay_supported: true,
    };
    let normalized = normalize_raw_basis_intent(raw, "replay")?;
    let eligibility = evaluate_basis_replay_eligibility(normalized)?;
    let scoped = scope_basis_for_replay(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::HistoricalMaterializationBasis,
        "HistoricalMaterializationDescriptor",
        "historical_replay_scoped_basis",
        "replay",
        source_digest(
            "historical_materialization",
            descriptor.basis_identity(),
            [descriptor.resolved_path_class().as_str()],
        ),
        &scoped,
    ))
}

fn read_context_inspection_proof(
    request: &QueryBasisContextRequest,
    raw: RawBasisIntent,
    source_kind: &'static str,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let normalized = normalize_raw_basis_intent(raw, "inspection")?;
    let eligibility = evaluate_basis_inspection_eligibility(normalized)?;
    let scoped = scope_basis_for_inspection(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        "QueryBasisContextRequest",
        "inspection_scoped_basis",
        "inspection",
        source_digest(
            source_kind,
            request.declared_basis_label(),
            [request.family().as_str()],
        ),
        &scoped,
    ))
}

fn read_context_mutation_proof(
    request: &QueryBasisContextRequest,
    raw: RawBasisIntent,
    source_kind: &'static str,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let normalized = normalize_raw_basis_intent(raw, "mutation_preparation")?;
    let eligibility = evaluate_basis_mutation_preparation_eligibility(normalized)?;
    let scoped = scope_basis_for_mutation_preparation(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        "QueryBasisContextRequest",
        "mutation_preparation_scoped_basis",
        "mutation_preparation",
        source_digest(
            source_kind,
            request.declared_basis_label(),
            [request.family().as_str()],
        ),
        &scoped,
    ))
}

fn read_context_replay_proof(
    request: &QueryBasisContextRequest,
    raw: RawBasisIntent,
    source_kind: &'static str,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let normalized = normalize_raw_basis_intent(raw, "replay")?;
    let eligibility = evaluate_basis_replay_eligibility(normalized)?;
    let scoped = scope_basis_for_replay(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        "QueryBasisContextRequest",
        "replay_scoped_basis",
        "replay",
        source_digest(
            source_kind,
            request.declared_basis_label(),
            [request.family().as_str()],
        ),
        &scoped,
    ))
}

fn read_context_advisory_proof(
    request: &QueryBasisContextRequest,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let raw = RawBasisIntent::PreviewDerived {
        preview_identity: request.declared_basis_label().to_string(),
        source_basis_identity: "read-composition-source".to_string(),
    };
    let normalized = normalize_raw_basis_intent(raw, "inspection")?;
    let advisory = evaluate_basis_inspection_advisory_eligibility(normalized)?;
    Ok(advisory_proof(
        BasisLifecycleMigrationSurface::ReadCompositionBasisContext,
        "QueryBasisContextRequest",
        "inspection_advisory_lifecycle",
        "inspection",
        source_digest(
            "preview_derived_context",
            request.declared_basis_label(),
            [request.family().as_str()],
        ),
        advisory.decision_trace().trace_digest(),
    ))
}

fn subscription_proof(
    raw: RawBasisIntent,
    source_digest: String,
) -> Result<BasisLifecycleAdapterProof, BasisLifecycleAdapterError> {
    let normalized = normalize_raw_basis_intent(raw, "subscription_declaration")?;
    let eligibility = evaluate_basis_subscription_declaration_eligibility(normalized)?;
    let scoped = scope_basis_for_subscription_declaration(admit_basis_capability(eligibility));
    Ok(scoped_proof(
        BasisLifecycleMigrationSurface::SubscriptionBasisPosture,
        "QuerySubscriptionBasisPosture",
        "subscription_declaration_scoped_basis",
        "subscription_declaration",
        source_digest,
        &scoped,
    ))
}

#[cfg(test)]
mod tests;
