use crate::{
    failure::{StoreError, StoreErrorKind},
    FetchedSubscriptionSupportArtifact, PostActionResumeClassificationInput,
    SubscriptionResumeClassification, SubscriptionSupportClassificationReport,
    SubscriptionSupportFetchReport, SubscriptionSupportFetchRequest,
    SubscriptionSupportMissingSupportRecoveryReport,
    SubscriptionSupportMissingSupportRecoveryRequest,
    SubscriptionSupportOperationalVerdictTranslationRequest,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportResumeEvidence,
    SubscriptionSupportResumeRequest, SubscriptionSupportRuntimeHandoffReport,
    SubscriptionSupportRuntimeHandoffRequest, SubscriptionSupportStoredRecordKey,
};

use super::super::{StateBackedStoreBackend, StatePersistence};
use super::missing_support_classification::{
    classify_missing_support, collect_missing_support_classification_evidence,
};
use super::missing_support_maintenance::execute_missing_support_rebuild_maintenance;
use super::restart_reconstruction::{
    admit_restart_shard, classify_restart_shard_records, load_bounded_restart_shard_records,
    publish_restart_reconstruction,
};
use super::resume_classification::{
    admit_resume_classification_request, project_resume_classification,
    publish_resume_classification_report, resume_budget_admitted,
};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn fetch_subscription_support(
        &mut self,
        request: SubscriptionSupportFetchRequest,
    ) -> Result<FetchedSubscriptionSupportArtifact, StoreError> {
        if !self.state.subscription_support_access_structures_verified {
            self.state
                .subscription_support_counter_snapshot
                .record_access_structure_debt();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support direct fetch requires verified access structures; refusing hidden global scan",
            ));
        }
        let storage_key = crate::SubscriptionSupportStoredRecordKey::new(
            request.family_id(),
            request.artifact_id(),
        )
        .storage_key();
        let record_set = self
            .state
            .subscription_support_record_sets
            .get(&storage_key)
            .cloned()
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SubscriptionSupportPublicationViolation,
                    "subscription-support fetch found no durable record set for family/artifact key",
                )
            })?;
        if record_set.family_kind() != request.family_kind() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support fetch rejected cross-family support artifact reuse",
            ));
        }
        record_set.validate()?;
        let report = SubscriptionSupportFetchReport::direct_lookup(1);
        self.state
            .subscription_support_counter_snapshot
            .record_fetch(report.lookup_key_count(), report.rows_read());
        Ok(FetchedSubscriptionSupportArtifact::new(record_set, report))
    }

    pub fn classify_subscription_support_resume(
        &mut self,
        request: SubscriptionSupportResumeRequest,
    ) -> Result<SubscriptionSupportClassificationReport, StoreError> {
        let admission = admit_resume_classification_request(&request)?;
        if !resume_budget_admitted(&request, &admission) {
            self.state
                .subscription_support_counter_snapshot
                .record_budget_denial();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support classification exceeded its pre-resolved payload budget",
            ));
        }
        let projection = project_resume_classification(&request, &admission);
        self.state
            .subscription_support_counter_snapshot
            .record_classification(projection.classification);
        Ok(publish_resume_classification_report(
            &request,
            &admission,
            projection,
            self.state.subscription_support_counter_snapshot.clone(),
        ))
    }

    pub fn reconstruct_subscription_support_restart_shard(
        &mut self,
        request: SubscriptionSupportRestartReconstructionRequest,
    ) -> Result<SubscriptionSupportRestartReconstructionReport, StoreError> {
        let admission = admit_restart_shard(&request)?;
        let bounded_records = load_bounded_restart_shard_records(
            &self.state.subscription_support_record_sets,
            &admission,
        )?;
        let support_rows_read = bounded_records.support_rows_read;
        let reports = classify_restart_shard_records(self, bounded_records, &admission)?;
        Ok(publish_restart_reconstruction(
            &mut self.state.subscription_support_counter_snapshot,
            request.shard().clone(),
            reports,
            support_rows_read,
        ))
    }

    pub fn classify_missing_subscription_support(
        &mut self,
        request: SubscriptionSupportMissingSupportRecoveryRequest,
    ) -> Result<SubscriptionSupportMissingSupportRecoveryReport, StoreError> {
        let storage_key = SubscriptionSupportStoredRecordKey::new(
            request.family_id(),
            request.missing_artifact_id(),
        )
        .storage_key();
        let evidence = collect_missing_support_classification_evidence(
            &request,
            self.state
                .subscription_support_record_sets
                .contains_key(&storage_key),
        )?;
        let classification = classify_missing_support(evidence);
        self.record_missing_support_classification(classification);
        let maintenance_report =
            if classification == SubscriptionResumeClassification::RebuildRequired {
                Some(execute_missing_support_rebuild_maintenance(self, &request)?)
            } else {
                None
            };
        Ok(SubscriptionSupportMissingSupportRecoveryReport::new(
            &request,
            classification,
            maintenance_report,
        ))
    }

    fn record_missing_support_classification(
        &mut self,
        classification: SubscriptionResumeClassification,
    ) {
        if classification == SubscriptionResumeClassification::RebuildRequired {
            self.state
                .subscription_support_counter_snapshot
                .record_rebuild_basis_plan();
        }
        self.state
            .subscription_support_counter_snapshot
            .record_classification(classification);
    }

    pub fn handoff_subscription_support_runtime(
        &mut self,
        request: SubscriptionSupportRuntimeHandoffRequest,
    ) -> Result<SubscriptionSupportRuntimeHandoffReport, StoreError> {
        let fetched = self.fetch_subscription_support(SubscriptionSupportFetchRequest::new(
            request.family_id().clone(),
            request.family_kind(),
            request.artifact_id().clone(),
        ))?;
        let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 0, true)?;
        let plan = super::resume_plans::handoff_plan_for_record(fetched.record_set())?;
        let durable_report = self.classify_subscription_support_resume(
            SubscriptionSupportResumeRequest::new(fetched, evidence, plan),
        )?;
        self.state
            .subscription_support_counter_snapshot
            .record_runtime_handoff();
        Ok(SubscriptionSupportRuntimeHandoffReport::new(
            &request,
            durable_report,
        ))
    }

    pub fn translate_subscription_support_operational_verdict(
        &mut self,
        request: SubscriptionSupportOperationalVerdictTranslationRequest,
    ) -> Result<PostActionResumeClassificationInput, StoreError> {
        match request.into_plan() {
            Ok(plan) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_operational_verdict_translation();
                Ok(plan.lower())
            }
            Err(error) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_operational_verdict_translation_rejection();
                Err(error)
            }
        }
    }
}
