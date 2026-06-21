use worth_kernel::query_adoption::WorthKernelQueryConsumerKitAdoptionStatus;
use worth_kernel::query_graph_authority_gate::WorthGraphAuthorityGateReport;

fn requires_graph_authority(_: WorthGraphAuthorityGateReport) {}

fn promote_support_report(status: WorthKernelQueryConsumerKitAdoptionStatus) {
    requires_graph_authority(status);
}

fn main() {}
