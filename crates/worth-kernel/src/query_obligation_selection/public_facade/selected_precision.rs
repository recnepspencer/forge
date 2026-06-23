use super::super::selection_substrate::{
    QuerySelectorPrecisionPosture, QuerySelectorPrecisionReport,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySelectorPrecisionPosture {
    TouchedDescriptorBounded,
    BroadSelectorCappedResidue,
    QueryExpressivenessGap,
    CounterEvidenceUnbounded,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySelectorPrecisionReport {
    report: QuerySelectorPrecisionReport,
}

impl WorthQuerySelectorPrecisionReport {
    pub(crate) fn from_report(report: QuerySelectorPrecisionReport) -> Self {
        Self { report }
    }

    pub fn posture(&self) -> WorthQuerySelectorPrecisionPosture {
        self.report.posture().into()
    }

    pub fn is_touched_descriptor_bounded(&self) -> bool {
        self.report.is_touched_descriptor_bounded()
    }

    pub fn has_touched_descriptor_bounded_counters(&self) -> bool {
        self.report.has_touched_descriptor_bounded_counters()
    }

    pub fn has_clean_selector_closeout(&self) -> bool {
        self.report.has_clean_selector_closeout()
    }

    pub fn touch_lookup_key_count(&self) -> usize {
        self.report.touch_lookup_key_count()
    }

    pub fn operating_world_lookup_key_count(&self) -> usize {
        self.report.operating_world_lookup_key_count()
    }

    pub fn attempted_bucket_lookup_count(&self) -> usize {
        self.report.attempted_bucket_lookup_count()
    }

    pub fn matched_obligation_count(&self) -> usize {
        self.report.matched_obligation_count()
    }

    pub fn selected_obligation_count(&self) -> usize {
        self.report.selected_obligation_count()
    }

    pub fn registration_full_scan_count(&self) -> usize {
        self.report.registration_full_scan_count()
    }

    pub fn broad_selector_residue_count(&self) -> usize {
        self.report.broad_selector_residue_count()
    }

    pub fn query_selector_gap_count(&self) -> usize {
        self.report.query_selector_gap_count()
    }

    pub fn counters_digest(&self) -> &str {
        self.report.counters_digest()
    }

    pub fn report_digest(&self) -> &str {
        self.report.report_digest()
    }
}

impl From<QuerySelectorPrecisionPosture> for WorthQuerySelectorPrecisionPosture {
    fn from(posture: QuerySelectorPrecisionPosture) -> Self {
        match posture {
            QuerySelectorPrecisionPosture::TouchedDescriptorBounded => {
                Self::TouchedDescriptorBounded
            }
            QuerySelectorPrecisionPosture::BroadSelectorCappedResidue => {
                Self::BroadSelectorCappedResidue
            }
            QuerySelectorPrecisionPosture::QueryExpressivenessGap => Self::QueryExpressivenessGap,
            QuerySelectorPrecisionPosture::CounterEvidenceUnbounded => {
                Self::CounterEvidenceUnbounded
            }
        }
    }
}
