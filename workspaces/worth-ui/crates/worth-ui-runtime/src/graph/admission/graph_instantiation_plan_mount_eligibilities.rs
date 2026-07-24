use crate::declaration::{stable_text_digest, UiDeclarationIdentity};
use crate::graph::{
    UiGraphInstantiationPlan, UiGraphMountEligibilityReservation, UiGraphNodeIdentity,
    UiGraphWorldProfile, UiRepeatedInstanceBasis,
};

impl UiGraphInstantiationPlan {
    pub fn mount_eligibility_reservations(
        &self,
        world_profile: UiGraphWorldProfile,
    ) -> Vec<UiGraphMountEligibilityReservation> {
        self.node_entries()
            .iter()
            .map(|entry| {
                UiGraphMountEligibilityReservation::graph_owned_seed_slot(
                    graph_node_identity(
                        entry.declaration_identity(),
                        entry.repeated_instance_basis(),
                        &world_profile,
                    ),
                    entry.mount_eligibility_seed(),
                )
            })
            .collect()
    }
}

fn graph_node_identity(
    declaration_identity: &UiDeclarationIdentity,
    repeated_instance_basis: &UiRepeatedInstanceBasis,
    world_profile: &UiGraphWorldProfile,
) -> UiGraphNodeIdentity {
    UiGraphNodeIdentity::new(
        stable_text_digest("graph-node")
            ^ declaration_identity.digest().raw().rotate_left(11)
            ^ repeated_instance_basis.identity_digest().rotate_left(29)
            ^ world_profile.identity_digest().rotate_left(47),
    )
}
