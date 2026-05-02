use std::sync::Arc;

use super::digest::aggregate_digest;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeExistingTruthBindingFamily {
    DirectEntityIdentity,
    DirectRelationIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BridgeExistingTruthBindingOutcome {
    ExistingAuthoritativeTarget,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BridgeExistingTruthBindingBundle {
    family: BridgeExistingTruthBindingFamily,
    outcome: BridgeExistingTruthBindingOutcome,
    authoritative_identity: Arc<str>,
    resolved_target_identity: Arc<str>,
    target_collection: Option<Arc<str>>,
    binding_digest: Arc<str>,
}

impl BridgeExistingTruthBindingBundle {
    pub fn direct_entity(
        authoritative_identity: impl Into<Arc<str>>,
        resolved_entity_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self::new(
            BridgeExistingTruthBindingFamily::DirectEntityIdentity,
            authoritative_identity,
            resolved_entity_identity,
            target_collection,
        )
    }

    pub fn direct_relation(
        authoritative_identity: impl Into<Arc<str>>,
        resolved_relation_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        Self::new(
            BridgeExistingTruthBindingFamily::DirectRelationIdentity,
            authoritative_identity,
            resolved_relation_identity,
            target_collection,
        )
    }

    fn new(
        family: BridgeExistingTruthBindingFamily,
        authoritative_identity: impl Into<Arc<str>>,
        resolved_target_identity: impl Into<Arc<str>>,
        target_collection: Option<&str>,
    ) -> Self {
        let authoritative_identity: Arc<str> = authoritative_identity.into();
        let resolved_target_identity: Arc<str> = resolved_target_identity.into();
        let target_collection = target_collection.map(|value| Arc::from(value.to_owned()));
        let target_collection_label: &str = match target_collection.as_ref() {
            Some(value) => value,
            None => "none",
        };
        let binding_digest = aggregate_digest(
            "bridge-existing-truth-binding",
            [
                format!("family:{family:?}"),
                format!(
                    "outcome:{:?}",
                    BridgeExistingTruthBindingOutcome::ExistingAuthoritativeTarget
                ),
                format!("authoritative:{}", authoritative_identity.as_ref()),
                format!("resolved:{}", resolved_target_identity.as_ref()),
                format!("collection:{target_collection_label}"),
            ],
        );
        Self {
            family,
            outcome: BridgeExistingTruthBindingOutcome::ExistingAuthoritativeTarget,
            authoritative_identity,
            resolved_target_identity,
            target_collection,
            binding_digest,
        }
    }

    pub fn family(&self) -> BridgeExistingTruthBindingFamily {
        self.family
    }

    pub fn outcome(&self) -> BridgeExistingTruthBindingOutcome {
        self.outcome
    }

    pub fn authoritative_identity(&self) -> &str {
        self.authoritative_identity.as_ref()
    }

    pub fn resolved_target_identity(&self) -> &str {
        self.resolved_target_identity.as_ref()
    }

    pub fn resolved_entity_identity(&self) -> &str {
        self.resolved_target_identity.as_ref()
    }

    pub fn resolved_relation_identity(&self) -> &str {
        self.resolved_target_identity.as_ref()
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn binding_digest(&self) -> &str {
        self.binding_digest.as_ref()
    }
}
