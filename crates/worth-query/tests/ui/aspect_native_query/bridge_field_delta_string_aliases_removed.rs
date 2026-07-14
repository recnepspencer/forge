use worth_query::facade::foundation::BridgeFieldDelta;

fn main() {
    let delta = BridgeFieldDelta::new("identity", "id", Some("old"), Some("new"));
    let _ = delta.aspect();
    let _ = delta.field();
}
