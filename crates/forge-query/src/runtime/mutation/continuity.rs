use std::sync::Arc;

use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

use super::super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};

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
    prior_authoritative_identity: Arc<str>,
    successor_authoritative_identities: Vec<String>,
}

impl ForgeQueryContinuityMutationIntent {
    pub fn rebind_existing_target(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSingleSuccessor,
            prior_authoritative_identity,
            [successor_authoritative_identity],
        )
    }

    pub fn rebind_merge_successor(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Self::new(
            ForgeQueryContinuityMutationFamily::RebindExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesViaTruthLoweredCanonicalMergeSuccessor,
            prior_authoritative_identity,
            [successor_authoritative_identity],
        )
    }

    pub fn split_existing_target<I, S>(
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: I,
    ) -> Result<Self, ForgeQueryWorkspaceError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self::new(
            ForgeQueryContinuityMutationFamily::SplitExistingTarget,
            ForgeQueryContinuityMutationOutcomeClass::ContinuesAsSplitSuccessors,
            prior_authoritative_identity,
            successor_authoritative_identities,
        )
    }

    fn new<I, S>(
        family: ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityMutationOutcomeClass,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: I,
    ) -> Result<Self, ForgeQueryWorkspaceError>
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let successor_authoritative_identities = successor_authoritative_identities
            .into_iter()
            .map(Into::into)
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
            prior_authoritative_identity: validate_identity(
                "prior authoritative identity",
                prior_authoritative_identity,
            )?,
            successor_authoritative_identities: successor_authoritative_identities
                .into_iter()
                .map(|value| validate_identity("successor authoritative identity", value))
                .collect::<Result<Vec<_>, _>>()?
                .into_iter()
                .map(|value| value.as_ref().to_string())
                .collect(),
        })
    }

    pub fn family(&self) -> ForgeQueryContinuityMutationFamily {
        self.family
    }

    pub fn outcome_class(&self) -> ForgeQueryContinuityMutationOutcomeClass {
        self.outcome_class
    }

    pub fn prior_authoritative_identity(&self) -> &str {
        self.prior_authoritative_identity.as_ref()
    }

    pub fn successor_authoritative_identity(&self) -> &str {
        self.successor_authoritative_identities
            .first()
            .map(String::as_str)
            .expect("continuity intent must retain at least one successor authoritative identity")
    }

    pub fn successor_authoritative_identities(&self) -> &[String] {
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
    prior_authoritative_identity: String,
    successor_authoritative_identities: Vec<String>,
    basis_binding_digest: Option<String>,
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
        let basis_binding_digest = existing_truth_binding.map(|binding| binding.binding_digest());
        let denial_digest = hash_parts(&[
            "forge_query_continuity_mutation_denial_v1".to_string(),
            format!("family:{}", intent.family().as_str()),
            format!("kind:{}", kind.as_str()),
            format!("outcome:{}", intent.outcome_class().as_str()),
            format!("prior:{}", intent.prior_authoritative_identity()),
            format!(
                "successors:{}",
                intent.successor_authoritative_identities().join("|")
            ),
            format!(
                "basis-binding:{}",
                basis_binding_digest.as_deref().unwrap_or("none")
            ),
            format!("reason:{reason}"),
        ]);
        Self {
            family: intent.family(),
            kind,
            prior_authoritative_identity: intent.prior_authoritative_identity().to_string(),
            successor_authoritative_identities: intent
                .successor_authoritative_identities()
                .to_vec(),
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

    pub fn prior_authoritative_identity(&self) -> &str {
        &self.prior_authoritative_identity
    }

    pub fn successor_authoritative_identity(&self) -> Option<&str> {
        match self.successor_authoritative_identities.as_slice() {
            [only] => Some(only.as_str()),
            _ => None,
        }
    }

    pub fn successor_authoritative_identities(&self) -> &[String] {
        &self.successor_authoritative_identities
    }

    pub fn basis_binding_digest(&self) -> Option<&str> {
        self.basis_binding_digest.as_deref()
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
            "continuity mutation `{}` -> `{}` denied during {}: {}",
            self.prior_authoritative_identity,
            self.successor_authoritative_identities.join("|"),
            self.kind.as_str(),
            self.reason
        )
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

fn validate_identity(
    label: &str,
    value: impl Into<String>,
) -> Result<Arc<str>, ForgeQueryWorkspaceError> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(ForgeQueryWorkspaceError::new(format!(
            "{label} may not be empty"
        )));
    }
    Ok(Arc::from(value))
}
