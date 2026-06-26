use forge_query::facade::{ForgeQueryPortType, ForgeQueryTypedPort};

fn main() {
    let _ = ForgeQueryTypedPort::new("payload", ForgeQueryPortType::TerminalJson);
}
