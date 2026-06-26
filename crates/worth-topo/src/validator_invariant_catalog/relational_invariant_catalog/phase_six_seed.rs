#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthTopologyRelationalInvariantCatalogPhaseSixSeed {
    catalog_digest: String,
    selected_plan_digest: String,
    routing_closure_digest: String,
    validator_phase_five_seed_digest: String,
    query_registration_projection_digest: String,
    query_registration_bundle_digest: String,
    ordinary_authority_admission_digest: String,
    old_pack_residue_digest: String,
    source_firewall_digest: String,
    counters_digest: String,
    selected_invariant_family_count: usize,
    selected_validator_family_count: usize,
    execution_receipt_count: usize,
    selected_invariant_family_row_digests: Vec<String>,
    selected_validator_family_row_digests: Vec<String>,
    query_graph_obligation_registration_digests: Vec<String>,
    seed_digest: String,
}

impl WorthTopologyRelationalInvariantCatalogPhaseSixSeed {
    pub(in crate::validator_invariant_catalog) fn from_parts(
        catalog_digest: &str,
        selected_plan_digest: &str,
        routing_closure_digest: &str,
        validator_phase_five_seed_digest: &str,
        query_registration_projection_digest: &str,
        query_registration_bundle_digest: &str,
        ordinary_authority_admission_digest: &str,
        old_pack_residue_digest: &str,
        source_firewall_digest: &str,
        counters_digest: &str,
        selected_invariant_family_count: usize,
        selected_validator_family_count: usize,
        execution_receipt_count: usize,
        selected_invariant_family_row_digests: Vec<String>,
        selected_validator_family_row_digests: Vec<String>,
        query_graph_obligation_registration_digests: Vec<String>,
    ) -> Self {
        let mut seed_parts = vec![
            "worth-topo-relational-invariant-catalog-phase-six-seed-v1",
            catalog_digest,
            selected_plan_digest,
            routing_closure_digest,
            validator_phase_five_seed_digest,
            query_registration_projection_digest,
            query_registration_bundle_digest,
            ordinary_authority_admission_digest,
            old_pack_residue_digest,
            source_firewall_digest,
            counters_digest,
        ];
        let selected_invariant_family_count_string = selected_invariant_family_count.to_string();
        let selected_validator_family_count_string = selected_validator_family_count.to_string();
        let execution_receipt_count_string = execution_receipt_count.to_string();
        seed_parts.push(&selected_invariant_family_count_string);
        seed_parts.push(&selected_validator_family_count_string);
        seed_parts.push(&execution_receipt_count_string);
        let mut owned_seed_parts = seed_parts
            .into_iter()
            .map(str::to_string)
            .collect::<Vec<_>>();
        owned_seed_parts.extend(
            selected_invariant_family_row_digests
                .iter()
                .map(|digest| format!("selected-invariant:{digest}")),
        );
        owned_seed_parts.extend(
            selected_validator_family_row_digests
                .iter()
                .map(|digest| format!("selected-validator:{digest}")),
        );
        owned_seed_parts.extend(
            query_graph_obligation_registration_digests
                .iter()
                .map(|digest| format!("query-graph-obligation:{digest}")),
        );
        let seed_digest = owned_seed_parts.join("|");
        Self {
            catalog_digest: catalog_digest.to_string(),
            selected_plan_digest: selected_plan_digest.to_string(),
            routing_closure_digest: routing_closure_digest.to_string(),
            validator_phase_five_seed_digest: validator_phase_five_seed_digest.to_string(),
            query_registration_projection_digest: query_registration_projection_digest.to_string(),
            query_registration_bundle_digest: query_registration_bundle_digest.to_string(),
            ordinary_authority_admission_digest: ordinary_authority_admission_digest.to_string(),
            old_pack_residue_digest: old_pack_residue_digest.to_string(),
            source_firewall_digest: source_firewall_digest.to_string(),
            counters_digest: counters_digest.to_string(),
            selected_invariant_family_count,
            selected_validator_family_count,
            execution_receipt_count,
            selected_invariant_family_row_digests,
            selected_validator_family_row_digests,
            query_graph_obligation_registration_digests,
            seed_digest,
        }
    }

    pub fn catalog_digest(&self) -> &str {
        &self.catalog_digest
    }

    pub fn selected_plan_digest(&self) -> &str {
        &self.selected_plan_digest
    }

    pub fn routing_closure_digest(&self) -> &str {
        &self.routing_closure_digest
    }

    pub fn validator_phase_five_seed_digest(&self) -> &str {
        &self.validator_phase_five_seed_digest
    }

    pub fn query_registration_projection_digest(&self) -> &str {
        &self.query_registration_projection_digest
    }

    pub fn query_registration_bundle_digest(&self) -> &str {
        &self.query_registration_bundle_digest
    }

    pub fn ordinary_authority_admission_digest(&self) -> &str {
        &self.ordinary_authority_admission_digest
    }

    pub fn old_pack_residue_digest(&self) -> &str {
        &self.old_pack_residue_digest
    }

    pub fn source_firewall_digest(&self) -> &str {
        &self.source_firewall_digest
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub const fn selected_invariant_family_count(&self) -> usize {
        self.selected_invariant_family_count
    }

    pub const fn selected_validator_family_count(&self) -> usize {
        self.selected_validator_family_count
    }

    pub const fn execution_receipt_count(&self) -> usize {
        self.execution_receipt_count
    }

    pub fn selected_invariant_family_row_digests(&self) -> &[String] {
        &self.selected_invariant_family_row_digests
    }

    pub fn selected_validator_family_row_digests(&self) -> &[String] {
        &self.selected_validator_family_row_digests
    }

    pub fn query_graph_obligation_registration_digests(&self) -> &[String] {
        &self.query_graph_obligation_registration_digests
    }

    pub fn seed_digest(&self) -> &str {
        &self.seed_digest
    }
}
