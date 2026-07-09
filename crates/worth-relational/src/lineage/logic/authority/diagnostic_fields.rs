use crate::diagnostics::data::{RelationalDiagnosticFields, RelationalDiagnosticValue};
use crate::history::data::{BranchId, CommitId};
use crate::lineage::data::{
    CorrespondenceCandidateId, CorrespondencePromotionExecutionFailureClass,
    CorrespondencePromotionRejectionClass,
};

pub(super) fn candidate_recorded_fields(
    candidate_id: CorrespondenceCandidateId,
    branch_id: &BranchId,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "candidate_id",
            RelationalDiagnosticValue::CorrespondenceCandidateId(candidate_id),
        ),
        (
            "branch_id",
            RelationalDiagnosticValue::BranchId(branch_id.clone()),
        ),
    ])
    .into()
}

pub(super) fn promotion_rejection_fields(
    candidate_id: CorrespondenceCandidateId,
    rejection_class: CorrespondencePromotionRejectionClass,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "candidate_id",
            RelationalDiagnosticValue::CorrespondenceCandidateId(candidate_id),
        ),
        (
            "rejection_class",
            RelationalDiagnosticValue::string(rejection_class_label(rejection_class)),
        ),
    ])
    .into()
}

pub(super) fn promotion_published_fields(
    candidate_id: CorrespondenceCandidateId,
    event_id: u64,
    commit_id: CommitId,
    anchor_commit_id: CommitId,
    branch_id: &BranchId,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "candidate_id",
            RelationalDiagnosticValue::CorrespondenceCandidateId(candidate_id),
        ),
        ("event_id", RelationalDiagnosticValue::Unsigned(event_id)),
        ("commit_id", RelationalDiagnosticValue::CommitId(commit_id)),
        (
            "anchor_commit_id",
            RelationalDiagnosticValue::CommitId(anchor_commit_id),
        ),
        (
            "branch_id",
            RelationalDiagnosticValue::BranchId(branch_id.clone()),
        ),
    ])
    .into()
}

pub(super) fn execution_failure_fields(
    candidate_id: Option<CorrespondenceCandidateId>,
    event_id: u64,
    anchor_commit_id: Option<CommitId>,
    branch_id: Option<&BranchId>,
    failure_class: CorrespondencePromotionExecutionFailureClass,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "candidate_id",
            RelationalDiagnosticValue::optional(
                candidate_id.map(RelationalDiagnosticValue::CorrespondenceCandidateId),
            ),
        ),
        ("event_id", RelationalDiagnosticValue::Unsigned(event_id)),
        (
            "anchor_commit_id",
            RelationalDiagnosticValue::optional(
                anchor_commit_id.map(RelationalDiagnosticValue::CommitId),
            ),
        ),
        (
            "branch_id",
            RelationalDiagnosticValue::optional(
                branch_id.cloned().map(RelationalDiagnosticValue::BranchId),
            ),
        ),
        (
            "execution_failure_class",
            RelationalDiagnosticValue::string(execution_failure_class_label(failure_class)),
        ),
    ])
    .into()
}

pub(super) fn metadata_promotion_summary_fields(
    branch_id: &BranchId,
    commit_id: CommitId,
    candidate_id: CorrespondenceCandidateId,
) -> RelationalDiagnosticFields {
    RelationalDiagnosticValue::object([
        (
            "branch_id",
            RelationalDiagnosticValue::BranchId(branch_id.clone()),
        ),
        ("commit_id", RelationalDiagnosticValue::CommitId(commit_id)),
        (
            "candidate_id",
            RelationalDiagnosticValue::CorrespondenceCandidateId(candidate_id),
        ),
    ])
    .into()
}

fn rejection_class_label(class: CorrespondencePromotionRejectionClass) -> &'static str {
    match class {
        CorrespondencePromotionRejectionClass::CandidateMissing => "candidate_missing",
        CorrespondencePromotionRejectionClass::MissingLineageReference => {
            "missing_lineage_reference"
        }
        CorrespondencePromotionRejectionClass::EmptyEndpointSet => "empty_endpoint_set",
        CorrespondencePromotionRejectionClass::DuplicateEndpointReference => {
            "duplicate_endpoint_reference"
        }
        CorrespondencePromotionRejectionClass::OverlappingSourceAndTarget => {
            "overlapping_source_and_target"
        }
        CorrespondencePromotionRejectionClass::CommitBranchMismatch => "commit_branch_mismatch",
        CorrespondencePromotionRejectionClass::BranchScopeMismatch => "branch_scope_mismatch",
        CorrespondencePromotionRejectionClass::CommitNotBranchHead => "commit_not_branch_head",
        CorrespondencePromotionRejectionClass::AuthorityPublicationFailed => {
            "authority_publication_failed"
        }
    }
}

fn execution_failure_class_label(
    class: CorrespondencePromotionExecutionFailureClass,
) -> &'static str {
    match class {
        CorrespondencePromotionExecutionFailureClass::AnchorDriftedFromBranchHead => {
            "anchor_drifted_from_branch_head"
        }
        CorrespondencePromotionExecutionFailureClass::AuthorityPublicationFailed => {
            "authority_publication_failed"
        }
    }
}
