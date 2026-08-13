use std::{collections::BTreeSet, sync::Arc};

use super::installation::{
    WorthQueryConditionalRuntimeInstallationDenial,
    WorthQueryConditionalRuntimeInstallationDenialKind, WorthQueryPendingConditionalOperation,
};
use super::lifecycle::WorthQueryConditionalOperationRegistry;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphInstallationDenial;

pub(in crate::domain_computation::primary_graph) struct ConditionalRuntimeAffinity<'a> {
    runtime_authority: u64,
    installation_runtime: u64,
    installation_generation: u64,
    provider_identity: &'a str,
    branch_identity: &'a str,
}

impl ConditionalRuntimeAffinity<'_> {
    pub(super) fn bind(&self, identity: &str) -> Arc<str> {
        Arc::from(format!(
            "{identity}:runtime={}:installation={}:generation={}:provider={}:branch={}",
            self.runtime_authority,
            self.installation_runtime,
            self.installation_generation,
            self.provider_identity,
            self.branch_identity,
        ))
    }
}

pub(in crate::domain_computation::primary_graph) fn require_complete_binding_inventory<Schema>(
    expected: usize,
    bindings: &[Box<dyn WorthQueryPendingConditionalOperation<Schema>>],
) -> Result<(), WorthQueryConditionalRuntimeInstallationDenial> {
    let unique = bindings
        .iter()
        .map(|binding| binding.binding_identity())
        .collect::<BTreeSet<_>>();
    if bindings.len() == expected && unique.len() == expected {
        Ok(())
    } else {
        Err(WorthQueryConditionalRuntimeInstallationDenial::new(
            WorthQueryConditionalRuntimeInstallationDenialKind::IncompleteBindingInventory,
            format!(
                "expected {expected} exact conditional bindings, admitted {}",
                unique.len()
            ),
        ))
    }
}

#[allow(clippy::too_many_arguments)]
pub(in crate::domain_computation::primary_graph) fn install_pending_bindings<Schema>(
    bindings: Vec<Box<dyn WorthQueryPendingConditionalOperation<Schema>>>,
    bridge: &mut super::super::managed_bridge::WorthQueryInstalledApplicationBridge,
    runtime_authority: u64,
    installation_runtime: u64,
    installation_generation: u64,
    provider_identity: &str,
    branch_identity: &str,
) -> Result<
    WorthQueryConditionalOperationRegistry<Schema>,
    WorthQueryConditionalRuntimeInstallationDenial,
> {
    let affinity = ConditionalRuntimeAffinity {
        runtime_authority,
        installation_runtime,
        installation_generation,
        provider_identity,
        branch_identity,
    };
    let mut registry = WorthQueryConditionalOperationRegistry::default();
    for binding in bindings {
        let installed = binding.install(bridge.conditional_mut(), &affinity)?;
        registry.install(installed).map_err(|()| {
            WorthQueryConditionalRuntimeInstallationDenial::new(
                WorthQueryConditionalRuntimeInstallationDenialKind::DuplicateBinding,
                "duplicate installed conditional binding",
            )
        })?;
    }
    Ok(registry)
}

pub(in crate::domain_computation::primary_graph) fn publication_denial(
    denial: WorthQueryPrimaryGraphInstallationDenial,
) -> WorthQueryConditionalRuntimeInstallationDenial {
    WorthQueryConditionalRuntimeInstallationDenial::new(
        WorthQueryConditionalRuntimeInstallationDenialKind::PrimaryGraphPublication,
        denial.to_string(),
    )
}
