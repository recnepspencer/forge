use std::sync::Arc;

use super::*;

use crate::speculation::{
    BridgePreviewDiscardRecord, BridgePreviewExecutionRecord, BridgePreviewResidueClass,
    BridgePreviewPromotionRecord, BridgePreviewReplayBundle, BridgePreviewReuseEquivalence,
    BridgePreviewResidueReport, BridgePreviewSession, BridgePreviewSessionDeclaration,
    BridgePreviewSessionIdentity, BridgePromotionAdmissibilityProof, BridgeSpeculationCounters,
    PreviewActive, PreviewAdmitted, PreviewDeclared, PreviewDiscarded, PreviewPromoted,
    PreviewSessionActivation,
};

impl RuntimeBridge {
    pub fn validate_preview_session_declaration(
        &self,
        declaration: BridgePreviewSessionDeclaration,
    ) -> Result<ValidatedBridgePreviewSessionDeclaration, BridgeSpeculationError> {
        declaration.validate()
    }

    pub fn declare_preview_session(
        &self,
        session_identity: BridgePreviewSessionIdentity,
        declaration: BridgePreviewSessionDeclaration,
    ) -> Result<BridgePreviewSession<PreviewDeclared>, BridgeSpeculationError> {
        let validated = self.validate_preview_session_declaration(declaration)?;
        if !self
            .diagnostics
            .reserve_preview_session_identity(session_identity.as_str())
        {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewSessionIdentityConflict,
                format!(
                    "Preview session identity `{}` had already been reserved by this runtime.",
                    session_identity.as_str(),
                ),
            ));
        }
        Ok(BridgePreviewSession::declare(session_identity, validated))
    }

    pub fn admit_preview_session(
        &self,
        session_identity: BridgePreviewSessionIdentity,
        declaration: BridgePreviewSessionDeclaration,
    ) -> Result<BridgePreviewSession<PreviewAdmitted>, BridgeSpeculationError> {
        self.declare_preview_session(session_identity, declaration)
            .map(BridgePreviewSession::admit)
    }

    pub fn activate_preview_session(
        &self,
        session: BridgePreviewSession<PreviewAdmitted>,
        preview_artifact_count: usize,
        destroyable_artifact_count: usize,
        retained_non_authoritative_artifact_count: usize,
    ) -> (BridgePreviewSession<PreviewActive>, BridgePreviewExecutionRecord) {
        let counters = BridgeSpeculationCounters::for_preview_execution(
            preview_artifact_count,
            destroyable_artifact_count,
            retained_non_authoritative_artifact_count,
        );
        let execution_record =
            BridgePreviewExecutionRecord::from_admitted_session(&session, counters);
        let active = session.activate(PreviewSessionActivation::new(
            execution_record.record_identity().clone(),
        ));
        self.diagnostics
            .record_preview_execution(execution_record.clone());
        (active, execution_record)
    }

    pub fn admit_preview_reuse(
        &self,
        source_session: &BridgePreviewSession<PreviewActive>,
        source_execution_record: &BridgePreviewExecutionRecord,
        target_session: &BridgePreviewSession<PreviewAdmitted>,
    ) -> Result<BridgePreviewReuseEquivalence, BridgeSpeculationError> {
        self.ensure_execution_record_matches_active_session(source_session, source_execution_record)?;

        let equivalence =
            BridgePreviewReuseEquivalence::between_sessions(source_session, target_session);
        if !equivalence.matches_sessions(source_session, target_session) {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewReuseEquivalenceMismatch,
                format!(
                    "Preview reuse proof `{}` did not admit source session `{}` and target session `{}` as exactly equivalent.",
                    equivalence.equivalence_identity().as_str(),
                    source_session.session_identity().as_str(),
                    target_session.session_identity().as_str(),
                ),
            ));
        }

        Ok(equivalence)
    }

    pub fn activate_preview_session_with_reuse(
        &self,
        session: BridgePreviewSession<PreviewAdmitted>,
        source_session: &BridgePreviewSession<PreviewActive>,
        source_execution_record: &BridgePreviewExecutionRecord,
        reuse_equivalence: &BridgePreviewReuseEquivalence,
    ) -> Result<(BridgePreviewSession<PreviewActive>, BridgePreviewExecutionRecord), BridgeSpeculationError>
    {
        self.ensure_execution_record_matches_active_session(source_session, source_execution_record)?;

        if !reuse_equivalence.matches_sessions(source_session, &session) {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewReuseEquivalenceMismatch,
                format!(
                    "Preview reuse proof `{}` did not match source session `{}` and target session `{}`.",
                    reuse_equivalence.equivalence_identity().as_str(),
                    source_session.session_identity().as_str(),
                    session.session_identity().as_str(),
                ),
            ));
        }

        let counters = BridgeSpeculationCounters::for_preview_execution(
            source_execution_record.counters().preview_artifact_count(),
            source_execution_record.counters().discard_artifact_count(),
            source_execution_record
                .counters()
                .retained_non_authoritative_artifact_count(),
        );
        let execution_record = BridgePreviewExecutionRecord::from_reused_active_session(
            source_session,
            &session,
            source_execution_record,
            reuse_equivalence,
            counters,
        );
        let active = session.activate(PreviewSessionActivation::new(
            execution_record.record_identity().clone(),
        ));
        self.diagnostics
            .record_preview_execution(execution_record.clone());
        Ok((active, execution_record))
    }

    pub fn discard_preview_session(
        &self,
        session: BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        residue_classes: Vec<BridgePreviewResidueClass>,
    ) -> Result<
        (
            BridgePreviewSession<PreviewDiscarded>,
            BridgePreviewDiscardRecord,
        ),
        BridgeSpeculationError,
    > {
        self.ensure_execution_record_matches_active_session(&session, execution_record)?;
        self.ensure_session_not_terminal(session.session_identity().as_str())?;

        let residue_report = BridgePreviewResidueReport::new(residue_classes);
        if residue_report.authoritative_residue_count() > 0 {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewResidueClassificationMismatch,
                format!(
                    "Preview session `{}` cannot discard while residue report `{}` still contains authoritative residue.",
                    session.session_identity().as_str(),
                    residue_report.digest(),
                ),
            ));
        }

        let counters = BridgeSpeculationCounters::for_discard(
            residue_report.destroyable_residue_count(),
            residue_report.destroyable_residue_count(),
            residue_report.retained_non_authoritative_count(),
        );
        let discard_record = BridgePreviewDiscardRecord::from_active_session(
            &session,
            execution_record,
            residue_report,
            counters,
        );
        let discarded = session.discard();
        self.diagnostics.record_preview_discard(discard_record.clone());
        Ok((discarded, discard_record))
    }

    pub fn promote_preview_session(
        &self,
        session: BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
        proof: &BridgePromotionAdmissibilityProof,
        authoritative_commit_boundary_digest: impl Into<Arc<str>>,
        authoritative_artifact_digest: impl Into<Arc<str>>,
    ) -> Result<
        (
            BridgePreviewSession<PreviewPromoted>,
            BridgePreviewPromotionRecord,
        ),
        BridgeSpeculationError,
    > {
        self.ensure_execution_record_matches_active_session(&session, execution_record)?;
        self.ensure_session_not_terminal(session.session_identity().as_str())?;

        if !proof.matches_active_session(&session) {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch,
                format!(
                    "Promotion proof `{}` did not match active preview session `{}`.",
                    proof.proof_identity().as_str(),
                    session.session_identity().as_str(),
                ),
            ));
        }

        let counters = BridgeSpeculationCounters::for_promotion(9, 1, 2);
        let promotion_record = BridgePreviewPromotionRecord::from_active_session(
            &session,
            execution_record,
            proof,
            authoritative_commit_boundary_digest,
            authoritative_artifact_digest,
            counters,
        );
        let promoted = session.promote(proof)?;
        self.diagnostics
            .record_preview_promotion(promotion_record.clone());
        Ok((promoted, promotion_record))
    }

    pub fn replay_preview_bundle(
        &self,
        preview_session_identity: &str,
    ) -> Result<BridgePreviewReplayBundle, BridgeSpeculationError> {
        let execution_record = self
            .diagnostics
            .preview_execution_record_for_session_identity(preview_session_identity)
            .ok_or_else(|| {
                BridgeSpeculationError::new(
                    BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                    format!(
                        "No retained preview execution record existed for preview session `{preview_session_identity}`."
                    ),
                )
            })?;
        let discard_record = self
            .diagnostics
            .preview_discard_record_for_session_identity(preview_session_identity);
        let promotion_record = self
            .diagnostics
            .preview_promotion_record_for_session_identity(preview_session_identity);
        if discard_record.is_some() && promotion_record.is_some() {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::IllegalPreviewLifecycleTransition,
                format!(
                    "Preview session `{preview_session_identity}` retained both discard and promotion terminal records."
                ),
            ));
        }

        Ok(BridgePreviewReplayBundle::new(
            execution_record,
            discard_record,
            promotion_record,
        ))
    }

    fn ensure_execution_record_matches_active_session(
        &self,
        session: &BridgePreviewSession<PreviewActive>,
        execution_record: &BridgePreviewExecutionRecord,
    ) -> Result<(), BridgeSpeculationError> {
        let execution_record_identity = session
            .execution_record_identity()
            .expect("active preview sessions must carry execution record identity");
        if execution_record.record_identity() != execution_record_identity {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                format!(
                    "Preview execution record `{}` did not belong to active preview session `{}`.",
                    execution_record.record_identity().as_str(),
                    session.session_identity().as_str(),
                ),
            ));
        }
        if execution_record.preview_session_identity() != session.session_identity().as_str() {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                format!(
                    "Preview execution record `{}` was built for session `{}` but active preview session was `{}`.",
                    execution_record.record_identity().as_str(),
                    execution_record.preview_session_identity(),
                    session.session_identity().as_str(),
                ),
            ));
        }
        if execution_record.preview_declaration_digest() != session.declaration().digest() {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PreviewBranchBindingMismatch,
                format!(
                    "Preview execution record `{}` was built from declaration digest `{}` but active preview session declaration digest was `{}`.",
                    execution_record.record_identity().as_str(),
                    execution_record.preview_declaration_digest(),
                    session.declaration().digest(),
                ),
            ));
        }

        Ok(())
    }

    fn ensure_session_not_terminal(
        &self,
        preview_session_identity: &str,
    ) -> Result<(), BridgeSpeculationError> {
        if self
            .diagnostics
            .preview_discard_record_for_session_identity(preview_session_identity)
            .is_some()
        {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch,
                format!(
                    "Preview session `{preview_session_identity}` had already been discarded and cannot be promoted."
                ),
            ));
        }
        if self
            .diagnostics
            .preview_promotion_record_for_session_identity(preview_session_identity)
            .is_some()
        {
            return Err(BridgeSpeculationError::new(
                BridgeSpeculationErrorKind::PromotionAdmissibilityMismatch,
                format!(
                    "Preview session `{preview_session_identity}` had already been promoted and cannot be promoted again."
                ),
            ));
        }

        Ok(())
    }
}
