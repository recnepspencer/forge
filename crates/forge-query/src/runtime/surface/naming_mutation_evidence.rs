use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationTargetCollectionIdentity,
};
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

impl ForgeQueryNamingMutationOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AttachedToNewTarget => "attached_to_new_target",
            Self::AttachedToExistingTarget => "attached_to_existing_target",
            Self::ReboundTarget => "rebound_target",
            Self::Removed => "removed",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryNamingMutationEvidence {
    family: crate::runtime::ForgeQueryNamingMutationFamily,
    outcome: ForgeQueryNamingMutationOutcome,
    attachment_identity: ForgeQueryMutationAuthorityIdentity,
    prior_authoritative_identity: Option<ForgeQueryMutationAuthorityIdentity>,
    target_authoritative_identity: Option<ForgeQueryMutationAuthorityIdentity>,
    resolved_target_entity_identity: Option<ForgeQueryEntityIdentity>,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
}

impl ForgeQueryNamingMutationEvidence {
    #[cfg(test)]
    pub(in crate::runtime) fn from_bridge(bundle: &BridgeNamingMutationBundle) -> Self {
        Self::from_bridge_with_query_context(bundle, None, None)
    }

    pub(in crate::runtime) fn from_bridge_with_query_context(
        bundle: &BridgeNamingMutationBundle,
        resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
        target_collection: Option<&str>,
    ) -> Self {
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
            attachment_identity: ForgeQueryMutationAuthorityIdentity::from_bridge_naming_attachment(
                "naming-attachment",
                bundle.attachment_identity_handle(),
            ),
            prior_authoritative_identity: bundle.prior_authoritative_identity_handle().map(
                |identity| {
                    ForgeQueryMutationAuthorityIdentity::from_bridge_naming_authority(
                        "naming-prior",
                        identity,
                    )
                },
            ),
            target_authoritative_identity: bundle.target_authoritative_identity_handle().map(
                |identity| {
                    ForgeQueryMutationAuthorityIdentity::from_bridge_naming_authority(
                        "naming-target",
                        identity,
                    )
                },
            ),
            resolved_target_entity_identity: bundle
                .resolved_target_entity_identity_handle()
                .map(|identity| {
                    ForgeQueryEntityIdentity::from_relational_record(
                        identity.relational_record_parts(),
                    )
                })
                .or_else(|| resolved_target_entity_identity.cloned()),
            target_collection: bundle
                .target_collection()
                .or(target_collection)
                .map(|collection| {
                    ForgeQueryMutationTargetCollectionIdentity::new("naming-target", collection)
                }),
        }
    }

    pub(in crate::runtime) fn from_intent(
        intent: &crate::runtime::ForgeQueryNamingMutationIntent,
        resolved_target_entity_identity: Option<&ForgeQueryEntityIdentity>,
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
            attachment_identity: intent.attachment_identity().clone(),
            prior_authoritative_identity: intent.prior_authoritative_identity().cloned(),
            target_authoritative_identity: intent.target_authoritative_identity().cloned(),
            resolved_target_entity_identity: resolved_target_entity_identity.cloned(),
            target_collection: target_collection.map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new("naming-target", collection)
            }),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQueryNamingMutationFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQueryNamingMutationOutcome {
        self.outcome
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

    pub fn resolved_target_entity_identity(&self) -> Option<&ForgeQueryEntityIdentity> {
        self.resolved_target_entity_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }
}
