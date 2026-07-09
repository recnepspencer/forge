use worth_query::facade::WorthQueryExistingTruthProbe;

fn main() {
    let probe = probe_fixture();
    let _ = probe.field("title.value");
}

fn probe_fixture() -> WorthQueryExistingTruthProbe {
    panic!("fixture only")
}
