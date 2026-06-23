use worth_ui::facade::{
    WorthUiQueryEffectPostureReceipt, WorthUiQueryProjectionFactReceipt,
    WorthUiQueryRuntimeFactLoweringCounters, WorthUiQueryRuntimeFactLoweringInput,
    WorthUiQueryRuntimeFactLoweringReceipt, WorthUiQueryRuntimeFactLoweringStatus,
    WorthUiQueryStateSnapshotReceipt, WorthUiQuerySupportDenialKind,
    WorthUiQuerySupportDenialReceipt,
};

fn main() {
    let _: Option<WorthUiQueryEffectPostureReceipt> = None;
    let _: Option<WorthUiQueryProjectionFactReceipt> = None;
    let _: Option<WorthUiQueryRuntimeFactLoweringCounters> = None;
    let _: Option<WorthUiQueryRuntimeFactLoweringInput> = None;
    let _: Option<WorthUiQueryRuntimeFactLoweringReceipt> = None;
    let _: Option<WorthUiQueryStateSnapshotReceipt> = None;
    let _: Option<WorthUiQuerySupportDenialReceipt> = None;
    let _ = WorthUiQueryRuntimeFactLoweringStatus::AdmittedChanged;
    let _ = WorthUiQuerySupportDenialKind::LiveRebindDenied;
}
