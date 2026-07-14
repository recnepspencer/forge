use worth_ui_dsl::{UiDslLoweringReceipt, UiDslSemanticArtifact, UiDslSourceProvenance};

fn main() {
    let _ = UiDslLoweringReceipt::new(
        unsafe { std::mem::MaybeUninit::<UiDslSemanticArtifact>::zeroed().assume_init() },
        17,
        UiDslSourceProvenance::file_authored("app/Worthd.wui", 0),
    );
}
