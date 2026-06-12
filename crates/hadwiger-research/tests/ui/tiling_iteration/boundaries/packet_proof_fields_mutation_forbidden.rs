use hadwiger_research::facade::TilingIterationPacket;

fn forbidden(packet: &mut TilingIterationPacket) {
    packet.evidence_basis_mut().clear();
    packet.reactivation_obligations_mut().clear();
}

fn main() {}
