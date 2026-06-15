use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarDiagnosticBundleBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarDiagnosticAuthorityEntry {
    locus: String,
    value: String,
}

impl PlanarDiagnosticAuthorityEntry {
    pub(crate) fn new(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            value: value.into(),
        }
    }

    pub(crate) fn locus(&self) -> &str {
        &self.locus
    }

    pub(crate) fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("{}:{}", self.locus, self.value)
    }
}

pub(crate) fn planar_diagnostic_authority_entries(
    basis: &PlanarDiagnosticBundleBasis,
) -> Vec<PlanarDiagnosticAuthorityEntry> {
    let mut entries = vec![
        PlanarDiagnosticAuthorityEntry::new(
            "planar_diagnostic.subject_kind",
            format!("{:?}", basis.subject().kind()),
        ),
        PlanarDiagnosticAuthorityEntry::new(
            "planar_diagnostic.trigger_locality",
            format!("{:?}", basis.subject().trigger_locality()),
        ),
        PlanarDiagnosticAuthorityEntry::new(
            "planar_diagnostic.source",
            basis.subject().source_digest(),
        ),
        PlanarDiagnosticAuthorityEntry::new(
            "planar_diagnostic.truth_effect",
            format!("{:?}", basis.truth_effect()),
        ),
    ];
    for (index, evidence) in basis.subject().evidence().iter().enumerate() {
        entries.push(PlanarDiagnosticAuthorityEntry::new(
            format!("planar_diagnostic.evidence.{index}.kind"),
            format!("{:?}", evidence.kind()),
        ));
        entries.push(PlanarDiagnosticAuthorityEntry::new(
            format!("planar_diagnostic.evidence.{index}.digest"),
            evidence.evidence_digest(),
        ));
    }
    if let Some(causal_evidence) = basis.causal_evidence() {
        entries.extend([
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_reference",
                causal_evidence.reference_digest(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_anchor",
                causal_evidence.anchor_digest(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_reference_set",
                causal_evidence.reference_set_digest(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_request",
                causal_evidence.request_digest(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_admission",
                causal_evidence.admission_digest(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_richness",
                causal_evidence.richness().as_str(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_family",
                causal_evidence.explanation_family().as_str(),
            ),
            PlanarDiagnosticAuthorityEntry::new(
                "planar_diagnostic.causal_materialization",
                format!("{:?}", causal_evidence.materialization_policy()),
            ),
        ]);
    }
    entries
}

pub(crate) fn planar_diagnostic_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
