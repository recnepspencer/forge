use worth_query_declaration::facade::application_query::ApplicationQueryLiveCauseBinding;
use worth_runtime_bridge::facade::BridgeExecutionBasisTerminalDisposition;

use super::WorthQueryApplicationLiveLease;

impl<
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >
    WorthQueryApplicationLiveLease<
        '_,
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >
where
    Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
{
    pub(super) fn terminate(
        &mut self,
        disposition: BridgeExecutionBasisTerminalDisposition,
    ) -> bool {
        let Some(mut basis) = self.basis.take() else {
            return true;
        };
        if self.queue.release_all(&mut basis).is_err() {
            self.basis = Some(basis);
            return false;
        }
        let crate::domain_computation::managed_run::WorthQueryManagedLowerExecutionBasis {
            bridge,
            relational,
        } = basis;
        match bridge.finalize(disposition) {
            Ok(_) => true,
            Err(failure) => {
                self.basis = Some(
                    crate::domain_computation::managed_run::WorthQueryManagedLowerExecutionBasis {
                        bridge: failure.into_basis(),
                        relational,
                    },
                );
                false
            }
        }
    }
}

impl<
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    > Drop
    for WorthQueryApplicationLiveLease<
        '_,
        '_,
        Schema,
        Query,
        Parameters,
        QueryResult,
        Principal,
        PrincipalIdentity,
        Scope,
        Target,
        Binding,
    >
where
    Binding: ApplicationQueryLiveCauseBinding<Schema, Query, Scope, Target>,
{
    fn drop(&mut self) {
        let _ = self.terminate(BridgeExecutionBasisTerminalDisposition::Abandoned);
    }
}

#[cfg(test)]
#[path = "lifecycle_tests.rs"]
mod lifecycle_tests;
