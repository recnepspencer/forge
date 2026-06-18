use topology::facade::{
    current_topology_query_consumer_kit_adoption_status, WorthTopoQueryConsumerKitAdoptionError,
};
use worth_spatial::facade::query_adoption::{
    current_spatial_query_consumer_kit_adoption_status, WorthSpatialQueryConsumerKitAdoptionError,
};

use super::authority_boundary::WorthQueryAuthorityBoundaryReport;
use super::composition_honesty::{
    current_kernel_composition_honesty_report, WorthKernelCompositionHonestyError,
};
use super::consumer_kit::{
    current_kernel_query_consumer_kit_adoption_status, WorthKernelQueryConsumerKitAdoptionError,
};
use super::performance_counters::{
    current_worth_phase_eight_performance_counter_report, WorthPhaseEightDiagnosticPolicy,
    WorthPhaseEightPerformanceCounterError,
};
use super::report::{WorthQueryAdoptionInventoryError, WorthQueryAdoptionInventoryReport};
use super::synthetic_proof::{
    WorthQuerySyntheticProofDisposition, WorthQuerySyntheticProofDispositionError,
    WorthQuerySyntheticProofDispositionReport,
};

const CLOSEOUT_DOC: &str =
    include_str!("../../../../_docs/worth/query-native-hardening-closeout.md");
const WORTH_ROADMAP: &str = include_str!("../../../../_docs/worth/worth_roadmap.md");
const AI_README: &str = include_str!("../../../forge-query/docs/AI_README.md");

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryNativeHardeningCloseoutReport {
    audited_source_set_count: usize,
    admitted_source_set_count: usize,
    denied_source_set_count: usize,
    explicit_residue_source_set_count: usize,
    support_requirement_count: usize,
    support_observed_row_count: usize,
    support_matched_required_count: usize,
    support_snapshot_row_count: usize,
    boundary_audit_source_count: usize,
    synthetic_denial_localization_row_count: usize,
    kernel_receipt_breadth_count: usize,
    lower_crate_receipt_family_count: usize,
    topology_read_touched_scope_count: usize,
    spatial_witness_resolution_request_count: usize,
    spatial_witness_denial_count: usize,
    spatial_witness_catalog_lookup_count: usize,
    closeout_doc_agrees: bool,
    ai_readme_agrees: bool,
    roadmap_sequencing_agrees: bool,
}

impl WorthQueryNativeHardeningCloseoutReport {
    pub const fn audited_source_set_count(&self) -> usize {
        self.audited_source_set_count
    }

    pub const fn admitted_source_set_count(&self) -> usize {
        self.admitted_source_set_count
    }

    pub const fn denied_source_set_count(&self) -> usize {
        self.denied_source_set_count
    }

    pub const fn explicit_residue_source_set_count(&self) -> usize {
        self.explicit_residue_source_set_count
    }

    pub const fn support_requirement_count(&self) -> usize {
        self.support_requirement_count
    }

    pub const fn support_observed_row_count(&self) -> usize {
        self.support_observed_row_count
    }

    pub const fn support_matched_required_count(&self) -> usize {
        self.support_matched_required_count
    }

    pub const fn support_snapshot_row_count(&self) -> usize {
        self.support_snapshot_row_count
    }

    pub const fn boundary_audit_source_count(&self) -> usize {
        self.boundary_audit_source_count
    }

    pub const fn synthetic_denial_localization_row_count(&self) -> usize {
        self.synthetic_denial_localization_row_count
    }

    pub const fn kernel_receipt_breadth_count(&self) -> usize {
        self.kernel_receipt_breadth_count
    }

    pub const fn lower_crate_receipt_family_count(&self) -> usize {
        self.lower_crate_receipt_family_count
    }

    pub const fn topology_read_touched_scope_count(&self) -> usize {
        self.topology_read_touched_scope_count
    }

    pub const fn spatial_witness_resolution_request_count(&self) -> usize {
        self.spatial_witness_resolution_request_count
    }

    pub const fn spatial_witness_denial_count(&self) -> usize {
        self.spatial_witness_denial_count
    }

    pub const fn spatial_witness_catalog_lookup_count(&self) -> usize {
        self.spatial_witness_catalog_lookup_count
    }

    pub const fn closeout_doc_agrees(&self) -> bool {
        self.closeout_doc_agrees
    }

    pub const fn ai_readme_agrees(&self) -> bool {
        self.ai_readme_agrees
    }

    pub const fn roadmap_sequencing_agrees(&self) -> bool {
        self.roadmap_sequencing_agrees
    }

    pub const fn gate_closed(&self) -> bool {
        self.closeout_doc_agrees && self.ai_readme_agrees && self.roadmap_sequencing_agrees
    }
}

#[derive(Debug)]
pub enum WorthQueryNativeHardeningCloseoutError {
    Inventory(WorthQueryAdoptionInventoryError),
    KernelAdoption(WorthKernelQueryConsumerKitAdoptionError),
    TopologyAdoption(WorthTopoQueryConsumerKitAdoptionError),
    SpatialAdoption(WorthSpatialQueryConsumerKitAdoptionError),
    SyntheticProof(WorthQuerySyntheticProofDispositionError),
    Performance(WorthPhaseEightPerformanceCounterError),
    Composition(WorthKernelCompositionHonestyError),
    AuthorityParityFailed,
    DirtyAdoptionReport,
    CloseoutDocumentDrift,
    AiReadmeAgreementDrift,
    RoadmapSequencingDrift,
}

pub fn current_worth_query_native_hardening_closeout_report(
) -> Result<WorthQueryNativeHardeningCloseoutReport, WorthQueryNativeHardeningCloseoutError> {
    let inventory = WorthQueryAdoptionInventoryReport::cross_crate_reality_inventory()
        .map_err(WorthQueryNativeHardeningCloseoutError::Inventory)?;
    let authority = WorthQueryAuthorityBoundaryReport::from_inventory(&inventory);
    if !authority.all_rows_are_in_parity() {
        return Err(WorthQueryNativeHardeningCloseoutError::AuthorityParityFailed);
    }

    let kernel = current_kernel_query_consumer_kit_adoption_status()
        .map_err(WorthQueryNativeHardeningCloseoutError::KernelAdoption)?;
    let topology = current_topology_query_consumer_kit_adoption_status()
        .map_err(WorthQueryNativeHardeningCloseoutError::TopologyAdoption)?;
    let spatial = current_spatial_query_consumer_kit_adoption_status()
        .map_err(WorthQueryNativeHardeningCloseoutError::SpatialAdoption)?;
    if kernel.support_blocking_finding_count() != 0
        || topology.support_blocking_finding_count() != 0
        || spatial.support_blocking_finding_count() != 0
        || !kernel.hard_prohibition_audit_clean()
        || !topology.hard_prohibition_audit_clean()
        || !spatial.hard_prohibition_audit_clean()
    {
        return Err(WorthQueryNativeHardeningCloseoutError::DirtyAdoptionReport);
    }

    let synthetic = WorthQuerySyntheticProofDispositionReport::current()
        .map_err(WorthQueryNativeHardeningCloseoutError::SyntheticProof)?;
    let performance = current_worth_phase_eight_performance_counter_report(
        WorthPhaseEightDiagnosticPolicy::Minimal,
    )
    .map_err(WorthQueryNativeHardeningCloseoutError::Performance)?;
    let composition = current_kernel_composition_honesty_report()
        .map_err(WorthQueryNativeHardeningCloseoutError::Composition)?;

    let denied_source_set_count = synthetic
        .rows_for(WorthQuerySyntheticProofDisposition::DeniedByBoundary)
        .count();
    let explicit_residue_source_set_count = synthetic
        .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
        .count();
    let report = WorthQueryNativeHardeningCloseoutReport {
        audited_source_set_count: inventory.counters().audited_source_sets(),
        admitted_source_set_count: inventory.counters().production_source_sets(),
        denied_source_set_count,
        explicit_residue_source_set_count,
        support_requirement_count: performance.support_requirement_count(),
        support_observed_row_count: performance.support_observed_row_count(),
        support_matched_required_count: performance.support_matched_required_count(),
        support_snapshot_row_count: performance.support_snapshot_row_count(),
        boundary_audit_source_count: performance.boundary_audit_source_count(),
        synthetic_denial_localization_row_count: performance
            .synthetic_denial_localization_row_count(),
        kernel_receipt_breadth_count: composition.kernel_workload_receipt_family_count(),
        lower_crate_receipt_family_count: composition.lower_crate_receipt_family_count(),
        topology_read_touched_scope_count: performance.topology_read_touched_scope_count(),
        spatial_witness_resolution_request_count: performance
            .spatial_witness_resolution_request_count(),
        spatial_witness_denial_count: performance.spatial_witness_denial_count(),
        spatial_witness_catalog_lookup_count: performance.spatial_witness_catalog_lookup_count(),
        closeout_doc_agrees: closeout_doc_matches_counts(
            inventory.counters().audited_source_sets(),
            inventory.counters().production_source_sets(),
            denied_source_set_count,
            explicit_residue_source_set_count,
            &performance,
            &composition,
        ),
        ai_readme_agrees: ai_readme_names_query_native_consumer_kit_contract(),
        roadmap_sequencing_agrees: roadmap_names_gate_before_next_lanes(),
    };
    validate_closeout_report(&report)?;
    Ok(report)
}

fn closeout_doc_matches_counts(
    audited_source_set_count: usize,
    admitted_source_set_count: usize,
    denied_source_set_count: usize,
    explicit_residue_source_set_count: usize,
    performance: &super::performance_counters::WorthPhaseEightPerformanceCounterReport,
    composition: &super::composition_honesty::WorthKernelCompositionHonestyReport,
) -> bool {
    count_token("audited_source_sets", audited_source_set_count)
        && count_token("admitted_source_sets", admitted_source_set_count)
        && count_token("denied_source_sets", denied_source_set_count)
        && count_token(
            "explicit_residue_source_sets",
            explicit_residue_source_set_count,
        )
        && count_token(
            "support_requirements",
            performance.support_requirement_count(),
        )
        && count_token(
            "support_observed_rows",
            performance.support_observed_row_count(),
        )
        && count_token(
            "support_matched_required_rows",
            performance.support_matched_required_count(),
        )
        && count_token(
            "support_snapshot_rows_evaluated",
            performance.support_snapshot_row_count(),
        )
        && count_token(
            "boundary_audit_sources",
            performance.boundary_audit_source_count(),
        )
        && count_token(
            "synthetic_denial_localization_rows",
            performance.synthetic_denial_localization_row_count(),
        )
        && count_token(
            "kernel_receipt_families",
            composition.kernel_workload_receipt_family_count(),
        )
        && count_token(
            "lower_crate_receipt_families",
            composition.lower_crate_receipt_family_count(),
        )
        && count_token(
            "topology_read_touched_scope",
            performance.topology_read_touched_scope_count(),
        )
        && count_token(
            "spatial_witness_resolution_requests",
            performance.spatial_witness_resolution_request_count(),
        )
        && count_token(
            "spatial_witness_denials",
            performance.spatial_witness_denial_count(),
        )
        && count_token(
            "spatial_witness_catalog_lookups",
            performance.spatial_witness_catalog_lookup_count(),
        )
        && CLOSEOUT_DOC.contains("kernel_residue_follow_on_milestone: Milestone 6.5")
        && CLOSEOUT_DOC.contains("spatial_residue_follow_on_milestone: Milestone 6.5")
        && CLOSEOUT_DOC.contains("topology_residue_follow_on_milestone: Milestone 6.5")
        && CLOSEOUT_DOC.contains("kernel_residue_blocker:")
        && CLOSEOUT_DOC.contains("spatial_residue_blocker:")
        && CLOSEOUT_DOC.contains("topology_residue_blocker:")
}

fn count_token(name: &str, count: usize) -> bool {
    CLOSEOUT_DOC.contains(&format!("{name}: {count}"))
}

fn ai_readme_names_query_native_consumer_kit_contract() -> bool {
    AI_README.contains("## Consumer Kit")
        && AI_README.contains("The Consumer Kit is Query's product surface for downstream proof.")
        && AI_README.contains("evidence reports, hard-prohibition audits, support")
        && AI_README
            .contains("snapshots, support pins, in-memory test workspaces, and adoption/residue")
        && AI_README.contains("support_pinning_contract(...)")
        && AI_README.contains("fabricated test receipts")
        && CLOSEOUT_DOC.contains("AI_README.md")
        && CLOSEOUT_DOC.contains("Consumer Kit")
}

fn roadmap_names_gate_before_next_lanes() -> bool {
    WORTH_ROADMAP
        .contains("`Worth Query-Native Hardening Gate` -> `Milestone 6.5` ->\n  `Milestone 7`")
        && WORTH_ROADMAP.contains("query-native-hardening-closeout.md")
        && WORTH_ROADMAP
            .contains("`Worth Query-Native Hardening Gate`: Closed after Forge Query 9.7 and 9.8;")
        && !WORTH_ROADMAP
            .contains("`Worth Query-Native Hardening Gate`: Planned after Forge Query 9.7 and 9.8;")
}

fn validate_closeout_report(
    report: &WorthQueryNativeHardeningCloseoutReport,
) -> Result<(), WorthQueryNativeHardeningCloseoutError> {
    if !report.closeout_doc_agrees {
        return Err(WorthQueryNativeHardeningCloseoutError::CloseoutDocumentDrift);
    }
    if !report.ai_readme_agrees {
        return Err(WorthQueryNativeHardeningCloseoutError::AiReadmeAgreementDrift);
    }
    if !report.roadmap_sequencing_agrees {
        return Err(WorthQueryNativeHardeningCloseoutError::RoadmapSequencingDrift);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn closeout_report_proves_gate_counts_and_doc_agreement() {
        let report = current_worth_query_native_hardening_closeout_report()
            .expect("query-native hardening closeout");

        assert_eq!(report.audited_source_set_count(), 17);
        assert_eq!(report.admitted_source_set_count(), 9);
        assert_eq!(report.denied_source_set_count(), 5);
        assert_eq!(report.explicit_residue_source_set_count(), 3);
        assert_eq!(report.support_requirement_count(), 9);
        assert_eq!(report.support_observed_row_count(), 3);
        assert_eq!(report.support_matched_required_count(), 9);
        assert_eq!(report.support_snapshot_row_count(), 66);
        assert_eq!(report.boundary_audit_source_count(), 6);
        assert_eq!(report.synthetic_denial_localization_row_count(), 5);
        assert_eq!(report.kernel_receipt_breadth_count(), 8);
        assert_eq!(report.lower_crate_receipt_family_count(), 2);
        assert_eq!(report.topology_read_touched_scope_count(), 4);
        assert_eq!(report.spatial_witness_resolution_request_count(), 8);
        assert_eq!(report.spatial_witness_denial_count(), 4);
        assert_eq!(report.spatial_witness_catalog_lookup_count(), 2);
        assert!(report.closeout_doc_agrees());
        assert!(report.ai_readme_agrees());
        assert!(report.roadmap_sequencing_agrees());
        assert!(report.gate_closed());
    }

    #[test]
    fn closeout_keeps_known_synthetic_source_families_out_of_production_proof() {
        let synthetic = WorthQuerySyntheticProofDispositionReport::current()
            .expect("synthetic proof disposition report");

        for source_set in [
            "crates/worth-kernel/src/certification/public_facade_contracts",
            "crates/worth-spatial/src/certification/public_facade_contracts",
            "crates/worth-spatial/src/test_support",
            "crates/worth-topo/src/test_support",
            "crates/worth-topo/tests/ui",
        ] {
            assert_source_set_has_disposition(
                &synthetic,
                source_set,
                WorthQuerySyntheticProofDisposition::DeniedByBoundary,
            );
        }

        for source_set in [
            "crates/worth-kernel/src/binding/tests",
            "crates/worth-spatial/src/workload_platform/vocabulary",
            "crates/worth-topo/src/projection/runtime_boundary/query_support",
        ] {
            assert_source_set_has_disposition(
                &synthetic,
                source_set,
                WorthQuerySyntheticProofDisposition::ExplicitResidue,
            );
        }
    }

    fn assert_source_set_has_disposition(
        synthetic: &WorthQuerySyntheticProofDispositionReport,
        source_set: &str,
        expected: WorthQuerySyntheticProofDisposition,
    ) {
        let row = synthetic
            .require_source_set(source_set)
            .unwrap_or_else(|| panic!("missing synthetic source set {source_set}"));
        assert_eq!(row.disposition(), expected);
    }
}
