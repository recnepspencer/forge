#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceLookupMilestoneTwelveReplayReadinessPosture {
    PreCloseoutStageCutoverOnly,
    LookupScopeBoundedNoReplay,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupMilestoneTwelveSeed {
    milestone_eleven_closeout_digest: String,
    selected_route_family_identity: String,
    selected_compiled_product_identity_digest: String,
    selected_equivalence_family_identity: String,
    selected_reuse_basis_identity_digest: String,
    selected_lookup_plan_digest: String,
    lookup_execution_receipt_digest: String,
    lookup_product_output_digest: String,
    covered_family_identities: Vec<String>,
    query_surface_matrix_digest: String,
    query_consumer_kit_closeout_digest: String,
    source_firewall_digest: String,
    residue_audit_digest: String,
    family_coverage_digest: String,
    family_stage_row_count: usize,
    receipt_proof_row_count: usize,
    non_ordinary_residue_row_count: usize,
    query_imported_family_count: usize,
    topology_required_family_count: usize,
    replay_readiness_posture: EvidenceLookupMilestoneTwelveReplayReadinessPosture,
}

impl EvidenceLookupMilestoneTwelveSeed {
    pub(crate) fn new_stage_cutover(
        selected_lookup_plan_digest: String,
        lookup_execution_receipt_digest: String,
        lookup_product_output_digest: String,
        covered_family_identities: Vec<String>,
    ) -> Self {
        let covered_family_count = covered_family_identities.len();
        Self {
            milestone_eleven_closeout_digest: "not-closed".to_string(),
            selected_route_family_identity: "not-bound".to_string(),
            selected_compiled_product_identity_digest: "not-bound".to_string(),
            selected_equivalence_family_identity: "not-bound".to_string(),
            selected_reuse_basis_identity_digest: "not-bound".to_string(),
            selected_lookup_plan_digest,
            lookup_execution_receipt_digest,
            lookup_product_output_digest,
            covered_family_identities,
            query_surface_matrix_digest: "not-bound".to_string(),
            query_consumer_kit_closeout_digest: "not-bound".to_string(),
            source_firewall_digest: "not-bound".to_string(),
            residue_audit_digest: "not-bound".to_string(),
            family_coverage_digest: "stage-cutover-only".to_string(),
            family_stage_row_count: covered_family_count,
            receipt_proof_row_count: covered_family_count,
            non_ordinary_residue_row_count: 0,
            query_imported_family_count: 0,
            topology_required_family_count: 0,
            replay_readiness_posture:
                EvidenceLookupMilestoneTwelveReplayReadinessPosture::PreCloseoutStageCutoverOnly,
        }
    }

    pub(crate) fn new_public_closeout(
        milestone_eleven_closeout_digest: String,
        selected_route_family_identity: String,
        selected_compiled_product_identity_digest: String,
        selected_equivalence_family_identity: String,
        selected_reuse_basis_identity_digest: String,
        selected_lookup_plan_digest: String,
        lookup_execution_receipt_digest: String,
        lookup_product_output_digest: String,
        covered_family_identities: Vec<String>,
        query_surface_matrix_digest: String,
        query_consumer_kit_closeout_digest: String,
        source_firewall_digest: String,
        residue_audit_digest: String,
        family_coverage_digest: String,
        family_stage_row_count: usize,
        receipt_proof_row_count: usize,
        non_ordinary_residue_row_count: usize,
        query_imported_family_count: usize,
        topology_required_family_count: usize,
    ) -> Self {
        Self {
            milestone_eleven_closeout_digest,
            selected_route_family_identity,
            selected_compiled_product_identity_digest,
            selected_equivalence_family_identity,
            selected_reuse_basis_identity_digest,
            selected_lookup_plan_digest,
            lookup_execution_receipt_digest,
            lookup_product_output_digest,
            covered_family_identities,
            query_surface_matrix_digest,
            query_consumer_kit_closeout_digest,
            source_firewall_digest,
            residue_audit_digest,
            family_coverage_digest,
            family_stage_row_count,
            receipt_proof_row_count,
            non_ordinary_residue_row_count,
            query_imported_family_count,
            topology_required_family_count,
            replay_readiness_posture:
                EvidenceLookupMilestoneTwelveReplayReadinessPosture::LookupScopeBoundedNoReplay,
        }
    }

    pub fn milestone_eleven_closeout_digest(&self) -> &str {
        &self.milestone_eleven_closeout_digest
    }

    pub fn selected_route_family_identity(&self) -> &str {
        &self.selected_route_family_identity
    }

    pub fn selected_compiled_product_identity_digest(&self) -> &str {
        &self.selected_compiled_product_identity_digest
    }

    pub fn selected_equivalence_family_identity(&self) -> &str {
        &self.selected_equivalence_family_identity
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
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

    pub fn covered_family_identities(&self) -> &[String] {
        &self.covered_family_identities
    }

    pub fn query_surface_matrix_digest(&self) -> &str {
        &self.query_surface_matrix_digest
    }

    pub fn query_consumer_kit_closeout_digest(&self) -> &str {
        &self.query_consumer_kit_closeout_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn residue_audit_digest(&self) -> &str {
        &self.residue_audit_digest
    }

    pub fn family_coverage_digest(&self) -> &str {
        &self.family_coverage_digest
    }

    pub const fn family_stage_row_count(&self) -> usize {
        self.family_stage_row_count
    }

    pub const fn receipt_proof_row_count(&self) -> usize {
        self.receipt_proof_row_count
    }

    pub const fn non_ordinary_residue_row_count(&self) -> usize {
        self.non_ordinary_residue_row_count
    }

    pub const fn query_imported_family_count(&self) -> usize {
        self.query_imported_family_count
    }

    pub const fn topology_required_family_count(&self) -> usize {
        self.topology_required_family_count
    }

    pub const fn replay_readiness_posture(
        &self,
    ) -> EvidenceLookupMilestoneTwelveReplayReadinessPosture {
        self.replay_readiness_posture
    }
}
