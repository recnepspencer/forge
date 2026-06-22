use crate::planar_contracts::predicate_consumption::{
    PredicateCertificateConsumerKind, PredicateCertificateConsumptionBasis,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PredicateCertificateConsumptionInspectionRow {
    consumer_kind: PredicateCertificateConsumerKind,
    predicate_fact_digest: String,
}

impl PredicateCertificateConsumptionInspectionRow {
    pub(crate) fn from_basis(basis: &PredicateCertificateConsumptionBasis) -> Vec<Self> {
        basis
            .consumption_rows()
            .iter()
            .map(|row| Self {
                consumer_kind: row.consumer_kind(),
                predicate_fact_digest: row.predicate_fact_digest().to_string(),
            })
            .collect()
    }

    pub fn consumer_kind(&self) -> PredicateCertificateConsumerKind {
        self.consumer_kind
    }

    pub fn predicate_fact_digest(&self) -> &str {
        &self.predicate_fact_digest
    }
}
