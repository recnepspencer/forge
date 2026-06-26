use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationEvidenceDigest,
    ForgeQueryMutationTargetCollectionIdentity,
};
use forge_runtime_bridge::facade::{
    BridgeExistingTruthBindingBundle, BridgeExistingTruthBindingFamily,
    BridgeExistingTruthBindingOutcome,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeQueryExistingTruthBindingOutcome {
    ExistingAuthoritativeTarget,
}

impl ForgeQueryExistingTruthBindingOutcome {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExistingAuthoritativeTarget => "existing-authoritative-target",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryExistingTruthBindingEvidence {
    family: crate::runtime::ForgeQueryExistingTruthBindingFamily,
    outcome: ForgeQueryExistingTruthBindingOutcome,
    authoritative_identity: ForgeQueryMutationAuthorityIdentity,
    resolved_entity_identity: ForgeQueryEntityIdentity,
    target_collection: Option<ForgeQueryMutationTargetCollectionIdentity>,
    binding_digest: ForgeQueryMutationEvidenceDigest,
}

impl ForgeQueryExistingTruthBindingEvidence {
    pub(in crate::runtime) fn from_bridge(binding: &BridgeExistingTruthBindingBundle) -> Self {
        let family = match binding.family() {
            BridgeExistingTruthBindingFamily::DirectEntityIdentity => {
                crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectEntityIdentity
            }
            BridgeExistingTruthBindingFamily::DirectRelationIdentity => {
                crate::runtime::ForgeQueryExistingTruthBindingFamily::DirectRelationIdentity
            }
        };
        let outcome = match binding.outcome() {
            BridgeExistingTruthBindingOutcome::ExistingAuthoritativeTarget => {
                ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget
            }
        };
        let authoritative_identity =
            ForgeQueryMutationAuthorityIdentity::from_bridge_existing_truth_authority(
                "existing-truth-binding-authority",
                binding.authoritative_identity_handle(),
            );
        let resolved_entity_identity = ForgeQueryEntityIdentity::from_relational_record(
            binding
                .resolved_target_identity_handle()
                .relational_record_parts(),
        );
        let target_collection = binding.target_collection().map(|collection| {
            ForgeQueryMutationTargetCollectionIdentity::new("existing-truth-binding", collection)
        });
        let binding_identity =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
                .field_shape(ForgeQueryEvidenceTag::new("role"), "existing-truth-binding")
                .field_shape(ForgeQueryEvidenceTag::new("family"), family.as_str())
                .field_shape(ForgeQueryEvidenceTag::new("outcome"), outcome.as_str())
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("authoritative"),
                    authoritative_identity.evidence_identity(),
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("resolved"),
                    &resolved_entity_identity.evidence_identity(),
                )
                .optional_evidence_identity(
                    ForgeQueryEvidenceTag::new("collection"),
                    target_collection
                        .as_ref()
                        .map(ForgeQueryMutationTargetCollectionIdentity::evidence_identity),
                )
                .seal();
        Self {
            family,
            outcome,
            authoritative_identity,
            resolved_entity_identity,
            target_collection,
            binding_digest: ForgeQueryMutationEvidenceDigest::aggregate(
                "existing-truth-binding",
                binding_identity,
            ),
        }
    }

    pub(in crate::runtime) fn from_binding(
        binding: &crate::runtime::ForgeQueryExistingTruthTargetBinding,
    ) -> Self {
        let resolved_entity_identity = binding
            .resolved_target_identity()
            .relational_record_parts()
            .map(ForgeQueryEntityIdentity::from_relational_record)
            .expect(
                "existing-truth binding evidence must carry a relational record target identity",
            );
        Self {
            family: binding.family(),
            outcome: ForgeQueryExistingTruthBindingOutcome::ExistingAuthoritativeTarget,
            authoritative_identity: binding.authoritative_identity().clone(),
            resolved_entity_identity,
            target_collection: binding.target_collection_identity().cloned(),
            binding_digest: ForgeQueryMutationEvidenceDigest::source_identity(
                "existing-truth-binding",
                binding.binding_evidence_identity(),
            ),
        }
    }

    pub fn family(&self) -> crate::runtime::ForgeQueryExistingTruthBindingFamily {
        self.family
    }

    pub fn outcome(&self) -> ForgeQueryExistingTruthBindingOutcome {
        self.outcome
    }

    pub fn authoritative_identity(&self) -> &ForgeQueryMutationAuthorityIdentity {
        &self.authoritative_identity
    }

    pub fn resolved_target_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn resolved_entity_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn resolved_relation_identity(&self) -> &ForgeQueryEntityIdentity {
        &self.resolved_entity_identity
    }

    pub fn target_collection(&self) -> Option<&ForgeQueryMutationTargetCollectionIdentity> {
        self.target_collection.as_ref()
    }

    pub fn binding_digest(&self) -> &ForgeQueryMutationEvidenceDigest {
        &self.binding_digest
    }
}
