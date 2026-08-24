use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCommitPosture, WorthQueryBoundDomainOperation,
    WorthQueryExecutionProviderSession, WorthQueryExecutionResourceAttemptEvidence,
    WorthQueryGraphProviderCallKind, WorthQueryGraphProviderCallRequest,
    WorthQueryOperationGraphAccess, WorthQueryOperationGraphParticipation,
    WorthQueryOperationTouchContract,
};
use worth_query_execution::facade::integration::legacy_provider_execution::execute_legacy_one_shot;

use super::{
    WorthQueryBoundExecutionDenial, WorthQueryBoundExecutionDenialKind,
    WorthQueryBoundGraphExecutionReceipt, WorthQueryOperationExecutionCounters,
};

type BoundGraphParticipation =
    crate::domain_installation::operating_world::WorthQueryBoundGraphParticipation;
type InstalledGraphCommitAuthority =
    crate::domain_installation::graph_participation::WorthQueryInstalledGraphCommitAuthority;

struct BoundGraphInvocationPlan<'a> {
    reads: Vec<(&'a BoundGraphParticipation, WorthQueryGraphProviderCallKind)>,
    touches: Vec<&'a BoundGraphParticipation>,
    commit_groups: Vec<(std::sync::Arc<InstalledGraphCommitAuthority>, Vec<String>)>,
}

pub(super) struct BoundGraphInvocationRequest<'a> {
    pub(super) participation: &'a BoundGraphParticipation,
    pub(super) kind: WorthQueryGraphProviderCallKind,
    pub(super) scope_identity: &'a str,
    pub(super) stage_identity: Option<&'a str>,
    pub(super) expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
    pub(super) resources: &'a super::WorthQueryAdmittedExecutionResourcePlan,
    pub(super) resource_evidence: &'a WorthQueryExecutionResourceAttemptEvidence,
    pub(super) provider_session: &'a WorthQueryExecutionProviderSession,
}

pub(super) fn invoke_bound_graphs<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    resources: &super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &WorthQueryExecutionProviderSession,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<Vec<WorthQueryBoundGraphExecutionReceipt>, WorthQueryBoundExecutionDenial> {
    let plan = plan_bound_graph_invocations(bound);
    BoundGraphInvocation::new(
        bound,
        resources,
        resource_evidence,
        provider_session,
        expected_snapshot,
        counters,
    )
    .execute(plan)
}

fn plan_bound_graph_invocations<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
) -> BoundGraphInvocationPlan<'_> {
    let semantics = bound.definition().semantics();
    let mut reads = Vec::new();
    let mut touches = Vec::new();
    let mut commit_groups: Vec<(std::sync::Arc<InstalledGraphCommitAuthority>, Vec<String>)> =
        Vec::new();
    for participation in bound.graph_participations() {
        if let Some(read) = semantics.graph_reads.domain_roles().iter().find(|read| {
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
            reads.push((participation, kind));
        }
        if matches!(&semantics.touches, WorthQueryOperationTouchContract::Declared { graph_roles, .. } if graph_roles.contains(&participation.role))
        {
            touches.push(participation);
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
    BoundGraphInvocationPlan {
        reads,
        touches,
        commit_groups,
    }
}

struct BoundGraphInvocation<'a, D, O, F, L: BasisOperationLane> {
    bound: &'a WorthQueryBoundDomainOperation<D, O, F, L>,
    resources: &'a super::WorthQueryAdmittedExecutionResourcePlan,
    resource_evidence: &'a WorthQueryExecutionResourceAttemptEvidence,
    provider_session: &'a WorthQueryExecutionProviderSession,
    expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
    scope_identity: String,
    counters: &'a mut WorthQueryOperationExecutionCounters,
    receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
}

impl<'a, D, O, F, L: BasisOperationLane> BoundGraphInvocation<'a, D, O, F, L> {
    fn new(
        bound: &'a WorthQueryBoundDomainOperation<D, O, F, L>,
        resources: &'a super::WorthQueryAdmittedExecutionResourcePlan,
        resource_evidence: &'a WorthQueryExecutionResourceAttemptEvidence,
        provider_session: &'a WorthQueryExecutionProviderSession,
        expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
        counters: &'a mut WorthQueryOperationExecutionCounters,
    ) -> Self {
        Self {
            bound,
            resources,
            resource_evidence,
            provider_session,
            expected_snapshot,
            scope_identity: format!("direct-capability:{}", bound.capability_identity()),
            counters,
            receipts: Vec::new(),
        }
    }

    fn execute(
        mut self,
        plan: BoundGraphInvocationPlan<'a>,
    ) -> Result<Vec<WorthQueryBoundGraphExecutionReceipt>, WorthQueryBoundExecutionDenial> {
        self.contact_reads(plan.reads)?;
        self.contact_commit_groups(plan.commit_groups)?;
        self.contact_touches(plan.touches)?;
        Ok(self.receipts)
    }

    fn contact_reads(
        &mut self,
        reads: Vec<(&BoundGraphParticipation, WorthQueryGraphProviderCallKind)>,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        for (participation, kind) in reads {
            self.contact(participation, kind)?;
        }
        Ok(())
    }

    fn contact_touches(
        &mut self,
        touches: Vec<&BoundGraphParticipation>,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        for participation in touches {
            self.contact(participation, WorthQueryGraphProviderCallKind::TouchEffect)?;
        }
        Ok(())
    }

    fn contact(
        &mut self,
        participation: &BoundGraphParticipation,
        kind: WorthQueryGraphProviderCallKind,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        let request = BoundGraphInvocationRequest {
            participation,
            kind,
            scope_identity: &self.scope_identity,
            stage_identity: None,
            expected_snapshot: self.expected_snapshot,
            resources: self.resources,
            resource_evidence: self.resource_evidence,
            provider_session: self.provider_session,
        };
        let receipt = contact_graph(request, self.counters)
            .map_err(|denial| denial.with_graph_receipts(self.receipts.clone()))?;
        self.receipts.push(receipt);
        Ok(())
    }

    fn contact_commit_groups(
        &mut self,
        commit_groups: Vec<(std::sync::Arc<InstalledGraphCommitAuthority>, Vec<String>)>,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        for (authority, mut roles) in commit_groups {
            self.contact_commit_group(authority, &mut roles)?;
        }
        Ok(())
    }

    fn contact_commit_group(
        &mut self,
        authority: std::sync::Arc<InstalledGraphCommitAuthority>,
        roles: &mut [String],
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        roles.sort();
        self.counters.graph_provider_contacts += 1;
        let receipt = super::commit_execution::contact_commit_provider(
            &self.scope_identity,
            None,
            &authority,
            &self
                .bound
                .graph_participations()
                .iter()
                .filter(|participation| roles.contains(&participation.role))
                .map(|participation| participation.record.installation_authority.as_ref())
                .collect::<Vec<_>>(),
            self.resources,
            self.resource_evidence,
            self.provider_session,
        )
        .map_err(|failure| {
            WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::GraphProvider,
                failure.detail(),
                *self.counters,
            )
            .with_graph_receipts(self.receipts.clone())
        })?;
        self.receipts.push(receipt);
        Ok(())
    }
}

pub(super) fn contact_graph(
    request: BoundGraphInvocationRequest<'_>,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryBoundExecutionDenial> {
    let call = graph_provider_call(&request).map_err(|denial| {
        WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            denial.detail(),
            *counters,
        )
    })?;
    counters.graph_provider_contacts += 1;
    let receipt = execute_legacy_one_shot(request.participation.record.provider.as_ref(), &call)
        .map_err(|failure| {
            WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::GraphProvider,
                failure.detail(),
                *counters,
            )
        })?;
    call.admit_receipt(receipt).map_err(|denial| {
        WorthQueryBoundExecutionDenial::new(
            WorthQueryBoundExecutionDenialKind::GraphProvider,
            denial.detail(),
            *counters,
        )
    })
}

fn graph_provider_call(
    request: &BoundGraphInvocationRequest<'_>,
) -> Result<
    crate::domain_installation::WorthQueryGraphProviderCall,
    crate::domain_installation::WorthQueryGraphCallBindingDenial,
> {
    let call_request = match request.stage_identity {
        Some(stage_identity) => WorthQueryGraphProviderCallRequest::workflow_stage(
            request.kind,
            request.scope_identity,
            stage_identity,
        ),
        None => WorthQueryGraphProviderCallRequest::direct(request.kind, request.scope_identity),
    }
    .bind_execution_snapshot(request.expected_snapshot.evidence_identity().as_str());
    request.provider_session.bind_graph_provider_call(
        request.participation.record.installation_authority.as_ref(),
        call_request,
        request.resource_evidence,
        request.resources.shared_envelope(),
    )
}
