use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationSelectionCounters {
    touch_lookup_key_count: usize,
    operating_world_lookup_key_count: usize,
    attempted_bucket_lookup_count: usize,
    matched_bucket_count: usize,
    candidate_registration_count: usize,
    deduplicated_candidate_count: usize,
    matched_obligation_count: usize,
    registration_full_scan_count: usize,
    counters_digest: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct ForgeQueryGraphObligationSelectionCounterInput {
    pub touch_lookup_key_count: usize,
    pub operating_world_lookup_key_count: usize,
    pub attempted_bucket_lookup_count: usize,
    pub matched_bucket_count: usize,
    pub candidate_registration_count: usize,
    pub deduplicated_candidate_count: usize,
    pub matched_obligation_count: usize,
    pub registration_full_scan_count: usize,
}

impl ForgeQueryGraphObligationSelectionCounters {
    pub(super) fn new(input: ForgeQueryGraphObligationSelectionCounterInput) -> Self {
        let counters_digest = forge_query_evidence_identity(
            ForgeQueryEvidenceScope::GraphObligationSelectionCounters,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("touch_lookup_key_count"),
            input.touch_lookup_key_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("operating_world_lookup_key_count"),
            input.operating_world_lookup_key_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("attempted_bucket_lookup_count"),
            input.attempted_bucket_lookup_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("matched_bucket_count"),
            input.matched_bucket_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("candidate_registration_count"),
            input.candidate_registration_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("deduplicated_candidate_count"),
            input.deduplicated_candidate_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("matched_obligation_count"),
            input.matched_obligation_count,
        )
        .field_usize(
            ForgeQueryEvidenceTag::new("registration_full_scan_count"),
            input.registration_full_scan_count,
        )
        .seal();
        Self {
            touch_lookup_key_count: input.touch_lookup_key_count,
            operating_world_lookup_key_count: input.operating_world_lookup_key_count,
            attempted_bucket_lookup_count: input.attempted_bucket_lookup_count,
            matched_bucket_count: input.matched_bucket_count,
            candidate_registration_count: input.candidate_registration_count,
            deduplicated_candidate_count: input.deduplicated_candidate_count,
            matched_obligation_count: input.matched_obligation_count,
            registration_full_scan_count: input.registration_full_scan_count,
            counters_digest,
        }
    }

    pub fn touch_lookup_key_count(&self) -> usize {
        self.touch_lookup_key_count
    }

    pub fn operating_world_lookup_key_count(&self) -> usize {
        self.operating_world_lookup_key_count
    }

    pub fn attempted_bucket_lookup_count(&self) -> usize {
        self.attempted_bucket_lookup_count
    }

    pub fn matched_bucket_count(&self) -> usize {
        self.matched_bucket_count
    }

    pub fn visited_bucket_count(&self) -> usize {
        self.matched_bucket_count
    }

    pub fn candidate_registration_count(&self) -> usize {
        self.candidate_registration_count
    }

    pub fn deduplicated_candidate_count(&self) -> usize {
        self.deduplicated_candidate_count
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.matched_obligation_count
    }

    pub fn registration_full_scan_count(&self) -> usize {
        self.registration_full_scan_count
    }

    pub fn counters_digest(&self) -> &str {
        self.counters_digest.as_str()
    }

    pub(crate) fn counters_evidence_digest(&self) -> &ForgeQueryEvidenceIdentity {
        &self.counters_digest
    }
}
