use crate::planar_contracts::planar_diagnostics::PlanarDiagnosticBundleBasis;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanarDiagnosticInspectionKind {
    Source,
    Locality,
    Evidence,
    TruthEffect,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarDiagnosticInspectionRow {
    kind: PlanarDiagnosticInspectionKind,
    locus: String,
    value: String,
}

impl PlanarDiagnosticInspectionRow {
    pub(crate) fn from_basis(basis: &PlanarDiagnosticBundleBasis) -> Vec<Self> {
        let mut rows = vec![
            row(
                PlanarDiagnosticInspectionKind::Source,
                "planar_diagnostic.source",
                basis.subject().source_digest(),
            ),
            row(
                PlanarDiagnosticInspectionKind::Locality,
                "planar_diagnostic.trigger_locality",
                format!("{:?}", basis.subject().trigger_locality()),
            ),
            row(
                PlanarDiagnosticInspectionKind::TruthEffect,
                "planar_diagnostic.truth_effect",
                format!("{:?}", basis.truth_effect()),
            ),
        ];
        for evidence in basis.subject().evidence() {
            rows.push(row(
                PlanarDiagnosticInspectionKind::Evidence,
                format!("planar_diagnostic.evidence.{:?}", evidence.kind()),
                evidence.evidence_digest(),
            ));
        }
        if let Some(causal) = basis.causal_evidence() {
            rows.extend([
                row(
                    PlanarDiagnosticInspectionKind::Evidence,
                    "planar_diagnostic.causal_reference",
                    causal.reference_digest(),
                ),
                row(
                    PlanarDiagnosticInspectionKind::Evidence,
                    "planar_diagnostic.causal_anchor",
                    causal.anchor_digest(),
                ),
                row(
                    PlanarDiagnosticInspectionKind::Evidence,
                    "planar_diagnostic.causal_reference_set",
                    causal.reference_set_digest(),
                ),
                row(
                    PlanarDiagnosticInspectionKind::Evidence,
                    "planar_diagnostic.causal_request",
                    causal.request_digest(),
                ),
                row(
                    PlanarDiagnosticInspectionKind::Evidence,
                    "planar_diagnostic.causal_admission",
                    causal.admission_digest(),
                ),
            ]);
        }
        rows
    }
}

fn row(
    kind: PlanarDiagnosticInspectionKind,
    locus: impl Into<String>,
    value: impl ToString,
) -> PlanarDiagnosticInspectionRow {
    PlanarDiagnosticInspectionRow {
        kind,
        locus: locus.into(),
        value: value.to_string(),
    }
}
