use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessShape, ForgeQueryGraphReadRootPosture, ForgeQueryReadScopeClass,
};

fn main() {
    let _ = ForgeQueryGraphReadAccessShape {
        root_posture: ForgeQueryGraphReadRootPosture::Local,
        scope_class: ForgeQueryReadScopeClass::LocalNeighborhood,
    };
}
