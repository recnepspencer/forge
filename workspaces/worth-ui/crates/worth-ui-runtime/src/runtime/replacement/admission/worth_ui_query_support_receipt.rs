use crate::runtime::replacement::candidate::WorthUiCandidateDependencyMetadata;
use crate::source::WorthUiRuntimeDependencyHook;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthUiQuerySupportStatus {
    Supported,
    Deferred,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQuerySupportReceipt {
    status: WorthUiQuerySupportStatus,
    runtime_hook_count: usize,
    contract_identity: worth_ui_query_binding::WorthUiQueryBindingContractIdentity,
}

impl WorthUiQuerySupportReceipt {
    pub(crate) fn from_dependency_metadata(
        dependency_metadata: &WorthUiCandidateDependencyMetadata,
    ) -> Self {
        let graph = dependency_metadata
            .dependency_report()
            .basis()
            .dependency_graph();
        let mut hook_count = 0usize;
        let mut definitions = Vec::new();
        let mut status = WorthUiQuerySupportStatus::Supported;
        for hooks in graph.runtime_hooks().values() {
            for hook in hooks {
                hook_count += 1;
                status = status.combine(support_status_for_runtime_hook(hook));
                definitions.push(hook.definition().digest());
            }
        }
        Self {
            status,
            runtime_hook_count: hook_count,
            contract_identity:
                worth_ui_query_binding::WorthUiQueryBindingContractIdentity::from_definitions(
                    definitions,
                ),
        }
    }

    pub fn status(self) -> WorthUiQuerySupportStatus {
        self.status
    }

    pub fn runtime_hook_count(self) -> usize {
        self.runtime_hook_count
    }

    pub fn contract_identity(self) -> worth_ui_query_binding::WorthUiQueryBindingContractIdentity {
        self.contract_identity
    }

    #[cfg(test)]
    pub(crate) fn for_test(status: WorthUiQuerySupportStatus, contract_label: &str) -> Self {
        Self::with_runtime_hook_count_for_test(status, 1, contract_label)
    }

    #[cfg(test)]
    pub(crate) fn with_runtime_hook_count_for_test(
        status: WorthUiQuerySupportStatus,
        runtime_hook_count: usize,
        contract_label: &str,
    ) -> Self {
        let definition = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_snapshot(
            contract_label,
        )
        .expect("test Query contract label must be valid");
        Self {
            status,
            runtime_hook_count,
            contract_identity:
                worth_ui_query_binding::WorthUiQueryBindingContractIdentity::from_definitions([
                    definition.digest(),
                ]),
        }
    }
}

impl WorthUiQuerySupportStatus {
    fn combine(self, next: Self) -> Self {
        match (self, next) {
            (Self::Unsupported, _) | (_, Self::Unsupported) => Self::Unsupported,
            (Self::Deferred, _) | (_, Self::Deferred) => Self::Deferred,
            (Self::Supported, Self::Supported) => Self::Supported,
        }
    }
}

fn support_status_for_runtime_hook(
    _hook: &WorthUiRuntimeDependencyHook,
) -> WorthUiQuerySupportStatus {
    WorthUiQuerySupportStatus::Supported
}

#[cfg(test)]
mod tests {
    use crate::capability::{QueryDenialPresentation, ViewBindingId};
    use crate::runtime::replacement::admission::worth_ui_query_support_receipt::support_status_for_runtime_hook;
    use crate::runtime::replacement::admission::WorthUiQuerySupportStatus;
    use crate::source::{WorthUiRuntimeDependencyHook, WorthUiRuntimeDependencyHookKind};

    #[test]
    fn admitted_definition_makes_runtime_hook_supported() {
        let definition = worth_ui_query_binding::WorthUiQueryViewDefinition::measurement_live(
            "workspace.view_binding.selection",
        )
        .expect("definition should admit");
        let hook = WorthUiRuntimeDependencyHook::new(
            WorthUiRuntimeDependencyHookKind::LiveView,
            ViewBindingId::new("workspace.view_binding.selection").unwrap(),
            definition,
            QueryDenialPresentation::structured_status(),
        );
        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Supported
        );
    }
}
