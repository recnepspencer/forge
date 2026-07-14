use worth_query::facade::runtime::WorthQueryBackendAdmissibleMutation;

#[allow(unreachable_code)]
fn main() {
    let _ = WorthQueryBackendAdmissibleMutation {
        shape: shape_fixture(),
    };
}

fn shape_fixture() -> ! {
    panic!("fixture only")
}
