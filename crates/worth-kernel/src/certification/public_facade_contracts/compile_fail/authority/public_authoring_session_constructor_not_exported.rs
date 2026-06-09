use forge_query::facade::ForgeQueryWorkspace;
use worth_kernel::facade::authoring::construction::PrimitiveConstructionAuthoringSession;

fn main() {
    let _: Option<PrimitiveConstructionAuthoringSession<'static>> = None;
    let _ = std::any::type_name::<ForgeQueryWorkspace>();
}
