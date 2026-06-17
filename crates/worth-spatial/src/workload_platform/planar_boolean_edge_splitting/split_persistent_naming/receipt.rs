use super::counters::PlanarBooleanSplitPersistentNamingCounters;
use super::denial::PlanarBooleanSplitPersistentNamingDenial;
use super::identity::receipt_identity;
use super::input::PlanarBooleanSplitPersistentNamingInput;
use super::naming_row::{
    PlanarBooleanSplitPersistentNameRow, PlanarBooleanSplitSelectorResolutionRow,
    PlanarBooleanSplitSubshapeSignatureRow,
};
use super::query_evolution::{
    execute_split_identity_evolution, PlanarBooleanSplitIdentityEvolutionOutcomeKind,
    PlanarBooleanSplitIdentityEvolutionRow,
};
use super::row_building::build_persistent_name_rows;
use super::validation::validate_persistent_name_rows;

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarBooleanSplitPersistentNamingReceipt {
    receipt_identity: String,
    split_chain_validation_receipt_identity: String,
    split_edge_fragment_set_identity: String,
    split_vertex_identity_set_identity: String,
    overlap_edge_chain_set_identity: String,
    identity_evolution_rows: Vec<PlanarBooleanSplitIdentityEvolutionRow>,
    persistent_name_rows: Vec<PlanarBooleanSplitPersistentNameRow>,
    selector_resolution_rows: Vec<PlanarBooleanSplitSelectorResolutionRow>,
    subshape_signature_rows: Vec<PlanarBooleanSplitSubshapeSignatureRow>,
    counters: PlanarBooleanSplitPersistentNamingCounters,
}

impl PlanarBooleanSplitPersistentNamingReceipt {
    pub fn admit(
        input: PlanarBooleanSplitPersistentNamingInput<'_>,
    ) -> Result<Self, PlanarBooleanSplitPersistentNamingDenial> {
        input.validate_product_lineage()?;
        let mut counters = PlanarBooleanSplitPersistentNamingCounters::default();
        let source_edge_identities = source_edge_identities(&input);
        let mut identity_evolution_rows = Vec::new();
        for source_edge_identity in source_edge_identities {
            identity_evolution_rows.push(execute_split_identity_evolution(
                &source_edge_identity,
                input.query_basis(),
                input.split_chain_validation().receipt_identity(),
                &mut counters,
            )?);
        }
        let persistent_name_rows = build_persistent_name_rows(
            input.split_fragments(),
            input.split_vertices(),
            input.overlap_chains(),
            &identity_evolution_rows,
            &mut counters,
        )?;
        validate_persistent_name_rows(&persistent_name_rows, &mut counters)?;
        let selector_resolution_rows = persistent_name_rows
            .iter()
            .map(PlanarBooleanSplitSelectorResolutionRow::from_name_row)
            .inspect(|_| counters.emitted_selector_resolution_row())
            .collect::<Vec<_>>();
        let subshape_signature_rows = persistent_name_rows
            .iter()
            .map(PlanarBooleanSplitSubshapeSignatureRow::from_name_row)
            .inspect(|_| counters.emitted_subshape_signature_row())
            .collect::<Vec<_>>();
        let receipt_identity = receipt_identity(
            input.split_chain_validation().receipt_identity(),
            &identity_evolution_rows,
            &persistent_name_rows,
            &selector_resolution_rows,
            &subshape_signature_rows,
        );
        Ok(Self {
            receipt_identity,
            split_chain_validation_receipt_identity: input
                .split_chain_validation()
                .receipt_identity()
                .to_string(),
            split_edge_fragment_set_identity: input
                .split_fragments()
                .fragment_set_identity()
                .to_string(),
            split_vertex_identity_set_identity: input
                .split_vertices()
                .split_vertex_identity_set_identity()
                .to_string(),
            overlap_edge_chain_set_identity: input
                .overlap_chains()
                .chain_set_identity()
                .to_string(),
            identity_evolution_rows,
            persistent_name_rows,
            selector_resolution_rows,
            subshape_signature_rows,
            counters,
        })
    }

    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }
    pub fn split_chain_validation_receipt_identity(&self) -> &str {
        &self.split_chain_validation_receipt_identity
    }
    pub fn split_edge_fragment_set_identity(&self) -> &str {
        &self.split_edge_fragment_set_identity
    }
    pub fn split_vertex_identity_set_identity(&self) -> &str {
        &self.split_vertex_identity_set_identity
    }
    pub fn overlap_edge_chain_set_identity(&self) -> &str {
        &self.overlap_edge_chain_set_identity
    }
    pub fn identity_evolution_rows(&self) -> &[PlanarBooleanSplitIdentityEvolutionRow] {
        &self.identity_evolution_rows
    }
    pub fn persistent_name_rows(&self) -> &[PlanarBooleanSplitPersistentNameRow] {
        &self.persistent_name_rows
    }
    pub fn selector_resolution_rows(&self) -> &[PlanarBooleanSplitSelectorResolutionRow] {
        &self.selector_resolution_rows
    }
    pub fn subshape_signature_rows(&self) -> &[PlanarBooleanSplitSubshapeSignatureRow] {
        &self.subshape_signature_rows
    }
    pub fn counters(&self) -> PlanarBooleanSplitPersistentNamingCounters {
        self.counters
    }
    pub fn certifies_query_native_split_persistent_naming(&self) -> bool {
        !self.receipt_identity.is_empty()
            && self.identity_evolution_rows.iter().all(|row| {
                row.outcome_kind()
                    == PlanarBooleanSplitIdentityEvolutionOutcomeKind::PluralSplitSuccessors
            })
            && self.counters.source_identities_inspected() == self.identity_evolution_rows.len()
            && self.counters.identity_evolution_queries_admitted()
                == self.identity_evolution_rows.len()
            && self.counters.identity_evolution_queries_executed()
                == self.identity_evolution_rows.len()
            && self.counters.split_artifacts_named() == self.persistent_name_rows.len()
            && self.counters.selector_resolution_rows_emitted()
                == self.selector_resolution_rows.len()
            && self.counters.subshape_signature_rows_emitted() == self.subshape_signature_rows.len()
            && self.counters.duplicate_names_rejected() == 0
            && self.counters.dangling_references_rejected() == 0
            && self.counters.geometry_authority_attempts_rejected() == 0
            && self.counters.ambiguity_denials() == 0
            && self.counters.identity_break_denials() == 0
            && self.counters.foreign_artifact_denials() == 0
    }

    #[cfg(test)]
    pub(crate) fn with_rows_for_tests(
        &self,
        persistent_name_rows: Vec<PlanarBooleanSplitPersistentNameRow>,
        counters: PlanarBooleanSplitPersistentNamingCounters,
    ) -> Self {
        Self {
            receipt_identity: self.receipt_identity.clone(),
            split_chain_validation_receipt_identity: self
                .split_chain_validation_receipt_identity
                .clone(),
            split_edge_fragment_set_identity: self.split_edge_fragment_set_identity.clone(),
            split_vertex_identity_set_identity: self.split_vertex_identity_set_identity.clone(),
            overlap_edge_chain_set_identity: self.overlap_edge_chain_set_identity.clone(),
            identity_evolution_rows: self.identity_evolution_rows.clone(),
            persistent_name_rows,
            selector_resolution_rows: Vec::new(),
            subshape_signature_rows: Vec::new(),
            counters,
        }
    }
}

fn source_edge_identities(input: &PlanarBooleanSplitPersistentNamingInput<'_>) -> Vec<String> {
    let mut identities = input
        .split_fragments()
        .schedules()
        .iter()
        .map(|schedule| schedule.source_edge_identity().to_string())
        .collect::<Vec<_>>();
    identities.sort();
    identities.dedup();
    identities
}
