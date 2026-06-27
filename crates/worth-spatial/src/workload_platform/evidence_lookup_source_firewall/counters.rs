use super::row::{
    EvidenceLookupForbiddenAuthorityKind, EvidenceLookupSourceFirewallRow,
    EvidenceLookupSourceFirewallRowPosture,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupSourceFirewallCounters {
    scanned_root_count: usize,
    scanned_file_count: usize,
    total_row_count: usize,
    forbidden_row_count: usize,
    allowed_exception_row_count: usize,
    certification_only_exception_row_count: usize,
    documentation_report_exception_row_count: usize,
    test_support_exception_row_count: usize,
    raw_vector_row_count: usize,
    broad_receipt_scan_row_count: usize,
    copied_digest_row_count: usize,
    stage_local_nearby_row_count: usize,
    query_lookup_substitution_row_count: usize,
    public_exposure_row_count: usize,
}

impl EvidenceLookupSourceFirewallCounters {
    pub(crate) fn from_rows(
        scanned_root_count: usize,
        scanned_file_count: usize,
        rows: &[EvidenceLookupSourceFirewallRow],
    ) -> Self {
        let mut counters = Self {
            scanned_root_count,
            scanned_file_count,
            total_row_count: rows.len(),
            ..Self::default()
        };
        for row in rows {
            counters.count_row(row);
        }
        counters
    }

    fn count_row(&mut self, row: &EvidenceLookupSourceFirewallRow) {
        match row.posture() {
            EvidenceLookupSourceFirewallRowPosture::ForbiddenProductionAuthority => {
                self.forbidden_row_count += 1;
            }
            EvidenceLookupSourceFirewallRowPosture::AllowedNamedException => {
                self.allowed_exception_row_count += 1;
                self.count_exception(row);
            }
        }
        match row.forbidden_authority_kind() {
            EvidenceLookupForbiddenAuthorityKind::RawEvidenceVectorAccess => {
                self.raw_vector_row_count += 1;
            }
            EvidenceLookupForbiddenAuthorityKind::BroadReceiptScan => {
                self.broad_receipt_scan_row_count += 1;
            }
            EvidenceLookupForbiddenAuthorityKind::CopiedDigestLookup => {
                self.copied_digest_row_count += 1;
            }
            EvidenceLookupForbiddenAuthorityKind::StageLocalNearbyLookup => {
                self.stage_local_nearby_row_count += 1;
            }
            EvidenceLookupForbiddenAuthorityKind::QueryLookupProductSubstitution => {
                self.query_lookup_substitution_row_count += 1;
            }
            EvidenceLookupForbiddenAuthorityKind::PublicEvidenceRowExposure => {
                self.public_exposure_row_count += 1;
            }
        }
    }

    fn count_exception(&mut self, row: &EvidenceLookupSourceFirewallRow) {
        match row.exception_kind() {
            Some(super::row::EvidenceLookupSourceFirewallExceptionKind::CertificationOnlyCodec) => {
                self.certification_only_exception_row_count += 1;
            }
            Some(
                super::row::EvidenceLookupSourceFirewallExceptionKind::DocumentationReportCodec,
            ) => {
                self.documentation_report_exception_row_count += 1;
            }
            Some(super::row::EvidenceLookupSourceFirewallExceptionKind::TestSupportFixture) => {
                self.test_support_exception_row_count += 1;
            }
            None => {}
        }
    }

    pub const fn scanned_root_count(&self) -> usize {
        self.scanned_root_count
    }

    pub const fn scanned_file_count(&self) -> usize {
        self.scanned_file_count
    }

    pub const fn total_row_count(&self) -> usize {
        self.total_row_count
    }

    pub const fn forbidden_row_count(&self) -> usize {
        self.forbidden_row_count
    }

    pub const fn allowed_exception_row_count(&self) -> usize {
        self.allowed_exception_row_count
    }

    pub const fn certification_only_exception_row_count(&self) -> usize {
        self.certification_only_exception_row_count
    }

    pub const fn documentation_report_exception_row_count(&self) -> usize {
        self.documentation_report_exception_row_count
    }

    pub const fn test_support_exception_row_count(&self) -> usize {
        self.test_support_exception_row_count
    }

    pub const fn raw_vector_row_count(&self) -> usize {
        self.raw_vector_row_count
    }

    pub const fn broad_receipt_scan_row_count(&self) -> usize {
        self.broad_receipt_scan_row_count
    }

    pub const fn copied_digest_row_count(&self) -> usize {
        self.copied_digest_row_count
    }

    pub const fn stage_local_nearby_row_count(&self) -> usize {
        self.stage_local_nearby_row_count
    }

    pub const fn query_lookup_substitution_row_count(&self) -> usize {
        self.query_lookup_substitution_row_count
    }

    pub const fn public_exposure_row_count(&self) -> usize {
        self.public_exposure_row_count
    }
}
