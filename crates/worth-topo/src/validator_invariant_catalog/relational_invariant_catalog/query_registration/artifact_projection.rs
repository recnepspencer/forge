use forge_query::facade::{
    forge_query_domain, ForgeQueryIntentDeclaration, ForgeQueryIntentInput,
    ForgeQueryInvariantCatalogRegistrationArtifact,
};
use forge_relational::facade::runtime::InvariantCatalog;

use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
};

use super::graph_scoped_custom_invariant_registration::{
    graph_scoped_custom_invariant_projection_rows_from_catalog,
    WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection {
    lane: String,
    semantic_code: String,
    detail: String,
    query_materialization_digest: String,
    query_invariant_catalog_digest: String,
    query_graph_obligation_catalog_digest: String,
    graph_scoped_custom_invariant_rows:
        Vec<WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow>,
    query_graph_obligation_registration_count: usize,
    relational_invariant_family_count: usize,
    projection_digest: String,
}

impl WorthTopologyRelationalInvariantQueryRegistrationArtifactProjection {
    pub(in crate::validator_invariant_catalog) fn from_catalog(
        catalog: &WorthTopologyLegalityCatalog,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let artifact = materialize_query_owned_invariant_registration_artifact(catalog)?;
        let graph_scoped_custom_invariant_rows =
            graph_scoped_custom_invariant_projection_rows_from_catalog(catalog)?;
        let query_invariant_catalog_digest =
            artifact.invariant_catalog().canonical_registration_digest();
        let projection_digest = query_registration_artifact_projection_digest(
            catalog,
            &artifact,
            &query_invariant_catalog_digest,
            &graph_scoped_custom_invariant_rows,
        );
        Ok(Self {
            lane: artifact.lane().to_string(),
            semantic_code: artifact.semantic_code().to_string(),
            detail: artifact.detail().to_string(),
            query_materialization_digest: artifact.materialization_digest().to_string(),
            query_invariant_catalog_digest,
            query_graph_obligation_catalog_digest: catalog
                .query_projection()
                .query_catalog()
                .catalog_digest()
                .to_string(),
            query_graph_obligation_registration_count: graph_scoped_custom_invariant_rows.len(),
            graph_scoped_custom_invariant_rows,
            relational_invariant_family_count: catalog.invariant_family_count(),
            projection_digest,
        })
    }

    pub fn lane(&self) -> &str {
        &self.lane
    }

    pub fn semantic_code(&self) -> &str {
        &self.semantic_code
    }

    pub fn detail(&self) -> &str {
        &self.detail
    }

    pub fn query_materialization_digest(&self) -> &str {
        &self.query_materialization_digest
    }

    pub fn query_invariant_catalog_digest(&self) -> &str {
        &self.query_invariant_catalog_digest
    }

    pub fn query_graph_obligation_catalog_digest(&self) -> &str {
        &self.query_graph_obligation_catalog_digest
    }

    pub fn graph_scoped_custom_invariant_rows(
        &self,
    ) -> &[WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow] {
        &self.graph_scoped_custom_invariant_rows
    }

    pub fn graph_scoped_custom_invariant_count(&self) -> usize {
        self.graph_scoped_custom_invariant_rows.len()
    }

    pub const fn query_graph_obligation_registration_count(&self) -> usize {
        self.query_graph_obligation_registration_count
    }

    pub const fn relational_invariant_family_count(&self) -> usize {
        self.relational_invariant_family_count
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }
}

pub(in crate::validator_invariant_catalog::relational_invariant_catalog) fn materialize_query_owned_invariant_registration_artifact_from_catalog(
    catalog: &WorthTopologyLegalityCatalog,
) -> Result<ForgeQueryInvariantCatalogRegistrationArtifact, WorthTopologyLegalityCatalogError> {
    materialize_query_owned_invariant_registration_artifact(catalog)
}

fn materialize_query_owned_invariant_registration_artifact(
    catalog: &WorthTopologyLegalityCatalog,
) -> Result<ForgeQueryInvariantCatalogRegistrationArtifact, WorthTopologyLegalityCatalogError> {
    let declaration = ForgeQueryIntentDeclaration::strategy_commit(
        "worth.topology.relational_invariant_catalog",
        "validator_invariant_catalog.phase_five",
        "1",
        "worth.topology.relational_invariant_catalog",
        ForgeQueryIntentInput::object([
            (
                "catalog_digest",
                ForgeQueryIntentInput::string(catalog.catalog_digest()),
            ),
            (
                "query_obligation_catalog_digest",
                ForgeQueryIntentInput::string(
                    catalog.query_projection().query_catalog().catalog_digest(),
                ),
            ),
            (
                "relational_invariant_family_count",
                ForgeQueryIntentInput::unsigned_integer(catalog.invariant_family_count() as u64),
            ),
        ]),
    );
    forge_query_domain("worth.topology")
        .for_intent(&declaration)
        .register_invariant_catalog(
            "relational_invariant_catalog.registration_artifact",
            InvariantCatalog {
                registrations: Vec::new(),
            },
        )
        .because(
            "Worth relational invariant families are declared once in the Phase 2 catalog; \
             this Query artifact proves the public invariant-registration lane, not execution.",
        )
        .materialize()
        .map_err(|error| WorthTopologyLegalityCatalogError::QueryRegistration(format!("{error:?}")))
}

fn query_registration_artifact_projection_digest(
    catalog: &WorthTopologyLegalityCatalog,
    artifact: &ForgeQueryInvariantCatalogRegistrationArtifact,
    query_invariant_catalog_digest: &str,
    graph_scoped_custom_invariant_rows: &[WorthTopologyGraphScopedCustomInvariantRegistrationProjectionRow],
) -> String {
    [
        "worth-topo-relational-invariant-query-registration-artifact-projection-v1",
        artifact.lane(),
        artifact.semantic_code(),
        artifact.detail(),
        artifact.materialization_digest(),
        query_invariant_catalog_digest,
        catalog.query_projection().query_catalog().catalog_digest(),
        &graph_scoped_custom_invariant_rows
            .iter()
            .map(|row| row.row_digest())
            .collect::<Vec<_>>()
            .join(","),
        &graph_scoped_custom_invariant_rows.len().to_string(),
        &catalog.invariant_family_count().to_string(),
    ]
    .join("|")
}
