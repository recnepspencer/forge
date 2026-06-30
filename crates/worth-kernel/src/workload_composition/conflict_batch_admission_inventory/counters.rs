use super::row::{
    ConflictBatchAdmissionAuthorityKind, ConflictBatchAdmissionCertificationPosture,
    ConflictBatchAdmissionDisposition, ConflictBatchAdmissionInventoryRow,
    ConflictBatchAdmissionQuerySurface,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConflictBatchAdmissionInventoryCounters {
    row_count: usize,
    migrate_rows: usize,
    delete_rows: usize,
    cap_rows: usize,
    certification_only_rows: usize,
    query_gap_rows: usize,
    ordinary_authority_rows: usize,
    query_support_rows: usize,
    operational_overlap_rows: usize,
    seeded_surface_rows: usize,
}

impl ConflictBatchAdmissionInventoryCounters {
    pub(crate) fn from_rows(rows: &[ConflictBatchAdmissionInventoryRow]) -> Self {
        let mut counters = Self {
            row_count: rows.len(),
            ..Self::default()
        };
        for row in rows {
            counters.count_row(row);
        }
        counters
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
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

    pub const fn certification_only_rows(&self) -> usize {
        self.certification_only_rows
    }

    pub const fn query_gap_rows(&self) -> usize {
        self.query_gap_rows
    }

    pub const fn ordinary_authority_rows(&self) -> usize {
        self.ordinary_authority_rows
    }

    pub const fn query_support_rows(&self) -> usize {
        self.query_support_rows
    }

    pub const fn operational_overlap_rows(&self) -> usize {
        self.operational_overlap_rows
    }

    pub const fn seeded_surface_rows(&self) -> usize {
        self.seeded_surface_rows
    }

    fn count_row(&mut self, row: &ConflictBatchAdmissionInventoryRow) {
        match row.disposition() {
            ConflictBatchAdmissionDisposition::Migrate => self.migrate_rows += 1,
            ConflictBatchAdmissionDisposition::Delete => self.delete_rows += 1,
            ConflictBatchAdmissionDisposition::Cap => self.cap_rows += 1,
            ConflictBatchAdmissionDisposition::CertificationOnly => {
                self.certification_only_rows += 1;
            }
            ConflictBatchAdmissionDisposition::QueryGap => self.query_gap_rows += 1,
        }
        if row.certification_posture()
            == ConflictBatchAdmissionCertificationPosture::OrdinaryProductionReachable
        {
            self.ordinary_authority_rows += 1;
        }
        if row.query_surface() != ConflictBatchAdmissionQuerySurface::NotQuery {
            self.query_support_rows += 1;
        }
        if matches!(
            row.authority_kind(),
            ConflictBatchAdmissionAuthorityKind::OperationalOverlapExecution
                | ConflictBatchAdmissionAuthorityKind::OperationalOverlapReceipt
        ) {
            self.operational_overlap_rows += 1;
        }
        if is_seed_surface(row) {
            self.seeded_surface_rows += 1;
        }
    }
}

fn is_seed_surface(row: &ConflictBatchAdmissionInventoryRow) -> bool {
    row.surface_name().contains("WorthWorkload")
        || row.surface_name().contains("LookupConsumedWorkload")
        || row.surface_name().contains("CoplanarOverlap")
}
