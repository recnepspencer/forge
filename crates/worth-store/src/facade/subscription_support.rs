use crate::{
    AdmittedSubscriptionSupportDeclaration, FetchedSubscriptionSupportArtifact,
    PostActionResumeClassificationInput, PublishableSubscriptionSupportArtifact,
    PublishedSubscriptionSupportArtifact, RawSubscriptionSupportDeclaration, StoreError,
    SubscriptionSupportAccessStructureReport, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationReport, SubscriptionSupportCompatibilityBatchRequest,
    SubscriptionSupportCompatibilityReport, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportFetchRequest, SubscriptionSupportMaintenanceBatchRequest,
    SubscriptionSupportMaintenanceDebtReport, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportMissingSupportRecoveryReport,
    SubscriptionSupportMissingSupportRecoveryRequest,
    SubscriptionSupportOperationalVerdictTranslationRequest,
    SubscriptionSupportPortabilityBatchRequest, SubscriptionSupportPortabilityReport,
    SubscriptionSupportPostActionReport, SubscriptionSupportPublicationPipeline,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionBatchRequest, SubscriptionSupportRuntimeHandoffReport,
    SubscriptionSupportRuntimeHandoffRequest, SupportActionBreadthBudget, SupportActionId,
    SupportCompatibilityBatchPlan, SupportMaintenanceBatchPlan, SupportPortabilityBatchPlan,
    SupportProgramPathAdmissionRequest, SupportProgramPathPlan, SupportReclaimConsequence,
    SupportRetentionBatchPlan,
};

use super::WORTHStore;

impl WORTHStore {
    pub fn subscription_support_catalog(&self) -> SubscriptionSupportCatalog {
        SubscriptionSupportCatalog::first_ship()
    }

    pub fn admit_subscription_support_declaration(
        &self,
        declaration: RawSubscriptionSupportDeclaration,
    ) -> Result<AdmittedSubscriptionSupportDeclaration, StoreError> {
        SubscriptionSupportCatalog::first_ship().admit(declaration)
    }

    pub fn subscription_support_pipeline(&self) -> SubscriptionSupportPublicationPipeline {
        SubscriptionSupportPublicationPipeline::first_ship()
    }

    pub fn translate_subscription_support_operational_verdict(
        &mut self,
        request: SubscriptionSupportOperationalVerdictTranslationRequest,
    ) -> Result<PostActionResumeClassificationInput, StoreError> {
        self.backend
            .translate_subscription_support_operational_verdict(request)
    }

    pub fn persist_subscription_support_executed_action_for_publication(
        &mut self,
        action: crate::ExecutedSupportAction,
    ) -> Result<(), StoreError> {
        self.backend
            .persist_subscription_support_executed_action_for_publication(action)
    }

    pub fn recover_subscription_support_action_publication(
        &mut self,
        action_id: SupportActionId,
    ) -> Result<crate::SubscriptionSupportActionPublicationRecoveryReport, StoreError> {
        self.backend
            .recover_subscription_support_action_publication(action_id)
    }

    pub fn reject_subscription_support_global_scan_recovery(&mut self) -> Result<(), StoreError> {
        self.backend
            .reject_subscription_support_global_scan_recovery()
    }

    pub fn admit_subscription_support_program_path(
        &mut self,
        request: SupportProgramPathAdmissionRequest,
    ) -> Result<SupportProgramPathPlan, StoreError> {
        self.backend
            .admit_subscription_support_program_path(request)
    }

    pub fn reuse_subscription_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a crate::SupportBatchAdmissionReceipt, StoreError> {
        self.backend.reuse_subscription_support_batch_receipt(plan)
    }

    pub fn admit_subscription_support_retention_batch(
        &mut self,
        request: SubscriptionSupportRetentionBatchRequest,
    ) -> Result<SupportRetentionBatchPlan, StoreError> {
        self.backend
            .admit_subscription_support_retention_batch(request)
    }

    pub fn publish_subscription_support_retention_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SubscriptionSupportPostActionReport, StoreError> {
        self.backend
            .publish_subscription_support_retention_consequence(plan)
    }

    pub fn publish_subscription_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        self.backend
            .publish_subscription_support_reclaim_consequence(plan)
    }

    pub fn admit_subscription_support_compatibility_batch(
        &mut self,
        request: SubscriptionSupportCompatibilityBatchRequest,
    ) -> Result<SupportCompatibilityBatchPlan, StoreError> {
        self.backend
            .admit_subscription_support_compatibility_batch(request)
    }

    pub fn publish_subscription_support_compatibility_consequence(
        &mut self,
        plan: SupportCompatibilityBatchPlan,
    ) -> Result<SubscriptionSupportCompatibilityReport, StoreError> {
        self.backend
            .publish_subscription_support_compatibility_consequence(plan)
    }

    pub fn admit_subscription_support_portability_batch(
        &mut self,
        request: SubscriptionSupportPortabilityBatchRequest,
    ) -> Result<SupportPortabilityBatchPlan, StoreError> {
        self.backend
            .admit_subscription_support_portability_batch(request)
    }

    pub fn publish_subscription_support_portability_consequence(
        &mut self,
        plan: SupportPortabilityBatchPlan,
    ) -> Result<SubscriptionSupportPortabilityReport, StoreError> {
        self.backend
            .publish_subscription_support_portability_consequence(plan)
    }

    pub fn admit_subscription_support_maintenance_batch(
        &mut self,
        request: SubscriptionSupportMaintenanceBatchRequest,
    ) -> Result<SupportMaintenanceBatchPlan, StoreError> {
        self.backend
            .admit_subscription_support_maintenance_batch(request)
    }

    pub fn publish_subscription_support_maintenance_consequence(
        &mut self,
        plan: SupportMaintenanceBatchPlan,
    ) -> Result<SubscriptionSupportMaintenanceReport, StoreError> {
        self.backend
            .publish_subscription_support_maintenance_consequence(plan)
    }

    pub fn report_delayed_subscription_support_maintenance(
        &mut self,
        plan: &SupportMaintenanceBatchPlan,
        delay_reason: impl Into<String>,
        budget: SupportActionBreadthBudget,
        payload_header_bytes: u64,
    ) -> Result<SubscriptionSupportMaintenanceDebtReport, StoreError> {
        self.backend
            .report_delayed_subscription_support_maintenance(
                plan,
                delay_reason,
                budget,
                payload_header_bytes,
            )
    }

    pub fn publish_subscription_support(
        &mut self,
        artifact: PublishableSubscriptionSupportArtifact,
    ) -> Result<PublishedSubscriptionSupportArtifact, StoreError> {
        self.backend.publish_subscription_support(artifact)
    }

    pub fn fetch_subscription_support(
        &mut self,
        request: SubscriptionSupportFetchRequest,
    ) -> Result<FetchedSubscriptionSupportArtifact, StoreError> {
        self.backend.fetch_subscription_support(request)
    }

    pub fn classify_subscription_support_resume(
        &mut self,
        request: SubscriptionSupportResumeRequest,
    ) -> Result<SubscriptionSupportClassificationReport, StoreError> {
        self.backend.classify_subscription_support_resume(request)
    }

    pub fn reconstruct_subscription_support_restart_shard(
        &mut self,
        request: SubscriptionSupportRestartReconstructionRequest,
    ) -> Result<SubscriptionSupportRestartReconstructionReport, StoreError> {
        self.backend
            .reconstruct_subscription_support_restart_shard(request)
    }

    pub fn classify_missing_subscription_support(
        &mut self,
        request: SubscriptionSupportMissingSupportRecoveryRequest,
    ) -> Result<SubscriptionSupportMissingSupportRecoveryReport, StoreError> {
        self.backend.classify_missing_subscription_support(request)
    }

    pub fn handoff_subscription_support_runtime(
        &mut self,
        request: SubscriptionSupportRuntimeHandoffRequest,
    ) -> Result<SubscriptionSupportRuntimeHandoffReport, StoreError> {
        self.backend.handoff_subscription_support_runtime(request)
    }

    pub fn subscription_support_counters(&self) -> SubscriptionSupportCounterSnapshot {
        self.backend.subscription_support_counters()
    }

    pub fn subscription_support_access_structure_report(
        &self,
    ) -> SubscriptionSupportAccessStructureReport {
        self.backend.subscription_support_access_structure_report()
    }
}
