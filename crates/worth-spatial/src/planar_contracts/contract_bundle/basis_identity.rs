use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PlanarContractBundleValidationBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PlanarContractBundleIdentityEntry {
    locus: String,
    value: String,
}

impl PlanarContractBundleIdentityEntry {
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

pub(crate) fn planar_contract_bundle_identity_entries(
    basis: &PlanarContractBundleValidationBasis,
) -> Vec<PlanarContractBundleIdentityEntry> {
    let mut entries = vec![
        PlanarContractBundleIdentityEntry::new("policy", basis.policy().as_str()),
        PlanarContractBundleIdentityEntry::new("topology_basis", basis.topology_basis_identity()),
        PlanarContractBundleIdentityEntry::new(
            "topology_contract.fact",
            basis.topology_contract_receipt().fact_digest(),
        ),
        PlanarContractBundleIdentityEntry::new(
            "topology_contract.declaration",
            basis.topology_contract_receipt().declaration_digest(),
        ),
        PlanarContractBundleIdentityEntry::new(
            "movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        PlanarContractBundleIdentityEntry::new(
            "diagnostic_scope",
            basis.diagnostic_scope_identity(),
        ),
        PlanarContractBundleIdentityEntry::new(
            "admission.row_digest",
            basis.admission_receipt().row_digest(),
        ),
        PlanarContractBundleIdentityEntry::new(
            "admission.matrix_digest",
            basis.admission_receipt().matrix_digest(),
        ),
    ];
    for row in basis.family_rows() {
        entries.push(PlanarContractBundleIdentityEntry::new(
            format!("family.{}.count", row.family().as_str()),
            row.receipt_count().to_string(),
        ));
        for (index, digest) in row.retained_fact_digests().iter().enumerate() {
            entries.push(PlanarContractBundleIdentityEntry::new(
                format!("family.{}.fact.{index}", row.family().as_str()),
                digest,
            ));
        }
        for (index, digest) in row.declaration_digests().iter().enumerate() {
            entries.push(PlanarContractBundleIdentityEntry::new(
                format!("family.{}.declaration.{index}", row.family().as_str()),
                digest,
            ));
        }
        for (index, digest) in row.envelope_digests().iter().enumerate() {
            entries.push(PlanarContractBundleIdentityEntry::new(
                format!("family.{}.envelope.{index}", row.family().as_str()),
                digest,
            ));
        }
    }
    entries.sort_by(|left, right| {
        left.locus()
            .cmp(right.locus())
            .then_with(|| left.value().cmp(right.value()))
    });
    entries
}

pub(crate) fn planar_contract_bundle_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
