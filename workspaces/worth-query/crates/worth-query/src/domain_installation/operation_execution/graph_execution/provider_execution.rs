use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCommitPosture, WorthQueryBoundDomainOperation, WorthQueryGraphProviderCallKind,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationTouchContract,
};

use super::{
    WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryBoundGraphExecutionReceipt, WorthQueryOperationExecutionCounters,
};

pub(super) fn invoke_bound_graphs<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<Vec<WorthQueryBoundGraphExecutionReceipt>, WorthQueryBoundExecutionDenial> {
    let semantics = bound.definition().semantics();
    let scope_identity = format!("direct-capability:{}", bound.capability_identity());
    let mut receipts = Vec::new();
    let mut touch_participations = Vec::new();
    let mut commit_groups = Vec::<(
        std::sync::Arc<
            crate::domain_installation::graph_participation::WorthQueryInstalledGraphCommitAuthority,
        >,
        Vec<String>,
    )>::new();
    for participation in bound.graph_participations() {
        if let Some(read) = semantics.graph_reads.roles().iter().find(|read| {
            read.role == participation.role
                && matches!(
                    read.participation,
                    WorthQueryOperationGraphParticipation::SeparateAuthority { .. }
                )
        }) {
            let kind = match read.access {
                WorthQueryOperationGraphAccess::Observe => WorthQueryGraphProviderCallKind::Observe,
                WorthQueryOperationGraphAccess::Project => WorthQueryGraphProviderCallKind::Project,
            };
            let receipt = contact_graph(
                bound,
                participation,
                kind,
                &scope_identity,
                expected_snapshot,
                counters,
            )
            .map_err(|denial| denial.with_graph_receipts(receipts.clone()))?;
            receipts.push(receipt);
        }
        if matches!(&semantics.touches, WorthQueryOperationTouchContract::Declared { graph_roles, .. } if graph_roles.contains(&participation.role))
        {
            touch_participations.push(participation);
            if bound.commit_posture() == WorthQueryBoundCommitPosture::Atomic {
                if let Some(authority) = &participation.record.commit_authority {
                    match commit_groups.iter_mut().find(|(candidate, _)| {
                        std::sync::Arc::ptr_eq(candidate, authority)
                            && candidate.identity() == authority.identity()
                    }) {
                        Some((_, roles)) => roles.push(participation.role.clone()),
                        None => commit_groups.push((
                            std::sync::Arc::clone(authority),
                            vec![participation.role.clone()],
                        )),
                    }
                }
            }
        }
    }
    for (authority, mut roles) in commit_groups {
        roles.sort();
        counters.graph_provider_contacts += 1;
        let receipt = super::commit_execution::contact_commit_provider(
            &scope_identity,
            bound.definition().canonical_identity(),
            bound.binding_identity(),
            &authority,
            roles.clone(),
        )
        .map_err(|failure| {
            WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::GraphProvider,
                failure.detail(),
                *counters,
            )
            .with_graph_receipts(receipts.clone())
        })?;
        receipts.push(WorthQueryBoundGraphExecutionReceipt {
            role: format!("commit({})", roles.join(",")),
            kind: WorthQueryGraphProviderCallKind::CommitAdmission,
            provider_receipt: receipt.provider_receipt().to_string(),
            evidence_identity: crate::identity::hash_parts(&[
                "worth_query_bound_graph_commit_evidence_v1".into(),
                format!("operation:{}", bound.definition().canonical_identity()),
                format!("binding:{}", bound.binding_identity()),
                format!("scope:{scope_identity}"),
                format!("roles:{}", roles.join(",")),
            ]),
            projection: None,
        });
    }
    for participation in touch_participations {
        let receipt = contact_graph(
            bound,
            participation,
            WorthQueryGraphProviderCallKind::TouchEffect,
            &scope_identity,
            expected_snapshot,
            counters,
        )
        .map_err(|denial| denial.with_graph_receipts(receipts.clone()))?;
        receipts.push(receipt);
    }
    Ok(receipts)
}

pub(super) fn contact_graph<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    participation: &crate::domain_installation::operating_world::WorthQueryBoundGraphParticipation,
    kind: WorthQueryGraphProviderCallKind,
    scope_identity: &str,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryBoundExecutionDenial> {
    counters.graph_provider_contacts += 1;
    let expected_basis = bound
        .basis()
        .normalized()
        .lower_runtime_binding_digest()
        .unwrap_or_else(|| bound.basis().capability_digest());
    let call = crate::domain_installation::WorthQueryGraphProviderCall::new(
        scope_identity.into(),
        kind,
        bound.definition().canonical_identity().into(),
        bound.binding_identity().into(),
        participation.role.clone(),
        bound
            .definition()
            .semantics()
            .canonical_query
            .query()
            .digest()
            .as_str()
            .into(),
        expected_basis.into(),
    );
    let receipt = participation
        .record
        .provider
        .call(kind, &call)
        .map_err(|failure| {
            WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::GraphProvider,
                failure.detail(),
                *counters,
            )
        })?;
    if !receipt.binds_call(call.call_identity()) {
        return Err(WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            "graph provider returned a receipt minted for another Query call",
            *counters,
        ));
    }
    let provider_receipt = receipt.provider_receipt().to_string();
    let projection = receipt.into_projection();
    let projection_is_valid = projection.as_deref().is_some_and(|projection| {
        projection.receipt().canonical_query_digest()
            == bound
                .definition()
                .semantics()
                .canonical_query
                .query()
                .digest()
                .as_str()
            && projection_engine_matches_basis(
                projection.receipt().execution_engine(),
                bound.basis().normalized().family(),
            )
            && projection.receipt().snapshot_identity() == expected_snapshot
    });
    if kind == WorthQueryGraphProviderCallKind::Project && !projection_is_valid {
        let observed = projection.as_deref().map(|projection| {
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
                participation.role,
                bound
                    .definition()
                    .semantics()
                    .canonical_query
                    .query()
                    .digest()
                    .as_str(),
                bound.basis().normalized().family().as_str(),
                expected_snapshot.evidence_identity().as_str(),
                observed.as_deref().unwrap_or("none")
            ),
            *counters,
        ));
    }
    if kind != WorthQueryGraphProviderCallKind::Project && projection.is_some() {
        return Err(WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            format!(
                "graph role `{}` returned projection material for a non-projection call",
                participation.role
            ),
            *counters,
        ));
    }
    let evidence_identity = crate::identity::hash_parts(&[
        "worth_query_bound_graph_call_evidence_v1".into(),
        format!("operation:{}", bound.definition().canonical_identity()),
        format!("binding:{}", bound.binding_identity()),
        format!("role:{}", participation.role),
        format!("kind:{kind:?}"),
        format!("scope:{scope_identity}"),
        format!(
            "projection:{}",
            projection
                .as_deref()
                .map(|projection| projection.receipt().result_digest())
                .unwrap_or("not-projected")
        ),
    ]);
    Ok(WorthQueryBoundGraphExecutionReceipt {
        role: participation.role.clone(),
        kind,
        provider_receipt,
        evidence_identity,
        projection,
    })
}

fn projection_engine_matches_basis(
    engine: &crate::runtime::WorthQueryReadExecutionEngine,
    basis: crate::basis_lifecycle::BasisFamily,
) -> bool {
    use crate::basis_lifecycle::BasisFamily;
    use crate::runtime::WorthQueryReadExecutionEngine;

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
