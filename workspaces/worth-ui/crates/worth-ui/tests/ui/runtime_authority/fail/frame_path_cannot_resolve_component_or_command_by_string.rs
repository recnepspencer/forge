use worth_ui::facade::WorthUiRuntime;

fn main() {
    fn frame_path(host: &WorthUiRuntime) {
        let _component = host.resolve_component_handle("workspace.component.dashboard");
        let _command = host.resolve_command_handle("workspace.command.save");
    }
}
