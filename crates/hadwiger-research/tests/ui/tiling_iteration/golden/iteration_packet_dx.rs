use hadwiger_research::facade::{
    derive_tiling_iteration_packet_checked, replay_tiling_iteration_packet_checked,
    TilingIterationPacketRequest,
};

fn main() {
    let _request = TilingIterationPacketRequest::lower_bound_obstruction("packet-a")
        .with_evidence_basis("edge-local retained rejection")
        .with_required_checker_lane("exact_tile_contact")
        .with_reactivation_obligation("provide repaired exact coordinates")
        .with_expected_information_gain("extract reusable motif")
        .unwrap();

    let _derive = derive_tiling_iteration_packet_checked;
    let _replay = replay_tiling_iteration_packet_checked;
}
