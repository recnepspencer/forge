use worth_store_aspect_native::StoreTerminalProjectionText;
use worth_store_security::StoreAuthenticityRequirementClass;

fn require_authenticity_class(_: StoreAuthenticityRequirementClass) {}

fn main() {
    let projection_text =
        StoreTerminalProjectionText::new_terminal_projection_text("terminal projection");
    require_authenticity_class(projection_text);
}
