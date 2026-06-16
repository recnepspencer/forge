use worth_ui::facade::{WorthUiHeaderFramePlan, WorthUiHeaderMenuPlan, WorthUiHeaderThemePlan};

fn main() {
    let _forged = WorthUiHeaderFramePlan {
        menu_plan: forged_menu_plan(),
        theme_plan: forged_theme_plan(),
        frame_digest: 42,
    };
}

fn forged_menu_plan() -> WorthUiHeaderMenuPlan {
    panic!("compile-fail fixture")
}

fn forged_theme_plan() -> WorthUiHeaderThemePlan {
    panic!("compile-fail fixture")
}
