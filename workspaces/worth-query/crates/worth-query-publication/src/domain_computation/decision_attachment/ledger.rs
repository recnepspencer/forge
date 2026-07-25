use std::collections::BTreeMap;

use worth_query_installation::facade::WorthQueryStructuralCounterScope;

use super::{
    WorthQueryAdmittedStructuralCounter, WorthQueryDomainEvidenceAdmissionDenial,
    WorthQueryDomainEvidenceAdmissionDenialKind,
};

#[derive(Clone, Debug, Default)]
pub struct WorthQueryDomainEvidenceAdmissionLedger {
    observed: BTreeMap<(String, String), u64>,
}

impl WorthQueryDomainEvidenceAdmissionLedger {
    pub fn validate_and_retain(
        &mut self,
        contract_identity: &str,
        counters: &[WorthQueryAdmittedStructuralCounter],
    ) -> Result<(), WorthQueryDomainEvidenceAdmissionDenial> {
        let mut candidate = self.clone();
        for counter in counters {
            if !matches!(
                counter.schema().scope(),
                WorthQueryStructuralCounterScope::Operation | WorthQueryStructuralCounterScope::Run
            ) {
                continue;
            }
            let key = (
                contract_identity.to_owned(),
                counter.schema().name().as_str().to_owned(),
            );
            if candidate
                .observed
                .get(&key)
                .is_some_and(|prior| counter.initial() < *prior || counter.observed() < *prior)
            {
                return Err(WorthQueryDomainEvidenceAdmissionDenial::new(
                    WorthQueryDomainEvidenceAdmissionDenialKind::LedgerRegression,
                    counter.schema().name().as_str(),
                ));
            }
            candidate.observed.insert(key, counter.observed());
        }
        *self = candidate;
        Ok(())
    }
}
