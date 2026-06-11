use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarLocalRebuildParityBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarLocalRebuildParityEntry {
    locus: &'static str,
    value: String,
}

impl PlanarLocalRebuildParityEntry {
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

pub(crate) fn planar_local_rebuild_parity_authority_entries(
    basis: &PlanarLocalRebuildParityBasis,
) -> Vec<PlanarLocalRebuildParityEntry> {
    vec![
        entry(
            "geometry.local_rebuild.scope",
            basis.rebuild_scope().scope_identity(),
        ),
        entry(
            "geometry.local_rebuild.neighborhood",
            basis.neighborhood().fact_digest(),
        ),
        entry(
            "geometry.local_rebuild.rebinding_continuity",
            basis.rebinding().continuity_digest(),
        ),
        entry(
            "geometry.local_rebuild.structural_identity",
            basis.structural_identity().structural_identity_digest(),
        ),
        entry(
            "geometry.local_rebuild.retained_fact",
            basis.retained().retained_fact_digest(),
        ),
        entry(
            "geometry.local_rebuild.projection_consumption",
            basis.projection_consumed().projection_consumption_digest(),
        ),
        entry(
            "geometry.local_rebuild.motion_posture",
            basis.motion().retained_motion_digest(),
        ),
        entry(
            "geometry.local_rebuild.topology_contract",
            basis.topology().fact_digest(),
        ),
        entry(
            "geometry.local_rebuild.recovery",
            basis.recovery().recovery_posture_digest(),
        ),
        entry(
            "geometry.local_rebuild.diagnostics",
            basis.diagnostics().diagnostic_bundle_digest(),
        ),
    ]
}

pub(crate) fn planar_local_rebuild_parity_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn entry(locus: &'static str, value: impl ToString) -> PlanarLocalRebuildParityEntry {
    PlanarLocalRebuildParityEntry {
        locus,
        value: value.to_string(),
    }
}
