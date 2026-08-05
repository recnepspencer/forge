use std::sync::Arc;

use worth_query_installation::facade::WorthQueryExecutionResourceEnvelope;
use worth_relational::facade::bridge::{
    bridge_snapshot_identity_for_handle, RuntimeBridgeRelationalSource,
};
use worth_relational::facade::history::BranchId;
use worth_relational::facade::runtime::RelationalExecutionBasisLease;
use worth_runtime_bridge::facade::{
    BridgeAsyncRequestTruthViewBasis, BridgeBoundExecutionBasis, BridgeManagedExecutionIntent,
    BridgeManagedExecutionPartialEffectPosture, BridgeManagedExecutionStepContract,
    BridgeManagedExecutionStepLimits, BridgeTruthViewSelector, HistoricalEvaluationDeclaration,
    RuntimeBridge,
};

use super::WorthQueryManagedTruthReadRequest;

pub(in crate::domain_computation) struct WorthQueryManagedLowerExecutionBasis {
    pub bridge: BridgeBoundExecutionBasis,
    pub relational: RelationalExecutionBasisLease,
}

pub(in crate::domain_computation) struct WorthQueryManagedLowerBinding<'a> {
    operation_identity: &'a str,
    resource_attempt_identity: &'a str,
    resource_envelope: &'a WorthQueryExecutionResourceEnvelope,
}

impl<'a> WorthQueryManagedLowerBinding<'a> {
    pub(in crate::domain_computation) const fn new(
        operation_identity: &'a str,
        resource_attempt_identity: &'a str,
        resource_envelope: &'a WorthQueryExecutionResourceEnvelope,
    ) -> Self {
        Self {
            operation_identity,
            resource_attempt_identity,
            resource_envelope,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryManagedLowerAdmissionFailureKind {
    BridgeSourceProfile,
    RelationalBasis,
    BridgePlanning,
    InstalledStepContract,
    BridgeExecutionBasis,
}

pub(in crate::domain_computation) struct WorthQueryManagedLowerAdmissionFailure {
    pub kind: WorthQueryManagedLowerAdmissionFailureKind,
    pub detail: Arc<str>,
}

pub(in crate::domain_computation) fn admit_managed_lower_execution_basis(
    bridge: &RuntimeBridge,
    relational: &RuntimeBridgeRelationalSource,
    binding: WorthQueryManagedLowerBinding<'_>,
    request: WorthQueryManagedTruthReadRequest,
) -> Result<WorthQueryManagedLowerExecutionBasis, WorthQueryManagedLowerAdmissionFailure> {
    let expected_source = relational.authoritative_source_profile();
    if bridge.authoritative_source_profile() != Some(&expected_source) {
        return Err(WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::BridgeSourceProfile,
            detail: Arc::from(
                "Bridge runtime and Relational execution source do not share one authoritative adapter",
            ),
        });
    }
    let (version_id, branch, packet, replay, diagnostics, delivery) = request.into_parts();
    let relational_branch = branch
        .relational_branch_id()
        .map(|branch| BranchId(branch.to_owned()))
        .ok_or_else(|| WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
            detail: Arc::from("managed truth branch is not a Relational branch identity"),
        })?;
    let relational_basis = relational
        .admit_execution_basis(&relational_branch, version_id)
        .map_err(|denial| WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
            detail: Arc::from(denial.detail()),
        })?;
    let snapshot = bridge_snapshot_identity_for_handle(relational_basis.snapshot_handle());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(branch.clone(), snapshot.clone()),
        replay,
        diagnostics,
        delivery,
    );
    let planned = bridge
        .plan_truth_view_packet(declaration, packet)
        .map_err(|failure| WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::BridgePlanning,
            detail: Arc::from(format!("{failure:?}")),
        })?;
    let bridge_step = lower_installed_step_contract(binding.resource_envelope)?;
    let bridge_basis = bridge
        .admit_managed_execution_basis(
            BridgeManagedExecutionIntent::new(
                binding.operation_identity,
                binding.resource_attempt_identity,
            ),
            bridge_step,
            BridgeAsyncRequestTruthViewBasis::branch_head(branch, snapshot),
            planned,
        )
        .map_err(|denial| WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::BridgeExecutionBasis,
            detail: Arc::from(denial.detail()),
        })?;
    Ok(WorthQueryManagedLowerExecutionBasis {
        bridge: bridge_basis,
        relational: relational_basis,
    })
}

fn lower_installed_step_contract(
    resource_envelope: &WorthQueryExecutionResourceEnvelope,
) -> Result<BridgeManagedExecutionStepContract, WorthQueryManagedLowerAdmissionFailure> {
    let installed = resource_envelope
        .bounded_step_contract()
        .map_err(|detail| WorthQueryManagedLowerAdmissionFailure {
            kind: WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract,
            detail: Arc::from(detail),
        })?;
    let limits = BridgeManagedExecutionStepLimits::new(
        installed.max_work_units_per_step(),
        installed.queue_depth_ceiling(),
        installed.chunk_width_ceiling(),
    )
    .with_memory_ceilings(
        installed.scratch_bytes_ceiling(),
        installed.retained_bytes_ceiling(),
    )
    .with_deadline_nanos(installed.deadline_nanos());
    let partial_effects = if installed.partial_effects_may_remain() {
        BridgeManagedExecutionPartialEffectPosture::MayRemain
    } else {
        BridgeManagedExecutionPartialEffectPosture::None
    };
    BridgeManagedExecutionStepContract::new(
        installed.safe_point_family().as_str(),
        limits,
        partial_effects,
    )
    .map_err(|detail| WorthQueryManagedLowerAdmissionFailure {
        kind: WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract,
        detail: Arc::from(detail),
    })
}
