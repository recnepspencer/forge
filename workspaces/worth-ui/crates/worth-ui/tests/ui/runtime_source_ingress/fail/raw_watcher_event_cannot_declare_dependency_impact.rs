use worth_ui::facade::WorthUiWatcherEvent;

fn main() {
    let _impact = WorthUiWatcherEvent::modified("app/main.wui").dependency_impact();
}
