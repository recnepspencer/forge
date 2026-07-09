use worth_query::facade::{PreviewEvaluationClass, PreviewSessionQueryContext};
use worth_runtime_bridge::facade::{
    BridgePreviewPromotionRecord, BridgePreviewReplayBundle, BridgePreviewSession, PreviewActive,
};

fn main() {
    let session: BridgePreviewSession<PreviewActive> = todo!();
    let execution_record = todo!();
    let replay_bundle: BridgePreviewReplayBundle = todo!();
    let promotion_record: BridgePreviewPromotionRecord = todo!();

    let _ = PreviewSessionQueryContext::active(
        &session,
        &execution_record,
        PreviewEvaluationClass::read_only(),
    )
    .with_replay_bundle(&replay_bundle)
    .with_promotion_record(&promotion_record);
}
