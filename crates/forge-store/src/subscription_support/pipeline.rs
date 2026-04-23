use super::witnesses::{
    SubscriptionSupportBasisWitness, SubscriptionSupportCheckpointWitness,
    SubscriptionSupportCompatibilityWitness, SubscriptionSupportCursorWitness,
    SubscriptionSupportSchemaWitness,
};
use super::{
    classification_error, classify_causes, ensure_classification, ensure_report_matches_artifact,
    support_maintenance_batch, synthetic_support_maintenance_receipt,
    AdmittedSubscriptionSupportDeclaration, DegradedSubscriptionResumeHandle,
    ExactSubscriptionResumeHandle, ExecutedSupportAction, PostActionResumeClassificationInput,
    PublishableSubscriptionSupportArtifact, PublishedSubscriptionSupportArtifact,
    PublishedSupportConsequence, RawSubscriptionSupportDeclaration, RawSupportProgramAction,
    ResumeClassificationTranslationPlan, SubscriptionResumeClassification,
    SubscriptionResumeDeniedReport, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationReport,
    SubscriptionSupportCompatibilityDecision, SubscriptionSupportCompatibilityDecisionKind,
    SubscriptionSupportCompatibilityReport, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyKind,
    SubscriptionSupportMaintenanceDecision, SubscriptionSupportMaintenanceDecisionKind,
    SubscriptionSupportMaintenanceReport, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPlanFamily,
    SubscriptionSupportPortabilityDecision, SubscriptionSupportPortabilityDecisionKind,
    SubscriptionSupportPortabilityOutcome, SubscriptionSupportPortabilityReport,
    SubscriptionSupportPostActionReport, SubscriptionSupportRebuildPlanHandle,
    SubscriptionSupportResultCostSurface, SubscriptionSupportRetentionDecision,
    SubscriptionSupportRetentionMaterialization, SubscriptionSupportRole,
    SupportActionBreadthBudget, SupportActionId, SupportAffectedSet, SupportAllocationScope,
    SupportBatchAdmissionReceipt, SupportCompatibilityAffectedSet, SupportCompatibilityBatchPlan,
    SupportCompatibilityReceiptWitness, SupportDecodedRowSemanticAccess,
    SupportMaintenanceAffectedSet, SupportMaintenanceBatchPlan, SupportManifestAdmissionWitness,
    SupportPathClass, SupportPortabilityAffectedSet, SupportPortabilityBatchPlan,
    SupportPortabilityManifestBudget, SupportPortabilityScopeFootprint, SupportProgramDensityClass,
    SupportProgramPathPlan, SupportReclaimConsequence, SupportRetentionBatchPlan,
    SupportRetentionSurvivalWitness,
};
use crate::failure::StoreError;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct SubscriptionSupportPublicationPipeline {
    catalog: SubscriptionSupportCatalog,
    counters: SubscriptionSupportCounterSnapshot,
}

impl Default for SubscriptionSupportPublicationPipeline {
    fn default() -> Self {
        Self::first_ship()
    }
}

impl SubscriptionSupportPublicationPipeline {
    pub fn first_ship() -> Self {
        Self {
            catalog: SubscriptionSupportCatalog::first_ship(),
            counters: SubscriptionSupportCounterSnapshot::default(),
        }
    }

    pub fn admit(
        &mut self,
        declaration: RawSubscriptionSupportDeclaration,
    ) -> Result<AdmittedSubscriptionSupportDeclaration, StoreError> {
        match self.catalog.admit(declaration) {
            Ok(admitted) => {
                self.counters.record_admitted();
                Ok(admitted)
            }
            Err(err) => {
                self.counters.record_rejected();
                Err(err)
            }
        }
    }

    pub fn prepare_exact(
        &self,
        declaration: AdmittedSubscriptionSupportDeclaration,
        basis_digest: impl Into<String>,
        cursor_digest: impl Into<String>,
        checkpoint_digest: impl Into<String>,
        schema_digest: impl Into<String>,
        compatibility_digest: impl Into<String>,
    ) -> Result<PublishableSubscriptionSupportArtifact, StoreError> {
        PublishableSubscriptionSupportArtifact::new(
            declaration,
            SubscriptionSupportBasisWitness::new(basis_digest)?,
            SubscriptionSupportCursorWitness::new(cursor_digest)?,
            SubscriptionSupportCheckpointWitness::new(checkpoint_digest)?,
            SubscriptionSupportSchemaWitness::new(schema_digest)?,
            SubscriptionSupportCompatibilityWitness::new(compatibility_digest)?,
        )
    }

    pub fn publish(
        &mut self,
        artifact: PublishableSubscriptionSupportArtifact,
    ) -> Result<PublishedSubscriptionSupportArtifact, StoreError> {
        let published = PublishedSubscriptionSupportArtifact::new(artifact)?;
        self.counters.record_published();
        Ok(published)
    }

    pub fn classify(
        &mut self,
        artifact: &PublishedSubscriptionSupportArtifact,
        plan: SubscriptionSupportClassificationPlan,
        payload_bytes: u64,
        support_rows: u64,
        causes: Vec<SubscriptionSupportDriftCause>,
    ) -> Result<SubscriptionSupportClassificationReport, StoreError> {
        let expected_density = self
            .catalog
            .density_for(artifact.declaration.family_kind())
            .ok_or_else(|| {
                classification_error(
                    "subscription-support classification requires an admitted catalog family",
                )
            })?;
        if expected_density != plan.density_class {
            return Err(classification_error(
                "subscription-support classification plan density does not match catalog family",
            ));
        }
        if !plan.budget().admits(payload_bytes, support_rows) {
            self.counters.record_budget_denial();
            return Err(classification_error(
                "subscription-support classification exceeded its pre-resolved payload budget",
            ));
        }

        let (primary_cause, suppressed_causes) = classify_causes(causes);
        let classification = classification_for(artifact, plan.plan_family(), primary_cause);
        self.counters.record_classification(classification);
        Ok(SubscriptionSupportClassificationReport {
            artifact_id: artifact.artifact_id.clone(),
            declaration_digest: artifact.declaration.declaration_digest.clone(),
            classification,
            primary_cause,
            suppressed_causes,
            cost_surface: SubscriptionSupportResultCostSurface::new(
                plan.plan_family,
                plan.density_class,
                payload_bytes,
                support_rows,
                u64::from(plan.restart_shard.is_some()),
                plan.allocation_scope,
            ),
            counter_snapshot: self.counters.clone(),
        })
    }

    pub fn exact_handle(
        &self,
        artifact: &PublishedSubscriptionSupportArtifact,
        report: &SubscriptionSupportClassificationReport,
    ) -> Result<ExactSubscriptionResumeHandle, StoreError> {
        ensure_report_matches_artifact(artifact, report)?;
        ensure_classification(
            report,
            SubscriptionResumeClassification::Exact,
            "exact subscription resume handles require exact classification evidence",
        )?;
        Ok(ExactSubscriptionResumeHandle::new(artifact))
    }

    pub fn degraded_handle(
        &self,
        artifact: &PublishedSubscriptionSupportArtifact,
        report: &SubscriptionSupportClassificationReport,
    ) -> Result<DegradedSubscriptionResumeHandle, StoreError> {
        ensure_report_matches_artifact(artifact, report)?;
        let Some(primary_cause) = report.primary_cause else {
            return Err(classification_error(
                "degraded subscription resume handles require a primary drift cause",
            ));
        };
        ensure_classification(
            report,
            SubscriptionResumeClassification::Degraded,
            "degraded subscription resume handles require degraded classification evidence",
        )?;
        Ok(DegradedSubscriptionResumeHandle::new(
            artifact,
            primary_cause,
        ))
    }

    pub fn rebuild_plan_handle(
        &self,
        artifact: &PublishedSubscriptionSupportArtifact,
        report: &SubscriptionSupportClassificationReport,
        retained_rebuild_basis_digest: impl Into<String>,
        missing_or_stale_families: Vec<SubscriptionSupportFamilyKind>,
    ) -> Result<SubscriptionSupportRebuildPlanHandle, StoreError> {
        ensure_report_matches_artifact(artifact, report)?;
        ensure_classification(
            report,
            SubscriptionResumeClassification::RebuildRequired,
            "rebuild plan handles require rebuild-required classification evidence",
        )?;
        SubscriptionSupportRebuildPlanHandle::new(
            artifact,
            retained_rebuild_basis_digest,
            missing_or_stale_families,
        )
    }

    pub fn denied_report(
        &self,
        artifact: &PublishedSubscriptionSupportArtifact,
        report: &SubscriptionSupportClassificationReport,
    ) -> Result<SubscriptionResumeDeniedReport, StoreError> {
        ensure_report_matches_artifact(artifact, report)?;
        let Some(primary_cause) = report.primary_cause else {
            return Err(classification_error(
                "denied subscription resume reports require a primary drift cause",
            ));
        };
        ensure_classification(
            report,
            SubscriptionResumeClassification::NotResumable,
            "denied subscription resume reports require not-resumable classification evidence",
        )?;
        Ok(SubscriptionResumeDeniedReport::new(
            artifact,
            primary_cause,
            report.suppressed_causes.clone(),
        ))
    }

    pub fn translate_operational_verdict(
        &mut self,
        verdict: SubscriptionSupportOperationalVerdict,
        basis: SubscriptionSupportOperationalBasis,
        maintenance_admission_key: Option<String>,
        policy_reason: Option<String>,
    ) -> Result<PostActionResumeClassificationInput, StoreError> {
        match ResumeClassificationTranslationPlan::from_operational_verdict(
            verdict,
            basis,
            maintenance_admission_key,
            policy_reason,
        ) {
            Ok(plan) => {
                self.counters.record_operational_verdict_translation();
                Ok(plan.lower())
            }
            Err(err) => {
                self.counters
                    .record_operational_verdict_translation_rejection();
                Err(err)
            }
        }
    }

    pub fn admit_support_program_path(
        &mut self,
        path_class: SupportPathClass,
        density_class: SupportProgramDensityClass,
        allocation_scope: SupportAllocationScope,
        budget: SupportActionBreadthBudget,
        affected_entries: u64,
        payload_header_bytes: u64,
    ) -> Result<SupportProgramPathPlan, StoreError> {
        match SupportProgramPathPlan::new(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        ) {
            Ok(plan) => Ok(plan),
            Err(err) => {
                match (path_class, density_class) {
                    (SupportPathClass::ForegroundResume | SupportPathClass::ForegroundRead, _) => {
                        self.counters.record_support_hot_path_rejection();
                    }
                    (_, SupportProgramDensityClass::StoreGlobalDebt) => {
                        self.counters.record_support_store_global_debt_rejection();
                    }
                    _ => {}
                }
                Err(err)
            }
        }
    }

    pub fn reuse_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a SupportBatchAdmissionReceipt, StoreError> {
        let receipt = plan.batch_receipt().ok_or_else(|| {
            classification_error("subscription-support path plan has no reusable batch receipt")
        })?;
        self.counters.record_support_batch_receipt_reuse();
        Ok(receipt)
    }

    pub fn publish_support_consequence(
        &mut self,
        action: ExecutedSupportAction,
    ) -> PublishedSupportConsequence {
        self.counters.record_support_action_envelope_publication();
        action.publish()
    }

    pub fn admit_support_retention_batch(
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
        let affected_set = SupportAffectedSet::from_retention_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let plan = SupportRetentionBatchPlan::new(action_id, affected_set, path_plan, decision)?;
        self.counters
            .record_support_retention_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_support_retention_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SubscriptionSupportPostActionReport, StoreError> {
        let (action_id, affected_set, path_plan, decision) = plan.into_parts();
        let decision_kind = decision.kind();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute())
            .complete();
        let survival_witness =
            SupportRetentionSurvivalWitness::new(&completed, decision.verdict(), &affected_set)?;
        let materialization =
            SubscriptionSupportRetentionMaterialization::from_decision(affected_set, &decision)?;
        if decision.is_reclaim() {
            self.counters.record_support_reclaim_consequence();
        }
        let report = SubscriptionSupportPostActionReport::new(
            completed,
            survival_witness,
            materialization,
            decision_kind,
            &path_plan,
        )?;
        match decision_kind {
            super::SubscriptionSupportRetentionDecisionKind::RetainExact
            | super::SubscriptionSupportRetentionDecisionKind::RetainDegraded => {
                self.counters.record_support_retained_family();
            }
            super::SubscriptionSupportRetentionDecisionKind::CompactExact => {
                self.counters.record_support_compacted_basis();
            }
            super::SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            | super::SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild => {
                self.counters.record_support_reclaimed_family();
            }
            super::SubscriptionSupportRetentionDecisionKind::ExpireByPolicy => {
                self.counters.record_support_expired_family();
                self.counters.record_support_policy_expiration();
            }
        }
        Ok(report)
    }

    pub fn publish_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        if !plan.decision().is_reclaim() {
            return Err(classification_error(
                "subscription-support reclaim consequences require a reclaim retention decision",
            ));
        }
        let report = self.publish_support_retention_consequence(plan)?;
        SupportReclaimConsequence::new(
            report.completed_action().clone(),
            report.survival_witness().clone(),
            report.retention_record().clone(),
            report.materialization().clone(),
        )
    }

    pub fn admit_support_compatibility_batch(
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
        let affected_set =
            SupportCompatibilityAffectedSet::from_compatibility_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let manifest_admission = SupportManifestAdmissionWitness::from_compatibility_receipt(
            compatibility_receipt,
            affected_set.primary_basis().compatibility_digest(),
        )?;
        let semantic_access = SupportDecodedRowSemanticAccess::from_manifest_admission(
            manifest_admission.clone(),
            semantic_digest,
        )?;
        let plan = SupportCompatibilityBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            manifest_admission,
            semantic_access,
            decision,
        )?;
        self.counters
            .record_support_compatibility_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_support_compatibility_consequence(
        &mut self,
        plan: SupportCompatibilityBatchPlan,
    ) -> Result<SubscriptionSupportCompatibilityReport, StoreError> {
        let (action_id, affected_set, _path_plan, manifest_admission, semantic_access, decision) =
            plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute())
            .complete();
        let report = SubscriptionSupportCompatibilityReport::new(
            completed,
            affected_set,
            manifest_admission,
            semantic_access,
            &decision,
        )?;
        match decision.kind() {
            SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
                self.counters.record_support_exact_compatible_migration();
            }
            SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
                self.counters.record_support_degraded_compatibility();
            }
            SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
                self.counters.record_support_version_skew_rejection();
            }
        }
        Ok(report)
    }

    pub fn admit_support_portability_batch(
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
        let affected_set = SupportPortabilityAffectedSet::from_portability_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        if !manifest_budget.admits(included_support_count, manifest_header_bytes) {
            self.counters
                .record_support_capsule_manifest_budget_denial();
            return Err(classification_error(
                "subscription-support capsule manifest exceeds portability manifest budget before footprint materialization",
            ));
        }
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            manifest_header_bytes,
        )?;
        let omitted_artifact_ids = decision.omitted_artifact_ids_for_scope(&affected_set);
        let basis_artifact_ids = decision.basis_artifact_ids_for_scope(&affected_set);
        let footprint = SupportPortabilityScopeFootprint::new(
            &affected_set,
            included_support_count,
            omitted_support_count,
            &omitted_artifact_ids,
            &basis_artifact_ids,
        )?;
        let manifest = match super::CapsuleSupportManifest::new(
            &affected_set,
            footprint.clone(),
            manifest_budget,
            manifest_header_bytes,
            &basis_artifact_ids,
        ) {
            Ok(manifest) => manifest,
            Err(error) => {
                self.counters
                    .record_support_capsule_manifest_budget_denial();
                return Err(error);
            }
        };
        let plan = SupportPortabilityBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            footprint,
            manifest,
            decision,
        )?;
        self.counters.record_support_portability_plan(
            plan.manifest().manifest_entry_count(),
            plan.manifest().required_basis_count(),
            plan.manifest().omitted_support_count(),
        );
        Ok(plan)
    }

    pub fn publish_support_portability_consequence(
        &mut self,
        plan: SupportPortabilityBatchPlan,
    ) -> Result<SubscriptionSupportPortabilityReport, StoreError> {
        let (action_id, affected_set, path_plan, _footprint, manifest, decision) =
            plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute())
            .complete();
        let report = SubscriptionSupportPortabilityReport::new(
            completed,
            affected_set,
            manifest,
            &decision,
            &path_plan,
        )?;
        match report.outcome() {
            SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) => {
                self.counters
                    .record_support_replication_inclusion(bundle.preserved_count());
            }
            SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) => {
                self.counters
                    .record_support_replication_omission(omission.omitted_count());
            }
            SubscriptionSupportPortabilityOutcome::Imported(_) => {
                self.counters.record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::ImportedNotResumable(_) => {
                self.counters.record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::Rejected(_) => {
                if decision.kind()
                    == SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
                {
                    self.counters.record_support_import_rejection();
                }
            }
        }
        Ok(report)
    }

    pub fn admit_support_maintenance_batch(
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
        let affected_set = SupportMaintenanceAffectedSet::from_maintenance_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let (descriptors, coalesced_duplicate_count) = affected_set.descriptors_for(&decision)?;
        let maintenance_batch = support_maintenance_batch(&action_id, &descriptors);
        let maintenance_receipt =
            synthetic_support_maintenance_receipt(&maintenance_batch, &descriptors);
        let plan = SupportMaintenanceBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        )?;
        self.counters.record_support_maintenance_plan(
            plan.descriptors().len() as u64,
            plan.coalesced_duplicate_count(),
        );
        Ok(plan)
    }

    pub fn publish_support_maintenance_consequence(
        &mut self,
        plan: SupportMaintenanceBatchPlan,
    ) -> Result<SubscriptionSupportMaintenanceReport, StoreError> {
        let (
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        ) = plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        let completed = self
            .publish_support_consequence(raw_action.plan().verify().execute())
            .complete();
        let report = SubscriptionSupportMaintenanceReport::new(
            completed,
            affected_set,
            descriptors,
            &maintenance_receipt,
            coalesced_duplicate_count,
            &decision,
            &path_plan,
        )?;
        match decision.kind() {
            SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
                self.counters.record_support_maintenance_rebuild_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
                self.counters.record_support_maintenance_refresh_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
                self.counters
                    .record_support_maintenance_compatibility_migration_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
                self.counters
                    .record_support_maintenance_degradation_recovery_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
                self.counters
                    .record_support_maintenance_interrupted_restart_recovery();
            }
        }
        Ok(report)
    }

    pub fn counters(&self) -> SubscriptionSupportCounterSnapshot {
        self.counters.clone()
    }
}

fn classification_for(
    artifact: &PublishedSubscriptionSupportArtifact,
    plan_family: SubscriptionSupportPlanFamily,
    primary_cause: Option<SubscriptionSupportDriftCause>,
) -> SubscriptionResumeClassification {
    if primary_cause.is_none()
        && artifact.declaration.role() == SubscriptionSupportRole::ExactContinuation
        && plan_family == SubscriptionSupportPlanFamily::ExactResumeClassificationPlan
    {
        return SubscriptionResumeClassification::Exact;
    }

    if primary_cause.is_none()
        && artifact.declaration.role() == SubscriptionSupportRole::DegradedContinuation
        && plan_family == SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan
    {
        return SubscriptionResumeClassification::Degraded;
    }

    if primary_cause == Some(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable)
    {
        return match (artifact.declaration.role(), plan_family) {
            (
                SubscriptionSupportRole::ExactContinuation,
                SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            ) => SubscriptionResumeClassification::Exact,
            (
                SubscriptionSupportRole::DegradedContinuation,
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
            ) => SubscriptionResumeClassification::Degraded,
            _ => SubscriptionResumeClassification::NotResumable,
        };
    }

    match (plan_family, primary_cause) {
        (SubscriptionSupportPlanFamily::RebuildPlanClassificationPlan, Some(_)) => {
            SubscriptionResumeClassification::RebuildRequired
        }
        _ => SubscriptionResumeClassification::NotResumable,
    }
}
