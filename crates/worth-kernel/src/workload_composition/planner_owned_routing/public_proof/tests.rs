use super::publish_from_parts;
use crate::workload_composition::public_closeout::tests::hostile_public_proof_input_with_foreign_reuse_basis;
use crate::workload_composition::WorthTouchedGraphConflictPublicCloseoutErrorKind;

use super::{current_public_closeout_components, current_worth_touched_graph_conflict_public_closeout};

#[test]
fn public_proof_chain_consumes_planner_route_products_only() {
    let closeout = current_worth_touched_graph_conflict_public_closeout()
        .expect("current public proof should assemble from planner-owned route products");
    let selected_route_packet =
        crate::workload_composition::current_worth_touched_graph_conflict_selected_route_packet()
            .expect("current selected-route packet should build");

    assert_eq!(
        closeout.proof_chain().selected_route_packet_digest(),
        selected_route_packet.packet_digest()
    );
}

#[test]
fn public_proof_rejects_foreign_route_or_firewall_identity() {
    let components = current_public_closeout_components().expect("current public closeout components");
    let hostile_public_proof_input =
        hostile_public_proof_input_with_foreign_reuse_basis("foreign-topology-selected-reuse-basis");

    let error = publish_from_parts(
        components.input().expect("current closeout input"),
        components.cutover(),
        components.selected_route_packet(),
        &hostile_public_proof_input,
    )
    .expect_err("public proof assembly must reject foreign planner proof input");

    assert_eq!(
        error.kind(),
        WorthTouchedGraphConflictPublicCloseoutErrorKind::IncompleteProofChain
    );
}
