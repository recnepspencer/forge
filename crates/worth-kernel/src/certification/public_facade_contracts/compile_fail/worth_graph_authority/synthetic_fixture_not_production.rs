use worth_kernel::query_graph_authority_gate::WorthGraphAuthorityGateReport;

struct SyntheticGraphAuthorityFixture {
    digest: String,
}

fn requires_production_graph_authority(_: WorthGraphAuthorityGateReport) {}

fn main() {
    requires_production_graph_authority(SyntheticGraphAuthorityFixture {
        digest: "synthetic".to_string(),
    });
}
