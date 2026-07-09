use crate::runtime::{
    WorthQueryGraphCompositionAssumptionSummary, WorthQueryGraphCompositionEvidence,
    WorthQueryGraphCompositionLifecycleOutcomes, WorthQueryGraphCompositionLineageSummary,
    WorthQueryGraphCompositionResolutionMap,
};

use super::WorthQueryBatchWriteReceipt;

impl WorthQueryBatchWriteReceipt {
    pub fn graph_composition_evidence(&self) -> Option<WorthQueryGraphCompositionEvidence> {
        let lifecycle_outcomes =
            WorthQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)?;
        WorthQueryGraphCompositionEvidence::derive(
            &self.write_receipts,
            &self.graph_composition_breadth,
            &lifecycle_outcomes,
            &self.graph_composition_resolution_map,
            self.affected_live_view_targets.len(),
            self.affected_derived_view_targets.len(),
            self.considered_computed_view_count,
        )
    }

    pub fn graph_composition_assumption_summary(
        &self,
    ) -> Option<WorthQueryGraphCompositionAssumptionSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        WorthQueryGraphCompositionAssumptionSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lineage_summary(
        &self,
    ) -> Option<WorthQueryGraphCompositionLineageSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        WorthQueryGraphCompositionLineageSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lifecycle_outcomes(
        &self,
    ) -> Option<WorthQueryGraphCompositionLifecycleOutcomes> {
        WorthQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)
    }

    pub fn graph_composition_resolution_map(&self) -> &WorthQueryGraphCompositionResolutionMap {
        &self.graph_composition_resolution_map
    }
}
