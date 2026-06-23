use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag,
};
use crate::memory_workspace::ForgeQueryEntityIdentity;
use crate::runtime::{
    ForgeQueryMutationAuthorityIdentity, ForgeQueryMutationEvidenceDigest,
    ForgeQueryMutationTargetCollectionIdentity,
};

use super::{ForgeQueryContinuityMutationEvidence, ForgeQueryContinuityOutcomeClass};

impl ForgeQueryContinuityMutationEvidence {
    pub(crate) fn with_test_family(
        mut self,
        family: crate::runtime::ForgeQueryContinuityMutationFamily,
    ) -> Self {
        self.family = family;
        self
    }

    pub(crate) fn test_only(
        family: crate::runtime::ForgeQueryContinuityMutationFamily,
        outcome_class: ForgeQueryContinuityOutcomeClass,
        prior_authoritative_identity: impl Into<String>,
        successor_authoritative_identities: Vec<String>,
        resolved_target_entity_identity: Option<ForgeQueryEntityIdentity>,
        target_collection: Option<&str>,
    ) -> Self {
        Self {
            family,
            outcome_class,
            prior_authoritative_identity: ForgeQueryMutationAuthorityIdentity::new(
                "continuity-prior",
                prior_authoritative_identity,
            ),
            successor_authoritative_identities: successor_authoritative_identities
                .into_iter()
                .map(|identity| {
                    ForgeQueryMutationAuthorityIdentity::new("continuity-successor", identity)
                })
                .collect(),
            basis_binding_digest: Some(test_only_digest(
                "continuity-basis-binding",
                "basis-binding:test",
            )),
            resolved_target_entity_identity,
            target_collection: target_collection.map(|collection| {
                ForgeQueryMutationTargetCollectionIdentity::new("continuity-target", collection)
            }),
            lineage_digest: test_only_digest("continuity-lineage", "lineage:test"),
            continuity_resolution_digest: test_only_digest(
                "continuity-resolution",
                "continuity-resolution:test",
            ),
        }
    }
}

fn test_only_digest(role: &'static str, value: &str) -> ForgeQueryMutationEvidenceDigest {
    let identity =
        forge_query_evidence_identity(ForgeQueryEvidenceScope::MutationEvidenceAggregateDigest)
            .field_shape(ForgeQueryEvidenceTag::new("role"), role)
            .field_value(ForgeQueryEvidenceTag::new("test_value"), value)
            .seal();
    ForgeQueryMutationEvidenceDigest::aggregate(role, identity)
}
