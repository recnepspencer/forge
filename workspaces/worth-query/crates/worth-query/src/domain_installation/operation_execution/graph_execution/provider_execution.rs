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

#[path = "provider_execution/projection_admission.rs"]
mod projection_admission;

use projection_admission::{
    admit_graph_projection_material, graph_call_evidence_identity, GraphProjectionAdmission,
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

pub(super) struct BoundGraphInvocationRequest<'a, D, O, F, L: BasisOperationLane> {
    pub(super) bound: &'a WorthQueryBoundDomainOperation<D, O, F, L>,
    pub(super) participation: &'a BoundGraphParticipation,
    pub(super) kind: WorthQueryGraphProviderCallKind,
    pub(super) scope_identity: &'a str,
    pub(super) expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
}

pub(super) fn invoke_bound_graphs<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    expected_snapshot: &crate::memory_workspace::WorthQuerySnapshotIdentity,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<Vec<WorthQueryBoundGraphExecutionReceipt>, WorthQueryBoundExecutionDenial> {
    let plan = plan_bound_graph_invocations(bound);
    BoundGraphInvocation::new(bound, expected_snapshot, counters).execute(plan)
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
    expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
    scope_identity: String,
    counters: &'a mut WorthQueryOperationExecutionCounters,
    receipts: Vec<WorthQueryBoundGraphExecutionReceipt>,
}

impl<'a, D, O, F, L: BasisOperationLane> BoundGraphInvocation<'a, D, O, F, L> {
    fn new(
        bound: &'a WorthQueryBoundDomainOperation<D, O, F, L>,
        expected_snapshot: &'a crate::memory_workspace::WorthQuerySnapshotIdentity,
        counters: &'a mut WorthQueryOperationExecutionCounters,
    ) -> Self {
        Self {
            bound,
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
            bound: self.bound,
            participation,
            kind,
            scope_identity: &self.scope_identity,
            expected_snapshot: self.expected_snapshot,
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
        roles: &mut Vec<String>,
    ) -> Result<(), WorthQueryBoundExecutionDenial> {
        roles.sort();
        self.counters.graph_provider_contacts += 1;
        let receipt = super::commit_execution::contact_commit_provider(
            &self.scope_identity,
            self.bound.definition().canonical_identity(),
            self.bound.binding_identity(),
            &authority,
            roles.clone(),
        )
        .map_err(|failure| {
            WorthQueryBoundExecutionDenial::new(
                WorthQueryBoundExecutionDenialKind::GraphProvider,
                failure.detail(),
                *self.counters,
            )
            .with_graph_receipts(self.receipts.clone())
        })?;
        self.receipts.push(WorthQueryBoundGraphExecutionReceipt {
            role: format!("commit({})", roles.join(",")),
            kind: WorthQueryGraphProviderCallKind::CommitAdmission,
            provider_receipt: receipt.provider_receipt().to_string(),
            evidence_identity: crate::identity::hash_parts(&[
                "worth_query_bound_graph_commit_evidence_v1".into(),
                format!("operation:{}", self.bound.definition().canonical_identity()),
                format!("binding:{}", self.bound.binding_identity()),
                format!("scope:{}", self.scope_identity),
                format!("roles:{}", roles.join(",")),
            ]),
            projection: None,
            commit_authority_identity: Some(authority.identity()),
            commit_graph_roles: std::mem::take(roles),
        });
        Ok(())
    }
}

pub(super) fn contact_graph<D, O, F, L: BasisOperationLane>(
    request: BoundGraphInvocationRequest<'_, D, O, F, L>,
    counters: &mut WorthQueryOperationExecutionCounters,
) -> Result<WorthQueryBoundGraphExecutionReceipt, WorthQueryBoundExecutionDenial> {
    counters.graph_provider_contacts += 1;
    let call = graph_provider_call(&request);
    let receipt = request
        .participation
        .record
        .provider
        .call(request.kind, &call)
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
    let projection_admission = GraphProjectionAdmission {
        bound: request.bound,
        participation: request.participation,
        kind: request.kind,
        expected_snapshot: request.expected_snapshot,
        scope_identity: request.scope_identity,
    };
    admit_graph_projection_material(&projection_admission, projection.as_deref(), *counters)?;
    let evidence_identity =
        graph_call_evidence_identity(&projection_admission, projection.as_deref());
    Ok(WorthQueryBoundGraphExecutionReceipt {
        role: request.participation.role.clone(),
        kind: request.kind,
        provider_receipt,
        evidence_identity,
        projection,
        commit_authority_identity: None,
        commit_graph_roles: Vec::new(),
    })
}

fn graph_provider_call<D, O, F, L: BasisOperationLane>(
    request: &BoundGraphInvocationRequest<'_, D, O, F, L>,
) -> crate::domain_installation::WorthQueryGraphProviderCall {
    let expected_basis = request
        .bound
        .basis()
        .normalized()
        .lower_runtime_binding_digest()
        .unwrap_or_else(|| request.bound.basis().capability_digest());
    crate::domain_installation::WorthQueryGraphProviderCall::new(
        request.scope_identity.into(),
        request.kind,
        request.bound.definition().canonical_identity().into(),
        request.bound.binding_identity().into(),
        request.participation.role.clone(),
        request
            .bound
            .definition()
            .semantics()
            .canonical_query
            .query()
            .digest()
            .as_str()
            .into(),
        expected_basis.into(),
    )
}
