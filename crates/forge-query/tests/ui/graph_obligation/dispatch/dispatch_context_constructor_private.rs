use forge_query::facade::runtime::{
    ForgeQueryGraphObligationDispatchContext, ForgeQueryGraphObligationDispatchContextKind,
};

fn main() {
    let _ = ForgeQueryGraphObligationDispatchContext {
        kind: ForgeQueryGraphObligationDispatchContextKind::GraphComposition,
        touch_descriptor_digest: "touch.digest".to_string(),
        operating_world_digest: "world.digest".to_string(),
    };
}
