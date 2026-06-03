use std::collections::BTreeMap;

use super::material_catalog::ShippingSpec;
use super::{
    PricingDomainWorld, PricingMaterial, PricingProduct, ProductPriceBreakdown,
    ProductPricingAttribution,
};

impl PricingDomainWorld {
    pub(in crate::harness::tests) fn price_product(
        &self,
        product: &PricingProduct,
    ) -> ProductPriceBreakdown {
        self.price_product_with_scenario(product, BTreeMap::new(), BTreeMap::new())
    }

    pub(in crate::harness::tests) fn price_product_with_overrides(
        &self,
        product: &PricingProduct,
        overrides: BTreeMap<PricingMaterial, i64>,
    ) -> ProductPriceBreakdown {
        self.price_product_with_scenario(product, overrides, BTreeMap::new())
    }

    pub(in crate::harness::tests) fn price_product_with_scenario(
        &self,
        product: &PricingProduct,
        overrides: BTreeMap<PricingMaterial, i64>,
        family_tariff_bps: BTreeMap<String, i64>,
    ) -> ProductPriceBreakdown {
        let baseline_material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                self.baseline_material_price_microunits(requirement.material)
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();
        let material_cost_cents = product
            .materials
            .iter()
            .map(|requirement| {
                self.material_price_with_overrides(requirement.material, &overrides)
                    * requirement.quantity_milliunits
                    / 1_000_000
            })
            .sum::<i64>();

        let baseline_shipping_cost_cents = self
            .shipping_cost_cents_with_prices(&product.shipping, &self.baseline_prices_microunits);
        let shipping_cost_cents =
            self.shipping_cost_cents_with_overrides(&product.shipping, &overrides);
        let baseline_landed_cost_cents =
            baseline_material_cost_cents + baseline_shipping_cost_cents;
        let pre_policy_landed_cost_cents = material_cost_cents + shipping_cost_cents;
        let tariff_bps = family_tariff_bps
            .get(&product.family)
            .copied()
            .unwrap_or(0)
            .max(0);
        let policy_surcharge_cents = pre_policy_landed_cost_cents * tariff_bps / 10_000;
        let landed_cost_cents = pre_policy_landed_cost_cents + policy_surcharge_cents;
        let landed_cost_delta_cents = (landed_cost_cents - baseline_landed_cost_cents).abs();
        let margin_cents = landed_cost_cents * product.margin_bps / 10_000;
        let retail_price_cents = landed_cost_cents + margin_cents;
        let repricing_threshold_cents =
            (baseline_landed_cost_cents * product.tolerance_gate.repricing_threshold_bps) / 10_000;
        let margin_floor_breached = product.margin_bps < product.tolerance_gate.margin_floor_bps;
        let repricing_triggered = (repricing_threshold_cents > 0
            && landed_cost_delta_cents >= repricing_threshold_cents)
            || margin_floor_breached;

        ProductPriceBreakdown {
            sku: product.sku.clone(),
            family: product.family.clone(),
            material_cost_cents,
            shipping_cost_cents,
            policy_surcharge_cents,
            baseline_landed_cost_cents,
            landed_cost_cents,
            landed_cost_delta_cents,
            margin_cents,
            retail_price_cents,
            repricing_threshold_cents,
            repricing_triggered,
            margin_floor_breached,
        }
    }

    pub(in crate::harness::tests) fn price_matrix(&self) -> Vec<ProductPriceBreakdown> {
        self.products
            .iter()
            .map(|product| self.price_product(product))
            .collect()
    }

    pub(in crate::harness::tests) fn price_matrix_with_overrides<I>(
        &self,
        overrides: I,
    ) -> Vec<ProductPriceBreakdown>
    where
        I: IntoIterator<Item = (PricingMaterial, i64)>,
    {
        let override_map = overrides.into_iter().collect::<BTreeMap<_, _>>();
        self.products
            .iter()
            .map(|product| self.price_product_with_overrides(product, override_map.clone()))
            .collect()
    }

    pub(in crate::harness::tests) fn price_matrix_with_scenario(
        &self,
        overrides: BTreeMap<PricingMaterial, i64>,
        family_tariff_bps: BTreeMap<String, i64>,
    ) -> Vec<ProductPriceBreakdown> {
        self.products
            .iter()
            .map(|product| {
                self.price_product_with_scenario(
                    product,
                    overrides.clone(),
                    family_tariff_bps.clone(),
                )
            })
            .collect()
    }

    pub(in crate::harness::tests) fn explain_product_price(
        &self,
        sku: &str,
    ) -> ProductPricingAttribution {
        let product = self
            .products
            .iter()
            .find(|product| product.sku == sku)
            .expect("product sku should exist in pricing domain");
        let breakdown = self.price_product(product);
        let material_contributions_cents = product
            .materials
            .iter()
            .map(|requirement| {
                (
                    requirement.material,
                    self.current_material_price_microunits(requirement.material)
                        * requirement.quantity_milliunits
                        / 1_000_000,
                )
            })
            .collect::<Vec<_>>();
        let override_map = BTreeMap::new();
        let (fuel_shipping_component_cents, packaging_surcharge_cents) =
            self.shipping_components_cents_with_overrides(&product.shipping, &override_map);

        ProductPricingAttribution {
            sku: breakdown.sku,
            retail_price_cents: breakdown.retail_price_cents,
            baseline_landed_cost_cents: breakdown.baseline_landed_cost_cents,
            landed_cost_cents: breakdown.landed_cost_cents,
            landed_cost_delta_cents: breakdown.landed_cost_delta_cents,
            material_cost_cents: breakdown.material_cost_cents,
            shipping_cost_cents: breakdown.shipping_cost_cents,
            margin_cents: breakdown.margin_cents,
            repricing_threshold_cents: breakdown.repricing_threshold_cents,
            repricing_triggered: breakdown.repricing_triggered,
            margin_floor_breached: breakdown.margin_floor_breached,
            fuel_shipping_component_cents,
            packaging_surcharge_cents,
            material_contributions_cents,
        }
    }

    fn shipping_cost_cents_with_overrides(
        &self,
        shipping: &ShippingSpec,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        self.shipping_cost_cents_with_prices(shipping, overrides)
    }

    fn shipping_cost_cents_with_prices(
        &self,
        shipping: &ShippingSpec,
        prices: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        let (fuel_component_cents, packaging_surcharge_cents) =
            self.shipping_components_cents_with_prices(shipping, prices);
        shipping.base_shipping_cents + fuel_component_cents + packaging_surcharge_cents
    }

    fn shipping_components_cents_with_overrides(
        &self,
        shipping: &ShippingSpec,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> (i64, i64) {
        self.shipping_components_cents_with_prices(shipping, overrides)
    }

    fn shipping_components_cents_with_prices(
        &self,
        shipping: &ShippingSpec,
        prices: &BTreeMap<PricingMaterial, i64>,
    ) -> (i64, i64) {
        let fuel_price_microunits = prices
            .get(&PricingMaterial::Fuel)
            .copied()
            .unwrap_or_else(|| self.current_material_price_microunits(PricingMaterial::Fuel));
        let weight_kg = shipping.shipment_weight_grams.max(0) / 1_000;
        let fuel_component_cents = fuel_price_microunits
            * shipping.fuel_burn_microliters_per_kg_km
            * weight_kg
            * shipping.route_distance_km
            / 1_000_000_000;
        let packaging_surcharge_cents = shipping.packaging_volume_cc / 25_000;
        (fuel_component_cents, packaging_surcharge_cents)
    }

    fn material_price_with_overrides(
        &self,
        material: PricingMaterial,
        overrides: &BTreeMap<PricingMaterial, i64>,
    ) -> i64 {
        overrides
            .get(&material)
            .copied()
            .unwrap_or_else(|| self.current_material_price_microunits(material))
    }
}
