use super::super::super::pricing_support::PricingWorkloadCertificationBundle;
use super::super::lineage_edges::PricingLineageProvenanceEdge;
use serde_json::json;

impl PricingWorkloadCertificationBundle {
    pub(in crate::harness::tests) fn lineage_provenance_edges_json(
        &self,
    ) -> Vec<serde_json::Value> {
        self.lineage_provenance_edges()
            .into_iter()
            .map(lineage_provenance_edge_json)
            .collect()
    }
}

fn lineage_provenance_edge_json(edge: PricingLineageProvenanceEdge) -> serde_json::Value {
    json!({
        "from": edge.from,
        "to": edge.to,
        "kind": edge.kind,
        "surface": edge.surface,
    })
}
