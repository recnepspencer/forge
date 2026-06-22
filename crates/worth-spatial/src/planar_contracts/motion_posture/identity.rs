use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::{PlanarMotionCancellation, PlanarMotionPostureBasis};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarMotionPostureAuthorityEntry {
    locus: String,
    value: String,
}

impl PlanarMotionPostureAuthorityEntry {
    pub(crate) fn new(locus: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            locus: locus.into(),
            value: value.into(),
        }
    }

    pub fn locus(&self) -> &str {
        &self.locus
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("{}:{}", self.locus, self.value)
    }
}

pub(crate) fn planar_motion_posture_authority_entries(
    basis: &PlanarMotionPostureBasis,
) -> Vec<PlanarMotionPostureAuthorityEntry> {
    let mut entries = vec![
        PlanarMotionPostureAuthorityEntry::new(
            "motion.boolean_readiness.fact",
            basis.boolean_readiness_receipt().fact_digest(),
        ),
        PlanarMotionPostureAuthorityEntry::new(
            "motion.boolean_readiness.declaration",
            basis.boolean_readiness_receipt().declaration_digest(),
        ),
        PlanarMotionPostureAuthorityEntry::new(
            "motion.rotation.posture",
            basis.rotation_posture().as_str(),
        ),
        PlanarMotionPostureAuthorityEntry::new(
            "motion.cancellation.policy",
            basis.cancellation().as_str(),
        ),
    ];
    entries.extend(canonical_motion_step_entries(basis));
    entries.sort_by(|left, right| {
        left.locus()
            .cmp(right.locus())
            .then_with(|| left.value().cmp(right.value()))
    });
    entries
}

pub(crate) fn planar_motion_posture_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn canonical_motion_step_entries(
    basis: &PlanarMotionPostureBasis,
) -> Vec<PlanarMotionPostureAuthorityEntry> {
    let mut step_identity_parts = basis
        .steps()
        .iter()
        .map(|step| (step.kind(), step.authority_value()))
        .collect::<Vec<_>>();
    if basis.cancellation() == PlanarMotionCancellation::ExactBasisReplay {
        step_identity_parts.sort_by(|left, right| left.0.cmp(right.0).then(left.1.cmp(&right.1)));
    }

    step_identity_parts
        .into_iter()
        .enumerate()
        .map(|(index, (kind, value))| {
            PlanarMotionPostureAuthorityEntry::new(format!("motion.step.{index}.{kind}"), value)
        })
        .collect()
}
