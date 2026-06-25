use std::collections::BTreeMap;

use forge_query::facade::{
    ForgeQueryGraphObligationIndex, ForgeQueryGraphObligationSelection,
    ForgeQueryGraphObligationSupportStatus,
};

use crate::topology_operators::TopologyTouchedGraphCounters;
use crate::validator_invariant_catalog::selection_from_touched_closure::{
    WorthTopologyLegalitySelectionCounters, WorthTopologyLegalitySelectionDenial,
    WorthTopologyLegalitySelectionPhaseFourSeed, WorthTopologySelectedLegalityObligationRow,
    WorthTopologyValidatorRoutingClosure,
};
use crate::validator_invariant_catalog::{
    WorthTopologyLegalityCatalog, WorthTopologyLegalityCatalogError,
    WorthTopologyQueryGraphObligationRegistrationProjectionRow,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthTopologySelectedLegalityObligationPlan {
    catalog_digest: String,
    query_catalog_digest: String,
    routing_closure_digest: String,
    query_selection_digest: String,
    selected_obligation_rows: Vec<WorthTopologySelectedLegalityObligationRow>,
    denial_rows: Vec<WorthTopologyLegalitySelectionDenial>,
    counters: WorthTopologyLegalitySelectionCounters,
    phase_four_seed: WorthTopologyLegalitySelectionPhaseFourSeed,
    selected_plan_digest: String,
}

impl WorthTopologySelectedLegalityObligationPlan {
    pub fn select_from_catalog_and_routing_closure(
        catalog: &WorthTopologyLegalityCatalog,
        routing_closure: &WorthTopologyValidatorRoutingClosure,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let index = ForgeQueryGraphObligationIndex::from_catalog(
            catalog.query_projection().query_catalog(),
        );
        let query_selection = index.select_for_touch(
            routing_closure.touch_descriptor(),
            routing_closure.query_operating_world_descriptor(),
        );
        Self::from_query_selection(catalog, routing_closure, query_selection)
    }

    fn from_query_selection(
        catalog: &WorthTopologyLegalityCatalog,
        routing_closure: &WorthTopologyValidatorRoutingClosure,
        query_selection: ForgeQueryGraphObligationSelection,
    ) -> Result<Self, WorthTopologyLegalityCatalogError> {
        let projection_rows_by_registration_digest =
            projection_rows_by_registration_digest(catalog);
        let mut selected_obligation_rows = Vec::new();
        let mut denial_rows = Vec::new();
        for registration in query_selection.matched_registrations() {
            let projection_row = projection_rows_by_registration_digest
                .get(registration.registration_digest())
                .ok_or_else(|| {
                    WorthTopologyLegalityCatalogError::MissingQueryProjectionRow(
                        registration.registration_digest().to_string(),
                    )
                })?;
            if !routing_closure.receipt_context_present() {
                denial_rows.push(
                    WorthTopologyLegalitySelectionDenial::missing_access_receipt(
                        registration,
                        routing_closure.milestone_eight_seed_digest(),
                    ),
                );
                continue;
            }
            if let Some(max_state_scope) =
                budget_exceeded_state_scope(registration, routing_closure.counters())
            {
                denial_rows.push(WorthTopologyLegalitySelectionDenial::budget_exceeded(
                    registration,
                    touched_state_scope(routing_closure.counters()),
                    max_state_scope,
                ));
                continue;
            }
            if registration.support_posture().status()
                == ForgeQueryGraphObligationSupportStatus::Supported
            {
                selected_obligation_rows.push(
                    WorthTopologySelectedLegalityObligationRow::from_registration(
                        registration,
                        projection_row,
                    ),
                );
            } else {
                denial_rows.push(
                    WorthTopologyLegalitySelectionDenial::support_posture_denied(registration),
                );
            }
        }
        let counters = WorthTopologyLegalitySelectionCounters::from_selection(
            routing_closure.counters(),
            query_selection.counters(),
            selected_obligation_rows.len(),
            &denial_rows,
        );
        let selected_plan_digest = selected_plan_digest(
            catalog.catalog_digest(),
            catalog.query_projection().query_catalog().catalog_digest(),
            routing_closure.closure_digest(),
            query_selection.selection_digest(),
            &selected_obligation_rows,
            &denial_rows,
            counters.counters_digest(),
        );
        let phase_four_seed = WorthTopologyLegalitySelectionPhaseFourSeed::from_selected_plan(
            &selected_plan_digest,
            routing_closure.closure_digest(),
            query_selection.selection_digest(),
            selected_obligation_rows.len(),
            denial_rows.len(),
        );
        Ok(Self {
            catalog_digest: catalog.catalog_digest().to_string(),
            query_catalog_digest: catalog
                .query_projection()
                .query_catalog()
                .catalog_digest()
                .to_string(),
            routing_closure_digest: routing_closure.closure_digest().to_string(),
            query_selection_digest: query_selection.selection_digest().to_string(),
            selected_obligation_rows,
            denial_rows,
            counters,
            phase_four_seed,
            selected_plan_digest,
        })
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn query_catalog_digest(&self) -> &str {
        &self.query_catalog_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn query_selection_digest(&self) -> &str {
        &self.query_selection_digest
    }

    pub fn selected_obligation_rows(&self) -> &[WorthTopologySelectedLegalityObligationRow] {
        &self.selected_obligation_rows
    }

    pub fn denial_rows(&self) -> &[WorthTopologyLegalitySelectionDenial] {
        &self.denial_rows
    }

    pub const fn counters(&self) -> &WorthTopologyLegalitySelectionCounters {
        &self.counters
    }

    pub const fn phase_four_seed(&self) -> &WorthTopologyLegalitySelectionPhaseFourSeed {
        &self.phase_four_seed
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub const fn claims_enforcement_receipts(&self) -> bool {
        false
    }
}

fn projection_rows_by_registration_digest(
    catalog: &WorthTopologyLegalityCatalog,
) -> BTreeMap<&str, &WorthTopologyQueryGraphObligationRegistrationProjectionRow> {
    catalog
        .query_projection()
        .registration_projection_rows()
        .iter()
        .map(|row| (row.registration_digest(), row))
        .collect()
}

fn budget_exceeded_state_scope(
    registration: &forge_query::facade::ForgeQueryGraphObligationRegistration,
    touched_counters: TopologyTouchedGraphCounters,
) -> Option<usize> {
    let max_state_scope = registration.execution_budget().max_state_scope()?;
    (touched_state_scope(touched_counters) > max_state_scope).then_some(max_state_scope)
}

fn touched_state_scope(touched_counters: TopologyTouchedGraphCounters) -> usize {
    touched_counters.entity_count()
        + touched_counters.relation_count()
        + touched_counters.relation_kind_count()
        + touched_counters.touched_aspect_count()
        + touched_counters.topology_scope_count()
}

fn selected_plan_digest(
    catalog_digest: &str,
    query_catalog_digest: &str,
    routing_closure_digest: &str,
    query_selection_digest: &str,
    selected_rows: &[WorthTopologySelectedLegalityObligationRow],
    denial_rows: &[WorthTopologyLegalitySelectionDenial],
    counters_digest: &str,
) -> String {
    let mut parts = vec![
        "worth-topo-selected-legality-obligation-plan-v1".to_string(),
        format!("catalog:{catalog_digest}"),
        format!("query-catalog:{query_catalog_digest}"),
        format!("routing-closure:{routing_closure_digest}"),
        format!("query-selection:{query_selection_digest}"),
        format!("counters:{counters_digest}"),
    ];
    parts.extend(
        selected_rows
            .iter()
            .map(|row| format!("selected:{}", row.row_digest())),
    );
    parts.extend(
        denial_rows
            .iter()
            .map(|row| format!("denial:{}", row.denial_digest())),
    );
    parts.join("|")
}
