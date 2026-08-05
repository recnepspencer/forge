use worth_query_declaration::facade::application_query::ErasedApplicationQueryDefinition;

use worth_foundational::facade::{
    CanonicalBasisDomain, CanonicalBasisEntryKind, CanonicalDigestDerivationDenial,
    CanonicalDigestId, CanonicalDigestWorkBudget,
};

use super::{digest, prepare_artifact, WorthQueryApplicationCanonicalArtifact, DOMAIN_NAME};
use crate::application_query::WorthQueryInstalledGraphReadContract;

pub(in crate::application_query) fn prepare_installed_query_basis(
    package_identity: &CanonicalDigestId,
    schema_identity: &CanonicalDigestId,
    definition: &ErasedApplicationQueryDefinition,
    graph: &WorthQueryInstalledGraphReadContract,
    budget: CanonicalDigestWorkBudget,
) -> Result<WorthQueryApplicationCanonicalArtifact, CanonicalDigestDerivationDenial> {
    let mut entries = vec![
        digest("package", package_identity),
        digest("schema", schema_identity),
        digest("read-graph", graph.digest()),
    ];
    entries.extend(definition.canonical_basis().embedded_entries(
        CanonicalBasisDomain::Future(DOMAIN_NAME),
        "query-meaning",
        CanonicalBasisEntryKind::Field,
    ));
    prepare_artifact("installed-query", entries, budget)
}
