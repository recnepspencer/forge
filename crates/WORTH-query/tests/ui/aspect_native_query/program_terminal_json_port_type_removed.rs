use worth_query::facade::{WorthQueryPortType, WorthQueryTypedPort};

fn main() {
    let _ = WorthQueryTypedPort::new("payload", WorthQueryPortType::TerminalJson);
}
