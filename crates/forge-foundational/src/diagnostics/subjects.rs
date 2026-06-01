use crate::identities::BoundaryArtifactId;
use crate::locators::{
    BoundaryArtifactField, BoundaryArtifactLocator, BoundaryMismatchLocator, BoundarySourceLocator,
    FoundationalTransitionLocator,
};
use crate::transitions::{
    FoundationalBranchCandidateId, FoundationalBranchId, FoundationalCommitId,
    FoundationalCommitReceiptIdentity,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticSubject {
    BranchCandidate {
        branch_id: FoundationalBranchId,
        candidate_id: FoundationalBranchCandidateId,
    },
    MergeVerdict {
        source_branch: FoundationalBranchId,
        target_branch: FoundationalBranchId,
    },
    CommittedAuthority {
        commit_id: FoundationalCommitId,
    },
    CommitReceipt {
        commit_id: FoundationalCommitId,
        receipt_identity: FoundationalCommitReceiptIdentity,
    },
    BranchDiscard {
        branch_id: FoundationalBranchId,
    },
    BoundaryArtifact {
        artifact_locator: BoundaryArtifactLocator,
    },
}

impl FoundationalDiagnosticSubject {
    pub fn canonical_key_fragment(&self) -> String {
        match self {
            Self::BranchCandidate {
                branch_id,
                candidate_id,
            } => format!(
                "subject.branch_candidate:{}:{}",
                branch_id.as_str(),
                candidate_id.handle().get()
            ),
            Self::MergeVerdict {
                source_branch,
                target_branch,
            } => format!(
                "subject.merge_verdict:{}:{}",
                source_branch.as_str(),
                target_branch.as_str()
            ),
            Self::CommittedAuthority { commit_id } => {
                format!("subject.committed_authority:{}", commit_id.handle().get())
            }
            Self::CommitReceipt {
                commit_id,
                receipt_identity,
            } => format!(
                "subject.commit_receipt:{}:{}",
                commit_id.handle().get(),
                receipt_identity.handle().get()
            ),
            Self::BranchDiscard { branch_id } => {
                format!("subject.branch_discard:{}", branch_id.as_str())
            }
            Self::BoundaryArtifact { artifact_locator } => format!(
                "subject.boundary_artifact:{}:{}",
                artifact_locator.artifact_id().get(),
                boundary_artifact_field_name(artifact_locator.field())
            ),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FoundationalDiagnosticLocator {
    Transition(FoundationalTransitionLocator),
    BoundaryArtifact(BoundaryArtifactLocator),
    Source(BoundarySourceLocator),
    Mismatch(BoundaryMismatchLocator),
}

impl FoundationalDiagnosticLocator {
    pub fn canonical_key_fragment(&self) -> String {
        match self {
            Self::Transition(locator) => match locator {
                FoundationalTransitionLocator::BranchCandidate(locator) => format!(
                    "locator.transition.branch_candidate:{}:{}",
                    locator.branch_id().as_str(),
                    locator.candidate_id().handle().get()
                ),
                FoundationalTransitionLocator::MergeConflict(locator) => format!(
                    "locator.transition.merge_conflict:{}:{}:{}:{}:{}",
                    locator.source_branch().as_str(),
                    locator.target_branch().as_str(),
                    locator.conflict_locus().category(),
                    locator.conflict_locus().source_detail(),
                    locator.conflict_locus().target_detail()
                ),
                FoundationalTransitionLocator::CommitParentage(locator) => format!(
                    "locator.transition.commit_parentage:{}:{}",
                    locator.commit_id().handle().get(),
                    locator.parent_basis().basis_id().get()
                ),
                FoundationalTransitionLocator::CommittedDelta(locator) => format!(
                    "locator.transition.committed_delta:{}:{}",
                    locator.commit_id().handle().get(),
                    locator.delta_locus().detail()
                ),
                FoundationalTransitionLocator::MergeScope(locator) => format!(
                    "locator.transition.merge_scope:{}:{}:{}",
                    diagnostic_fragment_part(locator.source_branch().as_str()),
                    diagnostic_fragment_part(locator.target_branch().as_str()),
                    merge_scope_family_name(locator.scope_family())
                ),
                FoundationalTransitionLocator::SelectedNodeScope(locator) => format!(
                    "locator.transition.selected_node_scope:{}:{}:{}",
                    diagnostic_fragment_part(locator.source_branch().as_str()),
                    diagnostic_fragment_part(locator.target_branch().as_str()),
                    diagnostic_fragment_part(locator.selected_node().as_str())
                ),
                FoundationalTransitionLocator::SelectedAspectScope(locator) => format!(
                    "locator.transition.selected_aspect_scope:{}:{}:{}:{}",
                    diagnostic_fragment_part(locator.source_branch().as_str()),
                    diagnostic_fragment_part(locator.target_branch().as_str()),
                    diagnostic_fragment_part(locator.selected_aspect().node().as_str()),
                    diagnostic_fragment_part(locator.selected_aspect().aspect().as_str())
                ),
            },
            Self::BoundaryArtifact(locator) => format!(
                "locator.boundary_artifact:{}:{}",
                locator.artifact_id().get(),
                boundary_artifact_field_name(locator.field())
            ),
            Self::Source(locator) => match locator {
                BoundarySourceLocator::Aspect(locator) => {
                    format!("locator.source.aspect:{}", locator.aspect_key().as_str())
                }
                BoundarySourceLocator::AspectField(locator) => format!(
                    "locator.source.aspect_field:{}:{}",
                    locator.aspect().aspect_key().as_str(),
                    canonical_field_path_fragment(locator.field_path())
                ),
                BoundarySourceLocator::BoundaryArtifact(locator) => format!(
                    "locator.source.boundary_artifact:{}:{}",
                    locator.artifact_id().get(),
                    boundary_artifact_field_name(locator.field())
                ),
            },
            Self::Mismatch(locator) => match locator {
                BoundaryMismatchLocator::Aspect(locator) => {
                    format!("locator.mismatch.aspect:{}", locator.aspect_key().as_str())
                }
                BoundaryMismatchLocator::AspectField(locator) => format!(
                    "locator.mismatch.aspect_field:{}:{}",
                    locator.aspect().aspect_key().as_str(),
                    canonical_field_path_fragment(locator.field_path())
                ),
                BoundaryMismatchLocator::BoundaryArtifact(locator) => format!(
                    "locator.mismatch.boundary_artifact:{}:{}",
                    locator.artifact_id().get(),
                    boundary_artifact_field_name(locator.field())
                ),
            },
        }
    }
}

pub fn foundational_diagnostic_branch_candidate_subject(
    branch_id: FoundationalBranchId,
    candidate_id: FoundationalBranchCandidateId,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::BranchCandidate {
        branch_id,
        candidate_id,
    }
}

pub fn foundational_diagnostic_merge_verdict_subject(
    source_branch: FoundationalBranchId,
    target_branch: FoundationalBranchId,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::MergeVerdict {
        source_branch,
        target_branch,
    }
}

pub fn foundational_diagnostic_committed_authority_subject(
    commit_id: FoundationalCommitId,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::CommittedAuthority { commit_id }
}

pub fn foundational_diagnostic_commit_receipt_subject(
    commit_id: FoundationalCommitId,
    receipt_identity: FoundationalCommitReceiptIdentity,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::CommitReceipt {
        commit_id,
        receipt_identity,
    }
}

pub fn foundational_diagnostic_branch_discard_subject(
    branch_id: FoundationalBranchId,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::BranchDiscard { branch_id }
}

pub fn foundational_diagnostic_boundary_artifact_subject(
    artifact_id: BoundaryArtifactId,
    field: BoundaryArtifactField,
) -> FoundationalDiagnosticSubject {
    FoundationalDiagnosticSubject::BoundaryArtifact {
        artifact_locator: BoundaryArtifactLocator::new(artifact_id, field),
    }
}

pub fn foundational_diagnostic_locator_transition(
    locator: FoundationalTransitionLocator,
) -> FoundationalDiagnosticLocator {
    FoundationalDiagnosticLocator::Transition(locator)
}

pub fn foundational_diagnostic_locator_boundary_artifact(
    locator: BoundaryArtifactLocator,
) -> FoundationalDiagnosticLocator {
    FoundationalDiagnosticLocator::BoundaryArtifact(locator)
}

pub fn foundational_diagnostic_locator_source(
    locator: BoundarySourceLocator,
) -> FoundationalDiagnosticLocator {
    FoundationalDiagnosticLocator::Source(locator)
}

pub fn foundational_diagnostic_locator_mismatch(
    locator: BoundaryMismatchLocator,
) -> FoundationalDiagnosticLocator {
    FoundationalDiagnosticLocator::Mismatch(locator)
}

fn boundary_artifact_field_name(field: BoundaryArtifactField) -> &'static str {
    match field {
        BoundaryArtifactField::Payload => "payload",
        BoundaryArtifactField::Proofs => "proofs",
        BoundaryArtifactField::Basis => "basis",
    }
}

fn diagnostic_fragment_part(value: &str) -> String {
    format!("{}#{value}", value.len())
}

fn merge_scope_family_name(
    family: crate::transitions::FoundationalMergeScopeFamily,
) -> &'static str {
    match family {
        crate::transitions::FoundationalMergeScopeFamily::FullBranch => "full-branch",
        crate::transitions::FoundationalMergeScopeFamily::SelectedNodes => "selected-nodes",
        crate::transitions::FoundationalMergeScopeFamily::SelectedAspects => "selected-aspects",
    }
}

fn canonical_field_path_fragment(path: &crate::aspects::CanonicalFieldPath) -> String {
    path.fields()
        .iter()
        .map(|field| field.as_str())
        .collect::<Vec<_>>()
        .join(".")
}
