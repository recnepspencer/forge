use std::collections::BTreeSet;

use crate::capabilities::LineageNodeSource;
use crate::diagnostics::data::{
    DiagnosticCode, DiagnosticsArtifactKind, DiagnosticsScope, RelationalDiagnosticsEntry,
};
use crate::history::data::BranchId;
use crate::identity::data::LineageId;
use crate::lineage::authority::diagnostic_fields::promotion_rejection_fields;
use crate::lineage::authority::phase_types::{
    BranchScopedLineageRef, RecordedCorrespondenceCandidate, ValidatedCorrespondenceCandidate,
};
use crate::lineage::authority::LineageAuthority;
use crate::lineage::data::{
    CorrespondenceCandidate, CorrespondenceCandidateId, CorrespondencePromotionRejectionClass,
    LineageDecisionKind, LineageDecisionRecord, LineageRejectionArtifact,
};

impl RecordedCorrespondenceCandidate {
    fn from_runtime(candidate: CorrespondenceCandidate) -> Self {
        Self { candidate }
    }
}

impl BranchScopedLineageRef {
    fn new(branch_id: BranchId, lineage_id: LineageId) -> Self {
        Self {
            branch_id,
            lineage_id,
        }
    }
}

impl ValidatedCorrespondenceCandidate {
    fn new(
        candidate: CorrespondenceCandidate,
        branch_scoped_sources: Vec<BranchScopedLineageRef>,
        branch_scoped_targets: Vec<BranchScopedLineageRef>,
    ) -> Self {
        Self {
            candidate,
            branch_scoped_sources,
            branch_scoped_targets,
        }
    }
}

impl<'runtime> LineageAuthority<'runtime> {
    pub(super) fn recorded_candidate(
        &self,
        candidate_id: CorrespondenceCandidateId,
    ) -> Result<RecordedCorrespondenceCandidate, CorrespondencePromotionRejectionClass> {
        let candidate = self
            .runtime
            .lineage
            .correspondence_candidates
            .iter()
            .find(|candidate| candidate.candidate_id == candidate_id)
            .cloned()
            .ok_or(CorrespondencePromotionRejectionClass::CandidateMissing)?;
        Ok(RecordedCorrespondenceCandidate::from_runtime(candidate))
    }

    pub(super) fn validate_candidate(
        &mut self,
        recorded: RecordedCorrespondenceCandidate,
    ) -> Result<ValidatedCorrespondenceCandidate, CorrespondencePromotionRejectionClass> {
        let candidate = recorded.candidate();
        let recorded_width = candidate.sources.len() + candidate.targets.len();
        if candidate.sources.is_empty() || candidate.targets.is_empty() {
            self.runtime
                .performance_access()
                .count_lineage_candidate_validation(recorded_width, 0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &candidate.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::EmptyEndpointSet,
                "correspondence promotion requires at least one source and one target lineage",
            );
            return Err(CorrespondencePromotionRejectionClass::EmptyEndpointSet);
        }
        let source_set = candidate.sources.iter().copied().collect::<BTreeSet<_>>();
        let target_set = candidate.targets.iter().copied().collect::<BTreeSet<_>>();
        if source_set.len() != candidate.sources.len()
            || target_set.len() != candidate.targets.len()
        {
            self.runtime
                .performance_access()
                .count_lineage_candidate_validation(recorded_width, 0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &candidate.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::DuplicateEndpointReference,
                "correspondence promotion referenced duplicate lineage endpoints",
            );
            return Err(CorrespondencePromotionRejectionClass::DuplicateEndpointReference);
        }
        if source_set.intersection(&target_set).next().is_some() {
            self.runtime
                .performance_access()
                .count_lineage_candidate_validation(recorded_width, 0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &candidate.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::OverlappingSourceAndTarget,
                "correspondence promotion cannot overlap source and target lineage sets",
            );
            return Err(CorrespondencePromotionRejectionClass::OverlappingSourceAndTarget);
        }
        if candidate
            .sources
            .iter()
            .chain(candidate.targets.iter())
            .any(|lineage_id| self.runtime.lineage_node(*lineage_id).is_none())
        {
            self.runtime
                .performance_access()
                .count_lineage_candidate_validation(recorded_width, 0);
            self.record_rejected_promotion_for_candidate(
                Some(candidate),
                &candidate.branch_id,
                candidate.candidate_id,
                CorrespondencePromotionRejectionClass::MissingLineageReference,
                "correspondence promotion referenced missing lineage",
            );
            return Err(CorrespondencePromotionRejectionClass::MissingLineageReference);
        }
        let branch_scoped_sources = candidate
            .sources
            .iter()
            .copied()
            .map(|lineage_id| BranchScopedLineageRef::new(candidate.branch_id.clone(), lineage_id))
            .collect();
        let branch_scoped_targets = candidate
            .targets
            .iter()
            .copied()
            .map(|lineage_id| BranchScopedLineageRef::new(candidate.branch_id.clone(), lineage_id))
            .collect();
        self.runtime
            .performance_access()
            .count_lineage_candidate_validation(recorded_width, recorded_width);
        Ok(ValidatedCorrespondenceCandidate::new(
            candidate.clone(),
            branch_scoped_sources,
            branch_scoped_targets,
        ))
    }

    pub(super) fn emit_promotion_rejection(
        &mut self,
        attempted_branch_id: &BranchId,
        candidate_id: CorrespondenceCandidateId,
        class: CorrespondencePromotionRejectionClass,
        message: &str,
    ) {
        self.record_rejected_promotion_artifact(
            attempted_branch_id.clone(),
            candidate_id,
            Vec::new(),
            Vec::new(),
            class,
        );
        self.emit_promotion_rejection_diagnostic(candidate_id, class, message);
    }

    pub(super) fn record_rejected_promotion_for_candidate(
        &mut self,
        candidate: Option<&CorrespondenceCandidate>,
        attempted_branch_id: &BranchId,
        candidate_id: CorrespondenceCandidateId,
        class: CorrespondencePromotionRejectionClass,
        message: &str,
    ) {
        let branch_id = candidate
            .map(|candidate| candidate.branch_id.clone())
            .unwrap_or_else(|| attempted_branch_id.clone());
        let sources = candidate
            .map(|candidate| candidate.sources.clone())
            .unwrap_or_default();
        let targets = candidate
            .map(|candidate| candidate.targets.clone())
            .unwrap_or_default();
        self.record_rejected_promotion_artifact(branch_id, candidate_id, sources, targets, class);
        self.emit_promotion_rejection_diagnostic(candidate_id, class, message);
    }

    fn record_rejected_promotion_artifact(
        &mut self,
        branch_id: BranchId,
        candidate_id: CorrespondenceCandidateId,
        sources: Vec<LineageId>,
        targets: Vec<LineageId>,
        class: CorrespondencePromotionRejectionClass,
    ) {
        self.runtime
            .performance_access()
            .count_lineage_promotion_rejection();
        let artifact = LineageRejectionArtifact::single_rejected_promotion(LineageDecisionRecord {
            branch_id,
            kind: LineageDecisionKind::CorrespondencePromotionRejected,
            event_id: None,
            candidate_id: Some(candidate_id),
            sources,
            targets,
            rejection_class: Some(class),
        });
        for decision in artifact.decision_log().decisions() {
            self.runtime
                .lineage
                .record_rejected_decision(decision.clone());
        }
    }

    fn emit_promotion_rejection_diagnostic(
        &mut self,
        candidate_id: CorrespondenceCandidateId,
        class: CorrespondencePromotionRejectionClass,
        message: &str,
    ) {
        self.runtime
            .publication_authority()
            .push_bounded_diagnostic(
                DiagnosticsScope::Lineage,
                DiagnosticsArtifactKind::Failure,
                vec![RelationalDiagnosticsEntry::new(
                    DiagnosticCode::InvariantViolation,
                    message,
                    promotion_rejection_fields(candidate_id, class),
                )],
            );
    }
}
