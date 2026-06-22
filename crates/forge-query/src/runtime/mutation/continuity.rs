use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryWorkspaceError;

use super::super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};
use crate::runtime::ForgeQueryMutationAuthorityIdentity;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityMutationFamily {
    RebindExistingTarget,
    SplitExistingTarget,
}

impl ForgeQueryContinuityMutationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RebindExistingTarget => "rebind_existing_target",
            Self::SplitExistingTarget => "split_existing_target",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityMutationOutcomeClass {
    ContinuesAsSingleSuccessor,
    ContinuesAsSplitSuccessors,
    ContinuesViaTruthLoweredCanonicalMergeSuccessor,
}

impl ForgeQueryContinuityMutationOutcomeClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContinuesAsSingleSuccessor => "continues_as_single_successor",
            Self::ContinuesAsSplitSuccessors => "continues_as_split_successors",
            Self::ContinuesViaTruthLoweredCanonicalMergeSuccessor => {
                "continues_via_truth_lowered_canonical_merge_successor"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityMutationIntent {
    family: ForgeQueryContinuityMutationFamily,
    outcome_class: ForgeQueryContinuityMutationOutcomeClass,
    prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    successor_authoritative_identities: Vec<ForgeQueryMutationAuthorityIdentity>,
}

impl ForgeQueryContinuityMutationIntent {
    pub fn rebind_existing_target(
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        successor_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor,
            prior_authoritative_identity,
            [successor_authoritative_identity],
        )
    }

    pub fn rebind_merge_successor(
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        successor_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor,
            prior_authoritative_identity,
            [successor_authoritative_identity],
        )
    }

    pub fn split_existing_target<I>(
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        successor_authoritative_identities: I,
    ) -> Result<Self, ForgeQueryWorkspaceError>
    where
        I: IntoIterator<Item = ForgeQueryMutationAuthorityIdentity>,
    {
        Self::new(
            ForgeQueryContinuityMutationFamily::SplitExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors,
            prior_authoritative_identity,
            successor_authoritative_identities,
        )
    }

    fn new<I>(
        family: ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityMutationOutcomeClass,
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        successor_authoritative_identities: I,
    ) -> Result<Self, ForgeQueryWorkspaceError>
    where
        I: IntoIterator<Item = ForgeQueryMutationAuthorityIdentity>,
    {
        let successor_authoritative_identities = successor_authoritative_identities
            .into_iter()
            .collect::<Vec<_>>();
        if successor_authoritative_identities.is_empty() {
            return Err(ForgeQueryWorkspaceError::new(
                "successor authoritative identity set may not be empty",
            ));
        }
        if family == ForgeQueryContinuityMutationFamily::SplitExistingTarget
            && successor_authoritative_identities.len() < 2
        {
            return Err(ForgeQueryWorkspaceError::new(
                "split-successor continuity requires at least two successor authoritative identities",
            ));
        }
        Ok(Self {
            family,
            outcome_class,
            prior_authoritative_identity,
            successor_authoritative_identities,
        })
    }

    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityMutationOutcomeClass {
        self.outcome_class
    }

    pub fn prior_authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        self.successor_authoritative_identities
            .first()
            .expect("continuity intent must retain at least one successor authoritative identity")
    }

    pub fn successor_authoritative_identities(&self) -> &[ForgeQueryMutationAuthorityIdentity] {
        &self.successor_authoritative_identities
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryContinuityMutationDenialKind {
    RequiresExistingTruthBinding,
    RequiresUpdateFamily,
    RequiresAuthoritativeLane,
}

impl ForgeQueryContinuityMutationDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresExistingTruthBinding => "requires_existing_truth_binding",
            Self::RequiresUpdateFamily => "requires_update_family",
            Self::RequiresAuthoritativeLane => "requires_authoritative_lane",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryContinuityMutationDenial {
    family: ForgeQueryContinuityMutationFamily,
    kind: ForgeQueryContinuityMutationDenialKind,
    prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    successor_authoritative_identities: Vec<ForgeQueryMutationAuthorityIdentity>,
    basis_binding_digest: Option<crate::runtime::ForgeQueryMutationEvidenceDigest>,
    reason: String,
    denial_digest: String,
}

impl ForgeQueryContinuityMutationDenial {
    pub(crate) fn new(
        intent: &ForgeQueryContinuityMutationIntent,
        existing_truth_binding: Option<&crate::runtime::ForgeQueryExistingTruthTargetBinding>,
        kind: ForgeQueryContinuityMutationDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let basis_binding_identity =
            existing_truth_binding.map(|binding| binding.binding_evidence_identity());
        let basis_binding_digest = basis_binding_identity.map(|identity| {
            crate::runtime::ForgeQueryMutationEvidenceDigest::source_identity(
                "continuity-basis-binding",
                identity,
            )
        });
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(
                    ForgeQueryEvidenceTag::new("role"),
                    "continuity-mutation-denial",
                )
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    intent.family().as_str(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_shape(
                    ForgeQueryEvidenceTag::new("outcome"),
                    intent.outcome_class().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("prior"),
                    intent.prior_authoritative_identity().evidence_identity(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("successor"),
                    intent
                        .successor_authoritative_identities()
                        .iter()
                        .map(ForgeQueryMutationAuthorityIdentity::evidence_identity),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("basis_binding"),
                    basis_binding_identity,
                )
                .field_value(ForgeQueryEvidenceTag::new("reason"), &reason)
                .seal()
                .as_str()
                .to_string();
        Self {
            family: intent.family(),
            kind,
            prior_authoritative_identity: intent.prior_authoritative_identity().clone(),
            successor_authoritative_identities: intent
                .successor_authoritative_identities()
                .iter()
                .cloned()
                .collect(),
            basis_binding_digest,
            reason,
            denial_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn kind(&self) -> ForgeQueryContinuityMutationDenialKind {
        self.kind
    }

    pub fn prior_authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> Option<&ForgeQueryMutationAuthorityIdentity> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[ForgeQueryMutationAuthorityIdentity] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&str> {
        self.basis_binding_digest
            .as_ref()
            .map(crate::runtime::ForgeQueryMutationEvidenceDigest::as_str)
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryContinuityMutationDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "continuity mutation `{}` -> ",
            self.prior_authoritative_identity.as_str(),
        )?;
        write!(f, "[")?;
        for (index, successor) in self.successor_authoritative_identities.iter().enumerate() {
            if index > 0 {
                write!(f, ", ")?;
            }
            write!(f, "`{}`", successor.as_str())?;
        }
        write!(f, "] denied during {}: {}", self.kind.as_str(), self.reason)
    }
}

impl std::error::Error for ForgeQueryContinuityMutationDenial {}

pub(crate) fn admit_continuity_intent(
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryContinuityMutationDenial> {
    let Some(intent) = command.continuity_intent() else {
        return Ok(());
    };
    if command.mutation_family() != ForgeQueryMutationFamily::Update {
        return Err(ForgeQueryContinuityMutationDenial::new(
            intent,
            command.existing_truth_binding(),
            ForgeQueryContinuityMutationDenialKind::RequiresUpdateFamily,
            "continuity-aware mutation currently requires an update family",
        ));
    }
    if command.existing_truth_binding().is_none() {
        return Err(ForgeQueryContinuityMutationDenial::new(
            intent,
            None,
            ForgeQueryContinuityMutationDenialKind::RequiresExistingTruthBinding,
            "continuity-aware mutation requires an existing-truth binding",
        ));
    }
    Ok(())
}
