use crate::backend::records::{AuthoritativeArtifactFamily, StableBasisRecord, StoreState};
use crate::failure::{StoreError, StoreErrorKind};
use crate::live_query::basis::{StableBasisPublicationPlan, StableBasisReadRequest};
use crate::live_query::restart::StableBasisSurvival;

use super::{stable_basis_artifact_id, stable_structural_digest};

impl StoreState {
    pub fn admit_stable_basis_publication(
        &self,
        request: StableBasisReadRequest,
    ) -> Result<StableBasisPublicationPlan, StoreError> {
        let plan = crate::StableBasisReadPlan::new(
            request.clone(),
            crate::StableBasisId::from_request(&request),
        )
        .into_publication_plan();
        let record = plan.to_record();
        let frontier_commit = self
            .commit_record(record.request.frontier_commit_id())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::StableBasisShapeViolation,
                    format!(
                        "stable basis `{}` references missing frontier commit {}",
                        record.artifact_id,
                        record.request.frontier_commit_id().0
                    ),
                )
            })?;
        if frontier_commit.envelope.branch_context != *record.request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                format!(
                    "stable basis `{}` expected branch `{}` but frontier commit {} belongs to `{}`",
                    record.artifact_id,
                    record.request.branch_id().0,
                    record.request.frontier_commit_id().0,
                    frontier_commit.envelope.branch_context.0
                ),
            ));
        }
        self.verify_stable_basis_authority_binding(&record, frontier_commit)?;
        self.verify_stable_basis_support_context_binding(&record)?;
        self.verify_stable_basis_schema_reference(&record)?;
        Ok(plan)
    }

    fn verify_stable_basis_authority_binding(
        &self,
        record: &StableBasisRecord,
        frontier_commit: &crate::backend::records::StoredCommitEnvelope,
    ) -> Result<(), StoreError> {
        if frontier_commit.envelope_digest != record.request.authority_basis_digest() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                format!(
                    "stable basis `{}` authority-basis digest drifted from frontier commit {}",
                    record.artifact_id,
                    record.request.frontier_commit_id().0
                ),
            ));
        }
        Ok(())
    }

    fn verify_stable_basis_support_context_binding(
        &self,
        record: &StableBasisRecord,
    ) -> Result<(), StoreError> {
        let summary = self
            .commit_support_summaries
            .get(&record.request.frontier_commit_id().0)
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::StableBasisSupportContextMismatch,
                    format!(
                        "stable basis `{}` requires commit support summary for frontier {}",
                        record.artifact_id,
                        record.request.frontier_commit_id().0
                    ),
                )
            })?;
        let expected_digest = stable_structural_digest(summary)?;
        if expected_digest != record.request.support_context_digest() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisSupportContextMismatch,
                format!(
                    "stable basis `{}` support-context digest drifted from frontier support summary",
                    record.artifact_id
                ),
            ));
        }
        Ok(())
    }

    fn verify_stable_basis_schema_reference(
        &self,
        record: &StableBasisRecord,
    ) -> Result<(), StoreError> {
        if let Some(schema_support) = self
            .schema_support_records
            .get(record.request.schema_boundary_artifact_id())
        {
            if schema_support.branch_id != *record.request.branch_id()
                || schema_support.commit_id != record.request.frontier_commit_id()
            {
                return Err(StoreError::new(
                    StoreErrorKind::StableBasisSchemaMismatch,
                    format!(
                        "stable basis `{}` drifted from schema support artifact `{}` for branch `{}` and frontier {}",
                        record.artifact_id,
                        record.request.schema_boundary_artifact_id(),
                        record.request.branch_id().0,
                        record.request.frontier_commit_id().0
                    ),
                ));
            }
            return Ok(());
        }

        if let Some(summary) = self
            .commit_support_summaries
            .get(&record.request.frontier_commit_id().0)
        {
            if summary.branch_id != *record.request.branch_id() {
                return Err(StoreError::new(
                    StoreErrorKind::StableBasisSchemaMismatch,
                    format!(
                        "stable basis `{}` support summary branch `{}` did not match requested branch `{}`",
                        record.artifact_id,
                        summary.branch_id.0,
                        record.request.branch_id().0
                    ),
                ));
            }
            if let Some(expected_schema_artifact_id) = summary.schema_support_artifact_id.as_deref()
            {
                if expected_schema_artifact_id != record.request.schema_boundary_artifact_id() {
                    return Err(StoreError::new(
                        StoreErrorKind::StableBasisSchemaMismatch,
                        format!(
                            "stable basis `{}` requested schema support artifact `{}` but frontier summary requires `{expected_schema_artifact_id}`",
                            record.artifact_id,
                            record.request.schema_boundary_artifact_id()
                        ),
                    ));
                }
            }
        }

        Ok(())
    }

    fn required_support_artifact_is_available(
        &self,
        record: &StableBasisRecord,
        artifact_id: &str,
    ) -> bool {
        if artifact_id == record.request.schema_boundary_artifact_id() {
            return self.schema_support_records.contains_key(artifact_id);
        }
        true
    }

    pub fn verify_stable_basis_record(&self, record: &StableBasisRecord) -> Result<(), StoreError> {
        if record.artifact_id.trim().is_empty()
            || record.request.support_context_digest().trim().is_empty()
            || record.request.authority_basis_digest().trim().is_empty()
        {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                "stable basis records must declare a non-empty artifact id, support-context digest, and authority-basis digest",
            ));
        }
        if record.artifact_id
            != stable_basis_artifact_id(record.requested_stable_basis_id().as_str())
        {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                format!(
                    "stable basis record `{}` drifted from its canonical stable-basis identity",
                    record.artifact_id
                ),
            ));
        }
        let frontier_commit = self
            .commit_record(record.request.frontier_commit_id())
            .ok_or_else(|| {
                StoreError::new(
                    StoreErrorKind::StableBasisShapeViolation,
                    format!(
                        "stable basis `{}` references missing frontier commit {}",
                        record.artifact_id,
                        record.request.frontier_commit_id().0
                    ),
                )
            })?;
        if frontier_commit.envelope.branch_context != *record.request.branch_id() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                format!(
                    "stable basis `{}` references frontier commit {} on branch `{}` but requested branch `{}`",
                    record.artifact_id,
                    record.request.frontier_commit_id().0,
                    frontier_commit.envelope.branch_context.0,
                    record.request.branch_id().0
                ),
            ));
        }
        self.verify_stable_basis_authority_binding(record, frontier_commit)?;
        self.verify_stable_basis_support_context_binding(record)?;
        self.verify_stable_basis_schema_reference(record)?;
        if record.minimum_retained_commit_id != record.request.frontier_commit_id() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisShapeViolation,
                format!(
                    "stable basis `{}` minimum retained commit {} must match frontier commit {} in phase 2",
                    record.artifact_id,
                    record.minimum_retained_commit_id.0,
                    record.request.frontier_commit_id().0
                ),
            ));
        }
        if !record
            .required_support_artifact_set
            .iter()
            .any(|artifact_id| artifact_id == record.request.schema_boundary_artifact_id())
        {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisSupportContextMismatch,
                format!(
                    "stable basis `{}` retention descriptor must retain schema support artifact `{}`",
                    record.artifact_id,
                    record.request.schema_boundary_artifact_id()
                ),
            ));
        }
        if record.schema_boundary_dependency != record.request.schema_boundary_artifact_id() {
            return Err(StoreError::new(
                StoreErrorKind::StableBasisSchemaMismatch,
                format!(
                    "stable basis `{}` schema dependency drifted from schema boundary artifact `{}`",
                    record.artifact_id,
                    record.request.schema_boundary_artifact_id()
                ),
            ));
        }
        self.require_digest_record(
            AuthoritativeArtifactFamily::StableBasisRecord,
            record.artifact_id.clone(),
            &stable_structural_digest(record)?,
        )?;
        Ok(())
    }

    pub fn classify_stable_basis_survival(
        &self,
        record: &StableBasisRecord,
    ) -> Result<StableBasisSurvival, StoreError> {
        self.verify_stable_basis_record(record)?;
        let missing_required_support = record
            .required_support_artifact_set
            .iter()
            .any(|artifact_id| !self.required_support_artifact_is_available(record, artifact_id));
        if !missing_required_support {
            return Ok(StableBasisSurvival::from_request(&record.request));
        }
        Ok(match StableBasisSurvival::from_request(&record.request) {
            StableBasisSurvival::Retained => StableBasisSurvival::DegradedButRecoverable {
                fallback_class: if record.authority_replay_fallback_class != "none" {
                    record.authority_replay_fallback_class.clone()
                } else {
                    "authority_replay".to_string()
                },
            },
            StableBasisSurvival::DegradedButRecoverable { fallback_class } => {
                StableBasisSurvival::DegradedButRecoverable { fallback_class }
            }
            StableBasisSurvival::Rejected { reason } => StableBasisSurvival::Rejected { reason },
        })
    }

    pub fn verify_live_query_record_family(&self) -> Result<(), StoreError> {
        for record in self.stable_basis_records.values() {
            self.verify_stable_basis_record(record)?;
        }
        Ok(())
    }
}
