use forge_runtime_bridge::facade::{
    BridgeNamingMutationBundle, BridgeNamingMutationFamily, BridgeNamingMutationOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryNamingMutationOutcome {
    AttachedToNewTarget,
    AttachedToExistingTarget,
    ReboundTarget,
    Removed,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryNamingMutationEvidence {
    family: crate::runtime::ForgeQueryNamingMutationFamily,
    outcome: ForgeQueryNamingMutationOutcome,
    attachment_identity: String,
    prior_authoritative_identity: Option<String>,
    target_authoritative_identity: Option<String>,
    resolved_target_entity_identity: Option<String>,
    target_collection: Option<String>,
}

impl ForgeQueryNamingMutationEvidence {
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeNamingMutationBundle) -> Self {
        Self {
            family: match bundle.family() {
                BridgeNamingMutationFamily::AttachNewTarget => {
                    crate::runtime::ForgeQueryNamingMutationFamily::AttachNewTarget
                }
                BridgeNamingMutationFamily::AttachExistingTarget => {
                    crate::runtime::ForgeQueryNamingMutationFamily::AttachExistingTarget
                }
                BridgeNamingMutationFamily::RebindTarget => {
                    crate::runtime::ForgeQueryNamingMutationFamily::RebindTarget
                }
                BridgeNamingMutationFamily::Remove => {
                    crate::runtime::ForgeQueryNamingMutationFamily::Remove
                }
            },
            outcome: match bundle.outcome() {
                BridgeNamingMutationOutcome::AttachedToNewTarget => {
                    ForgeQueryNamingMutationOutcome::AttachedToNewTarget
                }
                BridgeNamingMutationOutcome::AttachedToExistingTarget => {
                    ForgeQueryNamingMutationOutcome::AttachedToExistingTarget
                }
                BridgeNamingMutationOutcome::ReboundTarget => {
                    ForgeQueryNamingMutationOutcome::ReboundTarget
                }
                BridgeNamingMutationOutcome::Removed => ForgeQueryNamingMutationOutcome::Removed,
            },
            attachment_identity: bundle.attachment_identity().to_string(),
            prior_authoritative_identity: bundle.prior_authoritative_identity().map(str::to_string),
            target_authoritative_identity: bundle
                .target_authoritative_identity()
                .map(str::to_string),
            resolved_target_entity_identity: bundle
                .resolved_target_entity_identity()
                .map(str::to_string),
            target_collection: bundle.target_collection().map(str::to_string),
        }
    }

    pub(in crate::runtime) fn from_intent(
        intent: &crate::runtime::ForgeQueryNamingMutationIntent,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: intent.family(),
            outcome: match intent.family() {
                crate::runtime::ForgeQueryNamingMutationFamily::AttachNewTarget => {
                    ForgeQueryNamingMutationOutcome::AttachedToNewTarget
                }
                crate::runtime::ForgeQueryNamingMutationFamily::AttachExistingTarget => {
                    ForgeQueryNamingMutationOutcome::AttachedToExistingTarget
                }
                crate::runtime::ForgeQueryNamingMutationFamily::RebindTarget => {
                    ForgeQueryNamingMutationOutcome::ReboundTarget
                }
                crate::runtime::ForgeQueryNamingMutationFamily::Remove => {
                    ForgeQueryNamingMutationOutcome::Removed
                }
            },
            attachment_identity: intent.attachment_identity().to_string(),
            prior_authoritative_identity: intent.prior_authoritative_identity().map(str::to_string),
            target_authoritative_identity: intent
                .target_authoritative_identity()
                .map(str::to_string),
            resolved_target_entity_identity: resolved_target_entity_identity.map(str::to_string),
            target_collection: target_collection.map(str::to_string),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQueryNamingMutationOutcome {
        self.outcome
    }

    pub fn attachment_identity(&self) -> &str {
        &self.attachment_identity
    }

    pub fn prior_authoritative_identity(&self) -> Option<&str> {
        self.prior_authoritative_identity.as_deref()
    }

    pub fn target_authoritative_identity(&self) -> Option<&str> {
        self.target_authoritative_identity.as_deref()
    }

    pub fn resolved_target_entity_identity(&self) -> Option<&str> {
        self.resolved_target_entity_identity.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }
}
