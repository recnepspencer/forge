use worth_store_aspect_native::StoreTerminalProjectionText;
use worth_store_security::StoreCurrentKeyScopeWitness;

fn require_key_scope_witness(_: StoreCurrentKeyScopeWitness) {}

fn main() {
    let projection_text =
        StoreTerminalProjectionText::new_terminal_projection_text("terminal projection");
    require_key_scope_witness(projection_text);
}
