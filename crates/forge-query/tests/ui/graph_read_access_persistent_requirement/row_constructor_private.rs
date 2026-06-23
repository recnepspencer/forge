use forge_query::facade::runtime::{
    ForgeQueryGraphIndexInventoryMatchOutcome, ForgeQueryGraphIndexLifecycleClass,
    ForgeQueryGraphIndexLifecycleOwner, ForgeQueryGraphIndexSupportState,
    ForgeQueryGraphReadAccessAdmissionPosture, ForgeQueryGraphReadAccessComplexityContract,
    ForgeQueryGraphReadAccessInvalidationBasis, ForgeQueryGraphReadAccessRebuildBasis,
    ForgeQueryGraphReadAccessRequirementKind, ForgeQueryGraphReadRequiredCapabilityOwner,
    ForgeQueryPersistentGraphIndexRequirementRow,
};

fn main() {
    let _ = ForgeQueryPersistentGraphIndexRequirementRow {
        digest: String::new(),
        requirement_kind: ForgeQueryGraphReadAccessRequirementKind::PredicateSupport,
        requirement_row_digest: String::new(),
        requirement_semantic_slot: String::new(),
        support_row_digest: String::new(),
        match_outcome: ForgeQueryGraphIndexInventoryMatchOutcome::ExactMatch,
        support_state: ForgeQueryGraphIndexSupportState::StoreOwnedUnavailable,
        lifecycle_owner: ForgeQueryGraphIndexLifecycleOwner::StoreOwned,
        lifecycle_class: ForgeQueryGraphIndexLifecycleClass::StoreOwnedRequired,
        rebuild_basis: ForgeQueryGraphReadAccessRebuildBasis::SelectivityProof,
        invalidation_basis: ForgeQueryGraphReadAccessInvalidationBasis::AuthoritativeFieldDelta,
        complexity_contract: ForgeQueryGraphReadAccessComplexityContract::CandidatePredicateSupport,
        owning_milestone: Some(String::new()),
        required_owner: ForgeQueryGraphReadRequiredCapabilityOwner::PersistentStore,
        required_posture: ForgeQueryGraphReadAccessAdmissionPosture::PersistentIndexRequired,
    };
}
