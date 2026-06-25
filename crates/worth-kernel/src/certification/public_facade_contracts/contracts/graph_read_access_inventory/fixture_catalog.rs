mod adoption_fixtures;
mod declaration_fixtures;
mod inventory_fixtures;

pub(super) fn graph_read_access_inventory_expected_compile_fail_fixtures(
) -> Vec<(&'static str, &'static str)> {
    let mut fixtures = Vec::new();
    fixtures.extend_from_slice(inventory_fixtures::GRAPH_READ_ACCESS_INVENTORY_FIXTURES);
    fixtures.extend_from_slice(declaration_fixtures::GRAPH_READ_ACCESS_DECLARATION_FIXTURES);
    fixtures.extend_from_slice(adoption_fixtures::GRAPH_READ_ACCESS_PLAN_ADOPTION_FIXTURES);
    fixtures
}
