use worth_ui::facade::{
    registry::{CommandProjectionDescriptor, CommandProjectionId, CommandProjectionSurface},
};

fn main() {
    let _ = CommandProjectionDescriptor::new(
        CommandProjectionId::new("workspace.projection.palette").unwrap(),
        CommandProjectionSurface::command_palette(),
    )
    .with_label("Save")
    .with_handler("workspace.save");
}
