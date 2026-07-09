use worth_query::facade::{
    WorthQueryGraphCompositionCapabilityClass, WorthQueryGraphCompositionCapabilitySupportRow,
};

fn main() {
    let _ = WorthQueryGraphCompositionCapabilitySupportRow {
        capability_family: String::new(),
        capability_class: WorthQueryGraphCompositionCapabilityClass::LifecycleStep,
        row_digest: String::new(),
    };
}
