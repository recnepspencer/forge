use std::sync::Arc;

use crate::identity::hash_parts;
use crate::memory_workspace::ForgeQueryWorkspaceError;

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
    attachment_identity: Arc<str>,
    prior_authoritative_identity: Option<Arc<str>>,
    target_authoritative_identity: Option<Arc<str>>,
}

impl ForgeQueryNamingMutationIntent {
    pub fn attach_new_target(
        attachment_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            family: ForgeQueryNamingMutationFamily::AttachNewTarget,
            attachment_identity: validate_identity("attachment identity", attachment_identity)?,
            prior_authoritative_identity: None,
            target_authoritative_identity: None,
        })
    }

    pub fn attach_existing_target(
        attachment_identity: impl Into<String>,
        target_authoritative_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            family: ForgeQueryNamingMutationFamily::AttachExistingTarget,
            attachment_identity: validate_identity("attachment identity", attachment_identity)?,
            prior_authoritative_identity: None,
            target_authoritative_identity: Some(validate_identity(
                "target authoritative identity",
                target_authoritative_identity,
            )?),
        })
    }

    pub fn rebind_target(
        attachment_identity: impl Into<String>,
        prior_authoritative_identity: impl Into<String>,
        target_authoritative_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            family: ForgeQueryNamingMutationFamily::RebindTarget,
            attachment_identity: validate_identity("attachment identity", attachment_identity)?,
            prior_authoritative_identity: Some(validate_identity(
                "prior authoritative identity",
                prior_authoritative_identity,
            )?),
            target_authoritative_identity: Some(validate_identity(
                "target authoritative identity",
                target_authoritative_identity,
            )?),
        })
    }

    pub fn remove(
        attachment_identity: impl Into<String>,
        prior_authoritative_identity: impl Into<String>,
    ) -> Result<Self, ForgeQueryWorkspaceError> {
        Ok(Self {
            family: ForgeQueryNamingMutationFamily::Remove,
            attachment_identity: validate_identity("attachment identity", attachment_identity)?,
            prior_authoritative_identity: Some(validate_identity(
                "prior authoritative identity",
                prior_authoritative_identity,
            )?),
            target_authoritative_identity: None,
        })
    }

    pub fn family(&self) -> ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn attachment_identity(&self) -> &str {
        self.attachment_identity.as_ref()
    }

    pub fn prior_authoritative_identity(&self) -> Option<&str> {
        self.prior_authoritative_identity.as_deref()
    }

    pub fn target_authoritative_identity(&self) -> Option<&str> {
        self.target_authoritative_identity.as_deref()
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
    attachment_identity: String,
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
        let denial_digest = hash_parts(&[
            "forge_query_naming_mutation_denial_v1".to_string(),
            format!("family:{}", intent.family().as_str()),
            format!("attachment:{}", intent.attachment_identity()),
            format!("kind:{}", kind.as_str()),
            format!("reason:{reason}"),
        ]);
        Self {
            family: intent.family(),
            attachment_identity: intent.attachment_identity().to_string(),
            kind,
            reason,
            denial_digest,
        }
    }

    pub fn family(&self) -> ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn attachment_identity(&self) -> &str {
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
            self.attachment_identity,
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
