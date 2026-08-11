use worth_store::physical_runtime::StoreRecoveryBindingFreshnessSample;
use worth_store_physical_format::store_namespace::StableStoreIdentity;
use worth_store_recovery_physics::{
    ImmutablePhysicalRedoPlan, PhysicalSourceSelection, RecoveryPlanCost, RecoveryPlanningCounters,
};

use crate::entry::{AdmittedPlatformAuthority, PhysicalRecoveryOutcome};
use crate::handoff::RecoveryOperationFateSet;
use crate::orchestration::RecoveryCoordination;

use super::PhysicalRecoveryDiscoveryCounters;

pub struct PlannedPhysicalRecovery {
    authority: AdmittedPlatformAuthority,
    coordination: RecoveryCoordination,
    selection: PhysicalSourceSelection,
    discovery_counters: PhysicalRecoveryDiscoveryCounters,
    freshness: StoreRecoveryBindingFreshnessSample,
    fates: RecoveryOperationFateSet,
    redo: ImmutablePhysicalRedoPlan,
    plan_cost: RecoveryPlanCost,
    planning_counters: RecoveryPlanningCounters,
    staging: RecoveryStagingLayoutPlan,
    publication: RecoveryPublicationPlan,
    quiescence: RecoveryQuiescencePlan,
}

mod cancellation;
pub use cancellation::PhysicalRecoveryStagingCancellation;

impl PlannedPhysicalRecovery {
    pub(crate) const fn new(
        authority: AdmittedPlatformAuthority,
        coordination: RecoveryCoordination,
        selection: PhysicalSourceSelection,
        discovery_counters: PhysicalRecoveryDiscoveryCounters,
        freshness: StoreRecoveryBindingFreshnessSample,
        fates: RecoveryOperationFateSet,
        redo: ImmutablePhysicalRedoPlan,
        plan_cost: RecoveryPlanCost,
        planning_counters: RecoveryPlanningCounters,
        staging: RecoveryStagingLayoutPlan,
        publication: RecoveryPublicationPlan,
        quiescence: RecoveryQuiescencePlan,
    ) -> Self {
        Self {
            authority,
            coordination,
            selection,
            discovery_counters,
            freshness,
            fates,
            redo,
            plan_cost,
            planning_counters,
            staging,
            publication,
            quiescence,
        }
    }

    pub fn store_identity(&self) -> StableStoreIdentity {
        self.authority.media.store_identity()
    }
    pub const fn discovery_counters(&self) -> PhysicalRecoveryDiscoveryCounters {
        self.discovery_counters
    }
    pub const fn freshness_sample(&self) -> &StoreRecoveryBindingFreshnessSample {
        &self.freshness
    }
    pub const fn operation_fates(&self) -> &RecoveryOperationFateSet {
        &self.fates
    }
    pub const fn redo_plan(&self) -> &ImmutablePhysicalRedoPlan {
        &self.redo
    }
    pub const fn plan_cost(&self) -> RecoveryPlanCost {
        self.plan_cost
    }
    pub const fn planning_counters(&self) -> RecoveryPlanningCounters {
        self.planning_counters
    }
    pub const fn staging_layout(&self) -> &RecoveryStagingLayoutPlan {
        &self.staging
    }
    pub const fn publication_plan(&self) -> &RecoveryPublicationPlan {
        &self.publication
    }
    pub const fn quiescence_plan(&self) -> RecoveryQuiescencePlan {
        self.quiescence
    }
    pub const fn selected_sources(&self) -> &PhysicalSourceSelection {
        &self.selection
    }

    pub fn cancellation_after_command(
        &self,
        command_ordinal: u64,
    ) -> Option<PhysicalRecoveryStagingCancellation> {
        let index = usize::try_from(command_ordinal).ok()?;
        self.staging.commands().get(index)?;
        Some(PhysicalRecoveryStagingCancellation::new(
            self.publication.plan_identity(),
            command_ordinal.checked_add(1)?,
        ))
    }

    #[cfg(feature = "certification-test-authority")]
    pub fn certification_fail_staging_signal_settlement_at(
        &self,
        stage: worth_store::physical_runtime::PhysicalRecoveryStagingCommandStage,
    ) {
        self.coordination
            .fail_signal_settlement_at_for_certification(stage);
    }

    pub fn cancel_before_execution(self) -> PhysicalRecoveryOutcome {
        let Self {
            authority,
            coordination,
            ..
        } = self;
        assert!(coordination.shutdown_is_quiescent());
        let recovery_effects = authority.media.recovery_effect_count();
        let crate::entry::AdmittedPlatformAuthority { media, session, .. } = authority;
        drop(media);
        session.refuse();
        PhysicalRecoveryOutcome::Refused(crate::entry::PhysicalRecoveryRefusal::new(
            crate::entry::PhysicalRecoveryRefusalKind::CancelledBeforeExecution,
            recovery_effects,
        ))
    }

    /// Consumes the immutable Phase 4 basis and materializes one closed,
    /// non-current staging generation. No selector or serving root is changed.
    pub fn stage(self) -> Result<super::StagedPhysicalRecovery, PhysicalRecoveryOutcome> {
        self.stage_with_admitted_cancellation(
            crate::orchestration::RecoveryStagingCancellation::None,
        )
    }

    pub fn stage_with_cancellation(
        self,
        cancellation: PhysicalRecoveryStagingCancellation,
    ) -> Result<super::StagedPhysicalRecovery, PhysicalRecoveryOutcome> {
        let admitted = cancellation
            .admit(
                self.publication.plan_identity(),
                self.staging.commands().len() as u64,
            )
            .map_or(
                crate::orchestration::RecoveryStagingCancellation::Invalid,
                crate::orchestration::RecoveryStagingCancellation::AfterSettledCommands,
            );
        self.stage_with_admitted_cancellation(admitted)
    }

    fn stage_with_admitted_cancellation(
        self,
        cancellation: crate::orchestration::RecoveryStagingCancellation,
    ) -> Result<super::StagedPhysicalRecovery, PhysicalRecoveryOutcome> {
        let Self {
            authority,
            coordination,
            selection,
            discovery_counters,
            freshness,
            fates,
            redo: _,
            plan_cost: _,
            planning_counters,
            staging,
            publication,
            quiescence,
        } = self;
        crate::orchestration::stage_recovery(crate::orchestration::RecoveryStagingInput {
            authority,
            coordination,
            selection,
            discovery_counters,
            freshness,
            fates,
            planning_counters,
            staging,
            publication,
            quiescence,
            cancellation,
        })
    }
}
mod basis;

pub(crate) use basis::{
    derive_execution_basis, ExecutionBasisDenial, RecoveryPublicationSourceInventory,
};
pub use basis::{
    RecoveryBaseImageAction, RecoveryBaseImagePlan, RecoveryPayloadManifestAction,
    RecoveryPublicationAction, RecoveryPublicationCandidateArtifact,
    RecoveryPublicationExpectation, RecoveryPublicationPlan, RecoveryQuiescencePlan,
    RecoverySegmentRoutingAction, RecoveryStagingAction, RecoveryStagingCommandPlan,
    RecoveryStagingLayoutPlan, RecoveryStagingRedoStep,
};
