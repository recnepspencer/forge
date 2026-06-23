use worth_kernel::query_adoption::WorthQueryNativeHardeningCloseoutReport;
use worth_kernel::query_graph_authority_gate::WorthGraphAuthorityGateReport;

fn requires_graph_obligation_authority(_: WorthGraphAuthorityGateReport) {}

fn promote_adoption_closeout(report: WorthQueryNativeHardeningCloseoutReport) {
    requires_graph_obligation_authority(report);
}

fn main() {}
