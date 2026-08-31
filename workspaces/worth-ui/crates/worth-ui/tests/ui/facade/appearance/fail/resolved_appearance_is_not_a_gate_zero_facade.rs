use worth_ui::facade::appearance::UiAppearanceProjection;

// Gate 0 freezes the absence of the appearance facade module. The unresolved
// module proves that boundary, not the existence of this hypothetical type.
fn main() {
    let _default = UiAppearanceProjection::default();
}
