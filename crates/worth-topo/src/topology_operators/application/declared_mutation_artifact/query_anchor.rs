use forge_query::facade::ForgeQueryDeclarationInput;

use crate::query_domain::TopologyQueryDomain;

use super::super::{TopologyMutationApplicationError, TopologyRetainedApplicationHandoff};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct TopologyOperatorApplicationQueryAnchor {
    declaration_family_key: &'static str,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    envelope_digest: forge_foundational::facade::CanonicalDerivedDigest,
    receipt_digest: forge_foundational::facade::CanonicalDerivedDigest,
    contribution_digest: String,
}

impl TopologyOperatorApplicationQueryAnchor {
    pub(crate) fn from_retained_handoff<I>(handoff: &TopologyRetainedApplicationHandoff<I>) -> Self
    where
        I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
    {
        Self {
            declaration_family_key: handoff.declaration_family_key(),
            declaration_digest: handoff
                .declaration_receipt()
                .declaration_digest()
                .to_string(),
            progression_digest: handoff.progression_digest().to_string(),
            route_plan_digest: handoff.route_plan_digest().to_string(),
            envelope_digest: handoff.declaration_envelope().envelope_digest().clone(),
            receipt_digest: handoff.declaration_receipt().receipt_digest().clone(),
            contribution_digest: handoff.contribution_digest().to_string(),
        }
    }

    pub(crate) fn declaration_family_key(&self) -> &'static str {
        self.declaration_family_key
    }

    pub(crate) fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub(crate) fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub(crate) fn route_plan_digest(&self) -> &str {
        &self.route_plan_digest
    }

    pub(crate) fn envelope_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        &self.envelope_digest
    }

    pub(crate) fn receipt_digest(&self) -> &forge_foundational::facade::CanonicalDerivedDigest {
        &self.receipt_digest
    }

    pub(crate) fn contribution_digest(&self) -> &str {
        &self.contribution_digest
    }

    pub(crate) fn ensure_semantic_family(
        &self,
        semantic_family_key: &'static str,
    ) -> Result<(), TopologyMutationApplicationError> {
        if self.declaration_family_key == semantic_family_key {
            return Ok(());
        }

        Err(
            TopologyMutationApplicationError::QueryAnchorFamilyMismatch {
                semantic_family_key,
                query_declaration_family_key: self.declaration_family_key,
            },
        )
    }

    #[cfg(test)]
    pub(crate) fn with_family_for_test(
        declaration_family_key: &'static str,
        source: &TopologyOperatorApplicationQueryAnchor,
    ) -> Self {
        Self {
            declaration_family_key,
            declaration_digest: source.declaration_digest.clone(),
            progression_digest: source.progression_digest.clone(),
            route_plan_digest: source.route_plan_digest.clone(),
            envelope_digest: source.envelope_digest.clone(),
            receipt_digest: source.receipt_digest.clone(),
            contribution_digest: source.contribution_digest.clone(),
        }
    }
}

const _: () = {
    let _ = std::mem::size_of::<TopologyOperatorApplicationQueryAnchor>();
    let _ = TopologyOperatorApplicationQueryAnchor::from_retained_handoff::<
        crate::topology_operators::TopologyCreateTopologyEntityDeclaration,
    >;
    let _ = TopologyOperatorApplicationQueryAnchor::declaration_family_key;
    let _ = TopologyOperatorApplicationQueryAnchor::declaration_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::progression_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::route_plan_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::envelope_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::receipt_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::contribution_digest;
    let _ = TopologyOperatorApplicationQueryAnchor::ensure_semantic_family;
};
