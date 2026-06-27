use crate::workload_platform::evidence_lookup_inventory::EvidenceLookupQuerySurface;

use super::row::EvidenceLookupQuerySurfaceMatrixRow;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct EvidenceLookupQuerySurfaceMatrixCounters {
    row_count: usize,
    query_row_count: usize,
    non_query_row_count: usize,
    support_admission_row_count: usize,
    support_pinning_row_count: usize,
    projection_consumption_row_count: usize,
    lower_runtime_boundary_row_count: usize,
    typed_artifact_identity_row_count: usize,
    consumer_kit_proof_row_count: usize,
}

impl EvidenceLookupQuerySurfaceMatrixCounters {
    pub(crate) fn from_rows(rows: &[EvidenceLookupQuerySurfaceMatrixRow]) -> Self {
        let mut counters = Self {
            row_count: rows.len(),
            ..Self::default()
        };
        for row in rows {
            counters.count_row(row);
        }
        counters
    }

    fn count_row(&mut self, row: &EvidenceLookupQuerySurfaceMatrixRow) {
        match row.query_surface() {
            EvidenceLookupQuerySurface::NotQuery => self.non_query_row_count += 1,
            EvidenceLookupQuerySurface::SupportAdmission => {
                self.query_row_count += 1;
                self.support_admission_row_count += 1;
            }
            EvidenceLookupQuerySurface::SupportPinning => {
                self.query_row_count += 1;
                self.support_pinning_row_count += 1;
            }
            EvidenceLookupQuerySurface::ProjectionConsumption => {
                self.query_row_count += 1;
                self.projection_consumption_row_count += 1;
            }
            EvidenceLookupQuerySurface::LowerRuntimeBoundaryEnvelope => {
                self.query_row_count += 1;
                self.lower_runtime_boundary_row_count += 1;
            }
            EvidenceLookupQuerySurface::TypedArtifactIdentity => {
                self.query_row_count += 1;
                self.typed_artifact_identity_row_count += 1;
            }
            EvidenceLookupQuerySurface::ConsumerKitProof => {
                self.query_row_count += 1;
                self.consumer_kit_proof_row_count += 1;
            }
        }
    }

    pub const fn row_count(&self) -> usize {
        self.row_count
    }

    pub const fn query_row_count(&self) -> usize {
        self.query_row_count
    }

    pub const fn non_query_row_count(&self) -> usize {
        self.non_query_row_count
    }

    pub const fn support_admission_row_count(&self) -> usize {
        self.support_admission_row_count
    }

    pub const fn support_pinning_row_count(&self) -> usize {
        self.support_pinning_row_count
    }

    pub const fn projection_consumption_row_count(&self) -> usize {
        self.projection_consumption_row_count
    }

    pub const fn lower_runtime_boundary_row_count(&self) -> usize {
        self.lower_runtime_boundary_row_count
    }

    pub const fn typed_artifact_identity_row_count(&self) -> usize {
        self.typed_artifact_identity_row_count
    }

    pub const fn consumer_kit_proof_row_count(&self) -> usize {
        self.consumer_kit_proof_row_count
    }
}
