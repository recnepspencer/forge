use crate::identity::hash_parts;
use crate::live::{
    BridgeSliceCategory, LiveQueryFamily, LiveQueryPlan, LocalityAdmissionClass,
    LocalityAwareRelevanceContract, LocalityBreadthBudget, LocalityCostPosture,
    LocalityMaintenanceClass, LocalityPerformanceStatus, LocalityPredicateContract,
    LocalityScopeAdmission, LocalityScopeKind, LocalitySemanticBasis, LocalityWideningBudget,
    LocalityWideningPolicy, QueryRelevanceContract, RegionScopedLiveError, RegionScopedLivePlan,
    RegionScopedPlanningReport, RegionScopedSubscriptionIdentity, StreamLoweringAdmissionClass,
    StreamLoweringCostPosture, StreamMemberWidthBudget, StreamWindowWidthBudget,
};

#[cfg(test)]
pub(crate) fn admit_region_scoped_live_plan(
    live: &LiveQueryPlan,
    locality: LocalityPredicateContract,
) -> Result<RegionScopedLivePlan, RegionScopedLiveError> {
    let semantic_basis = derive_locality_semantic_basis(live.descriptor().relevance_contract());
    let scope_admission = derive_locality_scope_admission(live.descriptor().relevance_contract());
    let admission_class =
        derive_locality_admission_class(&semantic_basis, &scope_admission, &locality)?;
    let stream_lowering_admission =
        derive_stream_lowering_admission_class(&semantic_basis, &admission_class);

    let locality_digest = locality.digest().as_str().to_string();
    let query_digest = live.descriptor().query_digest().as_str().to_string();
    let locality_subscription_digest = hash_parts(&[
        format!("subscription:{}", live.subscription_digest().as_str()),
        format!("locality:{}", locality_digest),
        format!("admission:{}", admission_class.as_str()),
    ]);

    let (
        locality_cost_posture,
        locality_breadth_budget,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
        stream_window_width_budget,
    ) = match admission_class {
        LocalityAdmissionClass::DetailRegion | LocalityAdmissionClass::DetailPartition => (
            LocalityCostPosture::SingleSliceNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget { limit: 1 },
            StreamLoweringCostPosture::SingleDetailCurrentStateMember,
            StreamMemberWidthBudget::single_member(),
            StreamWindowWidthBudget::single_window(),
        ),
        LocalityAdmissionClass::OrderedCollectionPartition => (
            LocalityCostPosture::PartitionScopedMembershipNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::CdcPatchWithProjectedDeltas,
            StreamMemberWidthBudget::cdc_projected_patch(),
            StreamWindowWidthBudget::single_window(),
        ),
        LocalityAdmissionClass::BoundedMaterializationRegion => (
            LocalityCostPosture::BoundedTraversalRegionNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::BoundedMaterializationDeferred,
            StreamMemberWidthBudget::single_member(),
            StreamWindowWidthBudget::single_window(),
        ),
    };

    let subscription_identity = RegionScopedSubscriptionIdentity {
        digest: locality_subscription_digest.clone(),
        query_digest: query_digest.clone(),
        locality_digest: locality_digest.clone(),
        admission_class: admission_class.clone(),
    };
    let relevance_contract = LocalityAwareRelevanceContract {
        digest: hash_parts(&[
            format!("query:{query_digest}"),
            format!("locality:{locality_digest}"),
            format!("admission:{}", admission_class.as_str()),
            format!("semantic_basis:{}", semantic_basis.as_str()),
            format!("scope_admission:{}", scope_admission.as_str()),
            format!(
                "maintenance:{}",
                LocalityMaintenanceClass::NarrowPatch.as_str()
            ),
            format!(
                "stream_lowering_admission:{}",
                stream_lowering_admission.as_str()
            ),
            format!(
                "slice_category:{}",
                match locality.scope_kind() {
                    LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion.as_str(),
                    LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition.as_str(),
                }
            ),
        ]),
        locality_digest: locality_digest.clone(),
        admission_class: admission_class.clone(),
        semantic_basis: semantic_basis.clone(),
        scope_admission: scope_admission.clone(),
        maintenance_class: LocalityMaintenanceClass::NarrowPatch,
        stream_lowering_admission: stream_lowering_admission.clone(),
        expected_slice_category: match locality.scope_kind() {
            LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion,
            LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition,
        },
    };
    let locality_widening_policy = match admission_class {
        LocalityAdmissionClass::DetailRegion | LocalityAdmissionClass::DetailPartition => {
            LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice
        }
        LocalityAdmissionClass::OrderedCollectionPartition
        | LocalityAdmissionClass::BoundedMaterializationRegion => LocalityWideningPolicy::DenyAll,
    };
    let locality_performance_status = LocalityPerformanceStatus::VerifiedNarrowing;
    let planning_report = RegionScopedPlanningReport {
        query_digest: query_digest.clone(),
        locality_digest: locality_digest.clone(),
        subscription_identity_digest: subscription_identity.digest().to_string(),
        relevance_contract_digest: relevance_contract.digest().to_string(),
        semantic_basis,
        scope_admission,
        stream_lowering_admission,
        widening_policy: locality_widening_policy.clone(),
        performance_status: locality_performance_status.clone(),
    };

    Ok(RegionScopedLivePlan {
        live: live.clone(),
        locality,
        admission_class,
        subscription_identity,
        relevance_contract,
        planning_report,
        locality_cost_posture,
        locality_performance_status,
        locality_breadth_budget,
        locality_widening_policy,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
        stream_window_width_budget,
    })
}

#[cfg(test)]
fn derive_locality_semantic_basis(
    relevance_contract: &QueryRelevanceContract,
) -> LocalitySemanticBasis {
    if !relevance_contract.traversal_relations().is_empty() {
        LocalitySemanticBasis::BoundedTraversalMaterialization
    } else if relevance_contract.family() == &LiveQueryFamily::OrderedCollection
        || !relevance_contract.ordering_fields().is_empty()
    {
        LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering
    } else {
        LocalitySemanticBasis::DetailProjectionFields
    }
}

#[cfg(test)]
fn derive_locality_scope_admission(
    relevance_contract: &QueryRelevanceContract,
) -> LocalityScopeAdmission {
    match derive_locality_semantic_basis(relevance_contract) {
        LocalitySemanticBasis::DetailProjectionFields => LocalityScopeAdmission::RegionOrPartition,
        LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering => {
            LocalityScopeAdmission::PartitionOnly
        }
        LocalitySemanticBasis::BoundedTraversalMaterialization => {
            LocalityScopeAdmission::RegionOnly
        }
    }
}

#[cfg(test)]
fn derive_locality_admission_class(
    semantic_basis: &LocalitySemanticBasis,
    scope_admission: &LocalityScopeAdmission,
    locality: &LocalityPredicateContract,
) -> Result<LocalityAdmissionClass, RegionScopedLiveError> {
    match (semantic_basis, scope_admission, locality.scope_kind()) {
        (
            LocalitySemanticBasis::DetailProjectionFields,
            LocalityScopeAdmission::RegionOrPartition,
            LocalityScopeKind::Region,
        ) => Ok(LocalityAdmissionClass::DetailRegion),
        (
            LocalitySemanticBasis::DetailProjectionFields,
            LocalityScopeAdmission::RegionOrPartition,
            LocalityScopeKind::Partition,
        ) => Ok(LocalityAdmissionClass::DetailPartition),
        (
            LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering,
            LocalityScopeAdmission::PartitionOnly,
            LocalityScopeKind::Partition,
        ) => Ok(LocalityAdmissionClass::OrderedCollectionPartition),
        (
            LocalitySemanticBasis::BoundedTraversalMaterialization,
            LocalityScopeAdmission::RegionOnly,
            LocalityScopeKind::Region,
        ) => Ok(LocalityAdmissionClass::BoundedMaterializationRegion),
        _ => Err(RegionScopedLiveError::UnsupportedLocalityPredicate),
    }
}

#[cfg(test)]
fn derive_stream_lowering_admission_class(
    semantic_basis: &LocalitySemanticBasis,
    admission_class: &LocalityAdmissionClass,
) -> StreamLoweringAdmissionClass {
    match (semantic_basis, admission_class) {
        (LocalitySemanticBasis::DetailProjectionFields, _)
        | (_, LocalityAdmissionClass::DetailRegion)
        | (_, LocalityAdmissionClass::DetailPartition) => {
            StreamLoweringAdmissionClass::DetailCurrentStateOnly
        }
        (
            LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering,
            LocalityAdmissionClass::OrderedCollectionPartition,
        ) => StreamLoweringAdmissionClass::CollectionCdcProjectedPatchOnly,
        (
            LocalitySemanticBasis::BoundedTraversalMaterialization,
            LocalityAdmissionClass::BoundedMaterializationRegion,
        ) => StreamLoweringAdmissionClass::DeferredBoundedMaterialization,
        _ => StreamLoweringAdmissionClass::DetailCurrentStateOnly,
    }
}
