use worth_ui::facade::inspection::{
    UiHitTestRegionIndexIdentity, UiVisibleRegionIndexIdentity,
};

fn accepts_visible(_: UiVisibleRegionIndexIdentity) {}

fn substitution_is_rejected(hit_test: UiHitTestRegionIndexIdentity) {
    accepts_visible(hit_test);
}

fn main() {
    let _ = substitution_is_rejected;
}
