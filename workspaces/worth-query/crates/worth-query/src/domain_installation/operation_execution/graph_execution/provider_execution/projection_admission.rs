use crate::basis_lifecycle::{BasisFamily, BasisOperationLane};
use crate::domain_installation::{WorthQueryBoundDomainOperation, WorthQueryGraphProviderCallKind};
use crate::runtime::{WorthQueryReadExecutionEngine, WorthQueryReadResult};

use super::{
    BoundGraphParticipation, WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryOperationExecutionCounters,
};

pub(super) struct GraphProjectionAdmission<'a, D, O, F, L: BasisOperationLane> {
    pub(super) bound: &'a WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(super) participation: &'a BoundGraphParticipation,
    pub(super) kind: WorthQueryGraphProviderCallKind,
    pub(super) expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) scope_identity: &'a str,
    pub(super) resource_evidence:
        &'a crate::domain_installation::WorthQueryExecutionResourceAttemptEvidence,
}

pub(super) fn admit_graph_projection_material<D, O, F, L: BasisOperationLane>(
    admission: &GraphProjectionAdmission<'_, D, O, F, L>,
    projection: Option<&WorthQueryReadResult>,
    counters: WorthQueryOperationExecutionCounters,
) -> Result<(), WorthQueryBoundExecutionDenial> {
    let projection_is_valid = projection.is_some_and(|projection| {
        projection.receipt().canonical_query_digest()
            == admission
                .bound
                .definition()
                .semantics()
                .canonical_query
                .query()
                .digest()
                .as_str()
            && projection_engine_matches_basis(
                projection.receipt().execution_engine(),
                admission.bound.basis().normalized().family(),
            )
            && projection
                .receipt()
                .snapshot_identity()
                .is_same_current_identity_as(admission.expected_snapshot)
    });
    if admission.kind == WorthQueryGraphProviderCallKind::Project && !projection_is_valid {
        let observed = projection.map(|projection| {
            format!(
                "query={}, basis={}",
                projection.receipt().canonical_query_digest(),
                projection.receipt().basis_digest()
            )
        });
        return Err(WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            format!(
                "graph role `{}` returned no canonically bound projection material (expected query={}, basis-family={}, snapshot={}; observed {})",
                admission.participation.role,
                admission.bound.definition().semantics().canonical_query.query().digest().as_str(),
                admission.bound.basis().normalized().family().as_str(),
                admission.expected_snapshot.evidence_identity().as_str(),
                observed.as_deref().unwrap_or("none")
            ),
            counters,
        ));
    }
    if admission.kind != WorthQueryGraphProviderCallKind::Project && projection.is_some() {
        return Err(WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            format!(
                "graph role `{}` returned projection material for a non-projection call",
                admission.participation.role
            ),
            counters,
        ));
    }
    Ok(())
}

pub(super) fn graph_call_evidence_identity<D, O, F, L: BasisOperationLane>(
    admission: &GraphProjectionAdmission<'_, D, O, F, L>,
    projection: Option<&WorthQueryReadResult>,
) -> String {
    crate::identity::hash_parts(&[
        "worth_query_bound_graph_call_evidence_v1".into(),
        format!(
            "operation:{}",
            admission.bound.definition().canonical_identity()
        ),
        format!("binding:{}", admission.bound.binding_identity()),
        format!("role:{}", admission.participation.role),
        format!(
            "kind:{}",
            crate::domain_installation::operation_identity_basis::graph_call_kind_material(
                admission.kind
            )
        ),
        format!("scope:{}", admission.scope_identity),
        format!("resources:{}", admission.resource_evidence.identity()),
        format!(
            "projection:{}",
            projection
                .map(|projection| projection.receipt().result_digest())
                .unwrap_or("not-projected")
        ),
    ])
}

fn projection_engine_matches_basis(
    engine: &WorthQueryReadExecutionEngine,
    basis: BasisFamily,
) -> bool {
    matches!(
        (basis, engine),
        (
            BasisFamily::CurrentHead | BasisFamily::TenantScoped | BasisFamily::PolicyScoped,
            WorthQueryReadExecutionEngine::QueryRuntimeCurrent
        ) | (
            BasisFamily::BranchHead | BasisFamily::BranchSnapshot,
            WorthQueryReadExecutionEngine::QueryRuntimeBranch
        ) | (
            BasisFamily::Preview | BasisFamily::PreviewDerived,
            WorthQueryReadExecutionEngine::QueryRuntimePreviewDerived
        ) | (
            BasisFamily::RuntimeSnapshot
                | BasisFamily::HistoricalSnapshot
                | BasisFamily::HistoricalCommit
                | BasisFamily::StoreBacked
                | BasisFamily::DurableReload,
            WorthQueryReadExecutionEngine::QueryRuntimeHistorical
        )
    )
}
