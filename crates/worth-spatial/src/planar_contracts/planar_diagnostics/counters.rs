#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticCounters {
    source_receipts_inspected: usize,
    topology_surfaces_inspected: usize,
    causal_references_resolved: usize,
    locality_rows_emitted: usize,
    denied_evidence_rows: usize,
}

impl PlanarDiagnosticCounters {
    pub(crate) fn certified(
        source_receipts_inspected: usize,
        topology_surfaces_inspected: usize,
        causal_references_resolved: usize,
        locality_rows_emitted: usize,
        denied_evidence_rows: usize,
    ) -> Self {
        Self {
            source_receipts_inspected,
            topology_surfaces_inspected,
            causal_references_resolved,
            locality_rows_emitted,
            denied_evidence_rows,
        }
    }

    pub fn source_receipts_inspected(self) -> usize {
        self.source_receipts_inspected
    }

    pub fn topology_surfaces_inspected(self) -> usize {
        self.topology_surfaces_inspected
    }

    pub fn causal_references_resolved(self) -> usize {
        self.causal_references_resolved
    }

    pub fn locality_rows_emitted(self) -> usize {
        self.locality_rows_emitted
    }

    pub fn denied_evidence_rows(self) -> usize {
        self.denied_evidence_rows
    }
}
