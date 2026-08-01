use worth_foundational::facade::CanonicalDigestId;
use worth_query_admission::integration::WorthQueryExecutionCapacityReleaseReceipt;

use super::{basis_release::WorthQueryGraphWorkBasisRelease, WorthQueryManagedGraphWorkSession};

pub(in crate::domain_computation) struct WorthQueryGraphWorkSessionReleaseReceipt {
    session_identity: CanonicalDigestId,
    provider_session_identity: String,
    plan_identity: CanonicalDigestId,
    obligation_identity:
        worth_query_installation::facade::WorthQueryInstalledGraphObligationSetIdentity,
    required_obligation_count: usize,
    branch_id: worth_relational::facade::history::BranchId,
    capacity: WorthQueryExecutionCapacityReleaseReceipt,
    basis_released: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryGraphWorkSessionTerminalDenial {
    ProviderSessionMismatch,
    BranchMismatch,
    BasisNotReleased,
    CapacityMismatch,
}

impl WorthQueryGraphWorkSessionReleaseReceipt {
    pub(in crate::domain_computation) const fn session_identity(&self) -> &CanonicalDigestId {
        &self.session_identity
    }

    pub(in crate::domain_computation) const fn capacity(
        &self,
    ) -> &WorthQueryExecutionCapacityReleaseReceipt {
        &self.capacity
    }

    pub(in crate::domain_computation) fn provider_session_identity(&self) -> &str {
        &self.provider_session_identity
    }

    pub(in crate::domain_computation) const fn plan_identity(&self) -> &CanonicalDigestId {
        &self.plan_identity
    }

    pub(in crate::domain_computation) const fn obligation_identity(
        &self,
    ) -> &worth_query_installation::facade::WorthQueryInstalledGraphObligationSetIdentity {
        &self.obligation_identity
    }

    pub(in crate::domain_computation) const fn required_obligation_count(&self) -> usize {
        self.required_obligation_count
    }

    pub(in crate::domain_computation) const fn branch_id(
        &self,
    ) -> &worth_relational::facade::history::BranchId {
        &self.branch_id
    }

    pub(in crate::domain_computation) const fn basis_released(&self) -> bool {
        self.basis_released
    }
}

impl<Lane, Basis> WorthQueryManagedGraphWorkSession<Lane, Basis>
where
    Basis: WorthQueryGraphWorkBasisRelease,
{
    pub(super) fn release(mut self) -> WorthQueryGraphWorkSessionReleaseReceipt {
        let provider_session_identity = self.provider_session_identity().to_owned();
        let plan = self
            .plan
            .take()
            .expect("an active graph-work session releases one admitted plan");
        let plan_identity = *plan.identity();
        let obligation_identity = plan.obligation_identity().clone();
        let required_obligation_count = plan.required_obligations().len();
        let basis = self
            .basis
            .take()
            .expect("an active graph-work session releases one basis resource");
        let basis_released = basis.release_graph_work_basis();
        let capacity = match self.direct_attempt.take() {
            Some(attempt) => attempt.release().capacity().clone(),
            None => {
                drop(
                    self.read_provider_session
                        .take()
                        .expect("a read graph-work terminal owns one provider session"),
                );
                plan.release()
            }
        };
        WorthQueryGraphWorkSessionReleaseReceipt {
            session_identity: self.identity,
            provider_session_identity,
            plan_identity,
            obligation_identity,
            required_obligation_count,
            branch_id: self.affinity.branch().relational_branch().clone(),
            capacity,
            basis_released,
        }
    }

    pub(super) fn release_after_managed_cleanup(
        mut self,
        cleanup: &crate::domain_computation::managed_run::WorthQueryDirectRunCleanupReceipt,
    ) -> Result<WorthQueryGraphWorkSessionReleaseReceipt, WorthQueryGraphWorkSessionTerminalDenial>
    {
        let expected_session = self.identity.render_hex();
        if cleanup.attempt().provider_session_identity() != expected_session {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::ProviderSessionMismatch);
        }
        if cleanup.relational().identity().branch_id() != self.affinity.branch().relational_branch()
        {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::BranchMismatch);
        }
        if !cleanup.relational().released() {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::BasisNotReleased);
        }
        let capacity = cleanup.attempt().capacity();
        if self.operation_resource_plan_identity.as_deref()
            != Some(capacity.resource_plan_identity())
            || capacity.released_reservation_count() != self.reserved_capacity_count
        {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::CapacityMismatch);
        }
        if self.direct_attempt.is_some() {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::ProviderSessionMismatch);
        }
        if self.read_provider_session.is_some() {
            return Err(WorthQueryGraphWorkSessionTerminalDenial::ProviderSessionMismatch);
        }
        let plan = self
            .plan
            .take()
            .expect("a managed graph-work terminal retains its admitted plan shell");
        let plan_identity = *plan.identity();
        let obligation_identity = plan.obligation_identity().clone();
        let required_obligation_count = plan.required_obligations().len();
        let basis = self
            .basis
            .take()
            .expect("a managed graph-work terminal retains its transferred basis shell");
        drop(plan);
        drop(basis);
        Ok(WorthQueryGraphWorkSessionReleaseReceipt {
            session_identity: self.identity,
            provider_session_identity: cleanup.attempt().provider_session_identity().to_owned(),
            plan_identity,
            obligation_identity,
            required_obligation_count,
            branch_id: self.affinity.branch().relational_branch().clone(),
            capacity: capacity.clone(),
            basis_released: true,
        })
    }
}
