use forge_query::facade::ProjectionConsumptionReceipt;

use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily;
use crate::workload_platform::evidence_lookup_index_product::EvidenceLookupIndexProduct;
use crate::workload_platform::evidence_lookup_plan_selection::EvidenceLookupSelectedPlan;

use super::query_artifacts::EvidenceLookupExecutionQueryArtifacts;

pub struct EvidenceLookupExecutionRequest<'a> {
    selected_plan: &'a EvidenceLookupSelectedPlan,
    index_product: &'a EvidenceLookupIndexProduct,
    query_artifacts: EvidenceLookupExecutionQueryArtifacts<'a>,
}

impl<'a> EvidenceLookupExecutionRequest<'a> {
    pub fn new(
        selected_plan: &'a EvidenceLookupSelectedPlan,
        index_product: &'a EvidenceLookupIndexProduct,
    ) -> Self {
        Self {
            selected_plan,
            index_product,
            query_artifacts: EvidenceLookupExecutionQueryArtifacts::new(),
        }
    }

    pub fn with_projection_consumption_receipt(
        mut self,
        family_identity: impl Into<String>,
        fact_family: EvidenceLookupProjectionFactFamily,
        receipt: &'a ProjectionConsumptionReceipt,
    ) -> Self {
        self.query_artifacts = self.query_artifacts.with_projection_consumption_receipt(
            family_identity,
            fact_family,
            receipt,
        );
        self
    }

    pub(crate) const fn selected_plan(&self) -> &'a EvidenceLookupSelectedPlan {
        self.selected_plan
    }

    pub(crate) const fn index_product(&self) -> &'a EvidenceLookupIndexProduct {
        self.index_product
    }

    pub(crate) const fn query_artifacts(&self) -> &EvidenceLookupExecutionQueryArtifacts<'a> {
        &self.query_artifacts
    }
}
