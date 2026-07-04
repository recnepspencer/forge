use topology::facade::{
    current_topology_compiled_product_reuse_route_packet, TopologyCompiledProductReuseRoutePacket,
    TopologyDerivedReuseDecisionPosture,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};
use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;
use worth_spatial::facade::evidence_lookup_reuse_route::{
    current_evidence_lookup_reuse_route_packet, EvidenceLookupReuseRoutePacket,
};

use super::{PlannerOwnedRoutingError, PlannerOwnedRoutingErrorKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompiledProductReusePlannerRoutePacket {
    packet_identity: String,
    topology_route_packet: TopologyCompiledProductReuseRoutePacket,
    spatial_route_packet: EvidenceLookupReuseRoutePacket,
    topology_posture: TopologyDerivedReuseDecisionPosture,
    topology_reuse_decision_identity_digest: Option<String>,
    topology_rebuild_denial_identity_digest: Option<String>,
    spatial_posture: EvidenceLookupReuseDecisionPosture,
    spatial_reuse_decision_identity_digest: Option<String>,
    spatial_rebuild_denial_identity_digest: Option<String>,
}

pub fn current_worth_touched_graph_conflict_compiled_product_reuse_route_packet(
) -> Result<CompiledProductReusePlannerRoutePacket, PlannerOwnedRoutingError> {
    let topology_route_packet =
        current_topology_compiled_product_reuse_route_packet().map_err(|error| {
            PlannerOwnedRoutingError::new(
                PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
                error.detail(),
            )
        })?;
    let spatial_route_packet = current_evidence_lookup_reuse_route_packet().map_err(|error| {
        PlannerOwnedRoutingError::new(
            PlannerOwnedRoutingErrorKind::CurrentProofUnavailable,
            error.detail(),
        )
    })?;
    Ok(CompiledProductReusePlannerRoutePacket::from_parts(
        topology_route_packet,
        spatial_route_packet,
    ))
}

impl CompiledProductReusePlannerRoutePacket {
    pub(crate) fn from_parts(
        topology_route_packet: TopologyCompiledProductReuseRoutePacket,
        spatial_route_packet: EvidenceLookupReuseRoutePacket,
    ) -> Self {
        let packet_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-kernel:compiled-product-reuse-route-packet:v1".to_string(),
                format!("topology:{}", topology_route_packet.packet_identity()),
                format!("spatial:{}", spatial_route_packet.packet_identity()),
            ],
        );
        Self {
            packet_identity,
            topology_posture: topology_route_packet.posture(),
            topology_reuse_decision_identity_digest: topology_route_packet
                .reuse_decision_identity_digest()
                .map(str::to_string),
            topology_rebuild_denial_identity_digest: topology_route_packet
                .rebuild_denial_identity_digest()
                .map(str::to_string),
            spatial_posture: spatial_route_packet.posture(),
            spatial_reuse_decision_identity_digest: spatial_route_packet
                .reuse_decision_identity_digest()
                .map(str::to_string),
            spatial_rebuild_denial_identity_digest: spatial_route_packet
                .rebuild_denial_identity_digest()
                .map(str::to_string),
            topology_route_packet,
            spatial_route_packet,
        }
    }

    pub fn packet_identity(&self) -> &str {
        &self.packet_identity
    }

    pub fn selected_family_identity(&self) -> &str {
        self.topology_route_packet.selected_family_identity()
    }

    pub fn topology_selected_family_identity(&self) -> &str {
        self.topology_route_packet.selected_family_identity()
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        self.spatial_route_packet.selected_product_identity_digest()
    }

    pub fn topology_selected_product_identity_digest(&self) -> &str {
        self.topology_route_packet
            .selected_product_identity_digest()
    }

    pub fn topology_selected_equivalence_policy_identity_digest(&self) -> &str {
        self.topology_route_packet
            .selected_equivalence_policy_identity_digest()
    }

    pub fn topology_selected_compatibility_basis_identity_digest(&self) -> &str {
        self.topology_route_packet
            .selected_compatibility_basis_identity_digest()
    }

    pub fn topology_selected_reuse_basis_identity_digest(&self) -> &str {
        self.topology_route_packet
            .selected_reuse_basis_identity_digest()
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        self.topology_route_packet
            .selected_reuse_basis_identity_digest()
    }

    pub fn spatial_selected_family_identity(&self) -> &str {
        self.spatial_route_packet.selected_family_identity()
    }

    pub fn spatial_selected_product_identity_digest(&self) -> &str {
        self.spatial_route_packet.selected_product_identity_digest()
    }

    pub fn spatial_equivalence_policy_identity_digest(&self) -> &str {
        self.spatial_route_packet
            .equivalence_policy_identity_digest()
    }

    pub fn spatial_selected_compatibility_basis_identity_digest(&self) -> &str {
        self.spatial_route_packet
            .selected_compatibility_basis_identity_digest()
    }

    pub fn spatial_selected_reuse_basis_identity_digest(&self) -> &str {
        self.spatial_route_packet
            .selected_reuse_basis_identity_digest()
    }

    pub const fn topology_posture(&self) -> TopologyDerivedReuseDecisionPosture {
        self.topology_posture
    }

    pub const fn spatial_posture(&self) -> EvidenceLookupReuseDecisionPosture {
        self.spatial_posture
    }

    pub(crate) fn topology_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.topology_reuse_decision_identity_digest.as_deref()
    }

    pub(crate) fn topology_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.topology_rebuild_denial_identity_digest.as_deref()
    }

    pub(crate) fn spatial_reuse_decision_identity_digest(&self) -> Option<&str> {
        self.spatial_reuse_decision_identity_digest.as_deref()
    }

    pub(crate) fn spatial_rebuild_denial_identity_digest(&self) -> Option<&str> {
        self.spatial_rebuild_denial_identity_digest.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use topology::facade::{
        current_topology_compiled_product_reuse_route_packet, TopologyDerivedReuseDecisionPosture,
    };
    use worth_spatial::facade::evidence_lookup_reuse_route::current_evidence_lookup_reuse_route_packet;
    use worth_spatial::facade::evidence_lookup_reuse_route::EvidenceLookupReuseDecisionPosture;

    use crate::workload_composition::planner_owned_routing::{
        current_worth_touched_graph_conflict_derived_diagnostic_projection,
        current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader,
        current_worth_touched_graph_conflict_public_proof_input_with_packet_loader,
        current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders,
    };

    use super::*;

    #[test]
    fn reuse_explanation_consumes_milestone_fourteen_products_only() {
        let reuse_route =
            current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
                .expect("reuse route should build");
        let selected_route = crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("selected route should build");
        let proof = crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_public_proof_input()
            .expect("public proof input should build");
        let diagnostics_projection =
            current_worth_touched_graph_conflict_derived_diagnostic_projection()
                .expect("diagnostic projection should build");
        let rich_localization = diagnostics_projection
            .rich_localization()
            .expect("rich localization should remain available by default");

        assert_eq!(
            selected_route.compiled_product_reuse_route_packet_identity(),
            reuse_route.packet_identity()
        );
        assert_eq!(
            proof.compiled_product_reuse_route_packet_identity(),
            reuse_route.packet_identity()
        );
        assert_eq!(
            selected_route.selected_witness_identity_digest(),
            reuse_route.topology_reuse_decision_identity_digest()
        );
        assert_eq!(
            proof.selected_witness_identity_digest(),
            reuse_route.topology_reuse_decision_identity_digest()
        );
        assert_eq!(
            proof.rebuild_denial_identity_digest(),
            reuse_route.topology_rebuild_denial_identity_digest()
        );
        assert_eq!(
            proof.spatial_rebuild_denial_identity_digest(),
            reuse_route.spatial_rebuild_denial_identity_digest()
        );
        assert_eq!(
            rich_localization.compiled_product_reuse_route_packet_identity(),
            Some(reuse_route.packet_identity())
        );
        assert_eq!(
            rich_localization.spatial_rebuild_denial_identity_digest(),
            reuse_route.spatial_rebuild_denial_identity_digest()
        );
    }

    #[test]
    fn compatibility_without_reuse_remains_distinct_from_reuse() {
        let base_route = current_worth_touched_graph_conflict_compiled_product_reuse_route_packet()
            .expect("reuse route should build");
        let advisory_topology_route = current_topology_compiled_product_reuse_route_packet()
            .expect("topology reuse route should build")
            .with_test_posture(
                TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild,
                None,
                Some("phase-12-topology-advisory-rebuild"),
            );
        let advisory_spatial_route =
            current_evidence_lookup_reuse_route_packet().expect("spatial reuse route should build");
        let advisory_route = CompiledProductReusePlannerRoutePacket::from_parts(
            advisory_topology_route,
            advisory_spatial_route,
        );
        let selected_route_packet =
            current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
                topology::certification::current_topology_milestone_fifteen_planner_seed_support,
                worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
                crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_independence_route_packet,
                || Ok(advisory_route.clone()),
            )
            .expect("selected route with reuse route override");
        let proof =
            current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
                Ok(selected_route_packet.clone())
            })
            .expect("proof should lower");
        let diagnostics_projection =
            current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader(
                || Ok(selected_route_packet.clone()),
            )
            .expect("diagnostics should lower");
        let rich_localization = diagnostics_projection
            .rich_localization()
            .expect("rich localization should remain available by default");

        assert_eq!(
            base_route.topology_posture(),
            TopologyDerivedReuseDecisionPosture::ReuseAdmitted
        );
        assert_eq!(
            proof.topology_reuse_posture(),
            Some(TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild)
        );
        assert_eq!(
            rich_localization.topology_reuse_posture(),
            Some(TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild)
        );
        assert_ne!(
            rich_localization.topology_reuse_posture(),
            Some(TopologyDerivedReuseDecisionPosture::ReuseAdmitted)
        );
        assert!(matches!(
            base_route.spatial_posture(),
            EvidenceLookupReuseDecisionPosture::ReuseAdmitted
                | EvidenceLookupReuseDecisionPosture::AdvisoryMatchRequiresRebuild
        ));
    }

    #[test]
    fn spatial_rebuild_denial_identity_survives_public_proof_and_diagnostics() {
        let denied_topology_route = current_topology_compiled_product_reuse_route_packet()
            .expect("topology reuse route should build");
        let denied_spatial_route = current_evidence_lookup_reuse_route_packet()
            .expect("spatial reuse route should build")
            .with_test_posture(
                EvidenceLookupReuseDecisionPosture::Denied,
                None,
                Some("phase-12-spatial-rebuild-denial"),
            );
        let denied_route = CompiledProductReusePlannerRoutePacket::from_parts(
            denied_topology_route,
            denied_spatial_route,
        );
        let selected_route_packet =
            current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
                topology::certification::current_topology_milestone_fifteen_planner_seed_support,
                worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
                crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_independence_route_packet,
                || Ok(denied_route.clone()),
            )
            .expect("selected route with spatial denial override");
        let proof =
            current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
                Ok(selected_route_packet.clone())
            })
            .expect("proof should lower");
        let diagnostics_projection =
            current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader(
                || Ok(selected_route_packet.clone()),
            )
            .expect("diagnostics should lower");
        let rich_localization = diagnostics_projection
            .rich_localization()
            .expect("rich localization should remain available by default");

        assert_eq!(
            proof.spatial_reuse_posture(),
            Some(EvidenceLookupReuseDecisionPosture::Denied)
        );
        assert_eq!(
            proof.spatial_rebuild_denial_identity_digest(),
            Some("phase-12-spatial-rebuild-denial")
        );
        assert_eq!(rich_localization.spatial_reuse_posture(), Some("Denied"));
        assert_eq!(
            rich_localization.spatial_rebuild_denial_identity_digest(),
            Some("phase-12-spatial-rebuild-denial")
        );
    }

    #[test]
    fn spatial_reuse_decision_identity_survives_public_proof_and_diagnostics() {
        let reused_topology_route = current_topology_compiled_product_reuse_route_packet()
            .expect("topology reuse route should build");
        let reused_spatial_route = current_evidence_lookup_reuse_route_packet()
            .expect("spatial reuse route should build")
            .with_test_posture(
                EvidenceLookupReuseDecisionPosture::ReuseAdmitted,
                Some("phase-12-spatial-reuse-decision"),
                None,
            );
        let reused_route = CompiledProductReusePlannerRoutePacket::from_parts(
            reused_topology_route,
            reused_spatial_route,
        );
        let selected_route_packet =
            current_worth_touched_graph_conflict_selected_route_packet_with_route_loaders(
                topology::certification::current_topology_milestone_fifteen_planner_seed_support,
                worth_spatial::certification::current_spatial_milestone_fifteen_planner_seed_support,
                crate::workload_composition::planner_owned_routing::current_replay_undo_transaction_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_batch_admission_route_packet,
                crate::workload_composition::planner_owned_routing::current_worth_touched_graph_conflict_independence_route_packet,
                || Ok(reused_route.clone()),
            )
            .expect("selected route with spatial reuse override");
        let proof =
            current_worth_touched_graph_conflict_public_proof_input_with_packet_loader(|| {
                Ok(selected_route_packet.clone())
            })
            .expect("proof should lower");
        let diagnostics_projection =
            current_worth_touched_graph_conflict_derived_diagnostic_projection_with_packet_loader(
                || Ok(selected_route_packet.clone()),
            )
            .expect("diagnostics should lower");
        let rich_localization = diagnostics_projection
            .rich_localization()
            .expect("rich localization should remain available by default");

        assert_eq!(
            proof.spatial_reuse_posture(),
            Some(EvidenceLookupReuseDecisionPosture::ReuseAdmitted)
        );
        assert_eq!(
            proof.spatial_reuse_decision_identity_digest(),
            Some("phase-12-spatial-reuse-decision")
        );
        assert_eq!(
            rich_localization.spatial_reuse_posture(),
            Some("ReuseAdmitted")
        );
        assert_eq!(
            rich_localization.spatial_reuse_decision_identity_digest(),
            Some("phase-12-spatial-reuse-decision")
        );
    }
}
