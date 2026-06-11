#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleValidationCounters {
    inspected_bundle_rows: usize,
    consumed_certificate_families: usize,
    projection_consumed_rows: usize,
    retained_fact_rows: usize,
    support_posture_rows: usize,
    rejected_missing_family_rows: usize,
}

impl PlanarContractBundleValidationCounters {
    pub(crate) fn certified(
        inspected_bundle_rows: usize,
        consumed_certificate_families: usize,
        projection_consumed_rows: usize,
        retained_fact_rows: usize,
        support_posture_rows: usize,
    ) -> Self {
        Self {
            inspected_bundle_rows,
            consumed_certificate_families,
            projection_consumed_rows,
            retained_fact_rows,
            support_posture_rows,
            rejected_missing_family_rows: 0,
        }
    }

    pub(crate) fn rejected_missing_family() -> Self {
        Self {
            inspected_bundle_rows: 0,
            consumed_certificate_families: 0,
            projection_consumed_rows: 0,
            retained_fact_rows: 0,
            support_posture_rows: 0,
            rejected_missing_family_rows: 1,
        }
    }

    pub fn inspected_bundle_rows(self) -> usize {
        self.inspected_bundle_rows
    }

    pub fn consumed_certificate_families(self) -> usize {
        self.consumed_certificate_families
    }

    pub fn projection_consumed_rows(self) -> usize {
        self.projection_consumed_rows
    }

    pub fn retained_fact_rows(self) -> usize {
        self.retained_fact_rows
    }

    pub fn support_posture_rows(self) -> usize {
        self.support_posture_rows
    }

    pub fn rejected_missing_family_rows(self) -> usize {
        self.rejected_missing_family_rows
    }
}
