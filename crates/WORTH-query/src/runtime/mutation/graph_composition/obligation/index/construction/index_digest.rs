use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::WorthQueryGraphObligationRegistrationCatalog;

use super::super::support::{
    WorthQueryGraphObligationIndexComplexityContract, WorthQueryGraphObligationIndexSupportRow,
};
use super::{WorthQueryGraphObligationIndexBuildCounters, WorthQueryGraphObligationIndexEntry};

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_digest(
    catalog: &WorthQueryGraphObligationRegistrationCatalog,
    entries: &[WorthQueryGraphObligationIndexEntry],
    support_rows: &[WorthQueryGraphObligationIndexSupportRow],
    complexity_contracts: &[WorthQueryGraphObligationIndexComplexityContract],
    build_counters: &WorthQueryGraphObligationIndexBuildCounters,
    bucket_count: usize,
) -> WorthQueryEvidenceIdentity {
    let entry_digests = entries
        .iter()
        .map(WorthQueryGraphObligationIndexEntry::entry_evidence_digest)
        .collect::<Vec<_>>();
    let support_row_digests = support_rows
        .iter()
        .map(WorthQueryGraphObligationIndexSupportRow::row_evidence_digest)
        .collect::<Vec<_>>();
    let complexity_contract_digests = complexity_contracts
        .iter()
        .map(WorthQueryGraphObligationIndexComplexityContract::contract_evidence_digest)
        .collect::<Vec<_>>();
    worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationIndex)
        .field_value(
            WorthQueryEvidenceTag::new("registration_catalog"),
            catalog.catalog_digest(),
        )
        .field_usize(
            WorthQueryEvidenceTag::new("registration_count"),
            entries.len(),
        )
        .field_usize(WorthQueryEvidenceTag::new("bucket_count"), bucket_count)
        .field_evidence_identity_sequence(WorthQueryEvidenceTag::new("entry"), entry_digests)
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("support_row"),
            support_row_digests,
        )
        .field_evidence_identity_sequence(
            WorthQueryEvidenceTag::new("complexity_contract"),
            complexity_contract_digests,
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("build_counters"),
            build_counters.build_evidence_digest(),
        )
        .seal()
}
