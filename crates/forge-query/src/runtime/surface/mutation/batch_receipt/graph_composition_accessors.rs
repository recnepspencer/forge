use crate::runtime::{
    ForgeQueryGraphCompositionAssumptionSummary, ForgeQueryGraphCompositionEvidence,
    ForgeQueryGraphCompositionLifecycleOutcomes, ForgeQueryGraphCompositionLineageSummary,
    ForgeQueryGraphCompositionResolutionMap,
};

use super::ForgeQueryBatchWriteReceipt;

impl ForgeQueryBatchWriteReceipt {
    pub fn graph_composition_evidence(&self) -> Option<ForgeQueryGraphCompositionEvidence> {
        let lifecycle_outcomes =
            ForgeQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)?;
        ForgeQueryGraphCompositionEvidence::derive(
            &self.write_receipts,
            &self.graph_composition_breadth,
            &lifecycle_outcomes,
            &self.graph_composition_resolution_map,
            self.affected_live_view_ids.len(),
            self.affected_derived_view_ids.len(),
            self.considered_computed_view_count,
        )
    }

    pub fn graph_composition_assumption_summary(
        &self,
    ) -> Option<ForgeQueryGraphCompositionAssumptionSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        ForgeQueryGraphCompositionAssumptionSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lineage_summary(
        &self,
    ) -> Option<ForgeQueryGraphCompositionLineageSummary> {
        if self.graph_composition_program.is_empty() {
            return None;
        }
        ForgeQueryGraphCompositionLineageSummary::derive(&self.write_receipts)
    }

    pub fn graph_composition_lifecycle_outcomes(
        &self,
    ) -> Option<ForgeQueryGraphCompositionLifecycleOutcomes> {
        ForgeQueryGraphCompositionLifecycleOutcomes::derive(&self.graph_composition_program)
    }

    pub fn graph_composition_resolution_map(&self) -> &ForgeQueryGraphCompositionResolutionMap {
        &self.graph_composition_resolution_map
    }
}
