use crate::domain_installation::operation_authority_chain::{
    mint_operation_phase_proof, operation_phase_basis, WorthQueryImpactClassifiedPhase,
    WorthQueryOperationPhaseProof,
};
use crate::domain_installation::{
    WorthQueryCompiledSemanticAspectDependencyClosure, WorthQueryConditionalProvenance,
    WorthQueryImpactAdmissionDenial, WorthQueryImpactAdmissionDenialKind, WorthQueryImpactCounters,
};

enum WorthQueryCheckedImpactDeliveryBasis {
    Managed {
        delivery: crate::ordinary::live::WorthQueryManagedLiveDelivery,
    },
    OwnerConditional {
        commit_identity: worth_runtime_bridge::facade::TruthCommitIdentity,
        snapshot_identity: worth_runtime_bridge::facade::TruthSnapshotIdentity,
        dependency: worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
        conditional_lowering:
            std::sync::Arc<worth_runtime_bridge::facade::BridgeInstalledConditionalLowering>,
        conditional_location: worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    },
}

/// Private checked basis carried by an impact decision. The public impact
/// class and counters remain descriptive and cannot be readmitted on their own.
pub(super) struct WorthQueryCheckedImpactBasis {
    phase: WorthQueryOperationPhaseProof<WorthQueryImpactClassifiedPhase>,
    delivery: WorthQueryCheckedImpactDeliveryBasis,
}

impl std::fmt::Debug for WorthQueryCheckedImpactBasis {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("WorthQueryCheckedImpactBasis")
            .field(
                "operation",
                &operation_phase_basis(&self.phase).operation_identity,
            )
            .finish_non_exhaustive()
    }
}

impl WorthQueryCheckedImpactBasis {
    pub(super) fn managed(
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        delivery: &crate::ordinary::live::WorthQueryManagedLiveDelivery,
    ) -> Self {
        Self {
            phase: mint_operation_phase_proof(
                format!("impact-managed:{}", closure.basis_identity()),
                None,
                closure.affinity.clone(),
            ),
            delivery: WorthQueryCheckedImpactDeliveryBasis::Managed {
                delivery: delivery.clone(),
            },
        }
    }

    pub(super) fn owner_conditional(
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        receipt: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        conditional: &WorthQueryConditionalProvenance,
    ) -> Self {
        let change_set = receipt.change_set();
        Self {
            phase: mint_operation_phase_proof(
                format!(
                    "impact-owner:{}:{}",
                    closure.basis_identity(),
                    change_set.dependency().dependency_ordinal(),
                ),
                None,
                closure.affinity.clone(),
            ),
            delivery: WorthQueryCheckedImpactDeliveryBasis::OwnerConditional {
                commit_identity: change_set.commit_identity().clone(),
                snapshot_identity: change_set.snapshot_identity().clone(),
                dependency: change_set.dependency().clone(),
                conditional_lowering: std::sync::Arc::clone(&conditional._lowering),
                conditional_location: conditional.location().clone(),
            },
        }
    }

    pub(super) fn readmit_owner_conditional(
        &self,
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        receipt: &worth_runtime_bridge::facade::BridgeCorrespondenceDeliveryReceipt,
        conditional: &WorthQueryConditionalProvenance,
    ) -> Result<(), WorthQueryImpactAdmissionDenial> {
        let mut counters = WorthQueryImpactCounters::default();
        counters.operation_affinity_checks += 1;
        if operation_phase_basis(&self.phase) != &closure.affinity {
            return Err(WorthQueryImpactAdmissionDenial::new(
                WorthQueryImpactAdmissionDenialKind::ForeignOperation,
                counters,
            ));
        }
        counters.conditional_authority_checks += 1;
        let WorthQueryCheckedImpactDeliveryBasis::OwnerConditional {
            commit_identity,
            snapshot_identity,
            dependency,
            conditional_lowering,
            conditional_location,
        } = &self.delivery
        else {
            return Err(WorthQueryImpactAdmissionDenial::new(
                WorthQueryImpactAdmissionDenialKind::ConditionalAuthorityMismatch,
                counters,
            ));
        };
        let candidate = receipt.change_set();
        counters.delivery_identity_checks += 1;
        if !candidate.retains_delivery_identity(commit_identity, snapshot_identity) {
            return Err(delivery_mismatch(counters));
        }
        counters.dependency_membership_lookups += 1;
        if !candidate
            .dependency()
            .retains_same_installed_authority_as(dependency)
        {
            return Err(delivery_mismatch(counters));
        }
        counters.conditional_location_checks += 1;
        if conditional.location() != conditional_location {
            return Err(delivery_mismatch(counters));
        }
        counters.conditional_authority_checks += 1;
        if operation_phase_basis(&conditional._admission) != &closure.affinity {
            return Err(delivery_mismatch(counters));
        }
        counters.delivery_identity_checks += 1;
        if !conditional
            .bridge
            .retains_exact_lowering(conditional_lowering)
        {
            return Err(delivery_mismatch(counters));
        }
        counters.conditional_authority_checks += 1;
        if conditional.bridge.query_binding_identity() != closure.affinity.binding_identity {
            return Err(delivery_mismatch(counters));
        }
        counters.conditional_authority_checks += 1;
        if conditional.bridge.query_capability_identity() != closure.affinity.capability_identity {
            return Err(delivery_mismatch(counters));
        }
        counters.delivery_identity_checks += 1;
        if !conditional
            .bridge
            .retains_bridge_snapshot_identity(snapshot_identity)
        {
            return Err(delivery_mismatch(counters));
        }
        Ok(())
    }

    pub(super) fn readmit_managed(
        &self,
        closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
        delivery: &crate::ordinary::live::WorthQueryManagedLiveDelivery,
    ) -> bool {
        operation_phase_basis(&self.phase) == &closure.affinity
            && match &self.delivery {
                WorthQueryCheckedImpactDeliveryBasis::Managed { delivery: checked } => {
                    checked.is_same_retained_delivery_as(delivery)
                }
                WorthQueryCheckedImpactDeliveryBasis::OwnerConditional { .. } => false,
            }
    }
}

const fn delivery_mismatch(counters: WorthQueryImpactCounters) -> WorthQueryImpactAdmissionDenial {
    WorthQueryImpactAdmissionDenial::new(
        WorthQueryImpactAdmissionDenialKind::ConditionalDeliveryMismatch,
        counters,
    )
}
