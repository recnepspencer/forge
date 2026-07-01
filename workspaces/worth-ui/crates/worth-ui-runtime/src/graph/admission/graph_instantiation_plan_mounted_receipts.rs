use crate::declaration::{stable_text_digest, UiDeclarationIdentity};
use crate::graph::{
    UiGraphInstantiationPlan, UiGraphMountedReceiptReservation, UiGraphNodeIdentity,
    UiGraphWorldProfile, UiRepeatedInstanceBasis,
};

impl UiGraphInstantiationPlan {
    pub fn mounted_receipt_reservations(
        &self,
        world_profile: UiGraphWorldProfile,
    ) -> Vec<UiGraphMountedReceiptReservation> {
        self.node_entries()
            .iter()
            .map(|entry| UiGraphMountedReceiptReservation::graph_owned_seed_slot(
                graph_node_identity(
                    entry.declaration_identity(),
                    entry.repeated_instance_basis(),
                    &world_profile,
                ),
                entry.mounted_receipt_seed(),
            ))
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
