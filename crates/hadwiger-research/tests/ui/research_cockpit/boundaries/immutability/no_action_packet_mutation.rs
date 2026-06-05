use hadwiger_research::facade::ResearchCockpitActionPacket;

fn mutate(packet: &mut ResearchCockpitActionPacket) {
    let _ = packet.actions_mut();
}

fn main() {}
