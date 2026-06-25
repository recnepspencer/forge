use forge_query::facade::{ForgeQueryMutationMetadataKey, ForgeQueryRetainedRefreshContext};
use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::MutationOrigin;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use schema::facade::QueryAspectPath;
use serde::{Deserialize, Serialize};

use crate::projection::runtime_boundary::declared_query_surfaces::TopologyQuerySurfaceError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct TopologyQueryMutationEvidence {
    pub authority_snapshot_id: u64,
    pub authority_branch_id: String,
    pub authoritative_mutation_origin: MutationOrigin,
    pub derivation_origin: MutationOrigin,
    pub truth_basis_digest_hex: String,
    pub touched_aspect_paths: Vec<String>,
    pub precision_fallback_count: usize,
    pub precision_budget_fallback_count: usize,
}

impl TopologyQueryMutationEvidence {
    pub(crate) const fn metadata_key() -> &'static str {
        ".topology.read_basis"
    }

    pub(crate) fn from_read_basis(read_basis: &DerivedTopologyReadBasis) -> Self {
        Self {
            authority_snapshot_id: read_basis.snapshot().snapshot_id.0,
            authority_branch_id: read_basis.branch_id().0.clone(),
            authoritative_mutation_origin: read_basis.authoritative_mutation_origin(),
            derivation_origin: read_basis.derivation_origin(),
            truth_basis_digest_hex: read_basis
                .authority
                .truth_basis_identity
                .mutation_digest_hex
                .clone(),
            touched_aspect_paths: read_basis
                .touched_aspects()
                .iter()
                .map(|aspect| QueryAspectPath::from_aspect(*aspect).as_str().to_string())
                .collect(),
            precision_fallback_count: read_basis.precision_fallbacks.len(),
            precision_budget_fallback_count: read_basis.precision_budget_fallbacks.len(),
        }
    }

    pub(super) fn from_refresh(
        refresh: &ForgeQueryRetainedRefreshContext,
    ) -> Result<Self, TopologyQuerySurfaceError> {
        let key = ForgeQueryMutationMetadataKey::new(Self::metadata_key()).map_err(|error| {
            TopologyQuerySurfaceError::new(format!(
                "query-derived refresh metadata key failed to admit: {error}"
            ))
        })?;
        let Some(value) = refresh.refresh_metadata().get(&key) else {
            return Err(TopologyQuerySurfaceError::new(format!(
                "query-derived refresh context is missing `{}` metadata",
                Self::metadata_key()
            )));
        };
        serde_json::from_str(value.terminal_digest_text()).map_err(|error| {
            TopologyQuerySurfaceError::new(format!(
                "query-derived refresh metadata `{}` failed to decode: {error}",
                Self::metadata_key()
            ))
        })
    }

    pub(super) fn touched_aspects(&self) -> Result<Vec<Aspect>, TopologyQuerySurfaceError> {
        self.touched_aspect_paths
            .iter()
            .map(|path| {
                let path = QueryAspectPath::from_str(path).ok_or_else(|| {
                    TopologyQuerySurfaceError::new(format!(
                        "query-derived mutation metadata declared unsupported touched aspect `{path}`"
                    ))
                })?;
                Ok(path.into_aspect())
            })
            .collect()
    }
}
