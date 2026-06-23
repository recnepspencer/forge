use forge_query::facade::runtime::ForgeQueryGraphObligationSelectionCounters;

use super::{QueryBroadSelectorResidueRows, QuerySelectorExpressivenessGaps};
use crate::query_obligation_selection::selection_substrate::selected_obligations::QuerySelectedGraphObligations;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QuerySelectorPrecisionPosture {
    TouchedDescriptorBounded,
    BroadSelectorCappedResidue,
    QueryExpressivenessGap,
    CounterEvidenceUnbounded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySelectorPrecisionReport {
    posture: QuerySelectorPrecisionPosture,
    touch_lookup_key_count: usize,
    operating_world_lookup_key_count: usize,
    attempted_bucket_lookup_count: usize,
    matched_bucket_count: usize,
    candidate_registration_count: usize,
    deduplicated_candidate_count: usize,
    matched_obligation_count: usize,
    selected_obligation_count: usize,
    registration_full_scan_count: usize,
    broad_selector_residue_count: usize,
    query_selector_gap_count: usize,
    counters_digest: String,
    report_digest: String,
}

impl QuerySelectorPrecisionReport {
    pub(crate) fn from_selected(selected: &QuerySelectedGraphObligations) -> Self {
        let counters = selected.selection_counters();
        let broad_selector_residue = selected.broad_selector_residue_rows();
        let query_selector_gaps = selected.query_selector_gap_rows();
        let posture = classify_posture(&broad_selector_residue, &query_selector_gaps);
        let counters_digest = counters.counters_digest().to_string();
        let selected_obligation_count = selected.selected_obligation_count();
        let report_digest = precision_report_digest(
            posture,
            counters,
            selected_obligation_count,
            broad_selector_residue.len(),
            query_selector_gaps.len(),
        );
        Self {
            posture,
            touch_lookup_key_count: counters.touch_lookup_key_count(),
            operating_world_lookup_key_count: counters.operating_world_lookup_key_count(),
            attempted_bucket_lookup_count: counters.attempted_bucket_lookup_count(),
            matched_bucket_count: counters.matched_bucket_count(),
            candidate_registration_count: counters.candidate_registration_count(),
            deduplicated_candidate_count: counters.deduplicated_candidate_count(),
            matched_obligation_count: counters.matched_obligation_count(),
            selected_obligation_count,
            registration_full_scan_count: counters.registration_full_scan_count(),
            broad_selector_residue_count: broad_selector_residue.len(),
            query_selector_gap_count: query_selector_gaps.len(),
            counters_digest,
            report_digest,
        }
    }

    pub(crate) fn from_counter_only_certification(
        counters: &ForgeQueryGraphObligationSelectionCounters,
        selected_obligation_count: usize,
    ) -> Self {
        let posture = counter_only_precision_posture(counters, selected_obligation_count);
        let report_digest =
            precision_report_digest(posture, counters, selected_obligation_count, 0, 0);
        Self {
            posture,
            touch_lookup_key_count: counters.touch_lookup_key_count(),
            operating_world_lookup_key_count: counters.operating_world_lookup_key_count(),
            attempted_bucket_lookup_count: counters.attempted_bucket_lookup_count(),
            matched_bucket_count: counters.matched_bucket_count(),
            candidate_registration_count: counters.candidate_registration_count(),
            deduplicated_candidate_count: counters.deduplicated_candidate_count(),
            matched_obligation_count: counters.matched_obligation_count(),
            selected_obligation_count,
            registration_full_scan_count: counters.registration_full_scan_count(),
            broad_selector_residue_count: 0,
            query_selector_gap_count: 0,
            counters_digest: counters.counters_digest().to_string(),
            report_digest,
        }
    }

    pub fn posture(&self) -> QuerySelectorPrecisionPosture {
        self.posture
    }

    pub fn is_touched_descriptor_bounded(&self) -> bool {
        self.posture == QuerySelectorPrecisionPosture::TouchedDescriptorBounded
            && self.has_touched_descriptor_bounded_counters()
            && !self.has_broad_selector_residue()
            && !self.has_query_selector_gaps()
    }

    pub fn has_touched_descriptor_bounded_counters(&self) -> bool {
        has_touched_descriptor_bounded_counters(
            self.touch_lookup_key_count,
            self.operating_world_lookup_key_count,
            self.attempted_bucket_lookup_count,
            self.registration_full_scan_count,
            self.matched_obligation_count,
            self.selected_obligation_count,
        )
    }

    pub fn has_broad_selector_residue(&self) -> bool {
        self.broad_selector_residue_count > 0
    }

    pub fn has_query_selector_gaps(&self) -> bool {
        self.query_selector_gap_count > 0
    }

    pub fn has_unbounded_counter_evidence(&self) -> bool {
        !self.has_touched_descriptor_bounded_counters()
    }

    pub fn has_clean_selector_closeout(&self) -> bool {
        self.registration_full_scan_count == 0 && self.is_touched_descriptor_bounded()
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

    pub fn candidate_registration_count(&self) -> usize {
        self.candidate_registration_count
    }

    pub fn deduplicated_candidate_count(&self) -> usize {
        self.deduplicated_candidate_count
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.matched_obligation_count
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.selected_obligation_count
    }

    pub fn registration_full_scan_count(&self) -> usize {
        self.registration_full_scan_count
    }

    pub fn broad_selector_residue_count(&self) -> usize {
        self.broad_selector_residue_count
    }

    pub fn query_selector_gap_count(&self) -> usize {
        self.query_selector_gap_count
    }

    pub fn counters_digest(&self) -> &str {
        &self.counters_digest
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

fn classify_posture(
    broad_selector_residue: &QueryBroadSelectorResidueRows,
    query_selector_gaps: &QuerySelectorExpressivenessGaps,
) -> QuerySelectorPrecisionPosture {
    if !query_selector_gaps.is_empty() {
        QuerySelectorPrecisionPosture::QueryExpressivenessGap
    } else if !broad_selector_residue.is_empty() {
        QuerySelectorPrecisionPosture::BroadSelectorCappedResidue
    } else {
        QuerySelectorPrecisionPosture::TouchedDescriptorBounded
    }
}

fn counter_only_precision_posture(
    counters: &ForgeQueryGraphObligationSelectionCounters,
    selected_obligation_count: usize,
) -> QuerySelectorPrecisionPosture {
    if has_touched_descriptor_bounded_counters(
        counters.touch_lookup_key_count(),
        counters.operating_world_lookup_key_count(),
        counters.attempted_bucket_lookup_count(),
        counters.registration_full_scan_count(),
        counters.matched_obligation_count(),
        selected_obligation_count,
    ) {
        QuerySelectorPrecisionPosture::TouchedDescriptorBounded
    } else {
        QuerySelectorPrecisionPosture::CounterEvidenceUnbounded
    }
}

fn has_touched_descriptor_bounded_counters(
    touch_lookup_key_count: usize,
    operating_world_lookup_key_count: usize,
    attempted_bucket_lookup_count: usize,
    registration_full_scan_count: usize,
    matched_obligation_count: usize,
    selected_obligation_count: usize,
) -> bool {
    let Some(expected_bucket_lookups) =
        touch_lookup_key_count.checked_mul(operating_world_lookup_key_count)
    else {
        return false;
    };
    registration_full_scan_count == 0
        && attempted_bucket_lookup_count == expected_bucket_lookups
        && matched_obligation_count == selected_obligation_count
}

fn precision_report_digest(
    posture: QuerySelectorPrecisionPosture,
    counters: &ForgeQueryGraphObligationSelectionCounters,
    selected_obligation_count: usize,
    broad_selector_residue_count: usize,
    query_selector_gap_count: usize,
) -> String {
    format!(
        "worth.query.selector-precision:{}:{}:{}:{}:{}",
        posture.as_str(),
        counters.counters_digest(),
        selected_obligation_count,
        broad_selector_residue_count,
        query_selector_gap_count
    )
}

impl QuerySelectorPrecisionPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::TouchedDescriptorBounded => "touched-descriptor-bounded",
            Self::BroadSelectorCappedResidue => "broad-selector-capped-residue",
            Self::QueryExpressivenessGap => "query-expressiveness-gap",
            Self::CounterEvidenceUnbounded => "counter-evidence-unbounded",
        }
    }
}
