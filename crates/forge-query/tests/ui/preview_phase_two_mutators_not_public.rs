use forge_query::facade::{
    PreviewBindingIntent, PreviewEvaluationClass, PreviewSessionQueryContext,
};
use forge_runtime_bridge::facade::{
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
    .with_binding_intent(PreviewBindingIntent::preview_with_live_lane())
    .with_replay_bundle(&replay_bundle)
    .with_promotion_record(&promotion_record);
}
