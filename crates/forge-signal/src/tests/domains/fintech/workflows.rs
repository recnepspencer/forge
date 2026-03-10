use super::fixture::FintechDomainFixture;
use super::invariants::assert_fixture_shape;
use super::scales::FintechScale;
use super::scenarios::intraday_pricing_and_risk;

fn build_intraday_fixture(scale: FintechScale) -> FintechDomainFixture {
    let fixture = intraday_pricing_and_risk(scale);
    assert_fixture_shape(&fixture, scale);
    fixture
}

#[test]
fn fintech_intraday_fixture_smoke_builds_branchable_graph() {
    let mut fixture = build_intraday_fixture(FintechScale::smoke());
    let baseline_nodes = fixture.live_node_count();
    let analysis = fixture.runtime.create_branch("analysis").unwrap();
    fixture.runtime.switch_branch(analysis).unwrap();
    let snapshot = fixture.runtime.capture_snapshot();

    assert!(baseline_nodes > 0);
    assert_eq!(snapshot.graph.live_node_ids().len(), baseline_nodes);
    assert_eq!(fixture.instruments.len(), FintechScale::smoke().instruments);
}

#[test]
#[ignore = "stress shape for hostile domain workflows"]
fn fintech_intraday_fixture_stress_builds_over_10k_nodes() {
    let fixture = build_intraday_fixture(FintechScale::stress_10k());
    assert!(
        fixture.live_node_count() >= 10_000,
        "stress fixture should exceed 10k nodes"
    );
}
