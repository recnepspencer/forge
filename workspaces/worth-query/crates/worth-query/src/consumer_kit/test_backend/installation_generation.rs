use crate::runtime::WorthQueryWorkspace;

/// Advances the real installed-domain generation inside a Consumer Kit test
/// workspace so downstream runtimes can prove stale-authority rejection.
pub fn advance_test_workspace_domain_installation_generation(workspace: &mut WorthQueryWorkspace) {
    workspace
        .replace_domain_installation_with_successor_generation()
        .expect("test workspace successor generation must reinstall conditional execution");
}
