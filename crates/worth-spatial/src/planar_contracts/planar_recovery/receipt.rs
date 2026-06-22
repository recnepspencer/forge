use super::{
    planar_recovery_digest, planar_recovery_posture_authority_entries, PlanarRecoveryAction,
    PlanarRecoveryBlockerKind, PlanarRecoveryPostureBasis, PlanarRecoveryPostureCounters,
    PlanarRecoverySourcePosture, PlanarRecoveryTargetScope, PlanarRecoveryTruthEffect,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PlanarRecoveryPostureReceipt {
    basis: PlanarRecoveryPostureBasis,
    blocker_kind: PlanarRecoveryBlockerKind,
    source_posture: PlanarRecoverySourcePosture,
    recovery_action: PlanarRecoveryAction,
    target_scope: PlanarRecoveryTargetScope,
    truth_effect: PlanarRecoveryTruthEffect,
    declaration_digest: String,
    progression_digest: String,
    route_plan_digest: String,
    query_receipt_digest: String,
    envelope_digest: String,
    recovery_posture_digest: String,
    counters: PlanarRecoveryPostureCounters,
}

impl PlanarRecoveryPostureReceipt {
    pub(crate) fn new(
        basis: PlanarRecoveryPostureBasis,
        declaration_digest: String,
        progression_digest: String,
        route_plan_digest: String,
        query_receipt_digest: String,
        envelope_digest: String,
        recovery_posture_digest: String,
        counters: PlanarRecoveryPostureCounters,
    ) -> Self {
        Self {
            blocker_kind: basis.blocker_kind(),
            source_posture: basis.source_posture(),
            recovery_action: basis.recovery_action(),
            target_scope: basis.target_scope(),
            truth_effect: basis.truth_effect(),
            basis,
            declaration_digest,
            progression_digest,
            route_plan_digest,
            query_receipt_digest,
            envelope_digest,
            recovery_posture_digest,
            counters,
        }
    }

    pub(crate) fn recovery_posture_digest_for(
        basis: &PlanarRecoveryPostureBasis,
        declaration_digest: &str,
        progression_digest: &str,
        route_plan_digest: &str,
        query_receipt_digest: &str,
        envelope_digest: &str,
    ) -> String {
        let mut parts = planar_recovery_posture_authority_entries(basis)
            .into_iter()
            .map(|entry| entry.digest_part())
            .collect::<Vec<_>>();
        parts.push(format!("declaration:{declaration_digest}"));
        parts.push(format!("progression:{progression_digest}"));
        parts.push(format!("route_plan:{route_plan_digest}"));
        parts.push(format!("query_receipt:{query_receipt_digest}"));
        parts.push(format!("envelope:{envelope_digest}"));
        planar_recovery_digest(&parts)
    }

    pub fn basis(&self) -> &PlanarRecoveryPostureBasis {
        &self.basis
    }

    pub fn blocker_kind(&self) -> PlanarRecoveryBlockerKind {
        self.blocker_kind
    }

    pub fn source_posture(&self) -> PlanarRecoverySourcePosture {
        self.source_posture
    }

    pub fn recovery_action(&self) -> PlanarRecoveryAction {
        self.recovery_action
    }

    pub fn target_scope(&self) -> PlanarRecoveryTargetScope {
        self.target_scope
    }

    pub fn truth_effect(&self) -> PlanarRecoveryTruthEffect {
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

    pub fn recovery_posture_digest(&self) -> &str {
        &self.recovery_posture_digest
    }

    pub fn counters(&self) -> PlanarRecoveryPostureCounters {
        self.counters
    }
}
