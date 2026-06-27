use crate::workload_platform::evidence_ledger::{
    SpatialGeometryEvidenceTouchAuthority, WorkloadEvidenceStage,
};
use crate::workload_platform::evidence_lookup_execution::EvidenceLookupExecutionReceipt;
use crate::workload_platform::evidence_lookup_plan_selection::{
    EvidenceLookupPlanRowOutcome, EvidenceLookupSelectedPlan, EvidenceLookupSelectedPlanRow,
};
use crate::workload_platform::evidence_lookup_workload_cutover::{
    EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupWorkloadCutoverError,
};

use super::counters::EvidenceLookupStageCutoverCounters;
use super::error::{EvidenceLookupStageCutoverError, EvidenceLookupStageCutoverErrorKind};
use super::topology_derived_state::EvidenceLookupTopologyDerivedReceiptState;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupCoveredStageCutoverProof {
    family_identity: String,
    stage: WorkloadEvidenceStage,
    spatial_touch_digest: String,
    workload_stage_index_identity: String,
    stage_receipt_identity: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    topology_derived_receipt_state: EvidenceLookupTopologyDerivedReceiptState,
    covered_family_identities: Vec<String>,
    counters: EvidenceLookupStageCutoverCounters,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupCoveredStageCutoverExplanation {
    family_identity: String,
    stage: WorkloadEvidenceStage,
    stage_receipt_identity: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    covered_family_count: usize,
}

impl EvidenceLookupCoveredStageCutoverProof {
    pub fn prove(
        covered_stage: WorkloadEvidenceStage,
        spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
        stage_receipt_identity: &str,
        family_identity: &str,
        selected_plan: &EvidenceLookupSelectedPlan,
        execution_receipt: &EvidenceLookupExecutionReceipt,
    ) -> Result<Self, EvidenceLookupStageCutoverError> {
        require_matching_stage(covered_stage, spatial_touch_authority)?;
        require_matching_receipt_identities(spatial_touch_authority, stage_receipt_identity)?;
        require_lookup_phase_chain(selected_plan, execution_receipt)?;

        let selected_row = selected_plan
            .rows()
            .iter()
            .find(|row| row.family_identity() == family_identity)
            .ok_or_else(|| {
                EvidenceLookupStageCutoverError::new(
                    EvidenceLookupStageCutoverErrorKind::MissingCoveredFamily,
                    format!("selected lookup plan does not cover family `{family_identity}`"),
                )
            })?;
        require_covered_family_outcome(selected_row)?;

        let topology_derived_receipt_state =
            EvidenceLookupTopologyDerivedReceiptState::from_plan_topology_posture(
                selected_row.topology_posture(),
                family_identity,
            )?;
        let covered_family_identities = covered_family_identities(selected_plan);
        let counters = EvidenceLookupStageCutoverCounters::new(
            covered_family_identities.len(),
            topology_receipt_ref_count(&topology_derived_receipt_state),
            spatial_touch_authority
                .lookup_counters()
                .indexed_lookup_count(),
            spatial_touch_authority
                .lookup_counters()
                .raw_row_scan_count(),
            selected_plan.counters().broad_receipt_scan_count(),
            execution_receipt.counters().caller_owned_scan_count(),
            execution_receipt.counters().query_artifact_count(),
        );
        if counters.raw_row_scan_count() != 0 || counters.broad_receipt_scan_count() != 0 {
            return Err(EvidenceLookupStageCutoverError::new(
                EvidenceLookupStageCutoverErrorKind::RawEvidenceFallbackDenied,
                "covered lookup cutover cannot certify raw evidence scans or broad receipt scans",
            ));
        }
        if counters.caller_owned_scan_count() != 0 {
            return Err(EvidenceLookupStageCutoverError::new(
                EvidenceLookupStageCutoverErrorKind::ScopeExpansionDenied,
                "covered lookup cutover cannot certify caller-owned execution scans",
            ));
        }

        Ok(Self {
            family_identity: family_identity.to_string(),
            stage: covered_stage,
            spatial_touch_digest: spatial_touch_authority.digest().as_str().to_string(),
            workload_stage_index_identity: spatial_touch_authority
                .stage_index_identity()
                .to_string(),
            stage_receipt_identity: stage_receipt_identity.to_string(),
            selected_lookup_plan_digest: selected_plan.selected_plan_digest().to_string(),
            lookup_execution_receipt_digest: execution_receipt
                .execution_receipt_digest()
                .to_string(),
            lookup_product_output_digest: execution_receipt
                .lookup_product_output_digest()
                .to_string(),
            topology_derived_receipt_state,
            covered_family_identities,
            counters,
        })
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn spatial_touch_digest(&self) -> &str {
        &self.spatial_touch_digest
    }

    pub fn workload_stage_index_identity(&self) -> &str {
        &self.workload_stage_index_identity
    }

    pub fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }

    pub fn selected_lookup_plan_digest(&self) -> &str {
        &self.selected_lookup_plan_digest
    }

    pub fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }

    pub const fn topology_derived_receipt_state(
        &self,
    ) -> &EvidenceLookupTopologyDerivedReceiptState {
        &self.topology_derived_receipt_state
    }

    pub fn covered_family_identities(&self) -> &[String] {
        &self.covered_family_identities
    }

    pub const fn counters(&self) -> &EvidenceLookupStageCutoverCounters {
        &self.counters
    }

    pub fn explain(&self) -> EvidenceLookupCoveredStageCutoverExplanation {
        EvidenceLookupCoveredStageCutoverExplanation {
            family_identity: self.family_identity.clone(),
            stage: self.stage,
            stage_receipt_identity: self.stage_receipt_identity.clone(),
            selected_lookup_plan_digest: self.selected_lookup_plan_digest.clone(),
            lookup_execution_receipt_digest: self.lookup_execution_receipt_digest.clone(),
            lookup_product_output_digest: self.lookup_product_output_digest.clone(),
            covered_family_count: self.covered_family_identities.len(),
        }
    }

    pub fn lower_workload_handoff(
        &self,
    ) -> Result<EvidenceLookupConsumedWorkloadHandoff, EvidenceLookupWorkloadCutoverError> {
        EvidenceLookupConsumedWorkloadHandoff::lower_from_stage_proof(self)
    }

    #[cfg(test)]
    pub(crate) fn with_test_raw_row_scan_count(mut self, raw_row_scan_count: usize) -> Self {
        self.counters = EvidenceLookupStageCutoverCounters::new(
            self.counters.covered_family_count(),
            self.counters.topology_receipt_ref_count(),
            self.counters.indexed_lookup_count(),
            raw_row_scan_count,
            self.counters.broad_receipt_scan_count(),
            self.counters.caller_owned_scan_count(),
            self.counters.query_artifact_count(),
        );
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_broad_receipt_scan_count(
        mut self,
        broad_receipt_scan_count: usize,
    ) -> Self {
        self.counters = EvidenceLookupStageCutoverCounters::new(
            self.counters.covered_family_count(),
            self.counters.topology_receipt_ref_count(),
            self.counters.indexed_lookup_count(),
            self.counters.raw_row_scan_count(),
            broad_receipt_scan_count,
            self.counters.caller_owned_scan_count(),
            self.counters.query_artifact_count(),
        );
        self
    }

    #[cfg(test)]
    pub(crate) fn with_test_caller_owned_scan_count(
        mut self,
        caller_owned_scan_count: usize,
    ) -> Self {
        self.counters = EvidenceLookupStageCutoverCounters::new(
            self.counters.covered_family_count(),
            self.counters.topology_receipt_ref_count(),
            self.counters.indexed_lookup_count(),
            self.counters.raw_row_scan_count(),
            self.counters.broad_receipt_scan_count(),
            caller_owned_scan_count,
            self.counters.query_artifact_count(),
        );
        self
    }
}

impl EvidenceLookupCoveredStageCutoverExplanation {
    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub fn stage_receipt_identity(&self) -> &str {
        &self.stage_receipt_identity
    }

    pub fn selected_lookup_plan_digest(&self) -> &str {
        &self.selected_lookup_plan_digest
    }

    pub fn lookup_execution_receipt_digest(&self) -> &str {
        &self.lookup_execution_receipt_digest
    }

    pub fn lookup_product_output_digest(&self) -> &str {
        &self.lookup_product_output_digest
    }

    pub const fn covered_family_count(&self) -> usize {
        self.covered_family_count
    }
}

fn require_matching_stage(
    covered_stage: WorkloadEvidenceStage,
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
) -> Result<(), EvidenceLookupStageCutoverError> {
    if spatial_touch_authority.evidence_stage() == covered_stage {
        Ok(())
    } else {
        Err(EvidenceLookupStageCutoverError::new(
            EvidenceLookupStageCutoverErrorKind::StageMismatch,
            format!(
                "spatial touch authority stage `{:?}` does not match covered stage `{:?}`",
                spatial_touch_authority.evidence_stage(),
                covered_stage
            ),
        ))
    }
}

fn require_matching_receipt_identities(
    spatial_touch_authority: &SpatialGeometryEvidenceTouchAuthority,
    stage_receipt_identity: &str,
) -> Result<(), EvidenceLookupStageCutoverError> {
    let expected_stage_receipt_identity = spatial_touch_authority.evidence_identity();
    if stage_receipt_identity != expected_stage_receipt_identity {
        return Err(EvidenceLookupStageCutoverError::new(
            EvidenceLookupStageCutoverErrorKind::StageReceiptMismatch,
            "covered lookup cutover requires one matching stage receipt identity across the covered-stage authority and receipt",
        ));
    }
    Ok(())
}

fn require_lookup_phase_chain(
    selected_plan: &EvidenceLookupSelectedPlan,
    execution_receipt: &EvidenceLookupExecutionReceipt,
) -> Result<(), EvidenceLookupStageCutoverError> {
    if execution_receipt.selected_plan_digest() == selected_plan.selected_plan_digest()
        && execution_receipt.stage_receipt_digest() == selected_plan.stage_receipt_digest()
        && execution_receipt.spatial_touch_digest() == selected_plan.spatial_touch_digest()
    {
        Ok(())
    } else {
        Err(EvidenceLookupStageCutoverError::new(
            EvidenceLookupStageCutoverErrorKind::SelectedPlanMismatch,
            "lookup execution receipt does not match the selected lookup plan phase-chain identity",
        ))
    }
}

fn require_covered_family_outcome(
    selected_row: &EvidenceLookupSelectedPlanRow,
) -> Result<(), EvidenceLookupStageCutoverError> {
    if selected_row.outcome() == EvidenceLookupPlanRowOutcome::Selected {
        Ok(())
    } else {
        Err(EvidenceLookupStageCutoverError::new(
            EvidenceLookupStageCutoverErrorKind::UncoveredFamilyOutcome,
            format!(
                "family `{}` has non-covered outcome `{:?}`",
                selected_row.family_identity(),
                selected_row.outcome()
            ),
        ))
    }
}

fn covered_family_identities(selected_plan: &EvidenceLookupSelectedPlan) -> Vec<String> {
    selected_plan
        .rows()
        .iter()
        .filter(|row| row.outcome() == EvidenceLookupPlanRowOutcome::Selected)
        .map(|row| row.family_identity().to_string())
        .collect()
}

fn topology_receipt_ref_count(state: &EvidenceLookupTopologyDerivedReceiptState) -> usize {
    match state {
        EvidenceLookupTopologyDerivedReceiptState::NotRequired => 0,
        EvidenceLookupTopologyDerivedReceiptState::ReceiptRef(_) => 1,
    }
}
