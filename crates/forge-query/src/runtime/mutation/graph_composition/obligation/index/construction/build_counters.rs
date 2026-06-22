use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationIndexBuildCounters {
    registration_count: usize,
    entry_count: usize,
    bucket_count: usize,
    support_row_count: usize,
    complexity_contract_count: usize,
    registration_full_scan_count: usize,
    build_digest: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime::mutation::graph_composition::obligation::index) struct ForgeQueryGraphObligationIndexBuildCounterInput
{
    pub registration_count: usize,
    pub entry_count: usize,
    pub bucket_count: usize,
    pub support_row_count: usize,
    pub complexity_contract_count: usize,
    pub registration_full_scan_count: usize,
}

impl ForgeQueryGraphObligationIndexBuildCounters {
    pub(in crate::runtime::mutation::graph_composition::obligation::index) fn new(
        input: ForgeQueryGraphObligationIndexBuildCounterInput,
    ) -> Self {
        let build_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationIndexBuildCounters,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("registration_count"),
            input.registration_count,
        )
        .field_usize(ForgeQueryEvidenceTag::new("entry_count"), input.entry_count)
        .field_usize(
            ForgeQueryEvidenceTag::new("bucket_count"),
            input.bucket_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("support_row_count"),
            input.support_row_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("complexity_contract_count"),
            input.complexity_contract_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("registration_full_scan_count"),
            input.registration_full_scan_count,
        )
        .seal();
        Self {
            registration_count: input.registration_count,
            entry_count: input.entry_count,
            bucket_count: input.bucket_count,
            support_row_count: input.support_row_count,
            complexity_contract_count: input.complexity_contract_count,
            registration_full_scan_count: input.registration_full_scan_count,
            build_digest,
        }
    }

    pub fn registration_count(&self) -> usize {
        self.registration_count
    }

    pub fn entry_count(&self) -> usize {
        self.entry_count
    }

    pub fn bucket_count(&self) -> usize {
        self.bucket_count
    }

    pub fn support_row_count(&self) -> usize {
        self.support_row_count
    }

    pub fn complexity_contract_count(&self) -> usize {
        self.complexity_contract_count
    }

    pub fn registration_full_scan_count(&self) -> usize {
        self.registration_full_scan_count
    }

    pub fn build_digest(&self) -> &str {
        self.build_digest.as_str()
    }

    pub(crate) fn build_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.build_digest
    }
}
