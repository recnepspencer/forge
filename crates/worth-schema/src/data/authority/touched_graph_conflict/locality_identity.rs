use serde::Serialize;
#[cfg(feature = "conflict-routing-internal-authority")]
use std::any::type_name;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::data::authority::replay_undo_semantic_graph::ReplayUndoSemanticGraphLocalityScope;

use super::ConflictRoutingVocabularyError;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ConflictLocalityIdentity {
    scope: ReplayUndoSemanticGraphLocalityScope,
    authority_digest: String,
    locality_identity_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConflictLocalityIdentityInput {
    scope: ReplayUndoSemanticGraphLocalityScope,
    authority_digest: String,
}

impl ConflictLocalityIdentityInput {
    #[cfg(feature = "conflict-routing-internal-authority")]
    pub(crate) fn topology_touched_closure_digest(digest: impl Into<String>) -> Self {
        Self {
            scope: ReplayUndoSemanticGraphLocalityScope::TopologyTouchedClosure,
            authority_digest: digest.into(),
        }
    }

    #[cfg(feature = "conflict-routing-internal-authority")]
    pub(crate) fn spatial_touch_authority_digest(digest: impl Into<String>) -> Self {
        Self {
            scope: ReplayUndoSemanticGraphLocalityScope::SpatialTouchAuthority,
            authority_digest: digest.into(),
        }
    }
}

impl ConflictLocalityIdentity {
    pub const fn scope(&self) -> ReplayUndoSemanticGraphLocalityScope {
        self.scope
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn locality_identity_digest(&self) -> &str {
        &self.locality_identity_digest
    }

    pub fn canonical_part(&self) -> String {
        format!("{}:{}", self.scope.as_str(), self.locality_identity_digest)
    }
}

pub fn admit_conflict_locality_identity(
    input: ConflictLocalityIdentityInput,
) -> Result<ConflictLocalityIdentity, ConflictRoutingVocabularyError> {
    let authority_digest = input.authority_digest.trim();
    if authority_digest.is_empty() {
        return Err(ConflictRoutingVocabularyError::EmptyDigest(
            "conflict locality identity",
        ));
    }
    let locality_identity_digest = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-schema:touched-graph-conflict-locality-identity:v1".to_string(),
            input.scope.as_str().to_string(),
            authority_digest.to_string(),
        ],
    );
    Ok(ConflictLocalityIdentity {
        scope: input.scope,
        authority_digest: authority_digest.to_string(),
        locality_identity_digest,
    })
}

#[cfg(feature = "conflict-routing-internal-authority")]
pub fn admit_conflict_topology_touched_closure_locality_identity_from_digest<
    T: AsRef<str> + ?Sized + 'static,
>(
    digest: &T,
) -> Result<ConflictLocalityIdentity, ConflictRoutingVocabularyError> {
    authorize_internal_digest_type::<T>(
        "worth-topo touched-closure digest carrier",
        &[
            "topology::derived_topology::invalidation_plan::selection::touched_closure::DerivedInvalidationTouchedClosureDigest",
        ],
    )?;
    admit_conflict_locality_identity(
        ConflictLocalityIdentityInput::topology_touched_closure_digest(digest.as_ref()),
    )
}

#[cfg(feature = "conflict-routing-internal-authority")]
pub fn admit_conflict_spatial_touch_authority_locality_identity_from_digest<
    T: AsRef<str> + ?Sized + 'static,
>(
    digest: &T,
) -> Result<ConflictLocalityIdentity, ConflictRoutingVocabularyError> {
    authorize_internal_digest_type::<T>(
        "worth-spatial touch-authority digest carrier",
        &[
            "worth_spatial::workload_platform::evidence_ledger::spatial_touch_admission::digest::SpatialGeometryEvidenceTouchDigest",
        ],
    )?;
    admit_conflict_locality_identity(
        ConflictLocalityIdentityInput::spatial_touch_authority_digest(digest.as_ref()),
    )
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
