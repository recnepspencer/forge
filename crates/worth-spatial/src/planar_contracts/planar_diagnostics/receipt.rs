use super::{
    planar_diagnostic_authority_entries, planar_diagnostic_digest, PlanarDiagnosticBundleBasis,
    PlanarDiagnosticCounters, PlanarDiagnosticTriggerLocality, PlanarDiagnosticTruthEffect,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDiagnosticBundleReceipt {
    basis: PlanarDiagnosticBundleBasis,
    trigger_locality: PlanarDiagnosticTriggerLocality,
    truth_effect: PlanarDiagnosticTruthEffect,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    diagnostic_bundle_digest: String,
    counters: PlanarDiagnosticCounters,
}

impl PlanarDiagnosticBundleReceipt {
    pub(crate) fn new(
        basis: PlanarDiagnosticBundleBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        diagnostic_bundle_digest: String,
        counters: PlanarDiagnosticCounters,
    ) -> Self {
        Self {
            trigger_locality: basis.subject().trigger_locality(),
            truth_effect: basis.truth_effect(),
            basis,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            diagnostic_bundle_digest,
            counters,
        }
    }

    pub(crate) fn diagnostic_bundle_digest_for(
        basis: &PlanarDiagnosticBundleBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
    ) -> String {
        let mut parts = planar_diagnostic_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        planar_diagnostic_digest(&parts)
    }

    pub fn basis(&self) -> &PlanarDiagnosticBundleBasis {
        &self.basis
    }

    pub fn trigger_locality(&self) -> PlanarDiagnosticTriggerLocality {
        self.trigger_locality
    }

    pub fn truth_effect(&self) -> PlanarDiagnosticTruthEffect {
        self.truth_effect
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn progression_digest(&self) -> &str {
        &self.progression_digest
    }

    pub fn route_plan_digest(&self) -> &str {
        &self.route_plan_digest
    }

    pub fn query_receipt_digest(&self) -> &str {
        &self.query_receipt_digest
    }

    pub fn envelope_digest(&self) -> &str {
        &self.envelope_digest
    }

    pub fn diagnostic_bundle_digest(&self) -> &str {
        &self.diagnostic_bundle_digest
    }

    pub fn counters(&self) -> PlanarDiagnosticCounters {
        self.counters
    }
}
