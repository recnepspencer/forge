use topology::facade::{
    current_topology_phase_eight_performance_counters,
    current_topology_query_consumer_kit_adoption_status, WorthTopoQueryConsumerKitAdoptionError,
};
use worth_spatial::facade::query_adoption::{
    current_spatial_phase_eight_performance_counters,
    current_spatial_query_consumer_kit_adoption_status, WorthSpatialQueryConsumerKitAdoptionError,
};

use super::composition_honesty::{
    current_kernel_composition_honesty_report, WorthKernelCompositionHonestyError,
};
use super::consumer_kit::{
    current_kernel_query_consumer_kit_adoption_status, WorthKernelQueryConsumerKitAdoptionError,
};
use super::synthetic_proof::{
    WorthQuerySyntheticProofDisposition, WorthQuerySyntheticProofDispositionError,
    WorthQuerySyntheticProofDispositionReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthPhaseEightDiagnosticPolicy {
    Minimal,
    Rich,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthPhaseEightPerformanceCounterReport {
    support_requirement_count: usize,
    support_observed_row_count: usize,
    support_matched_required_count: usize,
    support_snapshot_row_count: usize,
    support_blocking_finding_count: usize,
    boundary_audit_source_count: usize,
    boundary_audit_coverage_row_count: usize,
    synthetic_denial_localization_row_count: usize,
    synthetic_replaced_row_count: usize,
    explicit_residue_row_count: usize,
    topology_read_touched_scope_count: usize,
    topology_mutation_lane_touched_scope_count: usize,
    topology_graph_composed_lane_count: usize,
    spatial_witness_resolution_request_count: usize,
    spatial_witness_denial_count: usize,
    spatial_witness_catalog_lookup_count: usize,
    kernel_receipt_breadth_count: usize,
    kernel_lower_crate_receipt_family_count: usize,
    diagnostic_detail_row_count: usize,
    domain_outcome_fingerprint: String,
}

impl WorthPhaseEightPerformanceCounterReport {
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

    pub const fn support_blocking_finding_count(&self) -> usize {
        self.support_blocking_finding_count
    }

    pub const fn boundary_audit_source_count(&self) -> usize {
        self.boundary_audit_source_count
    }

    pub const fn boundary_audit_coverage_row_count(&self) -> usize {
        self.boundary_audit_coverage_row_count
    }

    pub const fn synthetic_denial_localization_row_count(&self) -> usize {
        self.synthetic_denial_localization_row_count
    }

    pub const fn synthetic_replaced_row_count(&self) -> usize {
        self.synthetic_replaced_row_count
    }

    pub const fn explicit_residue_row_count(&self) -> usize {
        self.explicit_residue_row_count
    }

    pub const fn topology_read_touched_scope_count(&self) -> usize {
        self.topology_read_touched_scope_count
    }

    pub const fn topology_mutation_lane_touched_scope_count(&self) -> usize {
        self.topology_mutation_lane_touched_scope_count
    }

    pub const fn topology_graph_composed_lane_count(&self) -> usize {
        self.topology_graph_composed_lane_count
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

    pub const fn kernel_receipt_breadth_count(&self) -> usize {
        self.kernel_receipt_breadth_count
    }

    pub const fn kernel_lower_crate_receipt_family_count(&self) -> usize {
        self.kernel_lower_crate_receipt_family_count
    }

    pub const fn diagnostic_detail_row_count(&self) -> usize {
        self.diagnostic_detail_row_count
    }

    pub fn domain_outcome_fingerprint(&self) -> &str {
        &self.domain_outcome_fingerprint
    }
}

#[derive(Debug)]
pub enum WorthPhaseEightPerformanceCounterError {
    KernelAdoption(WorthKernelQueryConsumerKitAdoptionError),
    TopologyAdoption(WorthTopoQueryConsumerKitAdoptionError),
    SpatialAdoption(WorthSpatialQueryConsumerKitAdoptionError),
    SyntheticProof(WorthQuerySyntheticProofDispositionError),
    CompositionHonesty(WorthKernelCompositionHonestyError),
}

pub fn current_worth_phase_eight_performance_counter_report(
    diagnostic_policy: WorthPhaseEightDiagnosticPolicy,
) -> Result<WorthPhaseEightPerformanceCounterReport, WorthPhaseEightPerformanceCounterError> {
    let kernel = current_kernel_query_consumer_kit_adoption_status()
        .map_err(WorthPhaseEightPerformanceCounterError::KernelAdoption)?;
    let topology = current_topology_query_consumer_kit_adoption_status()
        .map_err(WorthPhaseEightPerformanceCounterError::TopologyAdoption)?;
    let spatial = current_spatial_query_consumer_kit_adoption_status()
        .map_err(WorthPhaseEightPerformanceCounterError::SpatialAdoption)?;
    let topology_counters = current_topology_phase_eight_performance_counters();
    let spatial_counters = current_spatial_phase_eight_performance_counters();
    let synthetic = WorthQuerySyntheticProofDispositionReport::current()
        .map_err(WorthPhaseEightPerformanceCounterError::SyntheticProof)?;
    let composition = current_kernel_composition_honesty_report()
        .map_err(WorthPhaseEightPerformanceCounterError::CompositionHonesty)?;

    let support_requirement_count = kernel.support_requirement_count()
        + topology.support_requirement_count()
        + spatial.support_requirement_count();
    let support_observed_row_count = kernel.support_observed_row_count()
        + topology.support_observed_row_count()
        + spatial.support_observed_row_count();
    let support_matched_required_count = kernel.support_matched_required_count()
        + topology.support_matched_required_count()
        + spatial.support_matched_required_count();
    let support_snapshot_row_count = kernel.support_snapshot_row_count()
        + topology.support_snapshot_row_count()
        + spatial.support_snapshot_row_count();
    let support_blocking_finding_count = kernel.support_blocking_finding_count()
        + topology.support_blocking_finding_count()
        + spatial.support_blocking_finding_count();
    let boundary_audit_source_count = kernel.boundary_audit_source_count()
        + topology.boundary_audit_source_count()
        + spatial.boundary_audit_source_count();
    let boundary_audit_coverage_row_count = kernel.boundary_audit_coverage_row_count()
        + topology.boundary_audit_coverage_row_count()
        + spatial.boundary_audit_coverage_row_count();
    let synthetic_denial_localization_row_count = synthetic
        .rows_for(WorthQuerySyntheticProofDisposition::DeniedByBoundary)
        .count();
    let synthetic_replaced_row_count = synthetic
        .rows_for(WorthQuerySyntheticProofDisposition::ReplacedByProductionSurface)
        .count();
    let explicit_residue_row_count = synthetic
        .rows_for(WorthQuerySyntheticProofDisposition::ExplicitResidue)
        .count();
    let diagnostic_detail_row_count = match diagnostic_policy {
        WorthPhaseEightDiagnosticPolicy::Minimal => 0,
        WorthPhaseEightDiagnosticPolicy::Rich => {
            boundary_audit_coverage_row_count + synthetic_denial_localization_row_count
        }
    };
    let domain_outcome_fingerprint = domain_outcome_fingerprint(
        support_requirement_count,
        support_observed_row_count,
        support_matched_required_count,
        support_snapshot_row_count,
        support_blocking_finding_count,
        boundary_audit_source_count,
        synthetic_denial_localization_row_count,
        topology_counters.query_read_family_touched_scope_count(),
        spatial_counters.witness_resolution_request_count(),
        spatial_counters.denied_witness_count(),
        spatial_counters.catalog_lookup_request_count(),
        composition.kernel_workload_receipt_family_count(),
    );

    Ok(WorthPhaseEightPerformanceCounterReport {
        support_requirement_count,
        support_observed_row_count,
        support_matched_required_count,
        support_snapshot_row_count,
        support_blocking_finding_count,
        boundary_audit_source_count,
        boundary_audit_coverage_row_count,
        synthetic_denial_localization_row_count,
        synthetic_replaced_row_count,
        explicit_residue_row_count,
        topology_read_touched_scope_count: topology_counters
            .query_read_family_touched_scope_count(),
        topology_mutation_lane_touched_scope_count: topology_counters
            .query_mutation_lane_touched_scope_count(),
        topology_graph_composed_lane_count: topology_counters.graph_composed_mutation_lane_count(),
        spatial_witness_resolution_request_count: spatial_counters
            .witness_resolution_request_count(),
        spatial_witness_denial_count: spatial_counters.denied_witness_count(),
        spatial_witness_catalog_lookup_count: spatial_counters.catalog_lookup_request_count(),
        kernel_receipt_breadth_count: composition.kernel_workload_receipt_family_count(),
        kernel_lower_crate_receipt_family_count: composition.lower_crate_receipt_family_count(),
        diagnostic_detail_row_count,
        domain_outcome_fingerprint,
    })
}

fn domain_outcome_fingerprint(
    support_requirement_count: usize,
    support_observed_row_count: usize,
    support_matched_required_count: usize,
    support_snapshot_row_count: usize,
    support_blocking_finding_count: usize,
    boundary_audit_source_count: usize,
    synthetic_denial_localization_row_count: usize,
    topology_read_touched_scope_count: usize,
    spatial_witness_resolution_request_count: usize,
    spatial_witness_denial_count: usize,
    spatial_witness_catalog_lookup_count: usize,
    kernel_receipt_breadth_count: usize,
) -> String {
    format!(
        "support_requirements={support_requirement_count};support_observed={support_observed_row_count};support_matched={support_matched_required_count};support_snapshot_rows={support_snapshot_row_count};blocking={support_blocking_finding_count};boundary_sources={boundary_audit_source_count};synthetic_denials={synthetic_denial_localization_row_count};topology_reads={topology_read_touched_scope_count};spatial_witnesses={spatial_witness_resolution_request_count};spatial_denials={spatial_witness_denial_count};spatial_catalog_lookups={spatial_witness_catalog_lookup_count};kernel_receipts={kernel_receipt_breadth_count}"
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_eight_counter_report_exposes_exact_cross_crate_breadth() {
        let report = current_worth_phase_eight_performance_counter_report(
            WorthPhaseEightDiagnosticPolicy::Minimal,
        )
        .expect("phase eight performance counter report");

        assert_eq!(report.support_requirement_count(), 9);
        assert_eq!(report.support_observed_row_count(), 3);
        assert_eq!(report.support_matched_required_count(), 9);
        assert_eq!(report.support_snapshot_row_count(), 66);
        assert_eq!(report.support_blocking_finding_count(), 0);
        assert_eq!(report.boundary_audit_source_count(), 6);
        assert!(report.boundary_audit_coverage_row_count() >= 6);
        assert_eq!(report.synthetic_denial_localization_row_count(), 5);
        assert_eq!(report.synthetic_replaced_row_count(), 5);
        assert_eq!(report.explicit_residue_row_count(), 3);
        assert_eq!(report.topology_read_touched_scope_count(), 4);
        assert_eq!(report.topology_mutation_lane_touched_scope_count(), 14);
        assert_eq!(report.topology_graph_composed_lane_count(), 7);
        assert_eq!(report.spatial_witness_resolution_request_count(), 8);
        assert_eq!(report.spatial_witness_denial_count(), 4);
        assert_eq!(report.spatial_witness_catalog_lookup_count(), 2);
        assert_eq!(report.kernel_receipt_breadth_count(), 8);
        assert_eq!(report.kernel_lower_crate_receipt_family_count(), 2);
    }

    #[test]
    fn phase_eight_diagnostic_policy_changes_richness_not_domain_outcome() {
        let minimal = current_worth_phase_eight_performance_counter_report(
            WorthPhaseEightDiagnosticPolicy::Minimal,
        )
        .expect("minimal report");
        let rich = current_worth_phase_eight_performance_counter_report(
            WorthPhaseEightDiagnosticPolicy::Rich,
        )
        .expect("rich report");

        assert_eq!(
            minimal.domain_outcome_fingerprint(),
            rich.domain_outcome_fingerprint()
        );
        assert_eq!(minimal.diagnostic_detail_row_count(), 0);
        assert!(rich.diagnostic_detail_row_count() > minimal.diagnostic_detail_row_count());
    }
}
