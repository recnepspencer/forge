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
    relational_basis: Option<RelationalExecutionBasisLease>,
}

impl WorthQueryManagedLowerAdmissionFailure {
    fn before_basis(
        kind: WorthQueryManagedLowerAdmissionFailureKind,
        detail: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            relational_basis: None,
        }
    }

    fn with_basis(
        kind: WorthQueryManagedLowerAdmissionFailureKind,
        detail: impl Into<Arc<str>>,
        relational_basis: RelationalExecutionBasisLease,
    ) -> Self {
        Self {
            kind,
            detail: detail.into(),
            relational_basis: Some(relational_basis),
        }
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        WorthQueryManagedLowerAdmissionFailureKind,
        Arc<str>,
        Option<RelationalExecutionBasisLease>,
    ) {
        (self.kind, self.detail, self.relational_basis)
    }
}

pub(in crate::domain_computation) fn admit_managed_lower_execution_basis(
    bridge: &RuntimeBridge,
    relational: &RuntimeBridgeRelationalSource,
    binding: WorthQueryManagedLowerBinding<'_>,
    request: WorthQueryManagedTruthReadRequest,
) -> Result<WorthQueryManagedLowerExecutionBasis, WorthQueryManagedLowerAdmissionFailure> {
    validate_source_profile(bridge, relational)?;
    let relational_branch = request
        .branch()
        .relational_branch_id()
        .map(|branch| BranchId(branch.to_owned()))
        .ok_or_else(|| {
            WorthQueryManagedLowerAdmissionFailure::before_basis(
                WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
                "managed truth branch is not a Relational branch identity",
            )
        })?;
    let relational_basis = relational
        .admit_execution_basis(&relational_branch, request.relational_version_id())
        .map_err(|denial| {
            WorthQueryManagedLowerAdmissionFailure::before_basis(
                WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
                denial.detail(),
            )
        })?;
    bind_managed_lower_execution_basis(bridge, binding, relational_basis, request)
}

pub(in crate::domain_computation) fn admit_managed_lower_execution_basis_from_retained(
    bridge: &RuntimeBridge,
    relational: &RuntimeBridgeRelationalSource,
    binding: WorthQueryManagedLowerBinding<'_>,
    request: WorthQueryManagedTruthReadRequest,
    relational_basis: RelationalExecutionBasisLease,
) -> Result<WorthQueryManagedLowerExecutionBasis, WorthQueryManagedLowerAdmissionFailure> {
    if let Err(failure) = validate_source_profile(bridge, relational) {
        return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
            failure.kind,
            failure.detail,
            relational_basis,
        ));
    }
    let requested_branch = match request.branch().relational_branch_id() {
        Some(branch) => BranchId(branch.to_owned()),
        None => {
            return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
                WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
                "managed truth branch is not a Relational branch identity",
                relational_basis,
            ));
        }
    };
    if relational_basis.identity().branch_id() != &requested_branch
        || relational_basis.version_id() != request.relational_version_id()
        || !relational_basis.is_live()
        || relational_basis.identity().runtime_instance_id()
            != relational
                .authoritative_source_profile()
                .runtime_instance_id()
    {
        return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
            WorthQueryManagedLowerAdmissionFailureKind::RelationalBasis,
            "retained Relational basis does not match the exact managed branch and version request",
            relational_basis,
        ));
    }
    bind_managed_lower_execution_basis(bridge, binding, relational_basis, request)
}

fn bind_managed_lower_execution_basis(
    bridge: &RuntimeBridge,
    binding: WorthQueryManagedLowerBinding<'_>,
    relational_basis: RelationalExecutionBasisLease,
    request: WorthQueryManagedTruthReadRequest,
) -> Result<WorthQueryManagedLowerExecutionBasis, WorthQueryManagedLowerAdmissionFailure> {
    let (_, branch, packet, replay, diagnostics, delivery) = request.into_parts();
    let snapshot = bridge_snapshot_identity_for_handle(relational_basis.snapshot_handle());
    let declaration = HistoricalEvaluationDeclaration::new(
        BridgeTruthViewSelector::branch_snapshot(branch.clone(), snapshot.clone()),
        replay,
        diagnostics,
        delivery,
    );
    let planned = match bridge.plan_truth_view_packet(declaration, packet) {
        Ok(planned) => planned,
        Err(failure) => {
            return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
                WorthQueryManagedLowerAdmissionFailureKind::BridgePlanning,
                format!("{failure:?}"),
                relational_basis,
            ));
        }
    };
    let bridge_step = match lower_installed_step_contract(binding.resource_envelope) {
        Ok(step) => step,
        Err(failure) => {
            return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
                failure.kind,
                failure.detail,
                relational_basis,
            ));
        }
    };
    let bridge_basis = match bridge.admit_managed_execution_basis(
        BridgeManagedExecutionIntent::new(
            binding.operation_identity,
            binding.resource_attempt_identity,
        ),
        bridge_step,
        BridgeAsyncRequestTruthViewBasis::branch_head(branch, snapshot),
        planned,
    ) {
        Ok(basis) => basis,
        Err(denial) => {
            return Err(WorthQueryManagedLowerAdmissionFailure::with_basis(
                WorthQueryManagedLowerAdmissionFailureKind::BridgeExecutionBasis,
                denial.detail(),
                relational_basis,
            ));
        }
    };
    Ok(WorthQueryManagedLowerExecutionBasis {
        bridge: bridge_basis,
        relational: relational_basis,
    })
}

fn validate_source_profile(
    bridge: &RuntimeBridge,
    relational: &RuntimeBridgeRelationalSource,
) -> Result<(), WorthQueryManagedLowerAdmissionFailure> {
    let expected_source = relational.authoritative_source_profile();
    if bridge.authoritative_source_profile() == Some(&expected_source) {
        Ok(())
    } else {
        Err(WorthQueryManagedLowerAdmissionFailure::before_basis(
            WorthQueryManagedLowerAdmissionFailureKind::BridgeSourceProfile,
            "Bridge runtime and Relational execution source do not share one authoritative adapter",
        ))
    }
}

fn lower_installed_step_contract(
    resource_envelope: &WorthQueryExecutionResourceEnvelope,
) -> Result<BridgeManagedExecutionStepContract, WorthQueryManagedLowerAdmissionFailure> {
    let installed = resource_envelope
        .bounded_step_contract()
        .map_err(|detail| {
            WorthQueryManagedLowerAdmissionFailure::before_basis(
                WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract,
                detail,
            )
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
    .map_err(|detail| {
        WorthQueryManagedLowerAdmissionFailure::before_basis(
            WorthQueryManagedLowerAdmissionFailureKind::InstalledStepContract,
            detail,
        )
    })
}
