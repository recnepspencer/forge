use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::ProjectionConsumedPlanarFactsBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProjectionConsumedPlanarFactAuthorityEntry {
    locus: String,
    value: String,
}

impl ProjectionConsumedPlanarFactAuthorityEntry {
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

pub(crate) fn projection_consumed_planar_fact_authority_entries(
    basis: &ProjectionConsumedPlanarFactsBasis,
) -> Vec<ProjectionConsumedPlanarFactAuthorityEntry> {
    let retained = basis.retained_planar_facts_receipt();
    let mut entries = vec![
        entry("retained_planar_fact", retained.retained_fact_digest()),
        entry("retained_declaration", retained.declaration_digest()),
        entry("retained_progression", retained.progression_digest()),
        entry("retained_route_plan", retained.route_plan_digest()),
        entry("retained_query_receipt", retained.query_receipt_digest()),
        entry("retained_envelope", retained.envelope_digest()),
        entry("structural_identity", basis.structural_identity_digest()),
        entry("motion_posture", basis.motion_posture_digest()),
        entry("topology_contract", basis.topology_contract_digest()),
        entry(
            "materialization_basis",
            basis.materialization_basis_identity(),
        ),
    ];
    for (index, receipt) in basis.projection_receipts().iter().enumerate() {
        entries.push(entry(
            format!("projection.{index}.fact"),
            receipt.fact_digest(),
        ));
        entries.push(entry(
            format!("projection.{index}.declaration"),
            receipt.declaration_digest(),
        ));
        entries.push(entry(
            format!("projection.{index}.envelope"),
            receipt.envelope_digest(),
        ));
        entries.push(entry(
            format!("projection.{index}.local_frame"),
            receipt.local_frame_fact_digest(),
        ));
    }
    entries
}

pub(crate) fn projection_consumed_planar_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn entry(
    locus: impl Into<String>,
    value: impl ToString,
) -> ProjectionConsumedPlanarFactAuthorityEntry {
    ProjectionConsumedPlanarFactAuthorityEntry {
        locus: locus.into(),
        value: value.to_string(),
    }
}
