use worth_query::facade::runtime::{WorthQueryGraphObligationDispatchContext, WorthQueryGraphObligationDispatchContextKind};

fn main() {
    let _ = WorthQueryGraphObligationDispatchContext {
        kind: WorthQueryGraphObligationDispatchContextKind::GraphComposition,
        touch_descriptor_digest: "touch.digest".to_string(),
        operating_world_digest: "world.digest".to_string(),
    };
}
