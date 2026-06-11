use super::TopologySeedKind;
use crate::workload_platform::{TopologyWorkload, TopologyWorkloadDenial, TopologyWorkloadReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TopologySeedQueryReceipts {
    declaration_receipt: TopologyWorkloadReceipt,
    query_surface: String,
}

impl TopologySeedQueryReceipts {
    pub(crate) fn new(
        kind: TopologySeedKind,
        declaration: String,
    ) -> Result<Self, TopologyWorkloadDenial> {
        let query_surface = format!(".topology.seed.{}", kind.as_str());
        let declaration_receipt = TopologyWorkload::declared(declaration)
            .from_query_declaration(query_surface.clone())?;
        Ok(Self {
            declaration_receipt,
            query_surface,
        })
    }

    pub fn declaration_receipt(&self) -> &TopologyWorkloadReceipt {
        &self.declaration_receipt
    }

    pub fn query_surface(&self) -> &str {
        &self.query_surface
    }
}
