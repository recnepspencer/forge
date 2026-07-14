use worth_harness::facade::{FeedStreamEventKind, FeedVolatilityRegime};

use super::material_catalog::PricingMaterial;
use crate::facade::{TruthBranchIdentity, TruthCommitIdentity, TruthSnapshotIdentity};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct ProductPriceBreakdown {
    pub(in crate::harness::tests) sku: String,
    pub(in crate::harness::tests) family: String,
    pub(in crate::harness::tests) material_cost_cents: i64,
    pub(in crate::harness::tests) shipping_cost_cents: i64,
    pub(in crate::harness::tests) policy_surcharge_cents: i64,
    pub(in crate::harness::tests) baseline_landed_cost_cents: i64,
    pub(in crate::harness::tests) landed_cost_cents: i64,
    pub(in crate::harness::tests) landed_cost_delta_cents: i64,
    pub(in crate::harness::tests) margin_cents: i64,
    pub(in crate::harness::tests) retail_price_cents: i64,
    pub(in crate::harness::tests) repricing_threshold_cents: i64,
    pub(in crate::harness::tests) repricing_triggered: bool,
    pub(in crate::harness::tests) margin_floor_breached: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct MaterialTick {
    pub(in crate::harness::tests) material: PricingMaterial,
    pub(in crate::harness::tests) event_kind: FeedStreamEventKind,
    pub(in crate::harness::tests) value_microunits: i64,
    pub(in crate::harness::tests) attribution: MaterialPriceAttribution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct MaterialTickWave {
    pub(in crate::harness::tests) sequence: u64,
    pub(in crate::harness::tests) industrial_factor_microunits: i64,
    pub(in crate::harness::tests) energy_factor_microunits: i64,
    pub(in crate::harness::tests) changed_materials: Vec<MaterialTick>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct MaterialPriceAttribution {
    pub(in crate::harness::tests) material: PricingMaterial,
    pub(in crate::harness::tests) event_kind: FeedStreamEventKind,
    pub(in crate::harness::tests) regime: FeedVolatilityRegime,
    pub(in crate::harness::tests) previous_value_microunits: i64,
    pub(in crate::harness::tests) current_value_microunits: i64,
    pub(in crate::harness::tests) delta_microunits: i64,
    pub(in crate::harness::tests) external_factor_microunits: i64,
    pub(in crate::harness::tests) factor_delta_microunits: i64,
    pub(in crate::harness::tests) trend_delta_microunits: i64,
    pub(in crate::harness::tests) mean_reversion_delta_microunits: i64,
    pub(in crate::harness::tests) idiosyncratic_noise_microunits: i64,
    pub(in crate::harness::tests) jump_delta_microunits: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct ProductPricingAttribution {
    pub(in crate::harness::tests) sku: String,
    pub(in crate::harness::tests) retail_price_cents: i64,
    pub(in crate::harness::tests) baseline_landed_cost_cents: i64,
    pub(in crate::harness::tests) landed_cost_cents: i64,
    pub(in crate::harness::tests) landed_cost_delta_cents: i64,
    pub(in crate::harness::tests) material_cost_cents: i64,
    pub(in crate::harness::tests) shipping_cost_cents: i64,
    pub(in crate::harness::tests) margin_cents: i64,
    pub(in crate::harness::tests) repricing_threshold_cents: i64,
    pub(in crate::harness::tests) repricing_triggered: bool,
    pub(in crate::harness::tests) margin_floor_breached: bool,
    pub(in crate::harness::tests) fuel_shipping_component_cents: i64,
    pub(in crate::harness::tests) packaging_surcharge_cents: i64,
    pub(in crate::harness::tests) material_contributions_cents: Vec<(PricingMaterial, i64)>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::harness::tests) struct PricingCommitAttribution {
    pub(in crate::harness::tests) commit_identity: TruthCommitIdentity,
    pub(in crate::harness::tests) snapshot_identity: TruthSnapshotIdentity,
    pub(in crate::harness::tests) branch_identity: TruthBranchIdentity,
    pub(in crate::harness::tests) material: PricingMaterial,
    pub(in crate::harness::tests) material_attribution: MaterialPriceAttribution,
    pub(in crate::harness::tests) shock_delta_microunits: i64,
    pub(in crate::harness::tests) shock_multiplier_per_mille: i64,
    pub(in crate::harness::tests) representative_product: ProductPricingAttribution,
}
