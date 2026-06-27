use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::{
    EvidenceLookupFamilyStageSelection, EvidenceLookupStageReceiptFamilyIdentity,
};

use super::counters::EvidenceLookupInputAdmissionCounters;
use super::product_separation::EvidenceLookupProductSeparationProof;
use super::query_support::EvidenceLookupQueryAdmissionSupport;
use super::topology_support::EvidenceLookupTopologyAdmissionSupport;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupAdmittedInput {
    admission_digest: String,
    catalog_digest: String,
    spatial_touch_digest: String,
    stage_receipt_digest: String,
    stage: WorkloadEvidenceStage,
    receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    family_selection: EvidenceLookupFamilyStageSelection,
    topology_support: Vec<EvidenceLookupTopologyAdmissionSupport>,
    query_support: Vec<EvidenceLookupQueryAdmissionSupport>,
    counters: EvidenceLookupInputAdmissionCounters,
    product_separation: EvidenceLookupProductSeparationProof,
}

pub(crate) struct EvidenceLookupAdmittedInputParts {
    pub(crate) catalog_digest: String,
    pub(crate) spatial_touch_digest: String,
    pub(crate) stage_receipt_digest: String,
    pub(crate) stage: WorkloadEvidenceStage,
    pub(crate) receipt_family: EvidenceLookupStageReceiptFamilyIdentity,
    pub(crate) family_selection: EvidenceLookupFamilyStageSelection,
    pub(crate) topology_support: Vec<EvidenceLookupTopologyAdmissionSupport>,
    pub(crate) query_support: Vec<EvidenceLookupQueryAdmissionSupport>,
    pub(crate) counters: EvidenceLookupInputAdmissionCounters,
}

impl EvidenceLookupAdmittedInput {
    pub(crate) fn from_parts(parts: EvidenceLookupAdmittedInputParts) -> Self {
        let product_separation = EvidenceLookupProductSeparationProof::admission_only();
        let admission_digest = admission_digest(&parts);
        Self {
            admission_digest,
            catalog_digest: parts.catalog_digest,
            spatial_touch_digest: parts.spatial_touch_digest,
            stage_receipt_digest: parts.stage_receipt_digest,
            stage: parts.stage,
            receipt_family: parts.receipt_family,
            family_selection: parts.family_selection,
            topology_support: parts.topology_support,
            query_support: parts.query_support,
            counters: parts.counters,
            product_separation,
        }
    }

    pub fn admission_digest(&self) -> &str {
        &self.admission_digest
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn stage_receipt_digest(&self) -> &str {
        &self.stage_receipt_digest
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub const fn receipt_family(&self) -> &EvidenceLookupStageReceiptFamilyIdentity {
        &self.receipt_family
    }

    pub const fn family_selection(&self) -> &EvidenceLookupFamilyStageSelection {
        &self.family_selection
    }

    pub fn topology_support(&self) -> &[EvidenceLookupTopologyAdmissionSupport] {
        &self.topology_support
    }

    pub fn query_support(&self) -> &[EvidenceLookupQueryAdmissionSupport] {
        &self.query_support
    }

    pub const fn counters(&self) -> &EvidenceLookupInputAdmissionCounters {
        &self.counters
    }

    pub const fn product_separation(&self) -> EvidenceLookupProductSeparationProof {
        self.product_separation
    }

    pub const fn claims_lookup_product_construction(&self) -> bool {
        false
    }

    pub const fn claims_lookup_execution(&self) -> bool {
        false
    }

    #[cfg(test)]
    pub(crate) fn with_reversed_support_for_plan_selection_tests(&self) -> Self {
        let mut topology_support = self.topology_support.clone();
        let mut query_support = self.query_support.clone();
        topology_support.reverse();
        query_support.reverse();
        self.with_support_for_plan_selection_tests(topology_support, query_support)
    }

    #[cfg(test)]
    pub(crate) fn with_duplicate_query_support_for_plan_selection_tests(&self) -> Self {
        let topology_support = self.topology_support.clone();
        let mut query_support = self.query_support.clone();
        let Some(first_query_support) = query_support.first().cloned() else {
            return self.clone();
        };
        query_support.push(first_query_support);
        self.with_support_for_plan_selection_tests(topology_support, query_support)
    }

    #[cfg(test)]
    pub(crate) fn without_query_support_for_plan_selection_tests(&self) -> Self {
        self.with_support_for_plan_selection_tests(self.topology_support.clone(), Vec::new())
    }

    #[cfg(test)]
    fn with_support_for_plan_selection_tests(
        &self,
        topology_support: Vec<EvidenceLookupTopologyAdmissionSupport>,
        query_support: Vec<EvidenceLookupQueryAdmissionSupport>,
    ) -> Self {
        Self::from_parts(EvidenceLookupAdmittedInputParts {
            catalog_digest: self.catalog_digest.clone(),
            spatial_touch_digest: self.spatial_touch_digest.clone(),
            stage_receipt_digest: self.stage_receipt_digest.clone(),
            stage: self.stage,
            receipt_family: self.receipt_family.clone(),
            family_selection: self.family_selection.clone(),
            topology_support,
            query_support,
            counters: self.counters,
        })
    }
}

fn admission_digest(parts: &EvidenceLookupAdmittedInputParts) -> String {
    let mut digest_parts = vec![
        "worth-spatial:evidence-lookup-admitted-input:v1".to_string(),
        format!("catalog:{}", parts.catalog_digest),
        format!("spatial-touch:{}", parts.spatial_touch_digest),
        format!("stage-receipt:{}", parts.stage_receipt_digest),
        format!("stage:{}", parts.stage.human_name()),
        format!("receipt-family:{}", parts.receipt_family.as_str()),
        format!(
            "candidate-families:{}",
            parts.counters.catalog_candidate_family_count()
        ),
    ];
    digest_parts.extend(
        parts
            .family_selection
            .family_identities()
            .iter()
            .map(|identity| format!("family:{identity}")),
    );
    let mut topology_support_parts = parts
        .topology_support
        .iter()
        .map(EvidenceLookupTopologyAdmissionSupport::digest_summary_part)
        .collect::<Vec<_>>();
    topology_support_parts.sort();
    digest_parts.extend(topology_support_parts);
    let mut query_support_parts = parts
        .query_support
        .iter()
        .map(EvidenceLookupQueryAdmissionSupport::digest_summary_part)
        .collect::<Vec<_>>();
    query_support_parts.sort();
    digest_parts.extend(query_support_parts);
    digest_parts.push("product-separation:admission-only".to_string());
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &digest_parts)
}
