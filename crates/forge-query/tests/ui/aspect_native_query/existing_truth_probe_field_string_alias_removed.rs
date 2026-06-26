use forge_query::facade::ForgeQueryExistingTruthProbe;

fn main() {
    let probe = probe_fixture();
    let _ = probe.field("title.value");
}

fn probe_fixture() -> ForgeQueryExistingTruthProbe {
    panic!("fixture only")
}
