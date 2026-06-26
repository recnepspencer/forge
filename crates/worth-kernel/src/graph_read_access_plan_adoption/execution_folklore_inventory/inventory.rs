use crate::graph_read_access_declarations::WorthGraphReadAccessDeclarationMilestoneEightSeed;

use super::inventory_counters::WorthGraphReadAccessPlanAdoptionInventoryCounters;
use super::inventory_row::WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow;
use super::stable_digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory {
    rows: Vec<WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow>,
    counters: WorthGraphReadAccessPlanAdoptionInventoryCounters,
    inventory_digest: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError {
    MissingInventoryRows,
}

impl WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventory {
    pub(in crate::graph_read_access_plan_adoption) fn from_milestone_eight_seed(
        seed: &WorthGraphReadAccessDeclarationMilestoneEightSeed,
    ) -> Result<Self, WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError> {
        let mut rows = seed
            .read_family_identities()
            .iter()
            .map(WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow::from_read_family_identity)
            .collect::<Vec<_>>();
        rows.extend(
            seed.requirement_row_evidence()
                .iter()
                .map(WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow::from_requirement_row),
        );
        rows.extend(
            seed.deletion_ledger_report()
                .rows()
                .iter()
                .map(WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow::from_deletion_row),
        );
        rows.extend(
            seed.admission_capability_gaps()
                .iter()
                .map(WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow::from_admission_gap),
        );
        rows.extend(
            seed.carried_requirement_derivation_gaps()
                .iter()
                .map(WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow::from_requirement_gap),
        );
        if rows.is_empty() {
            return Err(
                WorthGraphReadAccessPlanAdoptionExecutionFolkloreInventoryError::MissingInventoryRows,
            );
        }
        let counters = WorthGraphReadAccessPlanAdoptionInventoryCounters::from_rows(&rows);
        let mut digest_parts = vec![
            "worth_graph_read_access_plan_adoption_execution_folklore_inventory_v1".to_string(),
            format!("row_count:{}", counters.row_count()),
            format!("migrate_count:{}", counters.migrate_count()),
            format!("delete_count:{}", counters.delete_count()),
            format!("cap_count:{}", counters.cap_count()),
            format!("query_gap_count:{}", counters.query_gap_count()),
        ];
        digest_parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
        Ok(Self {
            rows,
            counters,
            inventory_digest: stable_digest(&digest_parts),
        })
    }

    pub fn rows(&self) -> &[WorthGraphReadAccessPlanAdoptionExecutionFolkloreRow] {
        &self.rows
    }

    pub(in crate::graph_read_access_plan_adoption) const fn counters(
        &self,
    ) -> &WorthGraphReadAccessPlanAdoptionInventoryCounters {
        &self.counters
    }

    pub fn inventory_digest(&self) -> &str {
        &self.inventory_digest
    }
}
