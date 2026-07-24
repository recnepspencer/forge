use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCommitPosture, WorthQueryBoundDomainOperation, WorthQueryGraphProviderCallKind,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
};

use super::bound_graph_execution::{contact_graph, BoundGraphInvocationRequest};
use super::{
    WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowAdvanceDenial,
    WorthQueryWorkflowAdvanceDenialKind, WorthQueryWorkflowRunCounters,
};

pub(super) fn invoke_stage_graphs<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    run_identity: &str,
    stage: &worth_query_installation::facade::WorthQueryPortableWorkflowStage,
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &super::WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &super::WorthQueryExecutionProviderSession,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: &mut WorthQueryWorkflowRunCounters,
) -> Result<Vec<WorthQueryBoundGraphExecutionReceipt>, WorthQueryWorkflowAdvanceDenial> {
    let scope_identity = format!("workflow:{run_identity}:stage:{}", stage.identity());
    let mut receipts = Vec::new();
    let mut touch_participations = Vec::new();
    let mut commit_groups = Vec::<(
        std::sync::Arc<
            crate::domain_installation::graph_participation::WorthQueryInstalledGraphCommitAuthority,
        >,
        Vec<String>,
    )>::new();
    for participation in bound.graph_participations() {
        if let Some(read) = bound
            .definition()
            .semantics()
            .graph_reads
            .roles()
            .iter()
            .find(|read| {
                read.role == participation.role
                    && stage.semantics().graph_read_roles.contains(&read.role)
                    && matches!(
                        read.participation,
                        WorthQueryOperationGraphParticipation::SeparateAuthority { .. }
                    )
            })
        {
            let kind = match read.access {
                WorthQueryOperationGraphAccess::Observe => WorthQueryGraphProviderCallKind::Observe,
                WorthQueryOperationGraphAccess::Project => WorthQueryGraphProviderCallKind::Project,
            };
            counters.graph_read_contacts += 1;
            let receipt = contact(
                bound,
                participation,
                kind,
                &scope_identity,
                resources,
                resource_evidence,
                provider_session,
                expected_snapshot,
                *counters,
            )
            .map_err(|denial| denial.with_graph_receipts(receipts.clone()))?;
            receipts.push(receipt);
        }
        if stage.semantics().touch_roles.contains(&participation.role) {
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
        counters.commit_admission_contacts += 1;
        let receipt = super::commit_execution::contact_commit_provider(
            &scope_identity,
            bound.definition().canonical_identity(),
            bound.binding_identity(),
            &authority,
            roles.clone(),
            resources,
            resource_evidence,
            provider_session,
        )
        .map_err(|denial| {
            WorthQueryWorkflowAdvanceDenial::new(
                WorthQueryWorkflowAdvanceDenialKind::GraphProvider(denial.detail().into()),
                *counters,
            )
            .with_graph_receipts(receipts.clone())
        })?;
        receipts.push(receipt);
    }
    for participation in touch_participations {
        counters.touch_effect_contacts += 1;
        let receipt = contact(
            bound,
            participation,
            WorthQueryGraphProviderCallKind::TouchEffect,
            &scope_identity,
            resources,
            resource_evidence,
            provider_session,
            expected_snapshot,
            *counters,
        )
        .map_err(|denial| denial.with_graph_receipts(receipts.clone()))?;
        receipts.push(receipt);
    }
    Ok(receipts)
}

fn contact<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    participation: &super::super::WorthQueryBoundGraphParticipation,
    kind: WorthQueryGraphProviderCallKind,
    scope_identity: &str,
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &super::WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &super::WorthQueryExecutionProviderSession,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: WorthQueryWorkflowRunCounters,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryWorkflowAdvanceDenial> {
    contact_graph(
        BoundGraphInvocationRequest {
            bound,
            participation,
            kind,
            scope_identity,
            expected_snapshot,
            resources,
            resource_evidence,
            provider_session,
        },
        &mut Default::default(),
    )
    .map_err(|denial| {
        WorthQueryWorkflowAdvanceDenial::new(
            WorthQueryWorkflowAdvanceDenialKind::GraphProvider(denial.detail().into()),
            counters,
        )
    })
}
