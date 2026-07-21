use worth_ui::facade::{
    WorthUiExecutionLane, WorthUiLaneAdapterHook, WorthUiLaneAdapterHookKind,
};

fn main() {
    let _hook = WorthUiLaneAdapterHook {
        hook_id: "canvas.draw".to_owned(),
        lane: WorthUiExecutionLane::CanvasSpatial,
        kind: WorthUiLaneAdapterHookKind::ComponentLowering,
    };
}
