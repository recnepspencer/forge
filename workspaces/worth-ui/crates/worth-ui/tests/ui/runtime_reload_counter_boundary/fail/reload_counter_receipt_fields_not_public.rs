use worth_ui::facade::{
    WorthUiReloadCounterStopStage, WorthUiReloadLoweringCounterReceipt,
    WorthUiMeasurementCounterPacket,
};

fn main() {
    let _receipt = WorthUiReloadLoweringCounterReceipt {
        stopped_at: WorthUiReloadCounterStopStage::CandidateAdmission,
        packets: Vec::<WorthUiMeasurementCounterPacket>::new(),
        carried_query_contract_identities: Vec::new(),
        query_support_rediscovery_count: 0,
    };
}
