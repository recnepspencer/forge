use std::sync::OnceLock;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::workload_platform::evidence_lookup_reuse_decision::EvidenceLookupReuseDecisionPosture;
use crate::workload_platform::planner_owned_routing::evidence_lookup_route::{
    current_evidence_lookup_route_source, EvidenceLookupRouteAdmissionError,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupReuseRoutePacket {
    packet_identity: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    equivalence_policy_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    posture: EvidenceLookupReuseDecisionPosture,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
}

pub fn current_evidence_lookup_reuse_route_packet(
) -> Result<EvidenceLookupReuseRoutePacket, EvidenceLookupRouteAdmissionError> {
    static CACHE: OnceLock<EvidenceLookupReuseRoutePacket> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let source = current_evidence_lookup_route_source()?;
    let boundary = source.left_boundary();
    let handoff = boundary.workload_handoff();
    let reuse_resolution = boundary.reuse_resolution();
    let decision = reuse_resolution.decision();
    let posture = decision.posture();
    let reuse_decision_identity_digest = decision
        .reuse_decision_identity_digest()
        .map(str::to_string);
    let rebuild_denial_identity_digest = reuse_resolution
        .denial()
        .map(|denial| denial.denial_identity_digest().to_string());
    let selected_family_identity = decision
        .selected_equivalence_family_identity()
        .as_str()
        .to_string();
    let selected_product_identity_digest = decision.compiled_product_identity_digest().to_string();
    let equivalence_policy_identity_digest =
        decision.equivalence_policy_identity_digest().to_string();
    let selected_compatibility_basis_identity_digest = decision
        .selected_compatibility_basis_identity_digest()
        .to_string();
    let selected_reuse_basis_identity_digest =
        decision.selected_reuse_basis_identity_digest().to_string();
    let packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-spatial:evidence-lookup-reuse-route-packet:v1".to_string(),
            format!("route-authority:{}", source.route_authority_digest()),
            format!("selected-plan:{}", handoff.selected_lookup_plan_digest()),
            format!(
                "lookup-receipt:{}",
                handoff.lookup_execution_receipt_digest()
            ),
            format!("selected-family:{selected_family_identity}"),
            format!("selected-product:{selected_product_identity_digest}"),
            format!("equivalence-policy:{equivalence_policy_identity_digest}"),
            format!("selected-compatibility-basis:{selected_compatibility_basis_identity_digest}"),
            format!("selected-reuse-basis:{selected_reuse_basis_identity_digest}"),
            format!("posture:{posture:?}"),
            format!(
                "reuse-decision:{}",
                reuse_decision_identity_digest
                    .as_deref()
                    .unwrap_or("not-applicable")
            ),
            format!(
                "rebuild-denial:{}",
                rebuild_denial_identity_digest
                    .as_deref()
                    .unwrap_or("not-applicable")
            ),
        ],
    );

    let packet = EvidenceLookupReuseRoutePacket {
        packet_identity,
        selected_family_identity,
        selected_product_identity_digest,
        equivalence_policy_identity_digest,
        selected_compatibility_basis_identity_digest,
        selected_reuse_basis_identity_digest,
        posture,
        reuse_decision_identity_digest,
        rebuild_denial_identity_digest,
    };
    let _ = CACHE.set(packet.clone());
    Ok(packet)
}

impl EvidenceLookupReuseRoutePacket {
    pub fn packet_identity(&self) -> &str {
        &self.packet_identity
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn equivalence_policy_identity_digest(&self) -> &str {
        &self.equivalence_policy_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub const fn posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.posture
    }

    pub fn reuse_decision_identity_digest(&self) -> Option<&str> {
        self.reuse_decision_identity_digest.as_deref()
    }

    pub fn rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.rebuild_denial_identity_digest.as_deref()
    }

    pub fn with_test_posture(
        mut self,
        posture: EvidenceLookupReuseDecisionPosture,
        reuse_decision_identity_digest: Option<&str>,
        rebuild_denial_identity_digest: Option<&str>,
    ) -> Self {
        self.posture = posture;
        self.reuse_decision_identity_digest = reuse_decision_identity_digest.map(str::to_string);
        self.rebuild_denial_identity_digest = rebuild_denial_identity_digest.map(str::to_string);
        self.packet_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-spatial:evidence-lookup-reuse-route-packet:v1".to_string(),
                format!("selected-family:{}", self.selected_family_identity),
                format!("selected-product:{}", self.selected_product_identity_digest),
                format!(
                    "equivalence-policy:{}",
                    self.equivalence_policy_identity_digest
                ),
                format!(
                    "selected-compatibility-basis:{}",
                    self.selected_compatibility_basis_identity_digest
                ),
                format!(
                    "selected-reuse-basis:{}",
                    self.selected_reuse_basis_identity_digest
                ),
                format!("posture:{posture:?}"),
                format!(
                    "reuse-decision:{}",
                    self.reuse_decision_identity_digest
                        .as_deref()
                        .unwrap_or("not-applicable")
                ),
                format!(
                    "rebuild-denial:{}",
                    self.rebuild_denial_identity_digest
                        .as_deref()
                        .unwrap_or("not-applicable")
                ),
            ],
        );
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_evidence_lookup_reuse_route_consumes_boundary_reuse_resolution() {
        let source = current_evidence_lookup_route_source().expect("current route source");
        let boundary = source.left_boundary();
        let resolution = boundary.reuse_resolution();
        let route = current_evidence_lookup_reuse_route_packet()
            .expect("current evidence lookup reuse route should build");

        assert_eq!(route.posture(), resolution.decision().posture());
        assert_eq!(
            route.reuse_decision_identity_digest(),
            resolution.decision().reuse_decision_identity_digest()
        );
        assert_eq!(
            route.rebuild_denial_identity_digest(),
            resolution
                .denial()
                .map(|denial| denial.denial_identity_digest())
        );
        assert_eq!(
            route.selected_family_identity(),
            resolution
                .decision()
                .selected_equivalence_family_identity()
                .as_str()
        );
        assert_eq!(
            route.selected_product_identity_digest(),
            resolution.decision().compiled_product_identity_digest()
        );
        assert_eq!(
            route.equivalence_policy_identity_digest(),
            resolution.decision().equivalence_policy_identity_digest()
        );
        assert_eq!(
            route.selected_compatibility_basis_identity_digest(),
            resolution
                .decision()
                .selected_compatibility_basis_identity_digest()
        );
        assert_eq!(
            route.selected_reuse_basis_identity_digest(),
            resolution.decision().selected_reuse_basis_identity_digest()
        );
    }
}
