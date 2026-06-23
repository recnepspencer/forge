use forge_query::facade::ForgeQueryBackendAdmissibleMutation;

#[allow(unreachable_code)]
fn main() {
    let _ = ForgeQueryBackendAdmissibleMutation {
        shape: shape_fixture(),
    };
}

fn shape_fixture() -> ! {
    panic!("fixture only")
}
