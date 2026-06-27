use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupExecutionCounters {
    selected_family_count: usize,
    selected_region_count: usize,
    evidence_candidate_count: usize,
    ledger_rows_touched_count: usize,
    index_rows_consumed_count: usize,
    resident_byte_count: usize,
    indexed_hit_count: usize,
    indexed_miss_count: usize,
    caller_owned_scan_count: usize,
    query_artifact_count: usize,
}

impl EvidenceLookupExecutionCounters {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        selected_family_count: usize,
        selected_region_count: usize,
        evidence_candidate_count: usize,
        ledger_rows_touched_count: usize,
        index_rows_consumed_count: usize,
        resident_byte_count: usize,
        indexed_hit_count: usize,
        indexed_miss_count: usize,
        caller_owned_scan_count: usize,
        query_artifact_count: usize,
    ) -> Self {
        Self {
            selected_family_count,
            selected_region_count,
            evidence_candidate_count,
            ledger_rows_touched_count,
            index_rows_consumed_count,
            resident_byte_count,
            indexed_hit_count,
            indexed_miss_count,
            caller_owned_scan_count,
            query_artifact_count,
        }
    }

    pub(crate) fn digest(self) -> String {
        truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-execution-counters:v1".to_string(),
                format!("selected-families:{}", self.selected_family_count),
                format!("selected-regions:{}", self.selected_region_count),
                format!("evidence-candidates:{}", self.evidence_candidate_count),
                format!("ledger-rows-touched:{}", self.ledger_rows_touched_count),
                format!("index-rows-consumed:{}", self.index_rows_consumed_count),
                format!("resident-bytes:{}", self.resident_byte_count),
                format!("indexed-hits:{}", self.indexed_hit_count),
                format!("indexed-misses:{}", self.indexed_miss_count),
                format!("caller-owned-scans:{}", self.caller_owned_scan_count),
                format!("query-artifacts:{}", self.query_artifact_count),
            ],
        )
    }

    pub fn selected_family_count(&self) -> usize {
        self.selected_family_count
    }

    pub fn selected_region_count(&self) -> usize {
        self.selected_region_count
    }

    pub fn evidence_candidate_count(&self) -> usize {
        self.evidence_candidate_count
    }

    pub fn ledger_rows_touched_count(&self) -> usize {
        self.ledger_rows_touched_count
    }

    pub fn index_rows_consumed_count(&self) -> usize {
        self.index_rows_consumed_count
    }

    pub fn resident_byte_count(&self) -> usize {
        self.resident_byte_count
    }

    pub fn indexed_hit_count(&self) -> usize {
        self.indexed_hit_count
    }

    pub fn indexed_miss_count(&self) -> usize {
        self.indexed_miss_count
    }

    pub fn caller_owned_scan_count(&self) -> usize {
        self.caller_owned_scan_count
    }

    pub fn query_artifact_count(&self) -> usize {
        self.query_artifact_count
    }
}
