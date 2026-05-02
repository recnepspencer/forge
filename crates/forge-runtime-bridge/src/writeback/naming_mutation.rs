use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeNamingMutationFamily {
    AttachNewTarget,
    AttachExistingTarget,
    RebindTarget,
    Remove,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeNamingMutationOutcome {
    AttachedToNewTarget,
    AttachedToExistingTarget,
    ReboundTarget,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeNamingMutationBundle {
    family: BridgeNamingMutationFamily,
    outcome: BridgeNamingMutationOutcome,
    attachment_identity: Arc<str>,
    prior_authoritative_identity: Option<Arc<str>>,
    target_authoritative_identity: Option<Arc<str>>,
    resolved_target_entity_identity: Option<Arc<str>>,
    target_collection: Option<Arc<str>>,
}

impl BridgeNamingMutationBundle {
    pub fn attach_new_target(
        attachment_identity: impl Into<Arc<str>>,
        resolved_target_entity_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: BridgeNamingMutationFamily::AttachNewTarget,
            outcome: BridgeNamingMutationOutcome::AttachedToNewTarget,
            attachment_identity: attachment_identity.into(),
            prior_authoritative_identity: None,
            target_authoritative_identity: None,
            resolved_target_entity_identity: Some(resolved_target_entity_identity.into()),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
        }
    }

    pub fn attach_existing_target(
        attachment_identity: impl Into<Arc<str>>,
        target_authoritative_identity: impl Into<Arc<str>>,
        resolved_target_entity_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: BridgeNamingMutationFamily::AttachExistingTarget,
            outcome: BridgeNamingMutationOutcome::AttachedToExistingTarget,
            attachment_identity: attachment_identity.into(),
            prior_authoritative_identity: None,
            target_authoritative_identity: Some(target_authoritative_identity.into()),
            resolved_target_entity_identity: Some(resolved_target_entity_identity.into()),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
        }
    }

    pub fn rebind_target(
        attachment_identity: impl Into<Arc<str>>,
        prior_authoritative_identity: impl Into<Arc<str>>,
        target_authoritative_identity: impl Into<Arc<str>>,
        resolved_target_entity_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: BridgeNamingMutationFamily::RebindTarget,
            outcome: BridgeNamingMutationOutcome::ReboundTarget,
            attachment_identity: attachment_identity.into(),
            prior_authoritative_identity: Some(prior_authoritative_identity.into()),
            target_authoritative_identity: Some(target_authoritative_identity.into()),
            resolved_target_entity_identity: Some(resolved_target_entity_identity.into()),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
        }
    }

    pub fn remove(
        attachment_identity: impl Into<Arc<str>>,
        prior_authoritative_identity: impl Into<Arc<str>>,
        resolved_target_entity_identity: Option<&str>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family: BridgeNamingMutationFamily::Remove,
            outcome: BridgeNamingMutationOutcome::Removed,
            attachment_identity: attachment_identity.into(),
            prior_authoritative_identity: Some(prior_authoritative_identity.into()),
            target_authoritative_identity: None,
            resolved_target_entity_identity: resolved_target_entity_identity
                .map(|value| Arc::from(value.to_owned())),
            target_collection: target_collection.map(|value| Arc::from(value.to_owned())),
        }
    }

    pub fn family(&self) -> BridgeNamingMutationFamily {
        self.family
    }

    pub fn outcome(&self) -> BridgeNamingMutationOutcome {
        self.outcome
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

    pub fn resolved_target_entity_identity(&self) -> Option<&str> {
        self.resolved_target_entity_identity.as_deref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }
}
