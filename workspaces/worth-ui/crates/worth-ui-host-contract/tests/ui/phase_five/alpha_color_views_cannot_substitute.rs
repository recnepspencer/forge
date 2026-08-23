use worth_ui_host_contract::{
    UiAlphaRasterRecordView, UiColorRasterRecordView, UiGlyphRasterRecordViewInput,
};

fn require_alpha(_record: UiAlphaRasterRecordView<'_>) {}

fn substitute(input: UiGlyphRasterRecordViewInput<'_>) {
    if let Ok(color) = UiColorRasterRecordView::from_text_mechanics(input) {
        require_alpha(color);
    }
}

fn main() {
    let _ = substitute;
}
