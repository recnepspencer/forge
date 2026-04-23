use crate::{
    failure::StoreError, FetchedSubscriptionSupportArtifact, PostActionResumeClassificationInput,
    PublishableSubscriptionSupportArtifact, PublishedSubscriptionSupportArtifact,
    SubscriptionSupportAccessStructureReport, SubscriptionSupportClassificationReport,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityReport,
    SubscriptionSupportCounterSnapshot, SubscriptionSupportFetchRequest,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportMissingSupportRecoveryReport,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityReport, SubscriptionSupportPostActionReport,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionDecision, SubscriptionSupportRuntimeHandoffReport,
    SubscriptionSupportRuntimeHandoffRequest, SupportActionBreadthBudget, SupportActionId,
    SupportAllocationScope, SupportBatchAdmissionReceipt, SupportCompatibilityBatchPlan,
    SupportCompatibilityReceiptWitness, SupportMaintenanceBatchPlan, SupportPathClass,
    SupportPortabilityBatchPlan, SupportPortabilityManifestBudget, SupportProgramDensityClass,
    SupportProgramPathPlan, SupportReclaimConsequence, SupportRetentionBatchPlan,
};

use super::{dispatch_mut, dispatch_ref, StoreBackend};

impl StoreBackend {
    pub fn publish_subscription_support(
        &mut self,
        publishable: PublishableSubscriptionSupportArtifact,
    ) -> Result<PublishedSubscriptionSupportArtifact, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support(publishable))
    }

    pub fn fetch_subscription_support(
        &mut self,
        request: SubscriptionSupportFetchRequest,
    ) -> Result<FetchedSubscriptionSupportArtifact, StoreError> {
        dispatch_mut!(self, |backend| backend.fetch_subscription_support(request))
    }

    pub fn classify_subscription_support_resume(
        &mut self,
        request: SubscriptionSupportResumeRequest,
    ) -> Result<SubscriptionSupportClassificationReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .classify_subscription_support_resume(request))
    }

    pub fn reconstruct_subscription_support_restart_shard(
        &mut self,
        request: SubscriptionSupportRestartReconstructionRequest,
    ) -> Result<SubscriptionSupportRestartReconstructionReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .reconstruct_subscription_support_restart_shard(request))
    }

    pub fn classify_missing_subscription_support(
        &mut self,
        request: SubscriptionSupportMissingSupportRecoveryRequest,
    ) -> Result<SubscriptionSupportMissingSupportRecoveryReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .classify_missing_subscription_support(request))
    }

    pub fn handoff_subscription_support_runtime(
        &mut self,
        request: SubscriptionSupportRuntimeHandoffRequest,
    ) -> Result<SubscriptionSupportRuntimeHandoffReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .handoff_subscription_support_runtime(request))
    }

    pub fn translate_subscription_support_operational_verdict(
        &mut self,
        verdict: SubscriptionSupportOperationalVerdict,
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: Option<String>,
        policy_reason: Option<String>,
    ) -> Result<PostActionResumeClassificationInput, StoreError> {
        dispatch_mut!(self, |backend| backend
            .translate_subscription_support_operational_verdict(
                verdict,
                basis,
                maintenance_admission_key,
                policy_reason,
            ))
    }

    pub fn admit_subscription_support_program_path(
        &mut self,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        affected_entries: u64,
        payload_header_bytes: u64,
    ) -> Result<SupportProgramPathPlan, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_subscription_support_program_path(
                path_class,
                density_class,
                allocation_scope,
                budget,
                affected_entries,
                payload_header_bytes,
            ))
    }

    pub fn reuse_subscription_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a SupportBatchAdmissionReceipt, StoreError> {
        dispatch_mut!(self, |backend| backend
            .reuse_subscription_support_batch_receipt(plan))
    }

    pub fn admit_subscription_support_retention_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        decision: SubscriptionSupportRetentionDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportRetentionBatchPlan, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_subscription_support_retention_batch(
                action_id,
                affected_bases,
                decision,
                path_class,
                density_class,
                allocation_scope,
                budget,
                payload_header_bytes,
            ))
    }

    pub fn publish_subscription_support_retention_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SubscriptionSupportPostActionReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support_retention_consequence(plan))
    }

    pub fn publish_subscription_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support_reclaim_consequence(plan))
    }

    pub fn admit_subscription_support_compatibility_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        compatibility_receipt: SupportCompatibilityReceiptWitness,
        semantic_digest: impl Into<String>,
        decision: SubscriptionSupportCompatibilityDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportCompatibilityBatchPlan, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_subscription_support_compatibility_batch(
                action_id,
                affected_bases,
                compatibility_receipt,
                semantic_digest,
                decision,
                path_class,
                density_class,
                allocation_scope,
                budget,
                payload_header_bytes,
            ))
    }

    pub fn publish_subscription_support_compatibility_consequence(
        &mut self,
        plan: SupportCompatibilityBatchPlan,
    ) -> Result<SubscriptionSupportCompatibilityReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support_compatibility_consequence(plan))
    }

    pub fn admit_subscription_support_portability_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        included_support_count: u64,
        omitted_support_count: u64,
        manifest_budget: SupportPortabilityManifestBudget,
        decision: SubscriptionSupportPortabilityDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        manifest_header_bytes: u64,
    ) -> Result<SupportPortabilityBatchPlan, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_subscription_support_portability_batch(
                action_id,
                affected_bases,
                included_support_count,
                omitted_support_count,
                manifest_budget,
                decision,
                path_class,
                density_class,
                allocation_scope,
                budget,
                manifest_header_bytes,
            ))
    }

    pub fn publish_subscription_support_portability_consequence(
        &mut self,
        plan: SupportPortabilityBatchPlan,
    ) -> Result<SubscriptionSupportPortabilityReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support_portability_consequence(plan))
    }

    pub fn admit_subscription_support_maintenance_batch(
        &mut self,
        action_id: SupportActionId,
        affected_bases: Vec<SubscriptionSupportOperationalBasis>,
        decision: SubscriptionSupportMaintenanceDecision,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SupportMaintenanceBatchPlan, StoreError> {
        dispatch_mut!(self, |backend| backend
            .admit_subscription_support_maintenance_batch(
                action_id,
                affected_bases,
                decision,
                path_class,
                density_class,
                allocation_scope,
                budget,
                payload_header_bytes,
            ))
    }

    pub fn publish_subscription_support_maintenance_consequence(
        &mut self,
        plan: SupportMaintenanceBatchPlan,
    ) -> Result<SubscriptionSupportMaintenanceReport, StoreError> {
        dispatch_mut!(self, |backend| backend
            .publish_subscription_support_maintenance_consequence(plan))
    }

    pub fn subscription_support_counters(&self) -> SubscriptionSupportCounterSnapshot {
        dispatch_ref!(self, |backend| backend.subscription_support_counters())
    }

    pub fn subscription_support_access_structure_report(
        &self,
    ) -> SubscriptionSupportAccessStructureReport {
        dispatch_ref!(self, |backend| backend
            .subscription_support_access_structure_report())
    }
}
