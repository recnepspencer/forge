use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationSelectionCounters {
    touch_lookup_key_count: usize,
    operating_world_lookup_key_count: usize,
    attempted_bucket_lookup_count: usize,
    matched_bucket_count: usize,
    candidate_registration_count: usize,
    deduplicated_candidate_count: usize,
    matched_obligation_count: usize,
    registration_full_scan_count: usize,
    counters_digest: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct WorthQueryGraphObligationSelectionCounterInput {
    pub touch_lookup_key_count: usize,
    pub operating_world_lookup_key_count: usize,
    pub attempted_bucket_lookup_count: usize,
    pub matched_bucket_count: usize,
    pub candidate_registration_count: usize,
    pub deduplicated_candidate_count: usize,
    pub matched_obligation_count: usize,
    pub registration_full_scan_count: usize,
}

impl WorthQueryGraphObligationSelectionCounters {
    pub(super) fn new(input: WorthQueryGraphObligationSelectionCounterInput) -> Self {
        let counters_digest = worth_query_evidence_identity(
            WorthQueryEvidenceScope::GraphObligationSelectionCounters,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("touch_lookup_key_count"),
            input.touch_lookup_key_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("operating_world_lookup_key_count"),
            input.operating_world_lookup_key_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("attempted_bucket_lookup_count"),
            input.attempted_bucket_lookup_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("matched_bucket_count"),
            input.matched_bucket_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("candidate_registration_count"),
            input.candidate_registration_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("deduplicated_candidate_count"),
            input.deduplicated_candidate_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("matched_obligation_count"),
            input.matched_obligation_count,
        )
        .field_usize(
            WorthQueryEvidenceTag::new("registration_full_scan_count"),
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

    pub(crate) fn counters_evidence_digest(&self) -> &WorthQueryEvidenceIdentity {
        &self.counters_digest
    }
}
