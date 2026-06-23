use worth_ui::facade::WorthUiDropdownAppearanceFrameReceipt;

fn main() {
    let _forged = WorthUiDropdownAppearanceFrameReceipt {
        menu_min_width: forged_length(),
        row_padding: forged_padding(),
        control_spacing: forged_spacing(),
    };
}

fn forged_length() -> worth_ui::facade::WorthUiLengthValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_padding() -> worth_ui::facade::WorthUiPaddingValue {
    panic!("compile-fail fixture should never execute");
}

fn forged_spacing() -> worth_ui::facade::WorthUiSpacingValue {
    panic!("compile-fail fixture should never execute");
}
