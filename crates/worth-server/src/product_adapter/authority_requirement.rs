#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthServerProductOperationAuthorityRequirement {
    SharedRead,
    DraftMutation {
        draft_scope: String,
    },
    DurableMutation {
        contract: crate::WorthServerDurableProductMutationContract,
    },
    SessionCoordination {
        coordination_lane: String,
    },
}
