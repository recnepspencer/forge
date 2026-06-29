use forge_relational::facade::identity::{EntityId, RelationId};
use serde::Serialize;
#[cfg(feature = "conflict-routing-internal-authority")]
use std::any::type_name;

use super::ConflictRoutingVocabularyError;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
pub enum ConflictParticipantAuthority {
    Entity,
    Relation,
    Evidence,
    Validator,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize)]
pub struct ConflictParticipantIdentity {
    authority: ConflictParticipantAuthority,
    digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictParticipantIdentityInput {
    authority: ConflictParticipantAuthority,
    digest: String,
}

impl ConflictParticipantIdentityInput {
    pub fn entity(entity_id: EntityId) -> Self {
        Self::new(
            ConflictParticipantAuthority::Entity,
            format!(
                "entity:{}:{}:{}",
                entity_id.partition_value_u64(),
                entity_id.local_slot_value(),
                entity_id.generation_value()
            ),
        )
    }

    pub fn relation(relation_id: RelationId) -> Self {
        Self::new(
            ConflictParticipantAuthority::Relation,
            format!(
                "relation:{}:{}:{}",
                relation_id.partition_value_u64(),
                relation_id.local_slot_value(),
                relation_id.generation_value()
            ),
        )
    }

    #[cfg(feature = "conflict-routing-internal-authority")]
    pub(crate) fn evidence_digest(digest: impl Into<String>) -> Self {
        Self::new(ConflictParticipantAuthority::Evidence, digest)
    }

    #[cfg(feature = "conflict-routing-internal-authority")]
    pub(crate) fn validator_digest(digest: impl Into<String>) -> Self {
        Self::new(ConflictParticipantAuthority::Validator, digest)
    }

    fn new(authority: ConflictParticipantAuthority, digest: impl Into<String>) -> Self {
        Self {
            authority,
            digest: digest.into(),
        }
    }
}

impl ConflictParticipantIdentity {
    pub const fn authority(&self) -> ConflictParticipantAuthority {
        self.authority
    }

    pub fn digest(&self) -> &str {
        &self.digest
    }

    pub fn canonical_part(&self) -> String {
        format!("{:?}:{}", self.authority, self.digest)
    }
}

pub fn admit_conflict_participant_identity(
    input: ConflictParticipantIdentityInput,
) -> Result<ConflictParticipantIdentity, ConflictRoutingVocabularyError> {
    let digest = input.digest.trim();
    if digest.is_empty() {
        return Err(ConflictRoutingVocabularyError::EmptyDigest(
            "conflict participant identity",
        ));
    }
    Ok(ConflictParticipantIdentity {
        authority: input.authority,
        digest: digest.to_string(),
    })
}

#[cfg(feature = "conflict-routing-internal-authority")]
pub fn admit_conflict_evidence_participant_identity_from_digest<
    T: AsRef<str> + ?Sized + 'static,
>(
    digest: &T,
) -> Result<ConflictParticipantIdentity, ConflictRoutingVocabularyError> {
    authorize_internal_digest_type::<T>(
        "worth-spatial evidence authority digest carrier",
        &[
            "worth_spatial::workload_platform::evidence_ledger::spatial_touch_admission::digest::SpatialGeometryEvidenceParticipantDigest",
            "worth_spatial::workload_platform::evidence_lookup_family_catalog::declaration::EvidenceLookupFamilyDeclarationDigest",
        ],
    )?;
    admit_conflict_participant_identity(ConflictParticipantIdentityInput::evidence_digest(
        digest.as_ref(),
    ))
}

#[cfg(feature = "conflict-routing-internal-authority")]
pub fn admit_conflict_validator_participant_identity_from_digest<
    T: AsRef<str> + ?Sized + 'static,
>(
    digest: &T,
) -> Result<ConflictParticipantIdentity, ConflictRoutingVocabularyError> {
    authorize_internal_digest_type::<T>(
        "worth-topo validator authority digest carrier",
        &[
            "topology::validator_invariant_catalog::family_identity::identity_digest::WorthTopologyLegalityFamilyIdentityDigest",
        ],
    )?;
    admit_conflict_participant_identity(ConflictParticipantIdentityInput::validator_digest(
        digest.as_ref(),
    ))
}

#[cfg(feature = "conflict-routing-internal-authority")]
fn authorize_internal_digest_type<T: ?Sized + 'static>(
    required: &'static str,
    allowed_types: &[&'static str],
) -> Result<(), ConflictRoutingVocabularyError> {
    let found = type_name::<T>();
    if allowed_types.contains(&found) {
        Ok(())
    } else {
        Err(ConflictRoutingVocabularyError::WrongInternalAuthorityType { required, found })
    }
}
