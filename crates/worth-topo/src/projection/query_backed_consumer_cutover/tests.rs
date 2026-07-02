use super::{
    current_topology_query_backed_consumer_cutover,
    current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides,
    TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError,
};

#[test]
fn legacy_current_cutover_entrypoint_is_a_direct_planner_reexport() {
    let legacy: fn() -> Result<
        TopologyQueryBackedConsumerCutover,
        TopologyQueryBackedConsumerCutoverCurrentError,
    > = current_topology_query_backed_consumer_cutover;
    let planner: fn() -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError> =
        crate::projection::planner_owned_routing::query_backed_read_family::current_topology_query_backed_consumer_cutover;

    assert_eq!(legacy as usize, planner as usize);
}

#[test]
fn legacy_hostile_basis_override_entrypoint_is_a_direct_planner_reexport() {
    let legacy: fn(
        Option<&str>,
        Option<&str>,
    ) -> Result<
        TopologyQueryBackedConsumerCutover,
        TopologyQueryBackedConsumerCutoverCurrentError,
    > = current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;
    let planner: fn(
        Option<&str>,
        Option<&str>,
    ) -> Result<TopologyQueryBackedConsumerCutover, TopologyQueryBackedConsumerCutoverCurrentError> =
        crate::projection::planner_owned_routing::query_backed_read_family::current_topology_query_backed_consumer_cutover_with_hostile_selected_basis_overrides;

    assert_eq!(legacy as usize, planner as usize);
}
