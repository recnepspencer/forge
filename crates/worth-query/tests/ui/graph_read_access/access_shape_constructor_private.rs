use worth_query::facade::runtime::{WorthQueryGraphReadAccessShape, WorthQueryGraphReadRootPosture, WorthQueryReadScopeClass};

fn main() {
    let _ = WorthQueryGraphReadAccessShape {
        root_posture: WorthQueryGraphReadRootPosture::Local,
        scope_class: WorthQueryReadScopeClass::LocalNeighborhood,
    };
}
