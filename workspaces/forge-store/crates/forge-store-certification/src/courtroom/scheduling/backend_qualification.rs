use forge_store_physical_certification::{
    BackendQualificationMatrix, BackendQualificationMatrixDenial, BackendQualificationRow,
    CertifiedBackendQualificationSupport,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct S6BackendQualificationMatrixCertification {
    matrix: BackendQualificationMatrix,
    row_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct S6BackendQualificationRowOutcome {
    row: BackendQualificationRow,
    certified_support:
        Result<CertifiedBackendQualificationSupport, BackendQualificationMatrixDenial>,
}

pub fn certify_io_pressure_backend_qualification_matrix(
    matrix: BackendQualificationMatrix,
) -> Result<S6BackendQualificationMatrixCertification, BackendQualificationMatrixDenial> {
    Ok(S6BackendQualificationMatrixCertification {
        row_count: matrix.rows().len(),
        matrix,
    })
}

impl S6BackendQualificationMatrixCertification {
    pub const fn row_count(&self) -> usize {
        self.row_count
    }

    pub const fn matrix(&self) -> &BackendQualificationMatrix {
        &self.matrix
    }

    pub fn certified_support_rows(&self) -> Vec<CertifiedBackendQualificationSupport> {
        self.matrix
            .iter()
            .filter_map(|row| row.require_certified_backend_support().ok())
            .collect()
    }

    pub fn row_outcomes(&self) -> Vec<S6BackendQualificationRowOutcome> {
        self.matrix
            .iter()
            .map(|row| S6BackendQualificationRowOutcome {
                row: *row,
                certified_support: row.require_certified_backend_support(),
            })
            .collect()
    }
}

impl S6BackendQualificationRowOutcome {
    pub const fn row(self) -> BackendQualificationRow {
        self.row
    }

    pub const fn certified_support(
        self,
    ) -> Result<CertifiedBackendQualificationSupport, BackendQualificationMatrixDenial> {
        self.certified_support
    }
}
