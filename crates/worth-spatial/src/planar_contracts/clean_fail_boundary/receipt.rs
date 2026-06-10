use super::{
    planar_clean_fail_boundary_authority_entries, planar_clean_fail_boundary_digest,
    PlanarBoundedConversion, PlanarCleanFailAction, PlanarCleanFailBoundaryBasis,
    PlanarCleanFailBoundaryCounters, PlanarCleanFailClass, PlanarCleanFailTruthEffect,
    PlanarRepairAttempt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarCleanFailBoundaryReceipt {
    basis: PlanarCleanFailBoundaryBasis,
    class: PlanarCleanFailClass,
    action: PlanarCleanFailAction,
    repair_attempt: PlanarRepairAttempt,
    bounded_conversion: PlanarBoundedConversion,
    truth_effect: PlanarCleanFailTruthEffect,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    clean_fail_boundary_digest: String,
    counters: PlanarCleanFailBoundaryCounters,
}

impl PlanarCleanFailBoundaryReceipt {
    pub(crate) fn new(
        basis: PlanarCleanFailBoundaryBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        clean_fail_boundary_digest: String,
        counters: PlanarCleanFailBoundaryCounters,
    ) -> Self {
        Self {
            class: basis.input().class(),
            action: basis.input().action(),
            repair_attempt: basis.repair_attempt(),
            bounded_conversion: basis.bounded_conversion(),
            truth_effect: basis.truth_effect(),
            basis,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            clean_fail_boundary_digest,
            counters,
        }
    }

    pub(crate) fn clean_fail_boundary_digest_for(
        basis: &PlanarCleanFailBoundaryBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
    ) -> String {
        let mut parts = planar_clean_fail_boundary_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        planar_clean_fail_boundary_digest(&parts)
    }

    pub fn basis(&self) -> &PlanarCleanFailBoundaryBasis {
        &self.basis
    }

    pub fn class(&self) -> PlanarCleanFailClass {
        self.class
    }

    pub fn action(&self) -> PlanarCleanFailAction {
        self.action
    }

    pub fn repair_attempt(&self) -> PlanarRepairAttempt {
        self.repair_attempt
    }

    pub fn bounded_conversion(&self) -> PlanarBoundedConversion {
        self.bounded_conversion
    }

    pub fn truth_effect(&self) -> PlanarCleanFailTruthEffect {
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

    pub fn clean_fail_boundary_digest(&self) -> &str {
        &self.clean_fail_boundary_digest
    }

    pub fn counters(&self) -> PlanarCleanFailBoundaryCounters {
        self.counters
    }
}
