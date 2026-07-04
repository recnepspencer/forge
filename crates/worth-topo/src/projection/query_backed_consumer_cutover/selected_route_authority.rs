#[cfg(any(test, feature = "test-support-lowering"))]
use super::{
    require_optional_match, require_string_match, TopologyQueryBackedReadFamilyAdmissionError,
    TopologyQueryBackedReadFamilyRouteInput,
};

#[cfg(any(test, feature = "test-support-lowering"))]
pub trait TopologyQueryBackedReadFamilySelectedRouteAuthority {
    fn topology_query_handle_identity_digest(&self) -> &str;
    fn topology_query_support_snapshot_digest(&self) -> &str;
    fn topology_query_operating_context_identity_digest(&self) -> &str;
    fn topology_query_parity_verified_count(&self) -> usize;
    fn topology_query_compiled_product_identity_digest(&self) -> Option<&str>;
    fn topology_query_equivalence_policy_identity_digest(&self) -> Option<&str>;
    fn topology_query_selected_equivalence_family_identity(&self) -> Option<&str>;
    fn topology_query_selected_equivalence_basis_identity_digest(&self) -> Option<&str>;
    fn topology_query_selected_compatibility_basis_identity_digest(&self) -> Option<&str>;
    fn topology_query_selected_reuse_basis_identity_digest(&self) -> Option<&str>;
    fn topology_query_reuse_decision_identity_digest(&self) -> Option<&str>;
    fn topology_query_rebuild_denial_identity_digest(&self) -> Option<&str>;
}

#[cfg(any(test, feature = "test-support-lowering"))]
pub(crate) fn require_selected_route_authority_matches<
    A: TopologyQueryBackedReadFamilySelectedRouteAuthority,
>(
    route_input: &TopologyQueryBackedReadFamilyRouteInput<'_>,
    authority: &A,
) -> Result<(), TopologyQueryBackedReadFamilyAdmissionError> {
    require_string_match(
        "query handle identity",
        route_input.handle_identity_digest(),
        authority.topology_query_handle_identity_digest(),
    )?;
    require_string_match(
        "query support snapshot",
        route_input.support_snapshot_digest(),
        authority.topology_query_support_snapshot_digest(),
    )?;
    require_string_match(
        "query operating context",
        route_input.operating_context_identity_digest(),
        authority.topology_query_operating_context_identity_digest(),
    )?;
    require_optional_match(
        "compiled product identity",
        route_input
            .equivalence_contract()
            .compiled_product_identity_digest(),
        authority.topology_query_compiled_product_identity_digest(),
    )?;
    require_optional_match(
        "equivalence policy identity",
        route_input
            .equivalence_contract()
            .equivalence_policy_identity_digest(),
        authority.topology_query_equivalence_policy_identity_digest(),
    )?;
    require_optional_match(
        "selected equivalence family identity",
        route_input
            .equivalence_contract()
            .selected_equivalence_family_identity()
            .map(|identity| identity.as_str()),
        authority.topology_query_selected_equivalence_family_identity(),
    )?;
    require_optional_match(
        "selected equivalence basis identity",
        route_input
            .equivalence_contract()
            .selected_equivalence_basis_identity_digest(),
        authority.topology_query_selected_equivalence_basis_identity_digest(),
    )?;
    require_optional_match(
        "selected compatibility basis identity",
        route_input
            .equivalence_contract()
            .selected_compatibility_basis_identity_digest(),
        authority.topology_query_selected_compatibility_basis_identity_digest(),
    )?;
    require_optional_match(
        "selected reuse basis identity",
        route_input
            .equivalence_contract()
            .selected_reuse_basis_identity_digest(),
        authority.topology_query_selected_reuse_basis_identity_digest(),
    )?;
    require_optional_match(
        "reuse decision identity",
        route_input
            .equivalence_contract()
            .reuse_decision_identity_ref()
            .map(|identity| identity.identity_digest()),
        authority.topology_query_reuse_decision_identity_digest(),
    )?;
    require_optional_match(
        "rebuild denial identity",
        None,
        authority.topology_query_rebuild_denial_identity_digest(),
    )?;
    if route_input.parity_verified_count() != authority.topology_query_parity_verified_count() {
        return Err(TopologyQueryBackedReadFamilyAdmissionError::new(format!(
            "query-backed route admission rejected mismatched parity verification count: expected {}, observed {}",
            authority.topology_query_parity_verified_count(),
            route_input.parity_verified_count()
        )));
    }
    Ok(())
}
