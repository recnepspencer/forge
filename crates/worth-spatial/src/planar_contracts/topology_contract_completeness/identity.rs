use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarTopologyContractCompletenessBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarTopologyContractAuthorityEntry {
    locus: String,
    value: String,
}

impl PlanarTopologyContractAuthorityEntry {
    pub(crate) fn new(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            value: value.into(),
        }
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("{}:{}", self.locus, self.value)
    }
}

pub(crate) fn planar_topology_contract_authority_entries(
    basis: &PlanarTopologyContractCompletenessBasis,
) -> Vec<PlanarTopologyContractAuthorityEntry> {
    let receipt = basis.topology_query_receipt();
    let mut entries = vec![
        PlanarTopologyContractAuthorityEntry::new("topology.receipt", receipt.receipt_digest()),
        PlanarTopologyContractAuthorityEntry::new("topology.facts", receipt.fact_digest()),
        PlanarTopologyContractAuthorityEntry::new("topology.birth", receipt.source_birth_digest()),
        PlanarTopologyContractAuthorityEntry::new(
            "topology.birth_class",
            receipt.topology_birth_class(),
        ),
        PlanarTopologyContractAuthorityEntry::new(
            "topology.declared_query_surface",
            basis.declared_query_surface_identity(),
        ),
        PlanarTopologyContractAuthorityEntry::new(
            "topology.planar_neighborhood",
            basis.planar_neighborhood_identity(),
        ),
    ];
    entries.extend(receipt.fact_rows().iter().map(|row| {
        PlanarTopologyContractAuthorityEntry::new(
            format!("topology.fact.{}", row.kind().as_str()),
            row.row_digest(),
        )
    }));
    entries.sort_by(|left, right| {
        left.locus()
            .cmp(right.locus())
            .then_with(|| left.value().cmp(right.value()))
    });
    entries
}

pub(crate) fn topology_contract_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
