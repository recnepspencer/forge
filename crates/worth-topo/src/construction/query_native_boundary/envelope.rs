use super::receipt::TopologyPrimitiveConstructionQueryReceipt;
use super::surface_vocab::{
    TopologyConstructionQueryFactKind, TopologyConstructionQueryFactProvenance,
    TopologyConstructionQueryFactRow, TopologyConstructionQueryInspectionSurface,
    TopologyConstructionQueryMutationSurface, TopologyConstructionQueryReadSurface,
};

use super::birth_synopsis::TopologyPrimitiveConstructionQueryBirthSynopsis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyPrimitiveConstructionQueryEnvelope {
    envelope_name: &'static str,
    source_birth_digest: String,
    topology_birth_class: String,
    receipt: TopologyPrimitiveConstructionQueryReceipt,
    envelope_digest: String,
}

impl TopologyPrimitiveConstructionQueryEnvelope {
    pub(crate) fn new(
        synopsis: &TopologyPrimitiveConstructionQueryBirthSynopsis,
        receipt: TopologyPrimitiveConstructionQueryReceipt,
    ) -> Self {
        let envelope_name = "worth-topo.query-native-construction-envelope";
        let source_birth_digest = synopsis.source_birth_digest().to_string();
        let topology_birth_class = synopsis.topology_birth_class().to_string();
        let parts = [
            envelope_name.to_string(),
            source_birth_digest.clone(),
            topology_birth_class.clone(),
            receipt.receipt_digest().to_string(),
            receipt.fact_digest().to_string(),
            receipt.mutation_surface().as_str().to_string(),
            receipt.read_surface().as_str().to_string(),
            receipt.inspection_surface().as_str().to_string(),
        ];
        Self {
            envelope_name,
            source_birth_digest,
            topology_birth_class,
            receipt,
            envelope_digest: super::digest_parts(&parts),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_tests(
        source_birth_digest: &str,
        topology_birth_class: &str,
        receipt: TopologyPrimitiveConstructionQueryReceipt,
    ) -> Self {
        let parts = [
            "worth-topo.query-native-construction-envelope".to_string(),
            source_birth_digest.to_string(),
            topology_birth_class.to_string(),
            receipt.receipt_digest().to_string(),
            receipt.fact_digest().to_string(),
            receipt.mutation_surface().as_str().to_string(),
            receipt.read_surface().as_str().to_string(),
            receipt.inspection_surface().as_str().to_string(),
        ];
        Self {
            envelope_name: "worth-topo.query-native-construction-envelope",
            source_birth_digest: source_birth_digest.to_string(),
            topology_birth_class: topology_birth_class.to_string(),
            receipt,
            envelope_digest: super::digest_parts(&parts),
        }
    }

    pub fn envelope_name(&self) -> &str {
        self.envelope_name
    }

    pub fn source_birth_digest(&self) -> &str {
        &self.source_birth_digest
    }

    pub fn topology_birth_class(&self) -> &str {
        &self.topology_birth_class
    }

    pub fn receipt_name(&self) -> &str {
        self.receipt.receipt_name()
    }

    pub fn receipt_digest(&self) -> &str {
        self.receipt.receipt_digest()
    }

    pub fn mutation_surface(&self) -> TopologyConstructionQueryMutationSurface {
        self.receipt.mutation_surface()
    }

    pub fn required_query_families(&self) -> &[forge_query::facade::ForgeQueryRuntimeFacadeFamily] {
        self.receipt.required_query_families()
    }

    pub fn read_surface(&self) -> TopologyConstructionQueryReadSurface {
        self.receipt.read_surface()
    }

    pub fn inspection_surface(&self) -> TopologyConstructionQueryInspectionSurface {
        self.receipt.inspection_surface()
    }

    pub fn fact_provenance(&self) -> TopologyConstructionQueryFactProvenance {
        self.receipt.fact_provenance()
    }

    pub fn fact_rows(&self) -> &[TopologyConstructionQueryFactRow] {
        self.receipt.fact_rows()
    }

    pub fn fact_digest(&self) -> &str {
        self.receipt.fact_digest()
    }

    pub fn row_for(
        &self,
        kind: TopologyConstructionQueryFactKind,
    ) -> Option<&TopologyConstructionQueryFactRow> {
        self.receipt.row_for(kind)
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }
}
