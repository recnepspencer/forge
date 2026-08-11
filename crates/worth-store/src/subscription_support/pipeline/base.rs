use super::super::witnesses::{
    SubscriptionSupportBasisWitness, SubscriptionSupportCheckpointWitness,
    SubscriptionSupportCompatibilityWitness, SubscriptionSupportCursorWitness,
    SubscriptionSupportSchemaWitness,
};
use super::super::{
    classification_error, classify_causes, ensure_classification, ensure_report_matches_artifact,
    AdmittedSubscriptionSupportDeclaration, DegradedSubscriptionResumeHandle,
    ExactSubscriptionResumeHandle, PublishableSubscriptionSupportArtifact,
    PublishedSubscriptionSupportArtifact, RawSubscriptionSupportDeclaration,
    SubscriptionResumeClassification, SubscriptionResumeDeniedReport,
    SubscriptionSupportClassificationPlan, SubscriptionSupportClassificationReport,
    SubscriptionSupportDriftCause, SubscriptionSupportFamilyKind, SubscriptionSupportPlanFamily,
    SubscriptionSupportRebuildPlanHandle, SubscriptionSupportResultCostSurface,
    SubscriptionSupportRole,
};
use super::SubscriptionSupportPublicationPipeline;
use crate::failure::StoreError;

impl SubscriptionSupportPublicationPipeline {
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
