use crate::subscription_support::classify_causes;
use crate::{
    failure::{StoreError, StoreErrorKind},
    support_maintenance_batch, FetchedSubscriptionSupportArtifact,
    PostActionResumeClassificationInput, PublishableSubscriptionSupportArtifact,
    PublishedSubscriptionSupportArtifact, RawSupportProgramAction,
    ResumeClassificationTranslationPlan, SubscriptionResumeClassification,
    SubscriptionSupportAccessStructureReport, SubscriptionSupportAllocationScope,
    SubscriptionSupportCatalog, SubscriptionSupportClassificationPlan,
    SubscriptionSupportClassificationReport, SubscriptionSupportCompatibilityDecision,
    SubscriptionSupportCompatibilityDecisionKind, SubscriptionSupportCompatibilityReport,
    SubscriptionSupportDensityClass, SubscriptionSupportDriftCause, SubscriptionSupportFetchReport,
    SubscriptionSupportFetchRequest, SubscriptionSupportMaintenanceDecision,
    SubscriptionSupportMaintenanceDecisionKind, SubscriptionSupportMaintenanceReport,
    SubscriptionSupportMissingSupportRecoveryReport,
    SubscriptionSupportMissingSupportRecoveryRequest, SubscriptionSupportOperationalBasis,
    SubscriptionSupportOperationalVerdict, SubscriptionSupportPayloadBudget,
    SubscriptionSupportPlanFamily, SubscriptionSupportPortabilityDecision,
    SubscriptionSupportPortabilityDecisionKind, SubscriptionSupportPortabilityOutcome,
    SubscriptionSupportPortabilityReport, SubscriptionSupportPostActionReport,
    SubscriptionSupportRestartReconstructionReport,
    SubscriptionSupportRestartReconstructionRequest, SubscriptionSupportResultCostSurface,
    SubscriptionSupportResumeEvidence, SubscriptionSupportResumeRequest,
    SubscriptionSupportRetentionDecision, SubscriptionSupportRetentionMaterialization,
    SubscriptionSupportRole, SubscriptionSupportRuntimeHandoffReport,
    SubscriptionSupportRuntimeHandoffRequest, SubscriptionSupportStoredRecordKey,
    SubscriptionSupportStoredRecordSet, SupportActionBreadthBudget, SupportActionId,
    SupportAffectedSet, SupportAllocationScope, SupportBatchAdmissionReceipt,
    SupportCompatibilityAffectedSet, SupportCompatibilityBatchPlan,
    SupportCompatibilityReceiptWitness, SupportDecodedRowSemanticAccess,
    SupportMaintenanceAffectedSet, SupportMaintenanceBatchPlan, SupportManifestAdmissionWitness,
    SupportPathClass, SupportPortabilityAffectedSet, SupportPortabilityBatchPlan,
    SupportPortabilityManifestBudget, SupportPortabilityScopeFootprint, SupportProgramDensityClass,
    SupportProgramPathPlan, SupportReclaimConsequence, SupportRetentionBatchPlan,
    SupportRetentionSurvivalWitness,
};

use super::{core::verify_durable_barrier, StateBackedStoreBackend, StatePersistence};

impl<P: StatePersistence> StateBackedStoreBackend<P> {
    pub fn publish_subscription_support(
        &mut self,
        publishable: PublishableSubscriptionSupportArtifact,
    ) -> Result<PublishedSubscriptionSupportArtifact, StoreError> {
        let published = PublishedSubscriptionSupportArtifact::new(publishable.clone())?;
        let record_set = SubscriptionSupportStoredRecordSet::from_publishable_and_published(
            &publishable,
            &published,
        )?;
        let storage_key = record_set.key().storage_key();

        if let Some(existing) = self
            .state
            .subscription_support_record_sets
            .get(&storage_key)
        {
            if existing == &record_set {
                let previous_counter_snapshot =
                    self.state.subscription_support_counter_snapshot.clone();
                self.state
                    .subscription_support_counter_snapshot
                    .record_duplicate_retry();
                let report = match self.persistence.persist_state(&self.state) {
                    Ok(report) => report,
                    Err(error) => {
                        self.state.subscription_support_counter_snapshot =
                            previous_counter_snapshot;
                        return Err(error);
                    }
                };
                verify_durable_barrier(&mut self.counters, &report)?;
                return Ok(published);
            }
            self.state
                .subscription_support_counter_snapshot
                .record_identity_collision();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportPublicationViolation,
                "subscription-support publication collided with a different durable record set",
            ));
        }

        let previous_counter_snapshot = self.state.subscription_support_counter_snapshot.clone();
        self.state
            .subscription_support_record_sets
            .insert(storage_key.clone(), record_set);
        self.state
            .subscription_support_counter_snapshot
            .record_published();
        self.state
            .subscription_support_counter_snapshot
            .record_family_catalog_lookup();
        if let Err(error) = self.state.verify_subscription_support_record_family() {
            self.state
                .subscription_support_record_sets
                .remove(&storage_key);
            self.state.subscription_support_counter_snapshot = previous_counter_snapshot;
            self.state
                .subscription_support_counter_snapshot
                .record_malformed_support_record();
            return Err(error);
        }

        let report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                self.state
                    .subscription_support_record_sets
                    .remove(&storage_key);
                self.state.subscription_support_counter_snapshot = previous_counter_snapshot;
                return Err(error);
            }
        };
        verify_durable_barrier(&mut self.counters, &report)?;
        Ok(published)
    }

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
        let fetched = request.fetched();
        let record_set = fetched.record_set();
        record_set.validate()?;

        let expected_density = SubscriptionSupportCatalog::first_ship()
            .density_for(record_set.family_kind())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support classification requires an admitted catalog family",
                )
            })?;
        if expected_density != request.plan().density_class {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support classification plan density does not match catalog family",
            ));
        }
        let support_rows = fetched.fetch_report().rows_read();
        if !request
            .plan()
            .budget()
            .admits(request.evidence().observed_payload_bytes(), support_rows)
        {
            self.state
                .subscription_support_counter_snapshot
                .record_budget_denial();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support classification exceeded its pre-resolved payload budget",
            ));
        }

        let causes = resume_drift_causes(&request);
        let (primary_cause, suppressed_causes) = classify_causes(causes);
        let classification = resume_classification(
            record_set.role(),
            record_set.basis_digest(),
            request.plan().plan_family(),
            request.evidence().retained_rebuild_basis_digest(),
            primary_cause,
        );
        self.state
            .subscription_support_counter_snapshot
            .record_classification(classification);

        Ok(SubscriptionSupportClassificationReport {
            artifact_id: crate::SubscriptionSupportArtifactId(
                record_set.key().artifact_id().to_string(),
            ),
            declaration_digest: crate::SubscriptionSupportDeclarationDigest(
                record_set.declaration_digest().to_string(),
            ),
            classification,
            primary_cause,
            suppressed_causes,
            cost_surface: SubscriptionSupportResultCostSurface::new(
                request.plan().plan_family,
                request.plan().density_class,
                request.evidence().observed_payload_bytes(),
                support_rows,
                u64::from(request.plan().restart_shard.is_some()),
                request.plan().allocation_scope,
            ),
            counter_snapshot: self.state.subscription_support_counter_snapshot.clone(),
        })
    }

    pub fn reconstruct_subscription_support_restart_shard(
        &mut self,
        request: SubscriptionSupportRestartReconstructionRequest,
    ) -> Result<SubscriptionSupportRestartReconstructionReport, StoreError> {
        if SubscriptionSupportCatalog::first_ship()
            .density_for(request.shard().family_kind())
            .is_none()
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support restart reconstruction requires an admitted catalog family",
            ));
        }
        let family_id = request.shard().family_id().as_str();
        let prefix = format!("{family_id}\u{1f}");
        let upper = format!("{family_id}\u{1f}\u{10ffff}");
        let shard_key = request.shard().shard_key();
        let record_sets = self
            .state
            .subscription_support_record_sets
            .range(prefix.clone()..upper)
            .filter(|(storage_key, _)| storage_key.starts_with(&prefix))
            .filter(|(_, record_set)| record_set.restart_shard() == Some(shard_key.as_str()))
            .map(|(_, record_set)| record_set.clone())
            .collect::<Vec<_>>();
        if record_sets.len() as u64 > request.max_support_rows() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support restart shard exceeded its bounded reconstruction row budget",
            ));
        }
        let support_rows_read = record_sets.len() as u64;

        let mut reports = Vec::new();
        for record_set in record_sets {
            if record_set.family_kind() != request.shard().family_kind() {
                return Err(StoreError::new(
                    StoreErrorKind::SubscriptionSupportClassificationViolation,
                    "subscription-support restart shard rejected cross-family-kind record reuse",
                ));
            }
            let fetched = FetchedSubscriptionSupportArtifact::new(
                record_set.clone(),
                SubscriptionSupportFetchReport::direct_lookup(1),
            );
            let evidence = SubscriptionSupportResumeEvidence::matching(&fetched, 0, true)?;
            let plan = restart_plan_for_record(&record_set, shard_key.clone())?;
            reports.push(self.classify_subscription_support_resume(
                SubscriptionSupportResumeRequest::new(fetched, evidence, plan),
            )?);
        }

        self.state
            .subscription_support_counter_snapshot
            .record_restart_reconstruction(1);
        Ok(SubscriptionSupportRestartReconstructionReport::new(
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
        if self
            .state
            .subscription_support_record_sets
            .contains_key(&storage_key)
        {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support missing recovery received an artifact that is still durable",
            ));
        }
        let admitted_family = SubscriptionSupportCatalog::first_ship()
            .density_for(request.family_kind())
            .is_some();
        let rebuild_evidence_present = !request.basis_digest().trim().is_empty()
            && !request.cursor_digest().trim().is_empty()
            && !request.checkpoint_digest().trim().is_empty();
        let classification = if admitted_family
            && rebuild_evidence_present
            && request.family_kind()
                == crate::SubscriptionSupportFamilyKind::MaterializedNarrowingSupport
            && request.retained_rebuild_basis_digest() == Some(request.basis_digest())
        {
            self.state
                .subscription_support_counter_snapshot
                .record_rebuild_basis_plan();
            SubscriptionResumeClassification::RebuildRequired
        } else {
            SubscriptionResumeClassification::NotResumable
        };
        self.state
            .subscription_support_counter_snapshot
            .record_classification(classification);
        let maintenance_report = if classification
            == SubscriptionResumeClassification::RebuildRequired
        {
            let maintenance_admission =
                request.rebuild_maintenance_admission().ok_or_else(|| {
                    StoreError::new(
                        StoreErrorKind::SubscriptionSupportClassificationViolation,
                        "subscription-support rebuild-required missing recovery requires maintenance admission planning",
                    )
                })?;
            let basis = SubscriptionSupportOperationalBasis::new(
                request.family_id().clone(),
                request.family_kind(),
                request.support_role(),
                request.missing_artifact_id().clone(),
                request.basis_digest(),
                request.cursor_digest(),
                request.checkpoint_digest(),
                request.compatibility_digest(),
                request.portability_digest(),
                crate::SubscriptionSupportActionOrigin::Maintenance,
            )?;
            let plan = self.admit_subscription_support_maintenance_batch(
                maintenance_admission.action_id().clone(),
                vec![basis],
                SubscriptionSupportMaintenanceDecision::rebuild_descriptor_admitted(
                    maintenance_admission
                        .retained_rebuild_basis_digest()
                        .ok_or_else(|| {
                            StoreError::new(
                                StoreErrorKind::SubscriptionSupportClassificationViolation,
                                "subscription-support rebuild-required missing recovery lost retained basis planning",
                            )
                        })?,
                )?,
                SupportPathClass::MaintenanceExecution,
                SupportProgramDensityClass::MaintenanceKeyBatch,
                SupportAllocationScope::FamilyLocalBatch,
                maintenance_admission.breadth_budget().clone(),
                maintenance_admission.payload_header_bytes(),
            )?;
            Some(self.publish_subscription_support_maintenance_consequence(plan)?)
        } else {
            None
        };
        Ok(SubscriptionSupportMissingSupportRecoveryReport::new(
            &request,
            classification,
            maintenance_report,
        ))
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
        let plan = handoff_plan_for_record(fetched.record_set())?;
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

    pub fn admit_subscription_support_program_path(
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
            Err(error) => {
                match (path_class, density_class) {
                    (SupportPathClass::ForegroundResume | SupportPathClass::ForegroundRead, _) => {
                        self.state
                            .subscription_support_counter_snapshot
                            .record_support_hot_path_rejection();
                    }
                    (_, SupportProgramDensityClass::StoreGlobalDebt) => {
                        self.state
                            .subscription_support_counter_snapshot
                            .record_support_store_global_debt_rejection();
                    }
                    _ => {}
                }
                Err(error)
            }
        }
    }

    pub fn reuse_subscription_support_batch_receipt<'a>(
        &mut self,
        plan: &'a SupportProgramPathPlan,
    ) -> Result<&'a SupportBatchAdmissionReceipt, StoreError> {
        let receipt = plan.batch_receipt().ok_or_else(|| {
            StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support path plan has no reusable batch receipt",
            )
        })?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_batch_receipt_reuse();
        Ok(receipt)
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
        let affected_set = SupportAffectedSet::from_retention_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_subscription_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let plan = SupportRetentionBatchPlan::new(action_id, affected_set, path_plan, decision)?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_retention_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_subscription_support_retention_consequence(
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
        self.state
            .subscription_support_counter_snapshot
            .record_support_action_envelope_publication();
        let completed = raw_action.plan().verify().execute().publish().complete();
        let survival_witness =
            SupportRetentionSurvivalWitness::new(&completed, decision.verdict(), &affected_set)?;
        let materialization =
            SubscriptionSupportRetentionMaterialization::from_decision(affected_set, &decision)?;
        let report = SubscriptionSupportPostActionReport::new(
            completed,
            survival_witness,
            materialization,
            decision_kind,
            &path_plan,
        )?;
        if decision.is_reclaim() {
            self.state
                .subscription_support_counter_snapshot
                .record_support_reclaim_consequence();
        }
        match decision_kind {
            crate::SubscriptionSupportRetentionDecisionKind::RetainExact
            | crate::SubscriptionSupportRetentionDecisionKind::RetainDegraded => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_retained_family();
            }
            crate::SubscriptionSupportRetentionDecisionKind::CompactExact => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_compacted_basis();
            }
            crate::SubscriptionSupportRetentionDecisionKind::ReclaimWithRebuild
            | crate::SubscriptionSupportRetentionDecisionKind::ReclaimWithoutRebuild => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_reclaimed_family();
            }
            crate::SubscriptionSupportRetentionDecisionKind::ExpireByPolicy => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_expired_family();
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_policy_expiration();
            }
        }
        Ok(report)
    }

    pub fn publish_subscription_support_reclaim_consequence(
        &mut self,
        plan: SupportRetentionBatchPlan,
    ) -> Result<SupportReclaimConsequence, StoreError> {
        if !plan.decision().is_reclaim() {
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support reclaim consequences require a reclaim retention decision",
            ));
        }
        let report = self.publish_subscription_support_retention_consequence(plan)?;
        SupportReclaimConsequence::new(
            report.completed_action().clone(),
            report.survival_witness().clone(),
            report.retention_record().clone(),
            report.materialization().clone(),
        )
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
        let affected_set =
            SupportCompatibilityAffectedSet::from_compatibility_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_subscription_support_program_path(
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
        self.state
            .subscription_support_counter_snapshot
            .record_support_compatibility_plan(plan.affected_set().affected_count());
        Ok(plan)
    }

    pub fn publish_subscription_support_compatibility_consequence(
        &mut self,
        plan: SupportCompatibilityBatchPlan,
    ) -> Result<SubscriptionSupportCompatibilityReport, StoreError> {
        let (action_id, affected_set, path_plan, manifest_admission, semantic_access, decision) =
            plan.into_parts();
        let raw_action = RawSupportProgramAction::new(
            action_id,
            affected_set.primary_basis().clone(),
            decision.verdict(),
        )?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_action_envelope_publication();
        let completed = raw_action.plan().verify().execute().publish().complete();
        let report = SubscriptionSupportCompatibilityReport::new(
            completed,
            affected_set,
            &path_plan,
            manifest_admission,
            semantic_access,
            &decision,
        )?;
        match decision.kind() {
            SubscriptionSupportCompatibilityDecisionKind::ExactCompatibleMigration => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_exact_compatible_migration();
            }
            SubscriptionSupportCompatibilityDecisionKind::DegradedCompatibility => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_degraded_compatibility();
            }
            SubscriptionSupportCompatibilityDecisionKind::OldReaderRejected
            | SubscriptionSupportCompatibilityDecisionKind::UnknownFamilyRejected
            | SubscriptionSupportCompatibilityDecisionKind::VersionSkewRejected => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_version_skew_rejection();
            }
        }
        Ok(report)
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
        let affected_set = SupportPortabilityAffectedSet::from_portability_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        if !manifest_budget.admits(included_support_count, manifest_header_bytes) {
            self.state
                .subscription_support_counter_snapshot
                .record_support_capsule_manifest_budget_denial();
            return Err(StoreError::new(
                StoreErrorKind::SubscriptionSupportClassificationViolation,
                "subscription-support capsule manifest exceeds portability manifest budget before footprint materialization",
            ));
        }
        let path_plan = self.admit_subscription_support_program_path(
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
        let manifest = match crate::CapsuleSupportManifest::new(
            &affected_set,
            footprint.clone(),
            manifest_budget,
            manifest_header_bytes,
            &basis_artifact_ids,
        ) {
            Ok(manifest) => manifest,
            Err(error) => {
                self.state
                    .subscription_support_counter_snapshot
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
        self.state
            .subscription_support_counter_snapshot
            .record_support_portability_plan(
                plan.manifest().manifest_entry_count(),
                plan.manifest().required_basis_count(),
                plan.manifest().omitted_support_count(),
            );
        Ok(plan)
    }

    pub fn publish_subscription_support_portability_consequence(
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
        self.state
            .subscription_support_counter_snapshot
            .record_support_action_envelope_publication();
        let completed = raw_action.plan().verify().execute().publish().complete();
        let report = SubscriptionSupportPortabilityReport::new(
            completed,
            affected_set,
            manifest,
            &decision,
            &path_plan,
        )?;
        match report.outcome() {
            SubscriptionSupportPortabilityOutcome::FullScopeReplicated(bundle) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_replication_inclusion(bundle.preserved_count());
            }
            SubscriptionSupportPortabilityOutcome::PartialScopeOmitted(omission) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_replication_omission(omission.omitted_count());
            }
            SubscriptionSupportPortabilityOutcome::Imported(_) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::ImportedNotResumable(_) => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_import_admission();
            }
            SubscriptionSupportPortabilityOutcome::Rejected(_) => {
                if decision.kind()
                    == SubscriptionSupportPortabilityDecisionKind::UnsupportedFamilyRejected
                {
                    self.state
                        .subscription_support_counter_snapshot
                        .record_support_import_rejection();
                }
            }
        }
        Ok(report)
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
        let affected_set = SupportMaintenanceAffectedSet::from_maintenance_bases(affected_bases)?;
        let affected_entries = affected_set.affected_count();
        let path_plan = self.admit_subscription_support_program_path(
            path_class,
            density_class,
            allocation_scope,
            budget,
            affected_entries,
            payload_header_bytes,
        )?;
        let (descriptors, coalesced_duplicate_count) = affected_set.descriptors_for(&decision)?;
        let maintenance_batch = support_maintenance_batch(&action_id, &descriptors);
        let maintenance_receipt = self.admit_maintenance_batch(maintenance_batch)?;
        let plan = SupportMaintenanceBatchPlan::new(
            action_id,
            affected_set,
            path_plan,
            descriptors,
            maintenance_receipt,
            coalesced_duplicate_count,
            decision,
        )?;
        self.state
            .subscription_support_counter_snapshot
            .record_support_maintenance_plan(
                plan.descriptors().len() as u64,
                plan.coalesced_duplicate_count(),
            );
        Ok(plan)
    }

    pub fn publish_subscription_support_maintenance_consequence(
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
        let completed = raw_action.plan().verify().execute().publish().complete();
        let report = SubscriptionSupportMaintenanceReport::new(
            completed,
            affected_set,
            descriptors,
            &maintenance_receipt,
            coalesced_duplicate_count,
            &decision,
            &path_plan,
        )?;
        let previous_counter_snapshot = self.state.subscription_support_counter_snapshot.clone();
        let mut inserted_keys = Vec::new();
        for record in report.descriptor_records() {
            let key = record.record_key().to_string();
            match self
                .state
                .subscription_support_maintenance_descriptor_records
                .get(&key)
            {
                Some(existing) if existing == record => {}
                Some(_) => {
                    for inserted_key in inserted_keys {
                        self.state
                            .subscription_support_maintenance_descriptor_records
                            .remove(&inserted_key);
                    }
                    return Err(StoreError::new(
                        StoreErrorKind::SubscriptionSupportPublicationViolation,
                        "subscription-support maintenance descriptor record collided with a different durable descriptor row",
                    ));
                }
                None => {
                    self.state
                        .subscription_support_maintenance_descriptor_records
                        .insert(key.clone(), record.clone());
                    inserted_keys.push(key);
                }
            }
        }
        self.state
            .subscription_support_counter_snapshot
            .record_support_action_envelope_publication();
        match decision.kind() {
            SubscriptionSupportMaintenanceDecisionKind::RebuildDescriptorAdmitted => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_maintenance_rebuild_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::RefreshDescriptorAdmitted => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_maintenance_refresh_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::CompatibilityMigrationDescriptorAdmitted => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_maintenance_compatibility_migration_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::DegradationRecoveryDescriptorAdmitted => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_maintenance_degradation_recovery_descriptor();
            }
            SubscriptionSupportMaintenanceDecisionKind::InterruptedRestartRecovered => {
                self.state
                    .subscription_support_counter_snapshot
                    .record_support_maintenance_interrupted_restart_recovery();
            }
        }
        let persist_report = match self.persistence.persist_state(&self.state) {
            Ok(report) => report,
            Err(error) => {
                for key in inserted_keys {
                    self.state
                        .subscription_support_maintenance_descriptor_records
                        .remove(&key);
                }
                self.state.subscription_support_counter_snapshot = previous_counter_snapshot;
                return Err(error);
            }
        };
        verify_durable_barrier(&mut self.counters, &persist_report)?;
        Ok(report)
    }

    pub fn subscription_support_counters(&self) -> crate::SubscriptionSupportCounterSnapshot {
        self.state.subscription_support_counter_snapshot.clone()
    }

    pub fn subscription_support_access_structure_report(
        &self,
    ) -> SubscriptionSupportAccessStructureReport {
        let required = SubscriptionSupportCatalog::first_ship()
            .access_structures()
            .required()
            .to_vec();
        if self.state.subscription_support_access_structures_verified {
            SubscriptionSupportCatalog::first_ship().access_structures()
        } else {
            let debted = if self
                .state
                .subscription_support_access_structure_debts
                .is_empty()
            {
                required
            } else {
                self.state
                    .subscription_support_access_structure_debts
                    .clone()
            };
            SubscriptionSupportAccessStructureReport::debt_for(debted)
        }
    }
}

fn handoff_plan_for_record(
    record_set: &SubscriptionSupportStoredRecordSet,
) -> Result<SubscriptionSupportClassificationPlan, StoreError> {
    match record_set.role() {
        SubscriptionSupportRole::ExactContinuation => {
            SubscriptionSupportClassificationPlan::exact_sparse_identity()
        }
        SubscriptionSupportRole::DegradedContinuation => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
                Some(format!(
                    "subscription-support-handoff:{}",
                    record_set.key().family_id()
                )),
            )
        }
        SubscriptionSupportRole::NarrowingMaterialization => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::FamilyLocalScratch,
                SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
                None,
            )
        }
    }
}

fn restart_plan_for_record(
    record_set: &SubscriptionSupportStoredRecordSet,
    restart_shard: String,
) -> Result<SubscriptionSupportClassificationPlan, StoreError> {
    match record_set.role() {
        SubscriptionSupportRole::ExactContinuation => SubscriptionSupportClassificationPlan::new(
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
            SubscriptionSupportAllocationScope::RestartShardBatch,
            SubscriptionSupportDensityClass::SparseIdentityClassification,
            Some(restart_shard),
        ),
        SubscriptionSupportRole::DegradedContinuation => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::RestartShardBatchClassification,
                Some(restart_shard),
            )
        }
        SubscriptionSupportRole::NarrowingMaterialization => {
            SubscriptionSupportClassificationPlan::new(
                SubscriptionSupportPlanFamily::DeniedResumeClassificationPlan,
                SubscriptionSupportPayloadBudget::new(16 * 1024, 64)?,
                SubscriptionSupportAllocationScope::RestartShardBatch,
                SubscriptionSupportDensityClass::FamilyBatchClassificationDebt,
                Some(restart_shard),
            )
        }
    }
}

fn resume_drift_causes(
    request: &SubscriptionSupportResumeRequest,
) -> Vec<SubscriptionSupportDriftCause> {
    let record_set = request.fetched().record_set();
    let evidence = request.evidence();
    let mut causes = Vec::new();
    if evidence.expected_family_kind() != record_set.family_kind() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportFamilyMismatch);
    }
    if evidence.compatibility_digest() != record_set.compatibility_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportCompatibilityDrift);
    }
    if evidence.basis_digest() != record_set.basis_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportBasisDrift);
    }
    if evidence.schema_digest() != record_set.schema_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportSchemaDrift);
    }
    if evidence.cursor_digest() != record_set.cursor_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportCursorDrift);
    }
    if evidence.checkpoint_digest() != record_set.checkpoint_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportCheckpointDrift);
    }
    if evidence.support_artifact_digest() != record_set.artifact_digest() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch);
    }
    if evidence.placement_unavailable() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable);
    }
    if !evidence.session_memory_present() {
        causes.push(SubscriptionSupportDriftCause::SubscriptionSupportSessionMemoryMissing);
    }
    causes
}

fn resume_classification(
    role: SubscriptionSupportRole,
    durable_basis_digest: &str,
    plan_family: SubscriptionSupportPlanFamily,
    retained_rebuild_basis_digest: Option<&str>,
    primary_cause: Option<SubscriptionSupportDriftCause>,
) -> SubscriptionResumeClassification {
    match (role, plan_family, primary_cause) {
        (
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            None,
        ) => SubscriptionResumeClassification::Exact,
        (
            SubscriptionSupportRole::DegradedContinuation,
            SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
            None,
        ) => SubscriptionResumeClassification::Degraded,
        (
            SubscriptionSupportRole::NarrowingMaterialization,
            SubscriptionSupportPlanFamily::RebuildPlanClassificationPlan,
            Some(SubscriptionSupportDriftCause::SubscriptionSupportDigestMismatch),
        ) if retained_rebuild_basis_digest == Some(durable_basis_digest) => {
            SubscriptionResumeClassification::RebuildRequired
        }
        (
            SubscriptionSupportRole::ExactContinuation,
            SubscriptionSupportPlanFamily::ExactResumeClassificationPlan,
            Some(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable),
        ) => SubscriptionResumeClassification::Exact,
        (
            SubscriptionSupportRole::DegradedContinuation,
            SubscriptionSupportPlanFamily::DegradedResumeClassificationPlan,
            Some(SubscriptionSupportDriftCause::SubscriptionSupportPlacementUnavailable),
        ) => SubscriptionResumeClassification::Degraded,
        _ => SubscriptionResumeClassification::NotResumable,
    }
}
