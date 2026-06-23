use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationAdoptionManifest,
    ForgeQueryGraphObligationExecutionBackedAdoptionProof, ForgeQueryGraphObligationExecutionProof,
    ForgeQueryGraphObligationResidueManifest,
};
use forge_query::facade::runtime::ForgeQueryGraphObligationSelectionCounters;
use worth_spatial::facade::workload_vocabulary::SpatialEvidenceQueryGapRow;

use super::local_ceremony_closeout::QuerySelectionLocalCeremonyCloseout;
use super::selection_request::{
    QueryObligationSelectionAuthorityKind, QueryObligationSelectionInput,
};
use super::selector_precision::{
    QueryBroadSelectorResidueRows, QuerySelectorExpressivenessGaps, QuerySelectorPrecisionReport,
};

#[derive(Clone, Debug)]
pub struct QuerySelectedGraphObligations {
    input: QueryObligationSelectionInput,
    proof: ForgeQueryGraphObligationExecutionBackedAdoptionProof,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectedGraphObligationCloseout {
    authority_kind: QueryObligationSelectionAuthorityKind,
    authority_digest: String,
    touch_descriptor_digest: String,
    operating_world_digest: String,
    selected_registration_digests: Vec<String>,
    selected_obligation_count: usize,
    execution_row_count: usize,
    execution_proof_digest: String,
    adoption_manifest_digest: String,
    residue_manifest_digest: String,
    selection_counters: ForgeQueryGraphObligationSelectionCounters,
    spatial_touch_digest: Option<String>,
    spatial_lookup_product_digest: Option<String>,
    spatial_query_gap_rows: Vec<SpatialEvidenceQueryGapRow>,
    query_selector_gap_rows: QuerySelectorExpressivenessGaps,
    broad_selector_residue_rows: QueryBroadSelectorResidueRows,
    selector_precision_report: QuerySelectorPrecisionReport,
    local_ceremony_closeout: QuerySelectionLocalCeremonyCloseout,
    graph_read_access_planning_claimed: bool,
}

impl QuerySelectedGraphObligations {
    pub(crate) fn from_query_proof(
        input: QueryObligationSelectionInput,
        proof: ForgeQueryGraphObligationExecutionBackedAdoptionProof,
    ) -> Self {
        Self { input, proof }
    }

    pub fn execution_proof(&self) -> &ForgeQueryGraphObligationExecutionProof {
        self.proof.execution_proof()
    }

    pub fn query_proof(&self) -> &ForgeQueryGraphObligationExecutionBackedAdoptionProof {
        &self.proof
    }

    pub fn manifest(&self) -> &ForgeQueryGraphObligationAdoptionManifest {
        self.proof.manifest()
    }

    pub fn residue_manifest(&self) -> &ForgeQueryGraphObligationResidueManifest {
        self.proof.residue_manifest()
    }

    pub fn closeout(&self) -> QuerySelectedGraphObligationCloseout {
        let local_ceremony_closeout =
            QuerySelectionLocalCeremonyCloseout::from_audit(self.proof.local_ceremony_audit());
        QuerySelectedGraphObligationCloseout {
            authority_kind: self.authority_kind(),
            authority_digest: self.authority_digest().to_string(),
            touch_descriptor_digest: self.touch_descriptor_digest().to_string(),
            operating_world_digest: self.operating_world_digest().to_string(),
            selected_registration_digests: self.selected_registration_digests(),
            selected_obligation_count: self.selected_obligation_count(),
            execution_row_count: self.execution_row_count(),
            execution_proof_digest: self.execution_proof_digest().to_string(),
            adoption_manifest_digest: self.adoption_manifest_digest().to_string(),
            residue_manifest_digest: self.manifest().residue_manifest_digest().to_string(),
            selection_counters: self.selection_counters().clone(),
            spatial_touch_digest: self.spatial_touch_digest().map(ToOwned::to_owned),
            spatial_lookup_product_digest: self
                .spatial_lookup_product_digest()
                .map(ToOwned::to_owned),
            spatial_query_gap_rows: self.spatial_query_gap_rows().to_vec(),
            query_selector_gap_rows: self.query_selector_gap_rows(),
            broad_selector_residue_rows: self.broad_selector_residue_rows(),
            selector_precision_report: self.selector_precision_report(),
            local_ceremony_closeout,
            graph_read_access_planning_claimed: self.graph_read_access_planning_claimed(),
        }
    }

    pub fn selection_counters(&self) -> &ForgeQueryGraphObligationSelectionCounters {
        self.execution_proof()
            .selection_proof()
            .selection_counters()
    }

    pub fn selected_registration_digests(&self) -> Vec<String> {
        self.execution_proof()
            .selection_proof()
            .selected_registration_digests()
            .map(ToOwned::to_owned)
            .collect()
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.execution_proof().selected_obligation_count()
    }

    pub fn execution_row_count(&self) -> usize {
        self.execution_proof().rows().len()
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        self.input.touch_descriptor().descriptor_digest()
    }

    pub fn operating_world_digest(&self) -> &str {
        self.input.operating_world().descriptor_digest()
    }

    pub fn authority_digest(&self) -> &str {
        self.input.authority_digest()
    }

    pub fn spatial_touch_digest(&self) -> Option<&str> {
        self.input
            .spatial_descriptor()
            .map(|descriptor| descriptor.spatial_touch_digest().as_str())
    }

    pub fn spatial_lookup_product_digest(&self) -> Option<&str> {
        self.input
            .spatial_descriptor()
            .map(|descriptor| descriptor.lookup_product_digest().as_str())
    }

    pub fn spatial_query_gap_rows(&self) -> &[SpatialEvidenceQueryGapRow] {
        self.input
            .spatial_descriptor()
            .map_or(&[], |descriptor| descriptor.gap_rows())
    }

    pub fn query_selector_gap_rows(&self) -> QuerySelectorExpressivenessGaps {
        QuerySelectorExpressivenessGaps::from_spatial_gap_rows(self.spatial_query_gap_rows())
    }

    pub fn broad_selector_residue_rows(&self) -> QueryBroadSelectorResidueRows {
        QueryBroadSelectorResidueRows::from_residue_manifest(self.residue_manifest())
    }

    pub fn selector_precision_report(&self) -> QuerySelectorPrecisionReport {
        QuerySelectorPrecisionReport::from_selected(self)
    }

    pub fn graph_read_access_planning_claimed(&self) -> bool {
        false
    }

    pub const fn authority_kind(&self) -> QueryObligationSelectionAuthorityKind {
        self.input.authority_kind()
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        self.proof.manifest().manifest_digest()
    }

    pub fn execution_proof_digest(&self) -> &str {
        self.execution_proof().proof_digest()
    }
}

impl QuerySelectedGraphObligationCloseout {
    pub const fn authority_kind(&self) -> QueryObligationSelectionAuthorityKind {
        self.authority_kind
    }

    pub fn authority_digest(&self) -> &str {
        &self.authority_digest
    }

    pub fn touch_descriptor_digest(&self) -> &str {
        &self.touch_descriptor_digest
    }

    pub fn operating_world_digest(&self) -> &str {
        &self.operating_world_digest
    }

    pub fn selected_registration_digests(&self) -> &[String] {
        &self.selected_registration_digests
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub fn execution_row_count(&self) -> usize {
        self.execution_row_count
    }

    pub fn execution_proof_digest(&self) -> &str {
        &self.execution_proof_digest
    }

    pub fn adoption_manifest_digest(&self) -> &str {
        &self.adoption_manifest_digest
    }

    pub fn residue_manifest_digest(&self) -> &str {
        &self.residue_manifest_digest
    }

    pub fn selection_counters(&self) -> &ForgeQueryGraphObligationSelectionCounters {
        &self.selection_counters
    }

    pub fn spatial_touch_digest(&self) -> Option<&str> {
        self.spatial_touch_digest.as_deref()
    }

    pub fn spatial_lookup_product_digest(&self) -> Option<&str> {
        self.spatial_lookup_product_digest.as_deref()
    }

    pub fn spatial_query_gap_rows(&self) -> &[SpatialEvidenceQueryGapRow] {
        &self.spatial_query_gap_rows
    }

    pub fn query_selector_gap_rows(&self) -> &QuerySelectorExpressivenessGaps {
        &self.query_selector_gap_rows
    }

    pub fn broad_selector_residue_rows(&self) -> &QueryBroadSelectorResidueRows {
        &self.broad_selector_residue_rows
    }

    pub fn selector_precision_report(&self) -> &QuerySelectorPrecisionReport {
        &self.selector_precision_report
    }

    pub fn local_ceremony_closeout(&self) -> &QuerySelectionLocalCeremonyCloseout {
        &self.local_ceremony_closeout
    }

    pub fn graph_read_access_planning_claimed(&self) -> bool {
        self.graph_read_access_planning_claimed
    }
}
