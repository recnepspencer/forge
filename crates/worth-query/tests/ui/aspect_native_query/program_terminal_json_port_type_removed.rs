use worth_query::facade::policy::{WorthQueryPortType, WorthQueryTypedPort};

fn main() {
    let _ = WorthQueryTypedPort::new("payload", WorthQueryPortType::TerminalJson);
}
