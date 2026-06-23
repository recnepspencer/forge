use worth_ui::facade::{
    WorthUiAdmittedHotReloadableSemanticSliceSet, WorthUiHotReloadableSemanticSlice,
    WorthUiSemanticSliceDescriptor,
};

fn main() {
    let _slice = WorthUiHotReloadableSemanticSlice {
        descriptor: impossible_descriptor(),
    };
    let _set = WorthUiAdmittedHotReloadableSemanticSliceSet {
        slices: Vec::new(),
    };
}

fn impossible_descriptor() -> &'static WorthUiSemanticSliceDescriptor {
    panic!("fixture only")
}
