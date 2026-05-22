use forge_query::facade::ForgeQueryWorkspace;
use worth_kernel::facade::PrimitiveConstructionAuthoringSession;

fn fake_workspace() -> &'static mut ForgeQueryWorkspace {
    panic!("compile-fail fixture should not execute");
}

fn main() {
    let workspace = fake_workspace();
    let _ = PrimitiveConstructionAuthoringSession::new(workspace);
}
