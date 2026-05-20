use crate::identity::hash_parts;
use crate::lower_runtime_routing::ForgeQueryLowerRuntimeSeamKey;

use super::{
    allowed_phase_six_synthetic_seams, forge_query_lower_runtime_representative_surface,
    ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSyntheticTailRow {
    seam_key: ForgeQueryLowerRuntimeSeamKey,
    justification: &'static str,
    evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
}

impl ForgeQueryLowerRuntimeSyntheticTailRow {
    fn new(
        seam_key: ForgeQueryLowerRuntimeSeamKey,
        justification: &'static str,
        evidence_source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
    ) -> Self {
        Self {
            seam_key,
            justification,
            evidence_source,
        }
    }

    pub fn seam_key(&self) -> ForgeQueryLowerRuntimeSeamKey {
        self.seam_key
    }

    pub fn justification(&self) -> &'static str {
        self.justification
    }

    pub fn evidence_source(&self) -> ForgeQueryLowerRuntimeRepresentativeEvidenceSource {
        self.evidence_source
    }

    fn row_digest(&self) -> String {
        hash_parts(&[
            self.seam_key.as_str().to_string(),
            self.justification.to_string(),
            evidence_source_label(self.evidence_source).to_string(),
        ])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryLowerRuntimeSyntheticTailReport {
    rows: Vec<ForgeQueryLowerRuntimeSyntheticTailRow>,
    report_digest: String,
    justification_digest: String,
}

impl ForgeQueryLowerRuntimeSyntheticTailReport {
    fn new(rows: Vec<ForgeQueryLowerRuntimeSyntheticTailRow>) -> Self {
        let report_digest = hash_parts(
            &rows
                .iter()
                .map(ForgeQueryLowerRuntimeSyntheticTailRow::row_digest)
                .collect::<Vec<_>>(),
        );
        let justification_digest = hash_parts(
            &rows
                .iter()
                .map(|row| format!("{}|{}", row.seam_key.as_str(), row.justification))
                .collect::<Vec<_>>(),
        );
        Self {
            rows,
            report_digest,
            justification_digest,
        }
    }

    pub fn rows(&self) -> &[ForgeQueryLowerRuntimeSyntheticTailRow] {
        &self.rows
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }

    pub fn justification_digest(&self) -> &str {
        &self.justification_digest
    }
}

pub fn forge_query_lower_runtime_synthetic_tail_report() -> ForgeQueryLowerRuntimeSyntheticTailReport
{
    let surface = forge_query_lower_runtime_representative_surface();
    let rows = allowed_phase_six_synthetic_seams()
        .iter()
        .map(|row| {
            let evidence_source = surface
                .evidence_source_for(row.seam_key())
                .unwrap_or_else(|| panic!("missing synthetic seam {}", row.seam_key().as_str()));
            assert_eq!(
                evidence_source,
                ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized,
                "synthetic-tail row {} must remain inventory-synthesized",
                row.seam_key().as_str()
            );
            ForgeQueryLowerRuntimeSyntheticTailRow::new(
                row.seam_key(),
                row.justification(),
                evidence_source,
            )
        })
        .collect::<Vec<_>>();

    ForgeQueryLowerRuntimeSyntheticTailReport::new(rows)
}

fn evidence_source_label(
    source: ForgeQueryLowerRuntimeRepresentativeEvidenceSource,
) -> &'static str {
    match source {
        ForgeQueryLowerRuntimeRepresentativeEvidenceSource::RuntimeBackedFixture => {
            "runtime-backed-fixture"
        }
        ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized => {
            "inventory-synthesized"
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_tail_report_matches_allowlist_exactly() {
        let report = forge_query_lower_runtime_synthetic_tail_report();

        assert_eq!(
            report.rows().len(),
            allowed_phase_six_synthetic_seams().len()
        );
        for allowed in allowed_phase_six_synthetic_seams() {
            assert!(report
                .rows()
                .iter()
                .any(|row| row.seam_key() == allowed.seam_key()));
        }
    }

    #[test]
    fn synthetic_tail_report_rows_stay_inventory_synthesized() {
        let report = forge_query_lower_runtime_synthetic_tail_report();

        for row in report.rows() {
            assert_eq!(
                row.evidence_source(),
                ForgeQueryLowerRuntimeRepresentativeEvidenceSource::InventorySynthesized
            );
        }
    }
}
