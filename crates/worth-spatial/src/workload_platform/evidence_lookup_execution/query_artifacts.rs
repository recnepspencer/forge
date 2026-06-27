use std::collections::BTreeMap;

use forge_query::facade::ProjectionConsumptionReceipt;

use crate::workload_platform::evidence_lookup_family_catalog::EvidenceLookupProjectionFactFamily;

pub(crate) struct EvidenceLookupExecutionQueryArtifacts<'a> {
    projection_receipts: BTreeMap<String, ProjectionConsumptionExecutionArtifact<'a>>,
}

pub(crate) struct ProjectionConsumptionExecutionArtifact<'a> {
    fact_family: EvidenceLookupProjectionFactFamily,
    receipt: &'a ProjectionConsumptionReceipt,
}

impl<'a> EvidenceLookupExecutionQueryArtifacts<'a> {
    pub(crate) fn new() -> Self {
        Self {
            projection_receipts: BTreeMap::new(),
        }
    }

    pub(crate) fn with_projection_consumption_receipt(
        mut self,
        family_identity: impl Into<String>,
        fact_family: EvidenceLookupProjectionFactFamily,
        receipt: &'a ProjectionConsumptionReceipt,
    ) -> Self {
        self.projection_receipts.insert(
            family_identity.into(),
            ProjectionConsumptionExecutionArtifact {
                fact_family,
                receipt,
            },
        );
        self
    }

    pub(crate) fn projection_receipt(
        &self,
        family_identity: &str,
    ) -> Option<&ProjectionConsumptionExecutionArtifact<'a>> {
        self.projection_receipts.get(family_identity)
    }

    pub(crate) fn projection_receipt_families(&self) -> impl Iterator<Item = &String> {
        self.projection_receipts.keys()
    }

    pub(crate) fn projection_receipt_count(&self) -> usize {
        self.projection_receipts.len()
    }
}

impl<'a> ProjectionConsumptionExecutionArtifact<'a> {
    pub(crate) const fn fact_family(&self) -> EvidenceLookupProjectionFactFamily {
        self.fact_family
    }

    pub(crate) const fn receipt(&self) -> &'a ProjectionConsumptionReceipt {
        self.receipt
    }
}
