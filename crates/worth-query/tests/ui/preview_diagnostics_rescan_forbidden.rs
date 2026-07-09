use worth_query::facade::{PreviewEvaluationClass, PreviewSessionQueryContext};
use worth_runtime_bridge::facade::{
    BridgePreviewReplayBundle, BridgePreviewSession, PreviewActive,
};

fn main() {
    let session: BridgePreviewSession<PreviewActive> = todo!();
    let execution_record = todo!();
    let replay_bundle: BridgePreviewReplayBundle = todo!();

    let _ = PreviewSessionQueryContext::active(
        &session,
        &execution_record,
        PreviewEvaluationClass::read_only(),
    )
    .with_replay_bundle(&replay_bundle);
}
