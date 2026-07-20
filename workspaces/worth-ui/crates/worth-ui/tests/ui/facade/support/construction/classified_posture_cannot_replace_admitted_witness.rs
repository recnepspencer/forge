use worth_ui::facade::{
    registry::CommandId,
    support::{AdmittedCapability, CapabilitySupportPosture},
};

fn requires_admitted(_capability: AdmittedCapability<CommandId>) {}

fn main() {
    let id = CommandId::new("app.command.save").expect("valid command id");
    let posture = CapabilitySupportPosture::admitted(id);

    requires_admitted(posture);
}
