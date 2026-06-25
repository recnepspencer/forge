use worth_ui::facade::WorthUiAppearanceEnabledPosture;

fn main() {
    let _posture = WorthUiAppearanceEnabledPosture {
        hovered: true,
        pressed: true,
        focused: true,
        selected: false,
    };

    panic!("compile-fail fixture only checks enabled appearance posture field privacy");
}
