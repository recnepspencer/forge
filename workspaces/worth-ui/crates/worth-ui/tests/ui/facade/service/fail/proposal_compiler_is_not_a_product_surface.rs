//! The proposal compiler coordinates runtime-service owners. It is not an
//! application construction surface, so product code cannot name it, publish
//! through it, or hold family state on it.

fn main() {
    let _ = worth_ui::facade::service::UiServiceProposalCompiler::new();
}
