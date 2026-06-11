use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::PredicateCertificateConsumptionBasis;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionIdentityEntry {
    locus: String,
    value: String,
}

impl PredicateCertificateConsumptionIdentityEntry {
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

pub(crate) fn predicate_certificate_consumption_identity_entries(
    basis: &PredicateCertificateConsumptionBasis,
) -> Vec<PredicateCertificateConsumptionIdentityEntry> {
    let mut entries = vec![
        PredicateCertificateConsumptionIdentityEntry::new(
            "topology_basis",
            basis.topology_basis_identity(),
        ),
        PredicateCertificateConsumptionIdentityEntry::new(
            "movement_rotation",
            basis.movement_rotation_posture_identity(),
        ),
        PredicateCertificateConsumptionIdentityEntry::new(
            "local_frame",
            basis.local_frame_identity(),
        ),
    ];
    for (index, row) in basis.consumption_rows().iter().enumerate() {
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.consumer_kind"),
            row.consumer_kind().as_str(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.consumer_fact"),
            row.consumer_fact_digest(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.predicate_fact"),
            row.predicate_fact_digest(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.certified_sign"),
            row.certified_sign_identity(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.precision"),
            row.precision_escalation_identity(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.predicate_declaration"),
            row.predicate_declaration_digest(),
        ));
        entries.push(PredicateCertificateConsumptionIdentityEntry::new(
            format!("row.{index}.predicate_envelope"),
            row.predicate_envelope_digest(),
        ));
    }
    entries.sort_by(|left, right| {
        left.locus()
            .cmp(right.locus())
            .then_with(|| left.value().cmp(right.value()))
    });
    entries
}

pub(crate) fn predicate_certificate_consumption_digest(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}
