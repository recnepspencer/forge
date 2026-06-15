use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::RetainedPlanarFactsBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct RetainedPlanarFactAuthorityEntry {
    locus: String,
    value: String,
}

impl RetainedPlanarFactAuthorityEntry {
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

pub(crate) fn retained_planar_fact_authority_entries(
    basis: &RetainedPlanarFactsBasis,
) -> Vec<RetainedPlanarFactAuthorityEntry> {
    let mut entries = vec![
        entry(
            "boolean_readiness.declaration",
            basis.boolean_readiness_receipt().declaration_digest(),
        ),
        entry(
            "boolean_readiness.envelope",
            basis.boolean_readiness_receipt().envelope_digest(),
        ),
        entry(
            "boolean_readiness.fact",
            basis.boolean_readiness_receipt().fact_digest(),
        ),
        entry(
            "structural_identity.declaration",
            basis.structural_identity_receipt().declaration_digest(),
        ),
        entry(
            "structural_identity.envelope",
            basis.structural_identity_receipt().envelope_digest(),
        ),
        entry(
            "structural_identity.fact",
            basis
                .structural_identity_receipt()
                .structural_identity_digest(),
        ),
        entry(
            "structural_identity.transform",
            basis
                .structural_identity_receipt()
                .canonical_transform_basis_digest(),
        ),
        entry(
            "motion_posture.declaration",
            basis.motion_posture_receipt().declaration_digest(),
        ),
        entry(
            "motion_posture.envelope",
            basis.motion_posture_receipt().envelope_digest(),
        ),
        entry(
            "motion_posture.retained_motion",
            basis.motion_posture_receipt().retained_motion_digest(),
        ),
        entry(
            "topology_contract.declaration",
            basis.topology_contract_receipt().declaration_digest(),
        ),
        entry(
            "topology_contract.envelope",
            basis.topology_contract_receipt().envelope_digest(),
        ),
        entry(
            "topology_contract.fact",
            basis.topology_contract_receipt().fact_digest(),
        ),
        entry(
            "retains_planar_classification",
            basis.retains_planar_classification().to_string(),
        ),
    ];
    for row in basis.boolean_readiness_receipt().basis().family_rows() {
        entries.push(entry(
            format!("family.{}.receipt_count", row.family().as_str()),
            row.receipt_count().to_string(),
        ));
        for (index, digest) in row.retained_fact_digests().iter().enumerate() {
            entries.push(entry(
                format!("family.{}.retained_fact.{index}", row.family().as_str()),
                digest,
            ));
        }
        for (index, digest) in row.declaration_digests().iter().enumerate() {
            entries.push(entry(
                format!("family.{}.declaration.{index}", row.family().as_str()),
                digest,
            ));
        }
        for (index, digest) in row.envelope_digests().iter().enumerate() {
            entries.push(entry(
                format!("family.{}.envelope.{index}", row.family().as_str()),
                digest,
            ));
        }
    }
    entries
}

pub(crate) fn retained_planar_fact_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn entry(locus: impl Into<String>, value: impl ToString) -> RetainedPlanarFactAuthorityEntry {
    RetainedPlanarFactAuthorityEntry {
        locus: locus.into(),
        value: value.to_string(),
    }
}
