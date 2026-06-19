use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};
use crate::runtime::ForgeQueryGraphObligationRegistrationCatalog;

use super::super::support::{
    ForgeQueryGraphObligationIndexComplexityContract, ForgeQueryGraphObligationIndexSupportRow,
};
use super::{ForgeQueryGraphObligationIndexBuildCounters, ForgeQueryGraphObligationIndexEntry};

pub(in crate::runtime::mutation::graph_composition::obligation::index) fn graph_obligation_index_digest(
    catalog: &ForgeQueryGraphObligationRegistrationCatalog,
    entries: &[ForgeQueryGraphObligationIndexEntry],
    support_rows: &[ForgeQueryGraphObligationIndexSupportRow],
    complexity_contracts: &[ForgeQueryGraphObligationIndexComplexityContract],
    build_counters: &ForgeQueryGraphObligationIndexBuildCounters,
    bucket_count: usize,
) -> ForgeQueryEvidenceIdentity {
    let entry_digests = entries
        .iter()
        .map(ForgeQueryGraphObligationIndexEntry::entry_evidence_digest)
        .collect::<Vec<_>>();
    let support_row_digests = support_rows
        .iter()
        .map(ForgeQueryGraphObligationIndexSupportRow::row_evidence_digest)
        .collect::<Vec<_>>();
    let complexity_contract_digests = complexity_contracts
        .iter()
        .map(ForgeQueryGraphObligationIndexComplexityContract::contract_evidence_digest)
        .collect::<Vec<_>>();
    forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationIndex)
        .field_value(
            ForgeQueryEvidenceTag::new("registration_catalog"),
            catalog.catalog_digest(),
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("registration_count"),
            entries.len(),
        )
        .field_usize(ForgeQueryEvidenceTag::new("bucket_count"), bucket_count)
        .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("entry"), entry_digests)
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("support_row"),
            support_row_digests,
        )
        .field_evidence_identity_sequence(
            ForgeQueryEvidenceTag::new("complexity_contract"),
            complexity_contract_digests,
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("build_counters"),
            build_counters.build_evidence_digest(),
        )
        .seal()
}
