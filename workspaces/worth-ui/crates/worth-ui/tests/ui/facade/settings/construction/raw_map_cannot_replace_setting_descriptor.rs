use std::collections::HashMap;

use worth_ui::facade::app::WorthUi;

fn main() {
    let raw_settings = HashMap::<String, String>::from([(
        "workspace.setting.wrap_lines".to_string(),
        "true".to_string(),
    )]);

    let _app = WorthUi::app().register_setting(raw_settings).freeze().expect("application preparation should succeed");
}
