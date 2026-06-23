use std::collections::BTreeMap;

use crate::runtime::{
    WorthUiQueryBindingComparison, WorthUiQueryLiveRebindOutcome, WorthUiQueryLiveRebindPlan,
    WorthUiQueryRuntimeFactLoweringReceipt, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

use super::{
    WorthUiSemanticChangedSliceRow, WorthUiSemanticChangedSliceSet,
    WorthUiSemanticSliceLoweringCause,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQuerySemanticSliceProjection {
    slices: WorthUiSemanticChangedSliceSet,
}

impl WorthUiQuerySemanticSliceProjection {
    pub fn project(
        inventory: &WorthUiSemanticSliceInventory,
        comparison: &WorthUiQueryBindingComparison,
        live_rebind_plan: &WorthUiQueryLiveRebindPlan,
        lowering_receipt: &WorthUiQueryRuntimeFactLoweringReceipt,
    ) -> Self {
        let mut rows = BTreeMap::new();
        for lowered_row in WorthUiSemanticChangedSliceSet::lower_runtime_change(
            inventory,
            &crate::runtime::WorthUiAdmittedRuntimeChangeEvidence::admit(
                crate::runtime::WorthUiClassifiedRuntimeChange::from_query_lowering_receipt(
                    crate::runtime::WorthUiRuntimeInstanceWitness::from_raw(0),
                    lowering_receipt,
                ),
                crate::runtime::WorthUiRuntimeInstanceWitness::from_raw(0),
            )
            .expect("query lowering receipt with coherent witness should admit"),
        )
        .rows()
        {
            rows.insert(lowered_row.descriptor().id(), *lowered_row);
        }

        if !comparison.entries().is_empty() {
            project_gap_slice(
                inventory,
                WorthUiSemanticSliceId::QueryBindingIdentity,
                &mut rows,
            );
        }

        for entry in live_rebind_plan.entries() {
            project_gap_slice(
                inventory,
                WorthUiSemanticSliceId::QueryLiveViewBinding,
                &mut rows,
            );
            match entry.outcome() {
                WorthUiQueryLiveRebindOutcome::Preserve(_) => project_gap_slice(
                    inventory,
                    WorthUiSemanticSliceId::QueryBindingPreservationPosture,
                    &mut rows,
                ),
                WorthUiQueryLiveRebindOutcome::Rebind(_) => project_gap_slice(
                    inventory,
                    WorthUiSemanticSliceId::QueryBindingRebindPosture,
                    &mut rows,
                ),
                WorthUiQueryLiveRebindOutcome::Retire(_) => project_gap_slice(
                    inventory,
                    WorthUiSemanticSliceId::QueryBindingRetirementPosture,
                    &mut rows,
                ),
                WorthUiQueryLiveRebindOutcome::Deny(_) => {}
            }
        }

        Self {
            slices: WorthUiSemanticChangedSliceSet::from_rows(rows.into_values().collect()),
        }
    }

    pub fn slices(&self) -> &WorthUiSemanticChangedSliceSet {
        &self.slices
    }
}

fn project_gap_slice(
    inventory: &WorthUiSemanticSliceInventory,
    slice_id: WorthUiSemanticSliceId,
    rows: &mut BTreeMap<WorthUiSemanticSliceId, WorthUiSemanticChangedSliceRow>,
) {
    let descriptor = inventory
        .slice(slice_id)
        .expect("Query semantic slice is registered");
    rows.entry(slice_id).or_insert_with(|| {
        WorthUiSemanticChangedSliceRow::new(
            descriptor,
            WorthUiSemanticSliceLoweringCause::QueryOwnedPostureProjection,
        )
    });
}
