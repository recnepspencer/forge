use forge_query::facade::{
    BasisSupportPosture, ForgeQueryCapabilityStatus, QuerySubscriptionSupportPosture,
};

use crate::runtime::candidate::WorthUiCandidateDependencyMetadata;
use crate::source::WorthUiRuntimeDependencyHook;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiQuerySupportStatus {
    Supported,
    Deferred,
    Unsupported,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiQuerySupportReceipt {
    status: WorthUiQuerySupportStatus,
    runtime_hook_count: usize,
    receipt_digest: u64,
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
        let mut hook_digest = 0x7175_6572_795f_0003u64;
        let mut status = WorthUiQuerySupportStatus::Supported;
        for (handle, hooks) in graph.runtime_hooks() {
            hook_digest ^= handle.node_index() as u64;
            hook_digest = hook_digest.rotate_left(5);
            for hook in hooks {
                hook_count += 1;
                status = status.combine(support_status_for_runtime_hook(hook));
                hook_digest ^= fold_text(&hook.digest_basis());
                hook_digest = hook_digest.rotate_left(11);
            }
        }
        Self {
            status,
            runtime_hook_count: hook_count,
            receipt_digest: hook_digest ^ (hook_count as u64).rotate_left(17),
        }
    }

    pub fn status(self) -> WorthUiQuerySupportStatus {
        self.status
    }

    pub fn runtime_hook_count(self) -> usize {
        self.runtime_hook_count
    }

    pub fn receipt_digest(self) -> u64 {
        self.receipt_digest
    }

    #[cfg(test)]
    pub(crate) fn for_test(status: WorthUiQuerySupportStatus, receipt_digest: u64) -> Self {
        Self::with_runtime_hook_count_for_test(status, 1, receipt_digest)
    }

    #[cfg(test)]
    pub(crate) fn with_runtime_hook_count_for_test(
        status: WorthUiQuerySupportStatus,
        runtime_hook_count: usize,
        receipt_digest: u64,
    ) -> Self {
        Self {
            status,
            runtime_hook_count,
            receipt_digest,
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
    hook: &WorthUiRuntimeDependencyHook,
) -> WorthUiQuerySupportStatus {
    capability_support_status(hook)
        .combine(basis_support_status(hook))
        .combine(live_support_status(hook))
}

fn capability_support_status(hook: &WorthUiRuntimeDependencyHook) -> WorthUiQuerySupportStatus {
    match hook.query_capability().status() {
        ForgeQueryCapabilityStatus::Admitted => WorthUiQuerySupportStatus::Supported,
        ForgeQueryCapabilityStatus::DeferredDebt => WorthUiQuerySupportStatus::Deferred,
        ForgeQueryCapabilityStatus::Unsupported => WorthUiQuerySupportStatus::Unsupported,
    }
}

fn basis_support_status(hook: &WorthUiRuntimeDependencyHook) -> WorthUiQuerySupportStatus {
    match hook.basis_posture().posture() {
        BasisSupportPosture::Admitted | BasisSupportPosture::Advisory => {
            WorthUiQuerySupportStatus::Supported
        }
        BasisSupportPosture::Deferred => WorthUiQuerySupportStatus::Deferred,
        BasisSupportPosture::Denied | BasisSupportPosture::Unsupported => {
            WorthUiQuerySupportStatus::Unsupported
        }
    }
}

fn live_support_status(hook: &WorthUiRuntimeDependencyHook) -> WorthUiQuerySupportStatus {
    match hook.live_compatibility().posture() {
        QuerySubscriptionSupportPosture::RuntimeBackedCertified => {
            WorthUiQuerySupportStatus::Supported
        }
        QuerySubscriptionSupportPosture::RuntimeBackedDeferred => {
            WorthUiQuerySupportStatus::Deferred
        }
        QuerySubscriptionSupportPosture::RuntimeBackedDenied
        | QuerySubscriptionSupportPosture::UncertifiedDenied => {
            WorthUiQuerySupportStatus::Unsupported
        }
    }
}

fn fold_text(text: &str) -> u64 {
    let mut digest = 0xcbf2_9ce4_8422_2325u64;
    for byte in text.as_bytes() {
        digest ^= u64::from(*byte);
        digest = digest.wrapping_mul(0x100_0000_01b3);
    }
    digest
}

#[cfg(test)]
mod tests {
    use forge_query::facade::{
        discover_basis_lifecycle_support, BasisFamily, ForgeQueryApplicationFacade,
        ForgeQueryCapabilityFamily, ForgeQueryConfig, ForgeQueryQueryConfig,
        ForgeQueryRelationalConfig, ForgeQueryRuntimeBridgeConfig, ForgeQuerySignalConfig,
        QuerySubscriptionFamily, QuerySubscriptionSupportPosture, ResultShapeFamily,
        ViewShapeDescriptor,
    };

    use crate::capability::{
        QueryBasisPostureReference, QueryDenialPresentation, QueryLiveCompatibility,
        QueryResultShapeReference, QueryViewCapabilityReference, ViewBindingId,
    };
    use crate::runtime::admission::worth_ui_query_support_receipt::support_status_for_runtime_hook;
    use crate::runtime::admission::WorthUiQuerySupportStatus;
    use crate::source::{WorthUiRuntimeDependencyHook, WorthUiRuntimeDependencyHookKind};

    #[test]
    fn runtime_hook_support_status_preserves_query_deferred_live_posture() {
        let hook = runtime_hook_with_query_postures(
            runtime_backed_query_capability(),
            BasisFamily::CurrentHead,
            "subscription_declaration",
            QuerySubscriptionSupportPosture::RuntimeBackedDeferred,
        );

        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Deferred
        );
    }

    #[test]
    fn runtime_hook_support_status_preserves_query_denied_live_posture() {
        let hook = runtime_hook_with_query_postures(
            runtime_backed_query_capability(),
            BasisFamily::CurrentHead,
            "subscription_declaration",
            QuerySubscriptionSupportPosture::UncertifiedDenied,
        );

        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Unsupported
        );
    }

    #[test]
    fn runtime_hook_support_status_preserves_disabled_query_capability() {
        let hook = runtime_hook_with_query_postures(
            disabled_query_capability(),
            BasisFamily::CurrentHead,
            "subscription_declaration",
            QuerySubscriptionSupportPosture::RuntimeBackedCertified,
        );

        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Unsupported
        );
    }

    #[test]
    fn runtime_hook_support_status_preserves_deferred_basis_posture() {
        let hook = runtime_hook_with_query_postures(
            runtime_backed_query_capability(),
            BasisFamily::StoreBacked,
            "observation",
            QuerySubscriptionSupportPosture::RuntimeBackedCertified,
        );

        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Deferred
        );
    }

    #[test]
    fn runtime_hook_support_status_unsupported_dominates_deferred_posture() {
        let hook = runtime_hook_with_query_postures(
            disabled_query_capability(),
            BasisFamily::StoreBacked,
            "observation",
            QuerySubscriptionSupportPosture::RuntimeBackedDeferred,
        );

        assert_eq!(
            support_status_for_runtime_hook(&hook),
            WorthUiQuerySupportStatus::Unsupported
        );
    }

    fn runtime_hook_with_query_postures(
        query_capability: QueryViewCapabilityReference,
        basis_family: BasisFamily,
        basis_context: &'static str,
        live_posture: QuerySubscriptionSupportPosture,
    ) -> WorthUiRuntimeDependencyHook {
        let basis_support = discover_basis_lifecycle_support(basis_family, basis_context);

        WorthUiRuntimeDependencyHook::new(
            WorthUiRuntimeDependencyHookKind::QueryLiveView,
            ViewBindingId::new("workspace.view_binding.selection").unwrap(),
            query_capability,
            "query-composition-profile",
            ViewShapeDescriptor::table(),
            QueryResultShapeReference::from_result_shape_family(ResultShapeFamily::Collection),
            QueryBasisPostureReference::from_basis_support_discovery(&basis_support),
            QueryLiveCompatibility::from_subscription_posture(
                QuerySubscriptionFamily::CollectionMembership,
                live_posture,
            ),
            QueryDenialPresentation::structured_status(),
        )
    }

    fn runtime_backed_query_capability() -> QueryViewCapabilityReference {
        query_capability_from_facade(ForgeQueryApplicationFacade::runtime_backed_default())
    }

    fn disabled_query_capability() -> QueryViewCapabilityReference {
        query_capability_from_facade(
            ForgeQueryApplicationFacade::new(
                ForgeQueryConfig::runtime_backed_default()
                    .with_query(ForgeQueryQueryConfig::disabled())
                    .with_signal(ForgeQuerySignalConfig::disabled())
                    .with_runtime_bridge(ForgeQueryRuntimeBridgeConfig::disabled())
                    .with_relational(ForgeQueryRelationalConfig::disabled()),
            )
            .expect("disabled query config still produces support posture"),
        )
    }

    fn query_capability_from_facade(
        facade: ForgeQueryApplicationFacade,
    ) -> QueryViewCapabilityReference {
        let query_support = facade.support_report();
        let query_capability = query_support
            .support_matrix()
            .descriptor(ForgeQueryCapabilityFamily::QueryComposition)
            .expect("query composition support posture");
        QueryViewCapabilityReference::from_query_capability_descriptor(query_capability)
    }
}
