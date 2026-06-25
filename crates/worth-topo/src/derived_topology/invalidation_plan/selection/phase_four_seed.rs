use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationPhaseFourSeed {
    selected_plan_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    selected_product_count: usize,
    denied_product_count: usize,
    unaffected_product_count: usize,
    seed_digest: String,
}

impl DerivedInvalidationPhaseFourSeed {
    pub(super) fn from_selected_plan(
        selected_plan_digest: &str,
        touched_closure_digest: &str,
        query_support_digest: &str,
        legality_support_digest: &str,
        selected_product_count: usize,
        denied_product_count: usize,
        unaffected_product_count: usize,
    ) -> Self {
        let seed_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-phase-four-seed:v1".to_string(),
            format!("selected-plan:{selected_plan_digest}"),
            format!("touched-closure:{touched_closure_digest}"),
            format!("query-support:{query_support_digest}"),
            format!("legality-support:{legality_support_digest}"),
            format!("selected:{selected_product_count}"),
            format!("denied:{denied_product_count}"),
            format!("unaffected:{unaffected_product_count}"),
        ]);
        Self {
            selected_plan_digest: selected_plan_digest.to_string(),
            touched_closure_digest: touched_closure_digest.to_string(),
            query_support_digest: query_support_digest.to_string(),
            legality_support_digest: legality_support_digest.to_string(),
            selected_product_count,
            denied_product_count,
            unaffected_product_count,
            seed_digest,
        }
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn touched_closure_digest(&self) -> &str {
        &self.touched_closure_digest
    }

    pub fn query_support_digest(&self) -> &str {
        &self.query_support_digest
    }

    pub fn legality_support_digest(&self) -> &str {
        &self.legality_support_digest
    }

    pub const fn selected_product_count(&self) -> usize {
        self.selected_product_count
    }

    pub const fn denied_product_count(&self) -> usize {
        self.denied_product_count
    }

    pub const fn unaffected_product_count(&self) -> usize {
        self.unaffected_product_count
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
