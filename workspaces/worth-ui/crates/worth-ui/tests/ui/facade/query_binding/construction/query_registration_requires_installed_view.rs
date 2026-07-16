use worth_ui::facade::app::WorthUi;
use worth_ui::facade::query_binding::WorthUiQueryViewDefinition;

fn main() {
    let definition = WorthUiQueryViewDefinition::measurement_snapshot("workspace.measurements")
        .expect("valid semantic identity");
    let _ = WorthUi::app().register_query_view(definition);
}
