use super::{
    IdentityEvolutionExecutionArtifact, IdentityEvolutionExecutionFamily,
    IdentityEvolutionOutcomeFamily, PromotionOrMergeAuthorityState,
};
use crate::runtime::{WorthQueryContinuityMutationEvidence, WorthQueryContinuityOutcomeClass};
#[path = "installed/authority_subjects.rs"]
mod authority_subjects;
#[path = "installed/foundational_attachment.rs"]
mod foundational_attachment;
use foundational_attachment::FoundationalInstalledLineageIdentities;
pub use foundational_attachment::WorthQueryFoundationalLineageAttachment;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstalledIdentityEvolutionKind {
    PreservedIdentity,
    SingularSuccessor,
    SplitSuccessors,
    MergedPredecessors,
    GeneratedIdentity,
    RetiredIdentity,
    AdvisoryCorrespondence,
    AmbiguousCorrespondence,
    ContinuityBreak,
    Denied,
}

/// Identity-evolution evidence executed under one exact installed workflow stage.
///
/// There is deliberately no public constructor. The only minting path is
/// `WorthQueryWorkflowStageExecutionContext::execute_identity_evolution`, which
/// runs Query's admitted identity-evolution engine and binds its artifact to the
/// installed operation, run, and stage that requested it.
#[derive(Clone, Debug)]
pub struct InstalledIdentityEvolutionOutcome {
    artifact: IdentityEvolutionExecutionArtifact,
    continuity: Option<WorthQueryContinuityMutationEvidence>,
    lifecycle_target: Option<crate::memory_workspace::WorthQueryEntityIdentity>,
    establishing_entity_targets: Vec<crate::memory_workspace::WorthQueryEntityIdentity>,
    operation_identity: String,
    run_identity: String,
    stage_identity: String,
    establishing_effect_receipt_identity: String,
    semantic_identity: String,
    foundational_identities: FoundationalInstalledLineageIdentities,
}

pub(crate) struct InstalledIdentityEvolutionBinding<'a> {
    pub(crate) operation_identity: &'a str,
    pub(crate) run_identity: &'a str,
    pub(crate) stage_identity: &'a str,
    pub(crate) effect_receipt_identity: String,
    pub(crate) establishing_entity_targets: Vec<crate::memory_workspace::WorthQueryEntityIdentity>,
}

impl InstalledIdentityEvolutionOutcome {
    pub(crate) fn from_execution(
        artifact: IdentityEvolutionExecutionArtifact,
        continuity: Option<WorthQueryContinuityMutationEvidence>,
        lifecycle_target: Option<crate::memory_workspace::WorthQueryEntityIdentity>,
        binding: InstalledIdentityEvolutionBinding<'_>,
    ) -> Option<Self> {
        if !engine_matches_authority(&artifact, continuity.as_ref(), lifecycle_target.as_ref()) {
            return None;
        }
        let semantic_evidence = semantic_evidence_identity(
            &artifact,
            continuity.as_ref(),
            lifecycle_target.as_ref(),
            &binding.establishing_entity_targets,
        );
        let semantic_identity = semantic_evidence
            .terminal_projection_for_reporting()
            .to_owned();
        let foundational_identities = FoundationalInstalledLineageIdentities::from_execution(
            &semantic_evidence,
            artifact.basis_digest(),
            &binding.effect_receipt_identity,
        );
        Some(Self {
            artifact,
            continuity,
            lifecycle_target,
            establishing_entity_targets: binding.establishing_entity_targets,
            operation_identity: binding.operation_identity.to_owned(),
            run_identity: binding.run_identity.to_owned(),
            stage_identity: binding.stage_identity.to_owned(),
            establishing_effect_receipt_identity: binding.effect_receipt_identity,
            semantic_identity,
            foundational_identities,
        })
    }

    /// Descriptive execution metadata from the existing identity-evolution
    /// engine. Operational continuity comes from `continuity_evidence()`.
    pub fn engine_artifact(&self) -> &IdentityEvolutionExecutionArtifact {
        &self.artifact
    }

    pub fn continuity_evidence(&self) -> Option<&WorthQueryContinuityMutationEvidence> {
        self.continuity.as_ref()
    }

    pub fn kind(&self) -> InstalledIdentityEvolutionKind {
        if self.artifact.family() == &IdentityEvolutionExecutionFamily::GeneratedIdentity {
            return InstalledIdentityEvolutionKind::GeneratedIdentity;
        }
        if self.artifact.family() == &IdentityEvolutionExecutionFamily::RetiredIdentity {
            return InstalledIdentityEvolutionKind::RetiredIdentity;
        }
        match self.artifact.result_bundle().outcome_family() {
            IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet => {
                return InstalledIdentityEvolutionKind::AdvisoryCorrespondence;
            }
            IdentityEvolutionOutcomeFamily::Ambiguity => {
                return InstalledIdentityEvolutionKind::AmbiguousCorrespondence;
            }
            IdentityEvolutionOutcomeFamily::IdentityBreak => {
                return InstalledIdentityEvolutionKind::ContinuityBreak;
            }
            _ => {}
        }
        let Some(continuity) = self.continuity.as_ref() else {
            return InstalledIdentityEvolutionKind::Denied;
        };
        match continuity.outcome_class() {
            WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor => {
                if continuity.successor_authoritative_identity()
                    == Some(continuity.prior_authoritative_identity())
                {
                    InstalledIdentityEvolutionKind::PreservedIdentity
                } else {
                    InstalledIdentityEvolutionKind::SingularSuccessor
                }
            }
            WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors => {
                InstalledIdentityEvolutionKind::SplitSuccessors
            }
            WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                InstalledIdentityEvolutionKind::MergedPredecessors
            }
            WorthQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor => {
                InstalledIdentityEvolutionKind::AmbiguousCorrespondence
            }
            WorthQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
            | WorthQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure => {
                InstalledIdentityEvolutionKind::ContinuityBreak
            }
            WorthQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass => {
                InstalledIdentityEvolutionKind::Denied
            }
        }
    }

    pub fn width(&self) -> usize {
        match self.kind() {
            InstalledIdentityEvolutionKind::GeneratedIdentity => 1,
            InstalledIdentityEvolutionKind::RetiredIdentity
            | InstalledIdentityEvolutionKind::AdvisoryCorrespondence
            | InstalledIdentityEvolutionKind::AmbiguousCorrespondence
            | InstalledIdentityEvolutionKind::ContinuityBreak
            | InstalledIdentityEvolutionKind::Denied => 0,
            _ => self.continuity.as_ref().map_or(0, |evidence| {
                evidence.successor_authoritative_identities().len()
            }),
        }
    }

    pub fn is_authoritative_continuity(&self) -> bool {
        matches!(
            self.kind(),
            InstalledIdentityEvolutionKind::PreservedIdentity
                | InstalledIdentityEvolutionKind::SingularSuccessor
                | InstalledIdentityEvolutionKind::SplitSuccessors
                | InstalledIdentityEvolutionKind::MergedPredecessors
                | InstalledIdentityEvolutionKind::GeneratedIdentity
        ) && self
            .artifact
            .result_bundle()
            .metadata()
            .promotion_or_merge_authority_state()
            != PromotionOrMergeAuthorityState::RequiredButUnavailable
    }

    pub fn foundational_outcome_kind(
        &self,
    ) -> worth_foundational::facade::FoundationalBoundaryEvidenceLineageOutcomeKind {
        use worth_foundational::facade::FoundationalBoundaryEvidenceLineageOutcomeKind as Kind;
        match self.kind() {
            InstalledIdentityEvolutionKind::PreservedIdentity => Kind::SingularContinuity,
            InstalledIdentityEvolutionKind::SingularSuccessor => Kind::SingularContinuity,
            InstalledIdentityEvolutionKind::SplitSuccessors => Kind::PluralSuccessorPredecessor,
            InstalledIdentityEvolutionKind::MergedPredecessors => Kind::MergeSuccessor,
            InstalledIdentityEvolutionKind::GeneratedIdentity => Kind::SingularContinuity,
            InstalledIdentityEvolutionKind::RetiredIdentity => Kind::IdentityBreak,
            InstalledIdentityEvolutionKind::AdvisoryCorrespondence => {
                Kind::AdvisoryCorrespondenceCandidate
            }
            InstalledIdentityEvolutionKind::AmbiguousCorrespondence => Kind::Ambiguity,
            InstalledIdentityEvolutionKind::ContinuityBreak
            | InstalledIdentityEvolutionKind::Denied => Kind::IdentityBreak,
        }
    }

    pub(crate) fn binds(
        &self,
        operation: &str,
        run: &str,
        stage: &str,
        stage_effect_receipt_identities: &std::collections::BTreeSet<&str>,
    ) -> bool {
        self.operation_identity == operation
            && self.run_identity == run
            && self.stage_identity == stage
            && stage_effect_receipt_identities
                .contains(self.establishing_effect_receipt_identity.as_str())
    }

    pub(crate) fn establishing_effect_receipt_identity(&self) -> Option<&str> {
        Some(&self.establishing_effect_receipt_identity)
    }

    pub(crate) fn semantic_identity(&self) -> &str {
        &self.semantic_identity
    }

    pub(crate) fn foundational_attested_lineage(
        &self,
    ) -> Option<WorthQueryFoundationalLineageAttachment> {
        self.foundational_identities
            .attested_lineage(self.kind(), self.is_authoritative_continuity())
    }
}

impl PartialEq for InstalledIdentityEvolutionOutcome {
    fn eq(&self, other: &Self) -> bool {
        self.semantic_identity == other.semantic_identity
    }
}

impl Eq for InstalledIdentityEvolutionOutcome {}

fn semantic_evidence_identity(
    artifact: &IdentityEvolutionExecutionArtifact,
    continuity: Option<&WorthQueryContinuityMutationEvidence>,
    lifecycle_target: Option<&crate::memory_workspace::WorthQueryEntityIdentity>,
    establishing_entity_targets: &[crate::memory_workspace::WorthQueryEntityIdentity],
) -> crate::evidence_identity::WorthQueryEvidenceIdentity {
    let identity = crate::evidence_identity::WorthQueryEvidenceIdentity::compose(
        crate::evidence_identity::WorthQueryEvidenceScope::InstalledDomainExecution,
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("identity_family"),
        "installed_identity_evolution_semantics_v3",
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("evolution_family"),
        artifact.family().as_str(),
    )
    .field_shape(
        crate::evidence_identity::WorthQueryEvidenceTag::new("evolution_outcome"),
        continuity
            .map(|evidence| evidence.outcome_class().as_str())
            .unwrap_or_else(|| artifact.result_bundle().outcome_family().as_str()),
    )
    .optional_evidence_identity(
        crate::evidence_identity::WorthQueryEvidenceTag::new("prior_identity"),
        continuity.map(|evidence| evidence.prior_authoritative_identity().evidence_identity()),
    );
    let identity = if let Some(continuity) = continuity {
        identity
            .field_evidence_identity_sequence(
                crate::evidence_identity::WorthQueryEvidenceTag::new("successor_identity"),
                continuity
                    .successor_authoritative_identities()
                    .iter()
                    .map(crate::runtime::WorthQueryMutationAuthorityIdentity::evidence_identity),
            )
            .field_evidence_identity(
                crate::evidence_identity::WorthQueryEvidenceTag::new("lineage_digest"),
                continuity.lineage_digest().evidence_identity(),
            )
            .field_evidence_identity(
                crate::evidence_identity::WorthQueryEvidenceTag::new("resolution_digest"),
                continuity
                    .continuity_resolution_digest()
                    .evidence_identity(),
            )
    } else {
        identity
    };
    let lifecycle_identity =
        lifecycle_target.map(crate::memory_workspace::WorthQueryEntityIdentity::evidence_identity);
    let establishing_target_identities = establishing_entity_targets
        .iter()
        .map(crate::memory_workspace::WorthQueryEntityIdentity::evidence_identity)
        .collect::<Vec<_>>();
    identity
        .field_evidence_identity_sequence(
            crate::evidence_identity::WorthQueryEvidenceTag::new("establishing_entity_target"),
            &establishing_target_identities,
        )
        .optional_evidence_identity(
            crate::evidence_identity::WorthQueryEvidenceTag::new("lifecycle_target"),
            lifecycle_identity.as_ref(),
        )
        .seal()
}

fn engine_matches_authority(
    artifact: &IdentityEvolutionExecutionArtifact,
    continuity: Option<&WorthQueryContinuityMutationEvidence>,
    lifecycle_target: Option<&crate::memory_workspace::WorthQueryEntityIdentity>,
) -> bool {
    if let Some(target) = lifecycle_target {
        let target_evidence_identity = target.evidence_identity();
        let target_identity = target_evidence_identity.as_str();
        return match artifact.family() {
            IdentityEvolutionExecutionFamily::GeneratedIdentity => artifact
                .result_bundle()
                .as_generated_identity()
                .is_some_and(|result| result.authoritative_identity() == target_identity),
            IdentityEvolutionExecutionFamily::RetiredIdentity => artifact
                .result_bundle()
                .as_retired_identity()
                .is_some_and(|result| result.authoritative_identity() == target_identity),
            _ => false,
        };
    }
    let Some(continuity) = continuity else {
        return artifact.family()
            == &IdentityEvolutionExecutionFamily::InstalledOperationComparison
            && matches!(
                artifact.result_bundle().outcome_family(),
                IdentityEvolutionOutcomeFamily::AdvisoryIdentityCandidateSet
                    | IdentityEvolutionOutcomeFamily::Ambiguity
                    | IdentityEvolutionOutcomeFamily::IdentityBreak
            );
    };
    let shape_matches = matches!(
        (artifact.family(), continuity.outcome_class()),
        (
            IdentityEvolutionExecutionFamily::DirectSuccessor,
            WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
        ) | (
            IdentityEvolutionExecutionFamily::DirectSplitSuccessors,
            WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
        ) | (
            IdentityEvolutionExecutionFamily::DirectMergeSuccessor,
            WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        )
    ) && matches!(
        (
            artifact.result_bundle().outcome_family(),
            continuity.outcome_class()
        ),
        (
            IdentityEvolutionOutcomeFamily::SingularIdentityContinuity,
            WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
                | WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor
        ) | (
            IdentityEvolutionOutcomeFamily::PluralIdentitySuccessorSet,
            WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors
        )
    );
    if !shape_matches {
        return false;
    }
    let exact_successors = continuity
        .successor_authoritative_identities()
        .iter()
        .map(|identity| identity.evidence_identity().as_str())
        .collect::<Vec<_>>();
    match continuity.outcome_class() {
        WorthQueryContinuityOutcomeClass::ContinuesAsSingleSuccessor
        | WorthQueryContinuityOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
            artifact
                .result_bundle()
                .as_singular_identity_continuity()
                .is_some_and(|result| {
                    exact_successors.as_slice() == [result.authoritative_identity()]
                })
        }
        WorthQueryContinuityOutcomeClass::ContinuesAsSplitSuccessors => artifact
            .result_bundle()
            .as_plural_identity_successor_set()
            .is_some_and(|result| {
                result
                    .successor_identities()
                    .iter()
                    .map(String::as_str)
                    .eq(exact_successors)
            }),
        WorthQueryContinuityOutcomeClass::RejectedNoAuthoritativeSuccessor
        | WorthQueryContinuityOutcomeClass::RejectedAmbiguousSuccessor
        | WorthQueryContinuityOutcomeClass::RejectedUnsupportedContinuityClass
        | WorthQueryContinuityOutcomeClass::RejectedHistoricalResolutionFailure => false,
    }
}
