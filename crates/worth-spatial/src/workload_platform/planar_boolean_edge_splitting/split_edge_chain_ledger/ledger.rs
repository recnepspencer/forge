use std::collections::BTreeSet;

use super::counters::PlanarBooleanSplitEdgeChainLedgerCounters;
use super::denial::{
    PlanarBooleanSplitEdgeChainLedgerDenial, PlanarBooleanSplitEdgeChainLedgerDenialKind,
};
use super::edge_chain::PlanarBooleanSplitEdgeChain;
use super::identity;
use super::input::PlanarBooleanSplitEdgeChainLedgerInput;
use super::ordering::SplitLedgerScheduleBindings;
use super::product_index::PlanarBooleanSplitEdgeChainProductIndex;
use super::receipt::PlanarBooleanSplitEdgeChainLedgerReceipt;
use super::validation::validate_product_lineage;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarBooleanSplitEdgeChainLedger {
    ledger_identity: String,
    declaration_identity: String,
    split_request_identity: String,
    event_ledger_lookup_selected_plan_digest: String,
    event_ledger_lookup_execution_receipt_digest: String,
    event_ledger_lookup_product_output_digest: String,
    split_chain_validation_receipt_identity: String,
    split_persistent_naming_receipt_identity: String,
    split_decision_log_receipt_identity: String,
    chains: Vec<PlanarBooleanSplitEdgeChain>,
    counters: PlanarBooleanSplitEdgeChainLedgerCounters,
}

impl PlanarBooleanSplitEdgeChainLedger {
    pub(crate) fn assemble(
        input: PlanarBooleanSplitEdgeChainLedgerInput<'_>,
    ) -> Result<
        (Self, PlanarBooleanSplitEdgeChainLedgerReceipt),
        PlanarBooleanSplitEdgeChainLedgerDenial,
    > {
        let mut counters = PlanarBooleanSplitEdgeChainLedgerCounters::default();
        validate_product_lineage(&input, &mut counters)?;
        let index = PlanarBooleanSplitEdgeChainProductIndex::build(&input, &mut counters)?;
        let schedule_bindings = SplitLedgerScheduleBindings::from_input(&input);
        let mut chains = Vec::new();
        let mut chain_identities = BTreeSet::new();
        for key in index.edge_keys() {
            let fragment_identities = index.fragment_identities(&key);
            let vertex_identities = index.vertex_identities(&key);
            let overlap_chain_identities = index.overlap_chain_identities(&key);
            let artifact_identities = artifact_identities(
                &fragment_identities,
                &vertex_identities,
                &overlap_chain_identities,
            );
            let persistent_name_row_identities = index.name_row_identities(&artifact_identities);
            let decision_identities = index.decision_identities(&artifact_identities);
            let chain_authority_bindings =
                ChainAuthorityBindings::from_indexed_products(&key, &schedule_bindings, &index);
            reject_incomplete_chain_authority_bindings(
                &key,
                &chain_authority_bindings,
                &mut counters,
            )?;
            let chain_identity = identity::chain_identity(
                input.declaration().declaration_identity(),
                &key.0,
                &key.1,
                &fragment_identities,
                &overlap_chain_identities,
                &persistent_name_row_identities,
                &decision_identities,
            );
            if !chain_identities.insert(chain_identity.clone()) {
                counters.rejected_duplicate_chain_identity();
                return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
                    PlanarBooleanSplitEdgeChainLedgerDenialKind::DuplicateLedgerChainIdentity,
                    chain_identity,
                    counters,
                    "split ledger chain identities must be unique",
                ));
            }
            counters.emitted_chain();
            chains.push(PlanarBooleanSplitEdgeChain::new(
                chain_identity,
                key.0.clone(),
                key.1.clone(),
                chain_authority_bindings.endpoint_boundary_schedule_identity,
                chain_authority_bindings.interval_subdivision_schedule_identity,
                chain_authority_bindings.vertex_schedule_identity,
                chain_authority_bindings.fragment_schedule_identity,
                fragment_identities,
                vertex_identities,
                overlap_chain_identities,
                persistent_name_row_identities,
                decision_identities,
                chain_authority_bindings.fragment_coverage_identities,
                chain_authority_bindings.overlap_coverage_identities,
            ));
        }
        chains.sort_by(|left, right| left.chain_identity().cmp(right.chain_identity()));
        let chain_identities = chains
            .iter()
            .map(|chain| chain.chain_identity().to_string())
            .collect::<Vec<_>>();
        let ledger_identity = identity::ledger_identity(
            input.declaration().declaration_identity(),
            &chain_identities,
        );
        let ledger = Self {
            ledger_identity,
            declaration_identity: input.declaration().declaration_identity().to_string(),
            split_request_identity: input.split_request().split_request_identity().to_string(),
            event_ledger_lookup_selected_plan_digest: input
                .split_request()
                .event_ledger_lookup_selected_plan_digest()
                .to_string(),
            event_ledger_lookup_execution_receipt_digest: input
                .split_request()
                .event_ledger_lookup_execution_receipt_digest()
                .to_string(),
            event_ledger_lookup_product_output_digest: input
                .split_request()
                .event_ledger_lookup_product_output_digest()
                .to_string(),
            split_chain_validation_receipt_identity: input
                .split_chain_validation()
                .receipt_identity()
                .to_string(),
            split_persistent_naming_receipt_identity: input
                .split_persistent_names()
                .receipt_identity()
                .to_string(),
            split_decision_log_receipt_identity: input
                .split_decision_log()
                .receipt()
                .receipt_identity()
                .to_string(),
            chains,
            counters,
        };
        let receipt = PlanarBooleanSplitEdgeChainLedgerReceipt::from_ledger(&ledger);
        Ok((ledger, receipt))
    }

    pub fn ledger_identity(&self) -> &str {
        &self.ledger_identity
    }
    pub fn declaration_identity(&self) -> &str {
        &self.declaration_identity
    }
    pub fn split_request_identity(&self) -> &str {
        &self.split_request_identity
    }
    pub fn event_ledger_lookup_selected_plan_digest(&self) -> &str {
        &self.event_ledger_lookup_selected_plan_digest
    }
    pub fn event_ledger_lookup_execution_receipt_digest(&self) -> &str {
        &self.event_ledger_lookup_execution_receipt_digest
    }
    pub fn event_ledger_lookup_product_output_digest(&self) -> &str {
        &self.event_ledger_lookup_product_output_digest
    }
    pub fn split_chain_validation_receipt_identity(&self) -> &str {
        &self.split_chain_validation_receipt_identity
    }
    pub fn split_persistent_naming_receipt_identity(&self) -> &str {
        &self.split_persistent_naming_receipt_identity
    }
    pub fn split_decision_log_receipt_identity(&self) -> &str {
        &self.split_decision_log_receipt_identity
    }
    pub fn chains(&self) -> &[PlanarBooleanSplitEdgeChain] {
        &self.chains
    }
    pub fn counters(&self) -> PlanarBooleanSplitEdgeChainLedgerCounters {
        self.counters
    }
}

pub(super) struct ChainAuthorityBindings {
    pub(super) endpoint_boundary_schedule_identity: String,
    pub(super) interval_subdivision_schedule_identity: String,
    pub(super) vertex_schedule_identity: String,
    pub(super) fragment_schedule_identity: String,
    pub(super) fragment_coverage_identities: Vec<String>,
    pub(super) overlap_coverage_identities: Vec<String>,
}

impl ChainAuthorityBindings {
    fn from_indexed_products(
        key: &(String, String),
        schedule_bindings: &SplitLedgerScheduleBindings,
        index: &PlanarBooleanSplitEdgeChainProductIndex<'_>,
    ) -> Self {
        Self {
            endpoint_boundary_schedule_identity: schedule_bindings
                .endpoint_boundary_schedule_identity(key),
            interval_subdivision_schedule_identity: schedule_bindings
                .interval_subdivision_schedule_identity(key),
            vertex_schedule_identity: schedule_bindings.vertex_schedule_identity(key),
            fragment_schedule_identity: schedule_bindings.fragment_schedule_identity(key),
            fragment_coverage_identities: index.fragment_coverage_identities(key),
            overlap_coverage_identities: index.overlap_coverage_identities(key),
        }
    }
}

pub(super) fn reject_incomplete_chain_authority_bindings(
    key: &(String, String),
    chain_authority_bindings: &ChainAuthorityBindings,
    counters: &mut PlanarBooleanSplitEdgeChainLedgerCounters,
) -> Result<(), PlanarBooleanSplitEdgeChainLedgerDenial> {
    let missing_schedule_binding = chain_authority_bindings
        .endpoint_boundary_schedule_identity
        .is_empty()
        || chain_authority_bindings
            .interval_subdivision_schedule_identity
            .is_empty()
        || chain_authority_bindings.vertex_schedule_identity.is_empty()
        || chain_authority_bindings
            .fragment_schedule_identity
            .is_empty();
    if missing_schedule_binding {
        counters.rejected_missing_validation();
        return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingScheduleBinding,
            format!("{}:{}", key.0, key.1),
            *counters,
            "split ledger chain requires every upstream schedule binding",
        ));
    }
    if chain_authority_bindings
        .fragment_coverage_identities
        .is_empty()
    {
        counters.rejected_missing_validation();
        return Err(PlanarBooleanSplitEdgeChainLedgerDenial::new(
            PlanarBooleanSplitEdgeChainLedgerDenialKind::MissingFragmentValidationCoverage,
            format!("{}:{}", key.0, key.1),
            *counters,
            "split ledger chain requires fragment validation coverage",
        ));
    }
    Ok(())
}

fn artifact_identities(
    fragment_identities: &[String],
    vertex_identities: &[String],
    overlap_chain_identities: &[String],
) -> Vec<String> {
    fragment_identities
        .iter()
        .chain(vertex_identities.iter())
        .chain(overlap_chain_identities.iter())
        .cloned()
        .collect()
}
