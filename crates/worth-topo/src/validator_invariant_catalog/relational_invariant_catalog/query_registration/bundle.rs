use forge_query::facade::{
    ForgeQueryGraphObligationRegistration, ForgeQueryGraphScopedCustomInvariantRegistration,
    ForgeQueryInvariantCatalogRegistrationArtifact,
};

use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
};

use super::artifact_projection::materialize_query_owned_invariant_registration_artifact_from_catalog;
use super::graph_scoped_custom_invariant_registration::{
    graph_obligation_registrations_from_catalog,
    graph_scoped_custom_invariant_registrations_from_catalog,
};

#[derive(Clone, Debug)]
pub struct WorthTopologyRelationalInvariantQueryRegistrationBundle {
    artifact: ForgeQueryInvariantCatalogRegistrationArtifact,
    graph_scoped_custom_invariants: Vec<ForgeQueryGraphScopedCustomInvariantRegistration>,
    graph_obligation_registrations: Vec<ForgeQueryGraphObligationRegistration>,
    bundle_digest: String,
}

impl WorthTopologyRelationalInvariantQueryRegistrationBundle {
    pub(in crate::validator_invariant_catalog) fn from_catalog(
        catalog: &WorthTopologyLegalityCatalog,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let artifact =
            materialize_query_owned_invariant_registration_artifact_from_catalog(catalog)?;
        let graph_scoped_custom_invariants =
            graph_scoped_custom_invariant_registrations_from_catalog(catalog)?;
        let graph_obligation_registrations = graph_obligation_registrations_from_catalog(catalog)?;
        let bundle_digest = query_registration_bundle_digest(
            &artifact,
            &graph_obligation_registrations,
            graph_scoped_custom_invariants.len(),
        );
        Ok(Self {
            artifact,
            graph_scoped_custom_invariants,
            graph_obligation_registrations,
            bundle_digest,
        })
    }

    pub fn artifact(&self) -> &ForgeQueryInvariantCatalogRegistrationArtifact {
        &self.artifact
    }

    pub fn graph_scoped_custom_invariants(
        &self,
    ) -> &[ForgeQueryGraphScopedCustomInvariantRegistration] {
        &self.graph_scoped_custom_invariants
    }

    pub fn graph_obligation_registrations(&self) -> &[ForgeQueryGraphObligationRegistration] {
        &self.graph_obligation_registrations
    }

    pub fn graph_scoped_custom_invariant_count(&self) -> usize {
        self.graph_scoped_custom_invariants.len()
    }

    pub fn graph_obligation_registration_count(&self) -> usize {
        self.graph_obligation_registrations.len()
    }

    pub fn bundle_digest(&self) -> &str {
        &self.bundle_digest
    }
}

fn query_registration_bundle_digest(
    artifact: &ForgeQueryInvariantCatalogRegistrationArtifact,
    graph_obligation_registrations: &[ForgeQueryGraphObligationRegistration],
    graph_scoped_custom_invariant_count: usize,
) -> String {
    let mut parts = vec![
        "worth-topo-relational-invariant-query-registration-bundle-v1".to_string(),
        format!("artifact:{}", artifact.materialization_digest()),
        format!("graph-scoped-custom-invariant-count:{graph_scoped_custom_invariant_count}"),
    ];
    parts.extend(
        graph_obligation_registrations
            .iter()
            .map(|registration| format!("graph-obligation:{}", registration.registration_digest())),
    );
    parts.join("|")
}
