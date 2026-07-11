use forge_store_aspect_native::StoreTerminalProjectionDisplayLabel;
use forge_store_security::StoreAuthenticityRequirementClass;

fn require_authenticity_class(_: StoreAuthenticityRequirementClass) {}

fn main() {
    let label = StoreTerminalProjectionDisplayLabel::new("terminal key scope").unwrap();
    require_authenticity_class(label);
}
