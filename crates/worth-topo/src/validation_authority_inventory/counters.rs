use super::authority_kind::WorthValidationAuthorityKind;
use super::disposition::WorthValidationAuthorityDisposition;
use super::inventory_row::WorthValidationAuthorityInventoryRow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WorthValidationAuthorityInventoryCounters {
    total_source_rows: usize,
    validator_report_rows: usize,
    rule_registry_rows: usize,
    invariant_registration_rows: usize,
    certification_expectation_rows: usize,
    whole_view_comparison_rows: usize,
    migrate_rows: usize,
    delete_rows: usize,
    cap_rows: usize,
    query_access_gap_rows: usize,
    out_of_scope_rows: usize,
}

impl WorthValidationAuthorityInventoryCounters {
    pub(super) fn from_rows(rows: &[WorthValidationAuthorityInventoryRow]) -> Self {
        let mut counters = Self {
            total_source_rows: rows.len(),
            ..Self::default()
        };
        for row in rows {
            counters.count_kind(row.authority_kind());
            counters.count_disposition(row.disposition());
            if row.certification_only_comparison_allowed() {
                counters.whole_view_comparison_rows += 1;
            }
        }
        counters
    }

    fn count_kind(&mut self, kind: WorthValidationAuthorityKind) {
        match kind {
            WorthValidationAuthorityKind::WholeViewValidatorEntry => {
                self.validator_report_rows += 1;
            }
            WorthValidationAuthorityKind::DerivedRuleRegistryEntry => {
                self.rule_registry_rows += 1;
            }
            WorthValidationAuthorityKind::RuntimeInvariantRegistrationPack => {
                self.invariant_registration_rows += 1;
            }
            WorthValidationAuthorityKind::CertificationExpectationArray => {
                self.certification_expectation_rows += 1;
            }
            WorthValidationAuthorityKind::OperatorCloseoutValidationProof
            | WorthValidationAuthorityKind::CertificationComparisonReport => {}
        }
    }

    fn count_disposition(&mut self, disposition: WorthValidationAuthorityDisposition) {
        match disposition {
            WorthValidationAuthorityDisposition::Migrate => self.migrate_rows += 1,
            WorthValidationAuthorityDisposition::Delete => self.delete_rows += 1,
            WorthValidationAuthorityDisposition::Cap => self.cap_rows += 1,
            WorthValidationAuthorityDisposition::QueryAccessGap => self.query_access_gap_rows += 1,
            WorthValidationAuthorityDisposition::OutOfScope => self.out_of_scope_rows += 1,
        }
    }

    pub const fn total_source_rows(&self) -> usize {
        self.total_source_rows
    }

    pub const fn validator_report_rows(&self) -> usize {
        self.validator_report_rows
    }

    pub const fn rule_registry_rows(&self) -> usize {
        self.rule_registry_rows
    }

    pub const fn invariant_registration_rows(&self) -> usize {
        self.invariant_registration_rows
    }

    pub const fn certification_expectation_rows(&self) -> usize {
        self.certification_expectation_rows
    }

    pub const fn whole_view_comparison_rows(&self) -> usize {
        self.whole_view_comparison_rows
    }

    pub const fn migrate_rows(&self) -> usize {
        self.migrate_rows
    }

    pub const fn delete_rows(&self) -> usize {
        self.delete_rows
    }

    pub const fn cap_rows(&self) -> usize {
        self.cap_rows
    }

    pub const fn query_access_gap_rows(&self) -> usize {
        self.query_access_gap_rows
    }
}
