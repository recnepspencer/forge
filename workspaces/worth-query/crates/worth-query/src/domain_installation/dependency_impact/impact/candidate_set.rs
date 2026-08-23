use crate::domain_installation::operation_authority_chain::WorthQueryOperationAuthorityBasis;

use super::super::{
    WorthQueryCompiledSemanticAspectDependencyClosure, WorthQuerySemanticDependencyRole,
};
use super::WorthQueryImpactAdmissionDenial;

pub struct WorthQueryInvalidationCandidateSet {
    pub(super) affinity: WorthQueryOperationAuthorityBasis,
    pub(super) delivery: worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
    pub(super) roles: Vec<WorthQuerySemanticDependencyRole>,
    index_lookups: usize,
}

impl WorthQueryInvalidationCandidateSet {
    pub fn roles(&self) -> &[WorthQuerySemanticDependencyRole] {
        &self.roles
    }

    pub const fn index_lookups(&self) -> usize {
        self.index_lookups
    }

    pub fn truth(&self) -> &worth_runtime_bridge::facade::BridgeDeliveredTruthChange {
        self.delivery.truth()
    }

    pub fn has_performed_signal_consequence(&self) -> bool {
        self.delivery.performed_signal().is_some()
    }
}

pub fn select_invalidation_candidates(
    closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
    delivery: worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
) -> Result<WorthQueryInvalidationCandidateSet, WorthQueryImpactAdmissionDenial> {
    super::classification::preflight_owner_delivered_truth(closure, delivery.truth())?;
    select_preflighted_invalidation_candidates(closure, delivery)
}

pub(crate) fn select_preflighted_invalidation_candidates(
    closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
    delivery: worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
) -> Result<WorthQueryInvalidationCandidateSet, WorthQueryImpactAdmissionDenial> {
    let dependency = delivery.truth().change_set().dependency();
    let location =
        crate::domain_installation::conditional_execution::query_location_from_bridge_candidate(
            dependency,
        );
    if !closure
        .invalidation_manifest()
        .admits_bridge_dependency(&location, dependency)
    {
        return Err(super::WorthQueryImpactAdmissionDenial::new(
            super::WorthQueryImpactAdmissionDenialKind::ConditionalDeliveryMismatch,
            super::WorthQueryImpactCounters {
                dependency_membership_lookups: 1,
                ..Default::default()
            },
        ));
    }
    let (roles, index_lookups) = closure
        .invalidation_manifest()
        .select_bridge_roles(
            &location,
            dependency,
            delivery.truth().change_set().changes(),
            delivery.performed_signal().is_some(),
        )
        .ok_or_else(|| {
            super::WorthQueryImpactAdmissionDenial::new(
                super::WorthQueryImpactAdmissionDenialKind::ConditionalDeliveryMismatch,
                super::WorthQueryImpactCounters {
                    dependency_membership_lookups: 1,
                    ..Default::default()
                },
            )
        })?;
    Ok(WorthQueryInvalidationCandidateSet {
        affinity: closure.affinity.clone(),
        delivery,
        roles,
        index_lookups,
    })
}

pub(crate) fn select_bound_primary_invalidation_candidates(
    closure: &WorthQueryCompiledSemanticAspectDependencyClosure,
    consumer_dependencies: &[&worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate],
    candidate_index_lookups: usize,
    delivery: worth_runtime_bridge::facade::BridgeGranularInvalidationDelivery,
) -> Result<WorthQueryInvalidationCandidateSet, WorthQueryImpactAdmissionDenial> {
    let dependency = delivery.truth().change_set().dependency();
    let (roles, index_lookups) = closure
        .invalidation_manifest()
        .select_bound_primary_roles(
            consumer_dependencies,
            dependency,
            delivery.truth().change_set().changes(),
            delivery.performed_signal().is_some(),
            candidate_index_lookups,
        )
        .unwrap_or_else(|| (Vec::new(), 1));
    Ok(WorthQueryInvalidationCandidateSet {
        affinity: closure.affinity.clone(),
        delivery,
        roles,
        index_lookups,
    })
}
