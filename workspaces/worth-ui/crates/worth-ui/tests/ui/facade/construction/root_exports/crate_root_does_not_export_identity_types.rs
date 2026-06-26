use worth_ui::{CommandId, ComponentId, SurfaceId};

fn main() {
    let _ = core::mem::size_of::<CommandId>();
    let _ = core::mem::size_of::<ComponentId>();
    let _ = core::mem::size_of::<SurfaceId>();
}
