use forge_query::facade::{
    ForgeQueryGraphCompositionCapabilityClass, ForgeQueryGraphCompositionCapabilitySupportRow,
};

fn main() {
    let _ = ForgeQueryGraphCompositionCapabilitySupportRow {
        capability_family: String::new(),
        capability_class: ForgeQueryGraphCompositionCapabilityClass::LifecycleStep,
        row_digest: String::new(),
    };
}
