use crate::subscription_support::classify_causes;
use crate::{
    failure::{StoreError, StoreErrorKind},
    SubscriptionResumeClassification, SubscriptionSupportArtifactId, SubscriptionSupportCatalog,
    SubscriptionSupportClassificationReport, SubscriptionSupportCounterSnapshot,
    SubscriptionSupportDeclarationDigest, SubscriptionSupportDriftCause,
    SubscriptionSupportPlanFamily, SubscriptionSupportResultCostSurface,
    SubscriptionSupportResumeRequest, SubscriptionSupportRole, SubscriptionSupportStoredRecordSet,
};

pub(super) struct ResumeClassificationAdmission<'a> {
    pub(super) record_set: &'a SubscriptionSupportStoredRecordSet,
    pub(super) support_rows: u64,
}

pub(super) fn admit_resume_classification_request<'a>(
    request: &'a SubscriptionSupportResumeRequest,
) -> Result<ResumeClassificationAdmission<'a>, StoreError> {
    let record_set = request.fetched().record_set();
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
    Ok(ResumeClassificationAdmission {
        record_set,
        support_rows: request.fetched().fetch_report().rows_read(),
    })
}

pub(super) fn resume_budget_admitted(
    request: &SubscriptionSupportResumeRequest,
    admission: &ResumeClassificationAdmission<'_>,
) -> bool {
    request.plan().budget().admits(
        request.evidence().observed_payload_bytes(),
        admission.support_rows,
    )
}

pub(super) struct ResumeClassificationProjection {
    pub(super) classification: SubscriptionResumeClassification,
    primary_cause: Option<SubscriptionSupportDriftCause>,
    suppressed_causes: Vec<SubscriptionSupportDriftCause>,
}

pub(super) fn project_resume_classification(
    request: &SubscriptionSupportResumeRequest,
    admission: &ResumeClassificationAdmission<'_>,
) -> ResumeClassificationProjection {
    let causes = resume_drift_causes(request);
    let (primary_cause, suppressed_causes) = classify_causes(causes);
    let classification = classify_resume_outcome(ResumeClassificationEvidence {
        role: admission.record_set.role(),
        durable_basis_digest: admission.record_set.basis_digest(),
        plan_family: request.plan().plan_family(),
        retained_rebuild_basis_digest: request.evidence().retained_rebuild_basis_digest(),
        primary_cause,
    });
    ResumeClassificationProjection {
        classification,
        primary_cause,
        suppressed_causes,
    }
}

pub(super) fn publish_resume_classification_report(
    request: &SubscriptionSupportResumeRequest,
    admission: &ResumeClassificationAdmission<'_>,
    projection: ResumeClassificationProjection,
    counter_snapshot: SubscriptionSupportCounterSnapshot,
) -> SubscriptionSupportClassificationReport {
    SubscriptionSupportClassificationReport {
        artifact_id: SubscriptionSupportArtifactId(
            admission.record_set.key().artifact_id().to_string(),
        ),
        declaration_digest: SubscriptionSupportDeclarationDigest(
            admission.record_set.declaration_digest().to_string(),
        ),
        classification: projection.classification,
        primary_cause: projection.primary_cause,
        suppressed_causes: projection.suppressed_causes,
        cost_surface: SubscriptionSupportResultCostSurface::new(
            request.plan().plan_family,
            request.plan().density_class,
            request.evidence().observed_payload_bytes(),
            admission.support_rows,
            u64::from(request.plan().restart_shard.is_some()),
            request.plan().allocation_scope,
        ),
        counter_snapshot,
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

struct ResumeClassificationEvidence<'a> {
    role: SubscriptionSupportRole,
    durable_basis_digest: &'a str,
    plan_family: SubscriptionSupportPlanFamily,
    retained_rebuild_basis_digest: Option<&'a str>,
    primary_cause: Option<SubscriptionSupportDriftCause>,
}

fn classify_resume_outcome(
    evidence: ResumeClassificationEvidence<'_>,
) -> SubscriptionResumeClassification {
    let ResumeClassificationEvidence {
        role,
        durable_basis_digest,
        plan_family,
        retained_rebuild_basis_digest,
        primary_cause,
    } = evidence;
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
