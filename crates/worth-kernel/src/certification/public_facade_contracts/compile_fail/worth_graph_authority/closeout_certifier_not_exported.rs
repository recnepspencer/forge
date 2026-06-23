use worth_kernel::query_graph_authority_gate::{
    certify_worth_graph_authority_closeout, current_worth_graph_authority_gate_report,
    WorthGraphAuthorityCloseoutBypassClass,
};

fn main() {
    let gate = current_worth_graph_authority_gate_report().unwrap();
    let _ = certify_worth_graph_authority_closeout(
        &gate,
        WorthGraphAuthorityCloseoutBypassClass::ALL.as_slice(),
        "forged closeout doc",
    );
}
