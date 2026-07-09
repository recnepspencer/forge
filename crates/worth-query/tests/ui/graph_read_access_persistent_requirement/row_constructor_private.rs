use worth_query::facade::runtime::{
    WorthQueryGraphIndexInventoryMatchOutcome, WorthQueryGraphIndexLifecycleClass,
    WorthQueryGraphIndexLifecycleOwner, WorthQueryGraphIndexSupportState,
    WorthQueryGraphReadAccessAdmissionPosture, WorthQueryGraphReadAccessComplexityContract,
    WorthQueryGraphReadAccessInvalidationBasis, WorthQueryGraphReadAccessRebuildBasis,
    WorthQueryGraphReadAccessRequirementKind, WorthQueryGraphReadRequiredCapabilityOwner,
    WorthQueryPersistentGraphIndexRequirementRow,
};

fn main() {
    let _ = WorthQueryPersistentGraphIndexRequirementRow {
        digest: String::new(),
        requirement_kind: WorthQueryGraphReadAccessRequirementKind::PredicateSupport,
        requirement_row_digest: String::new(),
        requirement_semantic_slot: String::new(),
        support_row_digest: String::new(),
        match_outcome: WorthQueryGraphIndexInventoryMatchOutcome::ExactMatch,
        support_state: WorthQueryGraphIndexSupportState::StoreOwnedUnavailable,
        lifecycle_owner: WorthQueryGraphIndexLifecycleOwner::StoreOwned,
        lifecycle_class: WorthQueryGraphIndexLifecycleClass::StoreOwnedRequired,
        rebuild_basis: WorthQueryGraphReadAccessRebuildBasis::SelectivityProof,
        invalidation_basis: WorthQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        complexity_contract: WorthQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
        owning_milestone: Some(String::new()),
        required_owner: WorthQueryGraphReadRequiredCapabilityOwner::PersistentStore,
        required_posture: WorthQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    };
}
