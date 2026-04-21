use crate::failure::StoreError;
use crate::tiering::{
    AuthoritativePlacementPlanningReport, DerivedPlacementPlanningReport,
    ReadPlacementPlanningReport,
};

use super::{StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn observe_working_set(
        &self,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
    ) -> Result<crate::WorkingSetObservationWindow, StoreError> {
        super::super::tiering::observe_working_set(self, scope_class, scope_key)
    }

    pub fn summarize_placement_demand(
        &self,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
    ) -> Result<crate::PlacementDemandSummary, StoreError> {
        super::super::tiering::summarize_placement_demand(self, scope_class, scope_key)
    }

    pub fn plan_authoritative_tier_move(
        &self,
        policy_class: crate::PlacementPolicyClass,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<AuthoritativePlacementPlanningReport, StoreError> {
        super::super::tiering::plan_authoritative_tier_move(
            self,
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
        super::super::tiering::plan_derived_tier_move(
            self,
            policy_class,
            family,
            artifact_id,
            execution_origin,
        )
    }

    pub fn plan_resident_read_lease(
        &self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<ReadPlacementPlanningReport, StoreError> {
        super::super::tiering::plan_resident_read_lease(self, artifact_ref, execution_origin)
    }

    pub fn plan_cold_recall_lease(
        &self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<ReadPlacementPlanningReport, StoreError> {
        super::super::tiering::plan_cold_recall_lease(self, artifact_ref, execution_origin)
    }

    pub fn plan_broadened_recall(
        &self,
        family: crate::ColdDerivedFamilyPolicy,
        scope_class: crate::PlacementObservationScopeClass,
        scope_key: &str,
        widened_artifact_keys: Vec<String>,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<crate::BroadenedRecallPlan, StoreError> {
        super::super::tiering::plan_broadened_recall(
            self,
            family,
            scope_class,
            scope_key,
            widened_artifact_keys,
            execution_origin,
        )
    }

    pub fn canonical_residency_manifest(&self) -> crate::CanonicalResidencyManifest {
        super::super::tiering::canonical_residency_manifest(self)
    }

    pub fn recover_tiering_state(&self) -> Result<crate::CanonicalResidencyManifest, StoreError> {
        super::super::tiering::recover_tiering_state(self)
    }

    pub fn resolve_resident_read_handle(
        &self,
        lease: &crate::ResidentReadLease,
    ) -> crate::PlacementResolvedReadHandle {
        super::super::tiering::resolve_resident_read_handle(lease)
    }

    pub fn resolve_cold_recall_read_handle(
        &self,
        lease: &crate::ColdRecallLease,
    ) -> crate::PlacementResolvedReadHandle {
        super::super::tiering::resolve_cold_recall_read_handle(lease)
    }

    pub fn observe_placement_read_interleaving(
        &self,
        handle: &crate::PlacementResolvedReadHandle,
    ) -> Result<crate::InterleavedReadParityReport, StoreError> {
        super::super::tiering::observe_placement_read_interleaving(self, handle)
    }

    pub fn observe_stable_basis_interleaving(
        &self,
        basis: &crate::StableBasisHandle,
    ) -> Result<crate::InterleavedReadParityReport, StoreError> {
        super::super::tiering::observe_stable_basis_interleaving(self, basis)
    }

    pub fn observe_continuation_interleaving(
        &self,
        plan: &crate::CursorContinuationPlan,
        result: Option<&crate::ContinuationBatchResult>,
    ) -> Result<crate::InterleavedContinuationParityReport, StoreError> {
        super::super::tiering::observe_continuation_interleaving(self, plan, result)
    }

    pub fn prepare_authoritative_tier_move(
        &mut self,
        plan: crate::AuthoritativeTierMovePlan,
    ) -> Result<crate::TierTransferIntent, StoreError> {
        super::super::tiering::prepare_authoritative_tier_move(self, plan)
    }

    pub fn prepare_derived_tier_move(
        &mut self,
        plan: crate::DerivedTierMovePlan,
    ) -> Result<crate::TierTransferIntent, StoreError> {
        super::super::tiering::prepare_derived_tier_move(self, plan)
    }

    pub fn transfer_tier_replica(
        &mut self,
        intent: crate::TierTransferIntent,
    ) -> Result<crate::TransferredTierReplica, StoreError> {
        super::super::tiering::transfer_tier_replica(self, intent)
    }

    pub fn verify_tier_replica(
        &mut self,
        transferred: crate::TransferredTierReplica,
    ) -> Result<crate::VerifiedTierReplica, StoreError> {
        super::super::tiering::verify_tier_replica(self, transferred)
    }

    pub fn cutover_tier_replica(
        &mut self,
        verified: crate::VerifiedTierReplica,
    ) -> Result<crate::TierCutoverWitness, StoreError> {
        super::super::tiering::cutover_tier_replica(self, verified)
    }

    pub fn retire_tier_replica(
        &mut self,
        cutover: crate::TierCutoverWitness,
    ) -> Result<crate::RetiredTierReplica, StoreError> {
        super::super::tiering::retire_tier_replica(self, cutover)
    }

    pub fn execute_cold_recall(
        &mut self,
        lease: crate::ColdRecallLease,
        witness: crate::RecallEligibilityWitness,
    ) -> Result<crate::CoalescedRecallReport, StoreError> {
        super::super::tiering::execute_cold_recall(self, lease, witness)
    }

    #[cfg(test)]
    pub(crate) fn admit_inflight_cold_recall(
        &mut self,
        artifact_ref: crate::PlacementBoundArtifactRef,
        execution_origin: crate::PlacementExecutionOrigin,
    ) -> Result<(), StoreError> {
        super::super::tiering::admit_inflight_cold_recall(self, artifact_ref, execution_origin)
    }
}
