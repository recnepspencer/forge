use forge_runtime_bridge::facade::{
    BridgeExistingTruthBindingBundle, BridgeExistingTruthBindingFamily,
    BridgeExistingTruthBindingOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthBindingOutcome {
    ExistingAuthoritativeTarget,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthBindingEvidence {
    family: crate::runtime::ForgeQueryExistingTruthBindingFamily,
    outcome: ForgeQueryExistingTruthBindingOutcome,
    authoritative_identity: String,
    resolved_entity_identity: String,
    target_collection: Option<String>,
    binding_digest: String,
}

impl ForgeQueryExistingTruthBindingEvidence {
    pub(in crate::runtime) fn from_bridge(binding: &BridgeExistingTruthBindingBundle) -> Self {
        Self {
            family: match binding.family() {
                BridgeExistingTruthBindingFamily::DirectEntityIdentity => {
                    crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
                }
                BridgeExistingTruthBindingFamily::DirectRelationIdentity => {
                    crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
                }
            },
            outcome: match binding.outcome() {
                BridgeExistingTruthBindingOutcome::ExistingAuthoritativeTarget => {
                    ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
                }
            },
            authoritative_identity: binding.authoritative_identity().to_string(),
            resolved_entity_identity: binding.resolved_target_identity().to_string(),
            target_collection: binding.target_collection().map(str::to_string),
            binding_digest: binding.binding_digest().to_string(),
        }
    }

    pub(in crate::runtime) fn from_binding(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
    ) -> Self {
        Self {
            family: binding.family(),
            outcome: ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget,
            authoritative_identity: binding.authoritative_identity().to_string(),
            resolved_entity_identity: binding.resolved_target_identity().to_string(),
            target_collection: binding.target_collection().map(str::to_string),
            binding_digest: binding.binding_digest(),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQueryExistingTruthBindingFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQueryExistingTruthBindingOutcome {
        self.outcome
    }

    pub fn authoritative_identity(&self) -> &str {
        &self.authoritative_identity
    }

    pub fn resolved_target_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn resolved_entity_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn resolved_relation_identity(&self) -> &str {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&str> {
        self.target_collection.as_deref()
    }

    pub fn binding_digest(&self) -> &str {
        &self.binding_digest
    }
}
