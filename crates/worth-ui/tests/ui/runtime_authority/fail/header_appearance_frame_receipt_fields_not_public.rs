use worth_ui::facade::WorthUiHeaderAppearanceFrameReceipt;

fn main() {
    let _forged = WorthUiHeaderAppearanceFrameReceipt {
        font_size: forged_font_size(),
        menu_min_width: forged_length(),
        border_width: forged_border_width(),
        panel_shadow: forged_shadow(),
        row_padding: forged_padding(),
        container_padding: forged_padding(),
        control_spacing: forged_spacing(),
    };
}

fn forged_font_size() -> worth_ui::facade::WorthUiFontSizeValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_length() -> worth_ui::facade::WorthUiLengthValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_border_width() -> worth_ui::facade::WorthUiBorderWidthValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_shadow() -> worth_ui::facade::WorthUiShadowValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_padding() -> worth_ui::facade::WorthUiPaddingValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_spacing() -> worth_ui::facade::WorthUiSpacingValue {
    panic!("compile-fail fixture should never execute");
}
