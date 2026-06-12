use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryMutationAuthorityIdentity;

use super::super::{ForgeQueryMutationFamily, ForgeQueryWriteCommand};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryNamingMutationFamily {
    AttachNewTarget,
    AttachExistingTarget,
    RebindTarget,
    Remove,
}

impl ForgeQueryNamingMutationFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttachNewTarget => "attach_new_target",
            Self::AttachExistingTarget => "attach_existing_target",
            Self::RebindTarget => "rebind_target",
            Self::Remove => "remove",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryNamingMutationIntent {
    family: ForgeQueryNamingMutationFamily,
    attachment_identity: ForgeQueryMutationAuthorityIdentity,
    prior_authoritative_identity: Option<ForgeQueryMutationAuthorityIdentity>,
    target_authoritative_identity: Option<ForgeQueryMutationAuthorityIdentity>,
}

impl ForgeQueryNamingMutationIntent {
    pub fn attach_new_target(attachment_identity: ForgeQueryMutationAuthorityIdentity) -> Self {
        Self {
            family: ForgeQueryNamingMutationFamily::AttachNewTarget,
            attachment_identity,
            prior_authoritative_identity: None,
            target_authoritative_identity: None,
        }
    }

    pub fn attach_existing_target(
        attachment_identity: ForgeQueryMutationAuthorityIdentity,
        target_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    ) -> Self {
        Self {
            family: ForgeQueryNamingMutationFamily::AttachExistingTarget,
            attachment_identity,
            prior_authoritative_identity: None,
            target_authoritative_identity: Some(target_authoritative_identity),
        }
    }

    pub fn rebind_target(
        attachment_identity: ForgeQueryMutationAuthorityIdentity,
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
        target_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    ) -> Self {
        Self {
            family: ForgeQueryNamingMutationFamily::RebindTarget,
            attachment_identity,
            prior_authoritative_identity: Some(prior_authoritative_identity),
            target_authoritative_identity: Some(target_authoritative_identity),
        }
    }

    pub fn remove(
        attachment_identity: ForgeQueryMutationAuthorityIdentity,
        prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    ) -> Self {
        Self {
            family: ForgeQueryNamingMutationFamily::Remove,
            attachment_identity,
            prior_authoritative_identity: Some(prior_authoritative_identity),
            target_authoritative_identity: None,
        }
    }

    pub fn family(&self) -> ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn attachment_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.attachment_identity
    }

    pub fn prior_authoritative_identity(&self) -> Option<&ForgeQueryMutationAuthorityIdentity> {
        self.prior_authoritative_identity.as_ref()
    }

    pub fn target_authoritative_identity(&self) -> Option<&ForgeQueryMutationAuthorityIdentity> {
        self.target_authoritative_identity.as_ref()
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryNamingMutationDenialKind {
    RequiresSameBatchTargetReference,
    RequiresExistingTruthBinding,
    RequiresDeleteFamily,
}

impl ForgeQueryNamingMutationDenialKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::RequiresSameBatchTargetReference => "requires_same_batch_target_reference",
            Self::RequiresExistingTruthBinding => "requires_existing_truth_binding",
            Self::RequiresDeleteFamily => "requires_delete_family",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryNamingMutationDenial {
    family: ForgeQueryNamingMutationFamily,
    attachment_identity: ForgeQueryMutationAuthorityIdentity,
    kind: ForgeQueryNamingMutationDenialKind,
    reason: String,
    denial_digest: String,
}

impl ForgeQueryNamingMutationDenial {
    pub(crate) fn new(
        intent: &ForgeQueryNamingMutationIntent,
        kind: ForgeQueryNamingMutationDenialKind,
        reason: impl Into<String>,
    ) -> Self {
        let reason = reason.into();
        let denial_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "naming-mutation-denial")
                .field_shape(
                    ForgeQueryEvidenceTag::new("family"),
                    intent.family().as_str(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("attachment"),
                    intent.attachment_identity().evidence_identity(),
                )
                .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
                .field_value(ForgeQueryEvidenceTag::new("reason"), &reason)
                .seal()
                .as_str()
                .to_string();
        Self {
            family: intent.family(),
            attachment_identity: intent.attachment_identity().clone(),
            kind,
            reason,
            denial_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn attachment_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.attachment_identity
    }

    pub fn kind(&self) -> ForgeQueryNamingMutationDenialKind {
        self.kind
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn denial_digest(&self) -> &str {
        &self.denial_digest
    }
}

impl std::fmt::Display for ForgeQueryNamingMutationDenial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "naming mutation `{}` denied during {}: {}",
            self.attachment_identity.as_str(),
            self.kind.as_str(),
            self.reason
        )
    }
}

impl std::error::Error for ForgeQueryNamingMutationDenial {}

pub(crate) fn admit_naming_intent(
    command: &ForgeQueryWriteCommand,
) -> Result<(), ForgeQueryNamingMutationDenial> {
    let Some(intent) = command.naming_intent() else {
        return Ok(());
    };
    match intent.family() {
        ForgeQueryNamingMutationFamily::AttachNewTarget => {
            if command.mutation_family() == ForgeQueryMutationFamily::Delete {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
                    "naming attach-to-new-target requires an insert/update lane, not delete",
                ));
            }
            if command.symbolic_target_reference().is_none() {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresSameBatchTargetReference,
                    "naming attach-to-new-target requires a same-batch symbolic target reference",
                ));
            }
        }
        ForgeQueryNamingMutationFamily::AttachExistingTarget => {
            if command.mutation_family() == ForgeQueryMutationFamily::Delete {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresExistingTruthBinding,
                    "naming attach-to-existing-target requires an insert/update lane, not delete",
                ));
            }
            if command.existing_truth_binding().is_none() {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresExistingTruthBinding,
                    "naming attach-to-existing-target requires an existing-truth binding",
                ));
            }
        }
        ForgeQueryNamingMutationFamily::RebindTarget => {
            if command.mutation_family() == ForgeQueryMutationFamily::Delete {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresExistingTruthBinding,
                    "naming rebind requires an insert/update lane, not delete",
                ));
            }
            if command.existing_truth_binding().is_none()
                && command.symbolic_target_reference().is_none()
            {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresExistingTruthBinding,
                    "naming rebind requires an existing-truth binding or same-batch target reference",
                ));
            }
        }
        ForgeQueryNamingMutationFamily::Remove => {
            if command.mutation_family() != ForgeQueryMutationFamily::Delete {
                return Err(ForgeQueryNamingMutationDenial::new(
                    intent,
                    ForgeQueryNamingMutationDenialKind::RequiresDeleteFamily,
                    "naming removal requires a delete mutation family",
                ));
            }
        }
    }
    Ok(())
}
