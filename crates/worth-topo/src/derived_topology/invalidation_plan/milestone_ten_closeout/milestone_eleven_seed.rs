use serde::Serialize;

use super::counters::DerivedInvalidationMilestoneTenCounters;
use super::performance_proof::DerivedInvalidationMilestoneTenPerformanceProof;
use super::product_summary::DerivedInvalidationMilestoneTenProductSummaryReport;
use crate::derived_topology::invalidation_plan::catalog::DerivedTopologyProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::deletion_closeout::DerivedInvalidationDeletionCloseout;
use crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutionReceipt;
use crate::derived_topology::invalidation_plan::selection::DerivedInvalidationSelectedPlan;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum DerivedInvalidationMilestoneElevenLookupReadiness {
    TopologyDerivedReceiptsReadySpatialEvidenceNotBound,
}

impl DerivedInvalidationMilestoneElevenLookupReadiness {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TopologyDerivedReceiptsReadySpatialEvidenceNotBound => {
                "topology_derived_receipts_ready_spatial_evidence_not_bound"
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneElevenSeed {
    milestone_ten_closeout_digest: String,
    selected_plan_digest: String,
    execution_receipt_digest: String,
    touched_closure_digest: String,
    query_support_digest: String,
    legality_support_digest: String,
    product_summary_digest: String,
    performance_proof_digest: String,
    deletion_audit_digest: String,
    counters_digest: String,
    lookup_readiness: DerivedInvalidationMilestoneElevenLookupReadiness,
    topology_derived_product_receipts: Vec<DerivedInvalidationMilestoneElevenProductReceiptRef>,
    seed_digest: String,
}

impl DerivedInvalidationMilestoneElevenSeed {
    pub(crate) fn from_closeout_parts(
        milestone_ten_closeout_digest: &str,
        selected_plan: &DerivedInvalidationSelectedPlan,
        execution_receipt: &DerivedInvalidationExecutionReceipt,
        deletion_closeout: &DerivedInvalidationDeletionCloseout,
        product_summary: &DerivedInvalidationMilestoneTenProductSummaryReport,
        performance_proof: &DerivedInvalidationMilestoneTenPerformanceProof,
        counters: &DerivedInvalidationMilestoneTenCounters,
    ) -> Self {
        let lookup_readiness =
            DerivedInvalidationMilestoneElevenLookupReadiness::TopologyDerivedReceiptsReadySpatialEvidenceNotBound;
        let topology_derived_product_receipts =
            product_receipt_refs_from_execution_receipt(execution_receipt);
        let seed_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-milestone-eleven-seed:v1".to_string(),
            format!("milestone-ten:{milestone_ten_closeout_digest}"),
            format!("selected-plan:{}", selected_plan.selected_plan_digest()),
            format!("execution:{}", execution_receipt.execution_receipt_digest()),
            format!("touched-closure:{}", selected_plan.touched_closure_digest()),
            format!("query-support:{}", selected_plan.query_support_digest()),
            format!(
                "legality-support:{}",
                selected_plan.legality_support_digest()
            ),
            format!("product-summary:{}", product_summary.report_digest()),
            format!("performance:{}", performance_proof.proof_digest()),
            format!(
                "deletion-audit:{}",
                deletion_closeout.deletion_audit().audit_digest()
            ),
            format!("counters:{}", counters.counters_digest()),
            format!("lookup-readiness:{}", lookup_readiness.as_str()),
            format!(
                "topology-derived-product-receipts:{}",
                topology_derived_product_receipts_digest(&topology_derived_product_receipts)
            ),
        ]);
        Self {
            milestone_ten_closeout_digest: milestone_ten_closeout_digest.to_string(),
            selected_plan_digest: selected_plan.selected_plan_digest().to_string(),
            execution_receipt_digest: execution_receipt.execution_receipt_digest().to_string(),
            touched_closure_digest: selected_plan.touched_closure_digest().to_string(),
            query_support_digest: selected_plan.query_support_digest().to_string(),
            legality_support_digest: selected_plan.legality_support_digest().to_string(),
            product_summary_digest: product_summary.report_digest().to_string(),
            performance_proof_digest: performance_proof.proof_digest().to_string(),
            deletion_audit_digest: deletion_closeout
                .deletion_audit()
                .audit_digest()
                .to_string(),
            counters_digest: counters.counters_digest().to_string(),
            lookup_readiness,
            topology_derived_product_receipts,
            seed_digest,
        }
    }

    pub fn milestone_ten_closeout_digest(&self) -> &str {
        &self.milestone_ten_closeout_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
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

    pub fn product_summary_digest(&self) -> &str {
        &self.product_summary_digest
    }

    pub fn performance_proof_digest(&self) -> &str {
        &self.performance_proof_digest
    }

    pub fn deletion_audit_digest(&self) -> &str {
        &self.deletion_audit_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub const fn lookup_readiness(&self) -> DerivedInvalidationMilestoneElevenLookupReadiness {
        self.lookup_readiness
    }

    pub fn topology_derived_product_receipts(
        &self,
    ) -> &[DerivedInvalidationMilestoneElevenProductReceiptRef] {
        &self.topology_derived_product_receipts
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }

    pub fn can_bootstrap_lookup_without_raw_scan(&self) -> bool {
        !self.milestone_ten_closeout_digest.is_empty()
            && !self.selected_plan_digest.is_empty()
            && !self.execution_receipt_digest.is_empty()
            && !self.touched_closure_digest.is_empty()
            && !self.query_support_digest.is_empty()
            && !self.legality_support_digest.is_empty()
            && !self.product_summary_digest.is_empty()
            && !self.performance_proof_digest.is_empty()
            && !self.deletion_audit_digest.is_empty()
            && !self.counters_digest.is_empty()
            && !self.topology_derived_product_receipts.is_empty()
            && self
                .topology_derived_product_receipts
                .iter()
                .all(|receipt_ref| receipt_ref.can_bootstrap_lookup_without_raw_scan())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DerivedInvalidationMilestoneElevenProductReceiptRef {
    family_identity: DerivedTopologyProductFamilyIdentity,
    execution_row_digest: String,
    product_output_digest: Option<String>,
    query_receipt_digest: Option<String>,
    legality_receipt_digest: Option<String>,
    ref_digest: String,
}

impl DerivedInvalidationMilestoneElevenProductReceiptRef {
    fn from_executed_row(
        row: &crate::derived_topology::invalidation_plan::execution::DerivedInvalidationExecutedProductRow,
    ) -> Self {
        let product_output_digest = row.product_output_digest().map(str::to_string);
        let query_receipt_digest = row.query_receipt_digest().map(str::to_string);
        let legality_receipt_digest = row.legality_receipt_digest().map(str::to_string);
        let ref_digest = super::super::catalog::catalog_digest([
            "worth-topo:derived-invalidation-milestone-eleven-product-receipt-ref:v1".to_string(),
            format!("family:{}", row.family_identity().as_str()),
            format!("execution-row:{}", row.row_digest()),
            format!(
                "product-output:{}",
                row.product_output_digest().unwrap_or("not-bound")
            ),
            format!(
                "query-receipt:{}",
                row.query_receipt_digest().unwrap_or("not-required")
            ),
            format!(
                "legality-receipt:{}",
                row.legality_receipt_digest().unwrap_or("not-required")
            ),
        ]);
        Self {
            family_identity: row.family_identity(),
            execution_row_digest: row.row_digest().to_string(),
            product_output_digest,
            query_receipt_digest,
            legality_receipt_digest,
            ref_digest,
        }
    }

    pub const fn family_identity(&self) -> DerivedTopologyProductFamilyIdentity {
        self.family_identity
    }

    pub fn execution_row_digest(&self) -> &str {
        &self.execution_row_digest
    }

    pub fn product_output_digest(&self) -> Option<&str> {
        self.product_output_digest.as_deref()
    }

    pub fn query_receipt_digest(&self) -> Option<&str> {
        self.query_receipt_digest.as_deref()
    }

    pub fn legality_receipt_digest(&self) -> Option<&str> {
        self.legality_receipt_digest.as_deref()
    }

    pub fn ref_digest(&self) -> &str {
        &self.ref_digest
    }

    pub fn can_bootstrap_lookup_without_raw_scan(&self) -> bool {
        !self.execution_row_digest.is_empty() && !self.ref_digest.is_empty()
    }
}

fn product_receipt_refs_from_execution_receipt(
    execution_receipt: &DerivedInvalidationExecutionReceipt,
) -> Vec<DerivedInvalidationMilestoneElevenProductReceiptRef> {
    execution_receipt
        .executed_rows()
        .iter()
        .map(DerivedInvalidationMilestoneElevenProductReceiptRef::from_executed_row)
        .collect()
}

fn topology_derived_product_receipts_digest(
    refs: &[DerivedInvalidationMilestoneElevenProductReceiptRef],
) -> String {
    let mut parts =
        vec!["worth-topo:derived-invalidation-milestone-eleven-product-receipts:v1".to_string()];
    parts.extend(refs.iter().map(|receipt_ref| {
        format!(
            "topology-derived-product-receipt:{}",
            receipt_ref.ref_digest()
        )
    }));
    super::super::catalog::catalog_digest(parts)
}
