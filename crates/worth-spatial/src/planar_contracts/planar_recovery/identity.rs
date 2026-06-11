use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarRecoveryPostureBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PlanarRecoveryPostureAuthorityEntry {
    locus: String,
    value: String,
}

impl PlanarRecoveryPostureAuthorityEntry {
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

pub(crate) fn planar_recovery_posture_authority_entries(
    basis: &PlanarRecoveryPostureBasis,
) -> Vec<PlanarRecoveryPostureAuthorityEntry> {
    let mut entries = vec![
        entry("source_kind", format!("{:?}", basis.source().kind())),
        entry("source_digest", basis.source().source_digest()),
        entry("source_family", basis.source().source_family()),
        entry(
            "original_outcome_class",
            basis.source().original_outcome_class(),
        ),
        entry("blocker_kind", format!("{:?}", basis.blocker_kind())),
        entry("source_posture", format!("{:?}", basis.source_posture())),
        entry("recovery_action", format!("{:?}", basis.recovery_action())),
        entry("target_scope", format!("{:?}", basis.target_scope())),
        entry("truth_effect", format!("{:?}", basis.truth_effect())),
    ];
    if let Some(retained) = basis.retained_planar_facts() {
        entries.push(entry(
            "retained_planar_fact",
            retained.retained_fact_digest(),
        ));
    }
    if let Some(projected) = basis.projection_consumed_facts() {
        entries.push(entry(
            "projection_consumed_planar_fact",
            projected.projection_consumption_digest(),
        ));
    }
    entries
}

pub(crate) fn planar_recovery_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn entry(locus: impl Into<String>, value: impl ToString) -> PlanarRecoveryPostureAuthorityEntry {
    PlanarRecoveryPostureAuthorityEntry {
        locus: locus.into(),
        value: value.to_string(),
    }
}
