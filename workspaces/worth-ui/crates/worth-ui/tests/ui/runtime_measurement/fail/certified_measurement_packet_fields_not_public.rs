use worth_ui::facade::{
    WorthUiCertifiedMeasurementPacket, WorthUiComplexityContract, WorthUiMeasurementCounterPacket,
};

fn main() {
    let _packet = WorthUiCertifiedMeasurementPacket {
        packet: worthd_packet(),
        contract: WorthUiComplexityContract::hot_path("fake"),
    };
}

fn worthd_packet() -> WorthUiMeasurementCounterPacket {
    loop {}
}
