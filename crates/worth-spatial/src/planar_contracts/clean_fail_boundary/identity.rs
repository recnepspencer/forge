use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarCleanFailBoundaryBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarCleanFailBoundaryEntry {
    locus: &'static str,
    value: String,
}

impl PlanarCleanFailBoundaryEntry {
    pub fn locus(&self) -> &'static str {
        self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn digest_part(&self) -> String {
        format!("{}:{}", self.locus, self.value)
    }
}

pub(crate) fn planar_clean_fail_boundary_authority_entries(
    basis: &PlanarCleanFailBoundaryBasis,
) -> Vec<PlanarCleanFailBoundaryEntry> {
    let input = basis.input();
    vec![
        entry("geometry.clean_fail.class", input.class().as_str()),
        entry("geometry.clean_fail.action", input.action().as_str()),
        entry("geometry.clean_fail.source", input.source_digest()),
        entry("geometry.clean_fail.source_detail", input.source_detail()),
        entry(
            "geometry.clean_fail.stable_topology_identity",
            input.stable_topology_identity().unwrap_or("none"),
        ),
        entry(
            "geometry.clean_fail.transform_posture",
            input.transform_posture_digest().unwrap_or("missing"),
        ),
        entry(
            "geometry.clean_fail.admission_row",
            input
                .admission_row()
                .map(|row| row.row_digest())
                .unwrap_or("missing"),
        ),
        entry(
            "geometry.clean_fail.recovery",
            basis.recovery().recovery_posture_digest(),
        ),
        entry(
            "geometry.clean_fail.diagnostics",
            basis.diagnostics().diagnostic_bundle_digest(),
        ),
        entry(
            "geometry.clean_fail.no_repair",
            basis.repair_attempt().as_str(),
        ),
        entry(
            "geometry.clean_fail.no_bounded_conversion",
            basis.bounded_conversion().as_str(),
        ),
        entry(
            "geometry.clean_fail.truth_effect",
            basis.truth_effect().as_str(),
        ),
    ]
}

pub(crate) fn planar_clean_fail_boundary_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn entry(locus: &'static str, value: impl ToString) -> PlanarCleanFailBoundaryEntry {
    PlanarCleanFailBoundaryEntry {
        locus,
        value: value.to_string(),
    }
}
