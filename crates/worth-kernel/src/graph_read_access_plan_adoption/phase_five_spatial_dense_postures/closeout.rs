use std::path::{Path, PathBuf};

use crate::graph_read_access_plan_adoption::WorthGraphReadAccessFirstVerticalSliceSeed;

use super::batch_admission::{
    build_grouped_admission_report, WorthGraphReadAccessGroupedAdmissionReport,
};
use super::bounded_execution::{
    build_bounded_execution_contract, WorthGraphReadAccessBoundedExecutionContract,
};
use super::cap_ledger::validate_spatial_dense_seed_cap_ledger;
use super::closeout_digest::spatial_dense_closeout_digest;
use super::counters::WorthGraphReadAccessSpatialDensePostureCounters;
use super::errors::{
    WorthGraphReadAccessSpatialDensePostureError, WorthGraphReadAccessSpatialDensePostureErrorKind,
};
use super::phase_six_seed::{
    WorthGraphReadAccessSpatialDensePhaseSixSeed, WorthGraphReadAccessSpatialDensePhaseSixSeedInput,
};
use super::query_posture_projection::{
    project_spatial_dense_postures, WorthGraphReadAccessSpatialDensePostureProjection,
};
use super::slice_classification::{
    classify_unresolved_slices, WorthGraphReadAccessUnresolvedSliceRow,
};
use super::source_firewall::{
    scan_workspace, WorthGraphReadAccessSpatialDenseSourceFirewallReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAccessSpatialDensePostureCloseout {
    phase_five_seed_digest: String,
    unresolved_slices: Vec<WorthGraphReadAccessUnresolvedSliceRow>,
    posture_projections: Vec<WorthGraphReadAccessSpatialDensePostureProjection>,
    grouped_admission_report: WorthGraphReadAccessGroupedAdmissionReport,
    bounded_execution_contract: WorthGraphReadAccessBoundedExecutionContract,
    source_firewall_report: WorthGraphReadAccessSpatialDenseSourceFirewallReport,
    counters: WorthGraphReadAccessSpatialDensePostureCounters,
    phase_six_seed: WorthGraphReadAccessSpatialDensePhaseSixSeed,
    closeout_digest: String,
}

pub fn current_worth_graph_read_access_spatial_dense_posture_closeout(
    seed: &WorthGraphReadAccessFirstVerticalSliceSeed,
) -> Result<
    WorthGraphReadAccessSpatialDensePostureCloseout,
    WorthGraphReadAccessSpatialDensePostureError,
> {
    closeout_for_workspace_root(seed, &workspace_root_from_manifest())
}

pub(crate) fn closeout_for_workspace_root(
    seed: &WorthGraphReadAccessFirstVerticalSliceSeed,
    workspace_root: &Path,
) -> Result<
    WorthGraphReadAccessSpatialDensePostureCloseout,
    WorthGraphReadAccessSpatialDensePostureError,
> {
    reject_invalid_seed(seed)?;
    validate_spatial_dense_seed_cap_ledger(seed.unresolved_postures(), seed.cap_rows())?;

    let unresolved_slices = classify_unresolved_slices(seed.unresolved_postures());
    let posture_projections = project_spatial_dense_postures(&unresolved_slices);
    let grouped_admission_report = build_grouped_admission_report(&posture_projections);
    reject_scalarized_caller_loops(&grouped_admission_report)?;
    let bounded_execution_contract = build_bounded_execution_contract(&posture_projections);
    reject_unbounded_dense_or_broad_execution(&bounded_execution_contract)?;
    let source_firewall_report = scan_workspace(workspace_root).map_err(|_| {
        WorthGraphReadAccessSpatialDensePostureError::new(
            WorthGraphReadAccessSpatialDensePostureErrorKind::SourceFirewallViolation,
        )
    })?;
    let counters = WorthGraphReadAccessSpatialDensePostureCounters::from_products(
        &posture_projections,
        &bounded_execution_contract,
        grouped_admission_report.scalarized_caller_loop_count(),
    );
    let closeout_digest = spatial_dense_closeout_digest(
        seed,
        &posture_projections,
        &grouped_admission_report,
        &bounded_execution_contract,
        &source_firewall_report,
    );
    let phase_six_seed = WorthGraphReadAccessSpatialDensePhaseSixSeed::from_input(
        WorthGraphReadAccessSpatialDensePhaseSixSeedInput {
            phase_five_closeout_digest: closeout_digest.clone(),
            phase_four_closeout_digest: seed.phase_four_closeout_digest().to_string(),
            phase_four_plan_projection: seed.plan_projection().clone(),
            phase_four_receipt_projection: seed.receipt_projection().clone(),
            phase_four_cutover_proof: seed.cutover_proof().clone(),
            posture_projections: posture_projections.clone(),
            grouped_admission_report: grouped_admission_report.clone(),
            bounded_execution_contract: bounded_execution_contract.clone(),
            source_firewall_report: source_firewall_report.clone(),
            cap_rows: seed.cap_rows().to_vec(),
        },
    );

    Ok(WorthGraphReadAccessSpatialDensePostureCloseout {
        phase_five_seed_digest: seed.seed_digest().to_string(),
        unresolved_slices,
        posture_projections,
        grouped_admission_report,
        bounded_execution_contract,
        source_firewall_report,
        counters,
        phase_six_seed,
        closeout_digest,
    })
}

fn workspace_root_from_manifest() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("worth-kernel manifest should live under workspace/crates/worth-kernel")
        .to_path_buf()
}

impl WorthGraphReadAccessSpatialDensePostureCloseout {
    pub fn phase_five_seed_digest(&self) -> &str {
        &self.phase_five_seed_digest
    }

    pub fn unresolved_slices(&self) -> &[WorthGraphReadAccessUnresolvedSliceRow] {
        &self.unresolved_slices
    }

    pub fn posture_projections(&self) -> &[WorthGraphReadAccessSpatialDensePostureProjection] {
        &self.posture_projections
    }

    pub const fn grouped_admission_report(&self) -> &WorthGraphReadAccessGroupedAdmissionReport {
        &self.grouped_admission_report
    }

    pub const fn bounded_execution_contract(
        &self,
    ) -> &WorthGraphReadAccessBoundedExecutionContract {
        &self.bounded_execution_contract
    }

    pub const fn source_firewall_report(
        &self,
    ) -> &WorthGraphReadAccessSpatialDenseSourceFirewallReport {
        &self.source_firewall_report
    }

    pub const fn counters(&self) -> &WorthGraphReadAccessSpatialDensePostureCounters {
        &self.counters
    }

    pub const fn phase_six_seed(&self) -> &WorthGraphReadAccessSpatialDensePhaseSixSeed {
        &self.phase_six_seed
    }

    pub fn closeout_digest(&self) -> &str {
        &self.closeout_digest
    }

    pub const fn claims_validator_selection(&self) -> bool {
        false
    }
}

fn reject_invalid_seed(
    seed: &WorthGraphReadAccessFirstVerticalSliceSeed,
) -> Result<(), WorthGraphReadAccessSpatialDensePostureError> {
    if seed.claims_validator_selection() {
        return Err(WorthGraphReadAccessSpatialDensePostureError::new(
            WorthGraphReadAccessSpatialDensePostureErrorKind::SeedAlreadyClaimsValidatorSelection,
        ));
    }
    if !seed.claims_graph_read_receipt() && seed.unresolved_postures().is_empty() {
        return Err(WorthGraphReadAccessSpatialDensePostureError::new(
            WorthGraphReadAccessSpatialDensePostureErrorKind::MissingPhaseFourReceiptAndUnresolvedWork,
        ));
    }
    Ok(())
}

fn reject_unbounded_dense_or_broad_execution(
    bounded_execution_contract: &WorthGraphReadAccessBoundedExecutionContract,
) -> Result<(), WorthGraphReadAccessSpatialDensePostureError> {
    if bounded_execution_contract.unbounded_ephemeral_index_count() > 0 {
        return Err(WorthGraphReadAccessSpatialDensePostureError::new(
            WorthGraphReadAccessSpatialDensePostureErrorKind::UnboundedEphemeralIndexForDenseOrBroadRead,
        ));
    }
    Ok(())
}

fn reject_scalarized_caller_loops(
    grouped_admission_report: &WorthGraphReadAccessGroupedAdmissionReport,
) -> Result<(), WorthGraphReadAccessSpatialDensePostureError> {
    if grouped_admission_report.scalarized_caller_loop_count() > 0 {
        return Err(WorthGraphReadAccessSpatialDensePostureError::new(
            WorthGraphReadAccessSpatialDensePostureErrorKind::ScalarizedCallerLoopDetected,
        ));
    }
    Ok(())
}
