use forge_query::facade::consumer_kit::ForgeQueryConsumerResidueClass;
use forge_query::facade::runtime::ForgeQueryRuntimeFacadeFamily;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;
use crate::workload_platform::evidence_lookup_query_surface_matrix::EvidenceLookupQuerySurfaceTouchpoint;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryConsumerKitBindingRow {
    family_identity: String,
    stage: WorkloadEvidenceStage,
    touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    query_surface: EvidenceLookupQuerySurface,
    matrix_row_digest: String,
    query_surface_proof_digest: Option<String>,
    support_pin_report_digest: Option<String>,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySupportPinRow {
    runtime_family: ForgeQueryRuntimeFacadeFamily,
    source_touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    required_query_surface: EvidenceLookupQuerySurface,
    query_support_surface: String,
    snapshot_row_digest: String,
    support_pin_report_digest: String,
    row_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQueryConsumerResidueRow {
    source_path: String,
    line: usize,
    column: usize,
    residue_class: ForgeQueryConsumerResidueClass,
    finding_identity: String,
    report_identity: String,
    source_inventory_digest: String,
    row_digest: String,
}

impl EvidenceLookupQueryConsumerKitBindingRow {
    pub(crate) fn from_matrix_row(
        family_identity: &str,
        stage: WorkloadEvidenceStage,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
        query_surface: EvidenceLookupQuerySurface,
        matrix_row_digest: &str,
        query_surface_proof_digest: Option<&str>,
        support_pin_report_digest: Option<&str>,
    ) -> Self {
        let row_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-query-consumer-kit-binding-row:v1".to_string(),
                family_identity.to_string(),
                format!("{stage:?}"),
                touchpoint.as_str().to_string(),
                format!("{query_surface:?}"),
                matrix_row_digest.to_string(),
                query_surface_proof_digest
                    .unwrap_or("not-required")
                    .to_string(),
                support_pin_report_digest
                    .unwrap_or("not-required")
                    .to_string(),
            ],
        );
        Self {
            family_identity: family_identity.to_string(),
            stage,
            touchpoint,
            query_surface,
            matrix_row_digest: matrix_row_digest.to_string(),
            query_surface_proof_digest: query_surface_proof_digest.map(str::to_string),
            support_pin_report_digest: support_pin_report_digest.map(str::to_string),
            row_digest,
        }
    }

    pub fn family_identity(&self) -> &str {
        &self.family_identity
    }

    pub const fn stage(&self) -> WorkloadEvidenceStage {
        self.stage
    }

    pub const fn touchpoint(&self) -> EvidenceLookupQuerySurfaceTouchpoint {
        self.touchpoint
    }

    pub const fn query_surface(&self) -> EvidenceLookupQuerySurface {
        self.query_surface
    }

    pub fn matrix_row_digest(&self) -> &str {
        &self.matrix_row_digest
    }

    pub fn query_surface_proof_digest(&self) -> Option<&str> {
        self.query_surface_proof_digest.as_deref()
    }

    pub fn support_pin_report_digest(&self) -> Option<&str> {
        self.support_pin_report_digest.as_deref()
    }

    pub fn requires_support_pin_linkage(&self) -> bool {
        self.query_surface == EvidenceLookupQuerySurface::SupportPinning
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl EvidenceLookupQuerySupportPinRow {
    pub(crate) fn new(
        runtime_family: ForgeQueryRuntimeFacadeFamily,
        source_touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
        required_query_surface: EvidenceLookupQuerySurface,
        query_support_surface: impl Into<String>,
        snapshot_row_digest: impl Into<String>,
        support_pin_report_digest: impl Into<String>,
    ) -> Self {
        let query_support_surface = query_support_surface.into();
        let snapshot_row_digest = snapshot_row_digest.into();
        let support_pin_report_digest = support_pin_report_digest.into();
        let row_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-query-support-pin-row:v1".to_string(),
                runtime_family.as_str().to_string(),
                source_touchpoint.as_str().to_string(),
                format!("{required_query_surface:?}"),
                query_support_surface.clone(),
                snapshot_row_digest.clone(),
                support_pin_report_digest.clone(),
            ],
        );
        Self {
            runtime_family,
            source_touchpoint,
            required_query_surface,
            query_support_surface,
            snapshot_row_digest,
            support_pin_report_digest,
            row_digest,
        }
    }

    pub const fn runtime_family(&self) -> ForgeQueryRuntimeFacadeFamily {
        self.runtime_family
    }

    pub const fn source_touchpoint(&self) -> EvidenceLookupQuerySurfaceTouchpoint {
        self.source_touchpoint
    }

    pub const fn required_query_surface(&self) -> EvidenceLookupQuerySurface {
        self.required_query_surface
    }

    pub fn query_support_surface(&self) -> &str {
        &self.query_support_surface
    }

    pub fn snapshot_row_digest(&self) -> &str {
        &self.snapshot_row_digest
    }

    pub fn support_pin_report_digest(&self) -> &str {
        &self.support_pin_report_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

impl EvidenceLookupQueryConsumerResidueRow {
    pub(crate) fn new(
        source_path: impl Into<String>,
        line: usize,
        column: usize,
        residue_class: ForgeQueryConsumerResidueClass,
        finding_identity: impl Into<String>,
        report_identity: impl Into<String>,
        source_inventory_digest: impl Into<String>,
    ) -> Self {
        let source_path = source_path.into();
        let finding_identity = finding_identity.into();
        let report_identity = report_identity.into();
        let source_inventory_digest = source_inventory_digest.into();
        let row_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-query-consumer-residue-row:v1".to_string(),
                source_path.clone(),
                line.to_string(),
                column.to_string(),
                residue_class.as_str().to_string(),
                finding_identity.clone(),
                report_identity.clone(),
                source_inventory_digest.clone(),
            ],
        );
        Self {
            source_path,
            line,
            column,
            residue_class,
            finding_identity,
            report_identity,
            source_inventory_digest,
            row_digest,
        }
    }

    pub fn source_path(&self) -> &str {
        &self.source_path
    }

    pub const fn line(&self) -> usize {
        self.line
    }

    pub const fn column(&self) -> usize {
        self.column
    }

    pub const fn residue_class(&self) -> ForgeQueryConsumerResidueClass {
        self.residue_class
    }

    pub fn finding_identity(&self) -> &str {
        &self.finding_identity
    }

    pub fn report_identity(&self) -> &str {
        &self.report_identity
    }

    pub fn source_inventory_digest(&self) -> &str {
        &self.source_inventory_digest
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}
