use worth_ui::facade::{
    WorthUiCertifiedMeasurementPacket, WorthUiComplexityContract, WorthUiMeasurementCounterPacket,
};

fn main() {
    let _packet = WorthUiCertifiedMeasurementPacket {
        packet: forged_packet(),
        contract: WorthUiComplexityContract::hot_path("fake"),
    };
}

fn forged_packet() -> WorthUiMeasurementCounterPacket {
    loop {}
}
