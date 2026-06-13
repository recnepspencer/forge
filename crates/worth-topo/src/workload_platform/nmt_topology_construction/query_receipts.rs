use super::denial::{query_admission, NmtTopologyConstructionDenial};
use super::pattern_spec::NmtTopologyPattern;
use crate::workload_platform::{TopologyWorkload, TopologyWorkloadDenial, TopologyWorkloadReceipt};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NmtTopologyConstructionQueryReceipts {
    declaration_receipt: TopologyWorkloadReceipt,
    query_surface: String,
}

impl NmtTopologyConstructionQueryReceipts {
    pub(crate) fn new(
        pattern: &NmtTopologyPattern,
        declaration: String,
    ) -> Result<Self, TopologyWorkloadDenial> {
        let query_surface = format!(".topology.nmt.construction.{}", pattern.query_key());
        let declaration_receipt = TopologyWorkload::declared(declaration)
            .from_query_declaration(query_surface.clone())?;
        Ok(Self {
            declaration_receipt,
            query_surface,
        })
    }

    pub(crate) fn map_denial(
        pattern: &NmtTopologyPattern,
        error: TopologyWorkloadDenial,
    ) -> NmtTopologyConstructionDenial {
        query_admission(pattern.clone(), error.human_reason())
    }

    pub(crate) fn declaration_receipt(&self) -> &TopologyWorkloadReceipt {
        &self.declaration_receipt
    }

    pub(crate) fn query_surface(&self) -> &str {
        &self.query_surface
    }
}
