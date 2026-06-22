use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarStructuralIdentityBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarStructuralAuthorityEntry {
    locus: String,
    value: String,
}

impl PlanarStructuralAuthorityEntry {
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

pub(crate) fn planar_structural_identity_authority_entries(
    basis: &PlanarStructuralIdentityBasis,
) -> Vec<PlanarStructuralAuthorityEntry> {
    let transform = basis.canonical_transform_basis();
    let mut entries = vec![
        PlanarStructuralAuthorityEntry::new(
            "structural.boolean_readiness.fact",
            basis.boolean_readiness_receipt().fact_digest(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.boolean_readiness.declaration",
            basis.boolean_readiness_receipt().declaration_digest(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.boolean_readiness.envelope",
            basis.boolean_readiness_receipt().envelope_digest(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.transform.local_frame",
            transform.local_frame_identity(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.transform.movement_rotation",
            transform.movement_rotation_posture_identity(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.transform.chain",
            transform.transform_chain_digest(),
        ),
        PlanarStructuralAuthorityEntry::new(
            "structural.transform.orientation_policy",
            transform.orientation_policy().as_str(),
        ),
    ];
    if let Some(receipt) = basis.motion_posture_receipt() {
        entries.push(PlanarStructuralAuthorityEntry::new(
            "structural.motion_posture.retained",
            receipt.retained_motion_digest(),
        ));
        entries.push(PlanarStructuralAuthorityEntry::new(
            "structural.motion_posture.declaration",
            receipt.declaration_digest(),
        ));
    }
    entries.sort_by(|left, right| {
        left.locus()
            .cmp(right.locus())
            .then_with(|| left.value().cmp(right.value()))
    });
    entries
}

pub(crate) fn planar_structural_identity_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
