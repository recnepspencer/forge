use schema::facade::platform::authority::touched_graph_conflict::{
    ConflictParticipantIdentity, ConflictRoutingVocabularyError,
};
use schema::facade::platform::authority::touched_graph_conflict_internal::admit_conflict_validator_participant_identity_from_digest;

use super::{legality_family_identity_digest, WorthTopologyLegalityFamilyIdentityDigest};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct WorthTopologyInvariantFamilyIdentity {
    namespace: String,
    name: String,
    semantic_version: String,
    stable_key: String,
    identity_digest: WorthTopologyLegalityFamilyIdentityDigest,
}

impl WorthTopologyInvariantFamilyIdentity {
    pub(in crate::validator_invariant_catalog) fn registered(
        name: impl Into<String>,
        semantic_version: impl Into<String>,
    ) -> Self {
        let namespace = "worth.topo.invariant";
        let name = name.into();
        let semantic_version = semantic_version.into();
        let stable_key = format!("invariant:{namespace}:{name}:{semantic_version}");
        let identity_digest = legality_family_identity_digest(&[
            "invariant",
            namespace,
            name.as_str(),
            semantic_version.as_str(),
        ]);
        Self {
            namespace: namespace.to_string(),
            name,
            semantic_version,
            stable_key,
            identity_digest,
        }
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn semantic_version(&self) -> &str {
        &self.semantic_version
    }

    pub fn stable_key(&self) -> &str {
        &self.stable_key
    }

    pub fn identity_digest(&self) -> &str {
        self.identity_digest.as_str()
    }

    pub fn authority_digest(&self) -> &WorthTopologyLegalityFamilyIdentityDigest {
        &self.identity_digest
    }

    pub fn conflict_participant_identity(
        &self,
    ) -> Result<ConflictParticipantIdentity, ConflictRoutingVocabularyError> {
        admit_conflict_validator_participant_identity_from_digest(self.authority_digest())
    }
}
