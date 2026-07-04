use crate::compiled_product_reuse_decision::TopologyDerivedReuseDecisionPosture;
use crate::facade::{
    current_topology_query_backed_consumer_cutover, TopologyQueryBackedConsumerCutoverCurrentError,
    TopologyReadModelReusePosture,
};
use crate::query_domain::TopologyReadRequestFamily;
use std::sync::OnceLock;
use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TopologyCompiledProductReuseRoutePacket {
    packet_identity: String,
    selected_family_identity: String,
    selected_product_identity_digest: String,
    selected_equivalence_policy_identity_digest: String,
    selected_compatibility_basis_identity_digest: String,
    selected_reuse_basis_identity_digest: String,
    posture: TopologyDerivedReuseDecisionPosture,
    reuse_decision_identity_digest: Option<String>,
    rebuild_denial_identity_digest: Option<String>,
}

pub fn current_topology_compiled_product_reuse_route_packet(
) -> Result<TopologyCompiledProductReuseRoutePacket, TopologyQueryBackedConsumerCutoverCurrentError>
{
    static CACHE: OnceLock<TopologyCompiledProductReuseRoutePacket> = OnceLock::new();
    if let Some(cached) = CACHE.get() {
        return Ok(cached.clone());
    }

    let cutover = current_topology_query_backed_consumer_cutover()?;
    let row = cutover
        .family_rows()
        .iter()
        .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
        .expect("loop-cycle row should exist on current topology cutover");
    let selected_family_identity = row
        .selected_equivalence_family_identity()
        .expect("typed topology reuse route should carry selected family")
        .to_string();
    let selected_product_identity_digest = row
        .compiled_product_identity_digest()
        .expect("typed topology reuse route should carry compiled product")
        .to_string();
    let selected_equivalence_policy_identity_digest = row
        .equivalence_policy_identity_digest()
        .expect("typed topology reuse route should carry equivalence policy")
        .to_string();
    let selected_compatibility_basis_identity_digest = row
        .selected_compatibility_basis_identity_digest()
        .expect("typed topology reuse route should carry compatibility basis")
        .to_string();
    let selected_reuse_basis_identity_digest = row
        .selected_reuse_basis_identity_digest()
        .expect("typed topology reuse route should carry reuse basis")
        .to_string();
    let posture = topology_route_posture(row.reuse_posture());
    let reuse_decision_identity_digest = row.reuse_decision_identity_digest().map(str::to_string);
    let rebuild_denial_identity_digest = row.rebuild_denial_identity_digest().map(str::to_string);
    let packet_identity = truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[
            "worth-topo:compiled-product-reuse-route-packet:v1".to_string(),
            format!("cutover:{}", cutover.closeout_digest()),
            format!("row:{}", row.row_digest()),
            format!("selected-family:{selected_family_identity}"),
            format!("selected-product:{selected_product_identity_digest}"),
            format!("equivalence-policy:{selected_equivalence_policy_identity_digest}"),
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

    let packet = TopologyCompiledProductReuseRoutePacket {
        packet_identity,
        selected_family_identity,
        selected_product_identity_digest,
        selected_equivalence_policy_identity_digest,
        selected_compatibility_basis_identity_digest,
        selected_reuse_basis_identity_digest,
        posture,
        reuse_decision_identity_digest,
        rebuild_denial_identity_digest,
    };
    let _ = CACHE.set(packet.clone());
    Ok(packet)
}

impl TopologyCompiledProductReuseRoutePacket {
    pub fn packet_identity(&self) -> &str {
        &self.packet_identity
    }

    pub fn selected_family_identity(&self) -> &str {
        &self.selected_family_identity
    }

    pub fn selected_product_identity_digest(&self) -> &str {
        &self.selected_product_identity_digest
    }

    pub fn selected_equivalence_policy_identity_digest(&self) -> &str {
        &self.selected_equivalence_policy_identity_digest
    }

    pub fn selected_compatibility_basis_identity_digest(&self) -> &str {
        &self.selected_compatibility_basis_identity_digest
    }

    pub fn selected_reuse_basis_identity_digest(&self) -> &str {
        &self.selected_reuse_basis_identity_digest
    }

    pub const fn posture(&self) -> TopologyDerivedReuseDecisionPosture {
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
        posture: TopologyDerivedReuseDecisionPosture,
        reuse_decision_identity_digest: Option<&str>,
        rebuild_denial_identity_digest: Option<&str>,
    ) -> Self {
        self.posture = posture;
        self.reuse_decision_identity_digest = reuse_decision_identity_digest.map(str::to_string);
        self.rebuild_denial_identity_digest = rebuild_denial_identity_digest.map(str::to_string);
        self.packet_identity = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &[
                "worth-topo:compiled-product-reuse-route-packet:v1".to_string(),
                format!("selected-family:{}", self.selected_family_identity),
                format!("selected-product:{}", self.selected_product_identity_digest),
                format!(
                    "equivalence-policy:{}",
                    self.selected_equivalence_policy_identity_digest
                ),
                format!(
                    "selected-compatibility-basis:{}",
                    self.selected_compatibility_basis_identity_digest
                ),
                format!(
                    "selected-reuse-basis:{}",
                    self.selected_reuse_basis_identity_digest
                ),
                format!("posture:{:?}", self.posture),
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

fn topology_route_posture(
    posture: TopologyReadModelReusePosture,
) -> TopologyDerivedReuseDecisionPosture {
    match posture {
        TopologyReadModelReusePosture::ReuseAdmitted => {
            TopologyDerivedReuseDecisionPosture::ReuseAdmitted
        }
        TopologyReadModelReusePosture::FreshRebuildRequired => {
            TopologyDerivedReuseDecisionPosture::FreshRebuildRequired
        }
        TopologyReadModelReusePosture::CompatibilityWithoutReuse => {
            TopologyDerivedReuseDecisionPosture::AdvisoryMatchRequiresRebuild
        }
        TopologyReadModelReusePosture::Denied => TopologyDerivedReuseDecisionPosture::Denied,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn current_topology_reuse_route_consumes_loop_cycle_cutover_row() {
        let cutover = current_topology_query_backed_consumer_cutover()
            .expect("current topology cutover should build");
        let row = cutover
            .family_rows()
            .iter()
            .find(|row| row.request_family() == TopologyReadRequestFamily::LoopCycleNeighborhood)
            .expect("loop-cycle row should exist on current topology cutover");
        let route = current_topology_compiled_product_reuse_route_packet()
            .expect("current topology reuse route should build");

        assert_eq!(
            route.selected_family_identity(),
            row.selected_equivalence_family_identity()
                .expect("route row should carry selected family")
        );
        assert_eq!(
            route.selected_product_identity_digest(),
            row.compiled_product_identity_digest()
                .expect("route row should carry compiled product")
        );
        assert_eq!(
            route.selected_equivalence_policy_identity_digest(),
            row.equivalence_policy_identity_digest()
                .expect("route row should carry equivalence policy")
        );
        assert_eq!(
            route.selected_compatibility_basis_identity_digest(),
            row.selected_compatibility_basis_identity_digest()
                .expect("route row should carry compatibility basis")
        );
        assert_eq!(
            route.selected_reuse_basis_identity_digest(),
            row.selected_reuse_basis_identity_digest()
                .expect("route row should carry selected reuse basis")
        );
        assert_eq!(route.posture(), topology_route_posture(row.reuse_posture()));
        assert_eq!(
            route.reuse_decision_identity_digest(),
            row.reuse_decision_identity_digest()
        );
        assert_eq!(
            route.rebuild_denial_identity_digest(),
            row.rebuild_denial_identity_digest()
        );
    }
}
