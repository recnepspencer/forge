use worth_ui::facade::WorthUiApp;

fn launch_twice(app: WorthUiApp) {
    let _first = app.launch();
    let _second = app.launch();
}

fn main() {}
