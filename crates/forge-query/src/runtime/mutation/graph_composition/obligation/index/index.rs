use crate::evidence_identity::ForgeQueryEvidenceIdentity;

use super::construction::{
    graph_obligation_index_digest, ForgeQueryGraphObligationIndexBuildCounterInput,
    ForgeQueryGraphObligationIndexBuildCounters, ForgeQueryGraphObligationIndexEntry,
    ForgeQueryGraphObligationIndexRegistrationBuckets, GraphObligationBuckets,
};
use super::selection::{
    select_graph_obligations_from_buckets, ForgeQueryGraphObligationOperatingWorldDescriptor,
    ForgeQueryGraphObligationSelection,
};
use super::support::{
    graph_obligation_index_complexity_contracts, ForgeQueryGraphObligationIndexComplexityContract,
};
use super::support::{
    graph_obligation_index_support_rows, ForgeQueryGraphObligationIndexSupportRow,
};
use crate::runtime::{
    ForgeQueryGraphObligationRegistrationCatalog, ForgeQueryGraphTouchDescriptor,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationIndex {
    entries: Vec<ForgeQueryGraphObligationIndexEntry>,
    buckets: GraphObligationBuckets,
    support_rows: Vec<ForgeQueryGraphObligationIndexSupportRow>,
    complexity_contracts: Vec<ForgeQueryGraphObligationIndexComplexityContract>,
    build_counters: ForgeQueryGraphObligationIndexBuildCounters,
    index_digest: ForgeQueryEvidenceIdentity,
}

impl ForgeQueryGraphObligationIndex {
    pub fn from_catalog(catalog: &ForgeQueryGraphObligationRegistrationCatalog) -> Self {
        let registration_buckets =
            ForgeQueryGraphObligationIndexRegistrationBuckets::from_catalog(catalog);
        let support_rows = graph_obligation_index_support_rows();
        let complexity_contracts = graph_obligation_index_complexity_contracts();
        let build_counters = ForgeQueryGraphObligationIndexBuildCounters::new(
            ForgeQueryGraphObligationIndexBuildCounterInput {
                registration_count: catalog.registration_count(),
                entry_count: registration_buckets.entry_count(),
                bucket_count: registration_buckets.bucket_count(),
                support_row_count: support_rows.len(),
                complexity_contract_count: complexity_contracts.len(),
                registration_full_scan_count: catalog.registration_count(),
            },
        );
        let index_digest = graph_obligation_index_digest(
            catalog,
            registration_buckets.entries(),
            &support_rows,
            &complexity_contracts,
            &build_counters,
            registration_buckets.bucket_count(),
        );
        let (entries, buckets) = registration_buckets.into_parts();
        Self {
            entries,
            buckets,
            support_rows,
            complexity_contracts,
            build_counters,
            index_digest,
        }
    }

    pub fn empty() -> Self {
        Self::from_catalog(&ForgeQueryGraphObligationRegistrationCatalog::empty())
    }

    pub fn entries(&self) -> &[ForgeQueryGraphObligationIndexEntry] {
        &self.entries
    }

    pub fn support_rows(&self) -> &[ForgeQueryGraphObligationIndexSupportRow] {
        &self.support_rows
    }

    pub fn complexity_contracts(&self) -> &[ForgeQueryGraphObligationIndexComplexityContract] {
        &self.complexity_contracts
    }

    pub fn build_counters(&self) -> &ForgeQueryGraphObligationIndexBuildCounters {
        &self.build_counters
    }

    pub fn registration_count(&self) -> usize {
        self.entries.len()
    }

    pub fn bucket_count(&self) -> usize {
        self.buckets.len()
    }

    pub fn index_digest(&self) -> &str {
        self.index_digest.as_str()
    }

    pub fn select_for_touch(
        &self,
        touch_descriptor: &ForgeQueryGraphTouchDescriptor,
        operating_world: &ForgeQueryGraphObligationOperatingWorldDescriptor,
    ) -> ForgeQueryGraphObligationSelection {
        select_graph_obligations_from_buckets(
            self.index_digest(),
            &self.buckets,
            touch_descriptor,
            operating_world,
        )
    }
}
