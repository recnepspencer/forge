#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingShockSimulationIterationTrace {
    pub(in crate::harness::tests) material: String,
    pub(in crate::harness::tests) branch_identity: String,
    pub(in crate::harness::tests) iteration_index: usize,
    pub(in crate::harness::tests) regime: String,
    pub(in crate::harness::tests) event_kind: String,
    pub(in crate::harness::tests) shock_multiplier_per_mille: i64,
    pub(in crate::harness::tests) baseline_total_retail_cents: i64,
    pub(in crate::harness::tests) shocked_total_retail_cents: i64,
    pub(in crate::harness::tests) total_retail_delta_cents: i64,
    pub(in crate::harness::tests) shipping_delta_cents: i64,
    pub(in crate::harness::tests) material_delta_cents: i64,
    pub(in crate::harness::tests) margin_floor_breach_count: usize,
    pub(in crate::harness::tests) repricing_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingShockSimulationMaterialSummary {
    pub(in crate::harness::tests) material: String,
    pub(in crate::harness::tests) branch_count: usize,
    pub(in crate::harness::tests) iterations_per_branch: usize,
    pub(in crate::harness::tests) mean_total_retail_delta_cents: i64,
    pub(in crate::harness::tests) mean_shipping_delta_cents: i64,
    pub(in crate::harness::tests) mean_material_delta_cents: i64,
    pub(in crate::harness::tests) mean_margin_floor_breach_count: i64,
    pub(in crate::harness::tests) mean_repricing_count: i64,
    pub(in crate::harness::tests) worst_branch_identity: String,
    pub(in crate::harness::tests) worst_branch_mean_total_delta_cents: i64,
    pub(in crate::harness::tests) damage_score: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingShockRankedMaterialDamage {
    material: String,
}

impl PricingShockRankedMaterialDamage {
    pub(in crate::harness::tests) fn from_material_summary(
        summary: &PricingShockSimulationMaterialSummary,
    ) -> Self {
        Self {
            material: summary.material.clone(),
        }
    }

    pub(in crate::harness::tests) fn material(&self) -> &str {
        self.material.as_str()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingShockRankedMaterialDamageSet {
    entries: Vec<PricingShockRankedMaterialDamage>,
}

impl PricingShockRankedMaterialDamageSet {
    pub(in crate::harness::tests) fn from_ranked_material_summaries<'a>(
        summaries: impl IntoIterator<Item = &'a PricingShockSimulationMaterialSummary>,
    ) -> Self {
        Self {
            entries: summaries
                .into_iter()
                .map(PricingShockRankedMaterialDamage::from_material_summary)
                .collect(),
        }
    }

    pub(in crate::harness::tests) fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub(in crate::harness::tests) fn first_material(&self) -> Option<&str> {
        self.entries.first().map(|entry| entry.material())
    }

    pub(in crate::harness::tests) fn material_names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|entry| entry.material())
    }

    pub(in crate::harness::tests) fn canonical_material_list(&self) -> String {
        self.material_names().collect::<Vec<_>>().join(",")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingShockSimulationSuite {
    pub(in crate::harness::tests) branch_count: usize,
    pub(in crate::harness::tests) iterations_per_branch: usize,
    pub(in crate::harness::tests) material_summaries: Vec<PricingShockSimulationMaterialSummary>,
    pub(in crate::harness::tests) ranked_materials_by_damage: PricingShockRankedMaterialDamageSet,
    pub(in crate::harness::tests) iteration_traces: Vec<PricingShockSimulationIterationTrace>,
}
