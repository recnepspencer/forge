use forge_query::facade::consumer_kit::{
    graph_read_bypass_adoption, ForgeQueryBoundaryAuditError,
    ForgeQueryGraphReadBypassAdoptionError, ForgeQueryGraphReadBypassAdoptionProof,
};

use super::bypass_audit::audit_construction_graph_read_bypass;
use super::residue_manifest::construction_graph_read_residue_manifest;
use super::source_inventory::{construction_dir, construction_graph_read_source_inventory};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthKernelGraphReadAccessAdoptionReport {
    covered_roots: Vec<String>,
    audited_source_labels: Vec<String>,
    source_inventory_identity: String,
    source_inventory_count: usize,
    evaluated_source_count: usize,
    unclassified_finding_count: usize,
    adoption_manifest_digest: String,
}

#[derive(Debug)]
pub enum WorthKernelGraphReadAccessAdoptionError {
    Audit(ForgeQueryBoundaryAuditError),
    Adoption(ForgeQueryGraphReadBypassAdoptionError),
}

pub fn current_worth_kernel_construction_graph_read_access_adoption(
) -> Result<WorthKernelGraphReadAccessAdoptionReport, WorthKernelGraphReadAccessAdoptionError> {
    let inventory = construction_graph_read_source_inventory()?;
    let source_inventory_identity = inventory
        .inventory_identity()
        .terminal_projection_for_reporting()
        .to_string();
    let source_inventory_count = inventory.boundary_sources().sources().len();
    let report = audit_construction_graph_read_bypass(&inventory)?;
    let evaluated_source_count = report.counters().evaluated_source_count();
    let audited_source_labels = report.audited_source_labels().to_vec();
    let adoption = graph_read_bypass_adoption("worth-kernel-phase-17-construction")
        .audit_report(report)
        .residue_manifest(construction_graph_read_residue_manifest())
        .certify()?;
    Ok(WorthKernelGraphReadAccessAdoptionReport::from_adoption(
        adoption,
        vec![construction_dir().display().to_string()],
        audited_source_labels,
        source_inventory_identity,
        source_inventory_count,
        evaluated_source_count,
    ))
}

impl WorthKernelGraphReadAccessAdoptionReport {
    fn from_adoption(
        adoption: ForgeQueryGraphReadBypassAdoptionProof,
        covered_roots: Vec<String>,
        audited_source_labels: Vec<String>,
        source_inventory_identity: String,
        source_inventory_count: usize,
        evaluated_source_count: usize,
    ) -> Self {
        Self {
            covered_roots,
            audited_source_labels,
            source_inventory_identity,
            source_inventory_count,
            evaluated_source_count,
            unclassified_finding_count: adoption.unclassified_finding_count(),
            adoption_manifest_digest: adoption.manifest().manifest_digest().to_string(),
        }
    }

    pub fn source_inventory_count(&self) -> usize {
        self.source_inventory_count
    }

    pub fn covered_roots(&self) -> &[String] {
        &self.covered_roots
    }

    pub fn audited_source_labels(&self) -> &[String] {
        &self.audited_source_labels
    }

    pub fn source_inventory_identity(&self) -> &str {
        &self.source_inventory_identity
    }

    pub fn evaluated_source_count(&self) -> usize {
        self.evaluated_source_count
    }

    pub fn unclassified_finding_count(&self) -> usize {
        self.unclassified_finding_count
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
    }
}

impl From<ForgeQueryBoundaryAuditError> for WorthKernelGraphReadAccessAdoptionError {
    fn from(error: ForgeQueryBoundaryAuditError) -> Self {
        Self::Audit(error)
    }
}

impl From<ForgeQueryGraphReadBypassAdoptionError> for WorthKernelGraphReadAccessAdoptionError {
    fn from(error: ForgeQueryGraphReadBypassAdoptionError) -> Self {
        Self::Adoption(error)
    }
}
