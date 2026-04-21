use crate::failure::StoreError;
use crate::tiering::{
    AuthoritativePlacementPlanningReport, DerivedPlacementPlanningReport,
    ReadPlacementPlanningReport,
};

use super::ForgeStore;

impl ForgeStore {
    pub fn observe_working_set(
        &self,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
    ) -> Result<crate::WorkingSetObservationWindow, StoreError> {
        self.backend.observe_working_set(scope_class, scope_key)
    }

    pub fn summarize_placement_demand(
        &self,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
    ) -> Result<crate::PlacementDemandSummary, StoreError> {
        self.backend
            .summarize_placement_demand(scope_class, scope_key)
    }

    pub fn plan_authoritative_tier_move(
        &self,
        policy_class: crate::PlacementPolicyClass,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<AuthoritativePlacementPlanningReport, StoreError> {
        self.backend.plan_authoritative_tier_move(
            policy_class,
            scope_class,
            scope_key,
            execution_origin,
        )
    }

    pub fn plan_derived_tier_move(
        &self,
        policy_class: crate::PlacementPolicyClass,
        family: crate::ColdDerivedFamilyPolicy,
        artifact_id: &str,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<DerivedPlacementPlanningReport, StoreError> {
        self.backend
            .plan_derived_tier_move(policy_class, family, artifact_id, execution_origin)
    }

    pub fn plan_resident_read_lease(
        &self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<ReadPlacementPlanningReport, StoreError> {
        self.backend
            .plan_resident_read_lease(artifact_ref, execution_origin)
    }

    pub fn plan_cold_recall_lease(
        &self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<ReadPlacementPlanningReport, StoreError> {
        self.backend
            .plan_cold_recall_lease(artifact_ref, execution_origin)
    }

    pub fn plan_broadened_recall(
        &self,
        family: crate::ColdDerivedFamilyPolicy,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
        widened_artifact_keys: Vec<String>,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<crate::BroadenedRecallPlan, StoreError> {
        self.backend.plan_broadened_recall(
            family,
            scope_class,
            scope_key,
            widened_artifact_keys,
            execution_origin,
        )
    }

    pub fn canonical_residency_manifest(&self) -> crate::CanonicalResidencyManifest {
        self.backend.canonical_residency_manifest()
    }

    pub fn recover_tiering_state(&self) -> Result<crate::CanonicalResidencyManifest, StoreError> {
        self.backend.recover_tiering_state()
    }

    pub fn resolve_resident_read_handle(
        &self,
        lease: &crate::ResidentReadLease,
    ) -> crate::PlacementResolvedReadHandle {
        self.backend.resolve_resident_read_handle(lease)
    }

    pub fn resolve_cold_recall_read_handle(
        &self,
        lease: &crate::ColdRecallLease,
    ) -> crate::PlacementResolvedReadHandle {
        self.backend.resolve_cold_recall_read_handle(lease)
    }

    pub fn observe_placement_read_interleaving(
        &self,
        handle: &crate::PlacementResolvedReadHandle,
    ) -> Result<crate::InterleavedReadParityReport, StoreError> {
        self.backend.observe_placement_read_interleaving(handle)
    }

    pub fn observe_stable_basis_interleaving(
        &self,
        basis: &crate::StableBasisHandle,
    ) -> Result<crate::InterleavedReadParityReport, StoreError> {
        self.backend.observe_stable_basis_interleaving(basis)
    }

    pub fn observe_continuation_interleaving(
        &self,
        plan: &crate::CursorContinuationPlan,
        result: Option<&crate::ContinuationBatchResult>,
    ) -> Result<crate::InterleavedContinuationParityReport, StoreError> {
        self.backend.observe_continuation_interleaving(plan, result)
    }

    pub fn prepare_authoritative_tier_move(
        &mut self,
        plan: crate::AuthoritativeTierMovePlan,
    ) -> Result<crate::TierTransferIntent, StoreError> {
        self.backend.prepare_authoritative_tier_move(plan)
    }

    pub fn prepare_derived_tier_move(
        &mut self,
        plan: crate::DerivedTierMovePlan,
    ) -> Result<crate::TierTransferIntent, StoreError> {
        self.backend.prepare_derived_tier_move(plan)
    }

    pub fn transfer_tier_replica(
        &mut self,
        intent: crate::TierTransferIntent,
    ) -> Result<crate::TransferredTierReplica, StoreError> {
        self.backend.transfer_tier_replica(intent)
    }

    pub fn verify_tier_replica(
        &mut self,
        transferred: crate::TransferredTierReplica,
    ) -> Result<crate::VerifiedTierReplica, StoreError> {
        self.backend.verify_tier_replica(transferred)
    }

    pub fn cutover_tier_replica(
        &mut self,
        verified: crate::VerifiedTierReplica,
    ) -> Result<crate::TierCutoverWitness, StoreError> {
        self.backend.cutover_tier_replica(verified)
    }

    pub fn retire_tier_replica(
        &mut self,
        cutover: crate::TierCutoverWitness,
    ) -> Result<crate::RetiredTierReplica, StoreError> {
        self.backend.retire_tier_replica(cutover)
    }

    pub fn execute_cold_recall(
        &mut self,
        lease: crate::ColdRecallLease,
        witness: crate::RecallEligibilityWitness,
    ) -> Result<crate::CoalescedRecallReport, StoreError> {
        self.backend.execute_cold_recall(lease, witness)
    }

    #[cfg(test)]
    pub(crate) fn admit_inflight_cold_recall(
        &mut self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<(), StoreError> {
        self.backend
            .admit_inflight_cold_recall(artifact_ref, execution_origin)
    }
}
