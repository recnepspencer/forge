use std::collections::BTreeSet;

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_ledger::WorkloadEvidenceStage;
use crate::workload_platform::evidence_lookup_family_catalog::current_evidence_lookup_family_catalog;

use super::counters::EvidenceLookupQuerySurfaceMatrixCounters;
use super::error::{
    EvidenceLookupQuerySurfaceMatrixError, EvidenceLookupQuerySurfaceMatrixErrorKind,
};
use super::row::{EvidenceLookupQuerySurfaceMatrixRow, EvidenceLookupQuerySurfaceTouchpoint};
use super::row_sources::current_query_surface_rows;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupQuerySurfaceMatrixCloseout {
    rows: Vec<EvidenceLookupQuerySurfaceMatrixRow>,
    counters: EvidenceLookupQuerySurfaceMatrixCounters,
    matrix_digest: String,
}

impl EvidenceLookupQuerySurfaceMatrixCloseout {
    pub(crate) fn from_rows(
        rows: Vec<EvidenceLookupQuerySurfaceMatrixRow>,
    ) -> Result<Self, EvidenceLookupQuerySurfaceMatrixError> {
        if rows.is_empty() {
            return Err(error(
                EvidenceLookupQuerySurfaceMatrixErrorKind::EmptyMatrix,
                "query surface matrix requires at least one row",
            ));
        }
        reject_duplicate_row_identities(&rows)?;
        let counters = EvidenceLookupQuerySurfaceMatrixCounters::from_rows(&rows);
        let matrix_digest = matrix_digest(&rows, &counters);
        Ok(Self {
            rows,
            counters,
            matrix_digest,
        })
    }

    pub fn rows(&self) -> &[EvidenceLookupQuerySurfaceMatrixRow] {
        &self.rows
    }

    pub const fn counters(&self) -> &EvidenceLookupQuerySurfaceMatrixCounters {
        &self.counters
    }

    pub fn matrix_digest(&self) -> &str {
        &self.matrix_digest
    }

    pub fn require_family_stage_touchpoint_row(
        &self,
        family_identity: &str,
        stage: WorkloadEvidenceStage,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    ) -> Result<&EvidenceLookupQuerySurfaceMatrixRow, EvidenceLookupQuerySurfaceMatrixError> {
        self.rows
            .iter()
            .find(|row| {
                row.family_identity() == family_identity
                    && row.stage() == stage
                    && row.touchpoint() == touchpoint
            })
            .ok_or_else(|| {
                error(
                    EvidenceLookupQuerySurfaceMatrixErrorKind::MissingFamilyStageTouchpointRow,
                    format!(
                        "missing query surface matrix row for family `{family_identity}`, stage {:?}, touchpoint {}",
                        stage,
                        touchpoint.as_str()
                    ),
                )
            })
    }

    pub fn rows_for_touchpoint(
        &self,
        touchpoint: EvidenceLookupQuerySurfaceTouchpoint,
    ) -> Vec<&EvidenceLookupQuerySurfaceMatrixRow> {
        self.rows
            .iter()
            .filter(|row| row.touchpoint() == touchpoint)
            .collect()
    }

    pub const fn claims_lookup_execution_authority(&self) -> bool {
        false
    }

    pub const fn claims_query_descriptor_authority(&self) -> bool {
        false
    }
}

pub fn current_evidence_lookup_query_surface_matrix(
) -> Result<EvidenceLookupQuerySurfaceMatrixCloseout, EvidenceLookupQuerySurfaceMatrixError> {
    let catalog = current_evidence_lookup_family_catalog().map_err(|catalog_error| {
        error(
            EvidenceLookupQuerySurfaceMatrixErrorKind::EmptyMatrix,
            format!(
                "family catalog required before query surface matrix: {:?}",
                catalog_error.kind()
            ),
        )
    })?;
    let rows = current_query_surface_rows(&catalog)?;
    EvidenceLookupQuerySurfaceMatrixCloseout::from_rows(rows)
}

fn reject_duplicate_row_identities(
    rows: &[EvidenceLookupQuerySurfaceMatrixRow],
) -> Result<(), EvidenceLookupQuerySurfaceMatrixError> {
    let mut identities = BTreeSet::new();
    for row in rows {
        let identity = format!(
            "{}::{:?}::{}",
            row.family_identity(),
            row.stage(),
            row.touchpoint().as_str()
        );
        if !identities.insert(identity) {
            return Err(error(
                EvidenceLookupQuerySurfaceMatrixErrorKind::DuplicateRowIdentity,
                format!("duplicate query surface matrix row `{}`", row.row_digest()),
            ));
        }
    }
    Ok(())
}

fn matrix_digest(
    rows: &[EvidenceLookupQuerySurfaceMatrixRow],
    counters: &EvidenceLookupQuerySurfaceMatrixCounters,
) -> String {
    let mut parts = vec![
        "worth-spatial:evidence-lookup-query-surface-matrix-closeout:v1".to_string(),
        format!("rows:{}", counters.row_count()),
        format!("query-rows:{}", counters.query_row_count()),
        format!("non-query-rows:{}", counters.non_query_row_count()),
        format!(
            "support-admission-rows:{}",
            counters.support_admission_row_count()
        ),
        format!(
            "projection-consumption-rows:{}",
            counters.projection_consumption_row_count()
        ),
        format!(
            "support-pinning-rows:{}",
            counters.support_pinning_row_count()
        ),
        format!(
            "lower-runtime-rows:{}",
            counters.lower_runtime_boundary_row_count()
        ),
        format!(
            "typed-artifact-rows:{}",
            counters.typed_artifact_identity_row_count()
        ),
        format!(
            "consumer-kit-proof-rows:{}",
            counters.consumer_kit_proof_row_count()
        ),
    ];
    parts.extend(rows.iter().map(|row| format!("row:{}", row.row_digest())));
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, &parts)
}

fn error(
    kind: EvidenceLookupQuerySurfaceMatrixErrorKind,
    message: impl Into<String>,
) -> EvidenceLookupQuerySurfaceMatrixError {
    EvidenceLookupQuerySurfaceMatrixError::new(kind, message)
}
