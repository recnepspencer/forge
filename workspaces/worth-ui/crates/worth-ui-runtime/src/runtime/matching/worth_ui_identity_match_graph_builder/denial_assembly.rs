use crate::runtime::{
    WorthUiIdentityMatchCounters, WorthUiIdentityMatchDenial, WorthUiIdentityMatchNodeSide,
};

pub(super) fn duplicate_identity_denial<T>(
    side: WorthUiIdentityMatchNodeSide,
    identity_basis: String,
    first_node_summary: String,
    second_node_summary: String,
    counters: &mut WorthUiIdentityMatchCounters,
) -> Result<T, WorthUiIdentityMatchDenial> {
    match side {
        WorthUiIdentityMatchNodeSide::Active => {
            counters.record_duplicate_active_identity();
            Err(WorthUiIdentityMatchDenial::DuplicateActiveIdentity {
                identity_basis,
                first_node_summary,
                second_node_summary,
                counters: Box::new(*counters),
            })
        }
        WorthUiIdentityMatchNodeSide::Candidate => {
            counters.record_duplicate_candidate_identity();
            Err(WorthUiIdentityMatchDenial::DuplicateCandidateIdentity {
                identity_basis,
                first_node_summary,
                second_node_summary,
                counters: Box::new(*counters),
            })
        }
    }
}
