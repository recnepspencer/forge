use std::collections::BTreeSet;
use std::marker::PhantomData;
use std::sync::Arc;

use worth_foundational::facade::{CanonicalDigestDerivationDenial, CanonicalDigestId};
use worth_query_admission::integration::WorthQueryAdmittedGraphWorkPlan;
use worth_query_installation::facade::WorthQueryCanonicalWorkEvidence;

use crate::domain_computation::{
    WorthQueryExecutionAttemptIdentity, WorthQueryExecutionProviderSession,
};

use super::branch_affinity::WorthQueryGraphWorkBranchAffinity;
use super::session_affinity::WorthQueryGraphWorkSessionAffinity;
use super::session_identity::derive_session_identity;

pub(in crate::domain_computation) enum WorthQueryReadGraphWorkLane {}
pub(in crate::domain_computation) enum WorthQueryMutationGraphWorkLane {}

pub(in crate::domain_computation) struct WorthQueryManagedGraphWorkSession<Lane, Basis> {
    pub(super) identity: CanonicalDigestId,
    pub(super) plan: Option<WorthQueryAdmittedGraphWorkPlan>,
    pub(super) basis: Option<Basis>,
    pub(super) affinity: WorthQueryGraphWorkSessionAffinity,
    pub(super) completed_owner_steps: BTreeSet<(u32, usize)>,
    pub(super) canonical_work: WorthQueryCanonicalWorkEvidence,
    pub(super) read_provider_session: Option<WorthQueryExecutionProviderSession>,
    pub(super) direct_attempt: Option<super::super::WorthQueryDirectExecutionResourceAttempt>,
    pub(super) operation_resource_plan_identity: Option<Arc<str>>,
    pub(super) reserved_capacity_count: usize,
    pub(super) _lane: PhantomData<fn() -> Lane>,
}

impl<Lane, Basis> WorthQueryManagedGraphWorkSession<Lane, Basis> {
    pub(in crate::domain_computation) const fn identity(&self) -> &CanonicalDigestId {
        &self.identity
    }

    pub(in crate::domain_computation) fn basis(&self) -> &Basis {
        self.basis
            .as_ref()
            .expect("an active graph-work session owns its basis resource")
    }

    pub(in crate::domain_computation) fn plan(&self) -> &WorthQueryAdmittedGraphWorkPlan {
        self.plan
            .as_ref()
            .expect("an active graph-work session owns its admitted plan")
    }

    pub(in crate::domain_computation) const fn canonical_work(
        &self,
    ) -> WorthQueryCanonicalWorkEvidence {
        self.canonical_work
    }

    pub(in crate::domain_computation) const fn branch_affinity(
        &self,
    ) -> &WorthQueryGraphWorkBranchAffinity {
        self.affinity.branch()
    }

    pub(in crate::domain_computation) fn provider_session_identity(&self) -> &str {
        match (&self.read_provider_session, &self.direct_attempt) {
            (Some(session), None) => session.identity(),
            (None, Some(attempt)) => attempt.provider_session().identity(),
            (None, None) => {
                unreachable!("a transferred mutation session is inspected through managed cleanup")
            }
            (Some(_), Some(_)) => {
                unreachable!("one graph-work progression owns exactly one provider session")
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryGraphWorkSessionStartDenial {
    WrongLane,
    PlanMismatch,
    ObligationMismatch,
    ProviderMismatch,
    BindingMismatch,
    BranchMismatch,
    InvalidAffinity,
    RunIdentityExhausted,
    MissingOperationCapacity,
    ResourceAdmission,
    CanonicalIdentity(CanonicalDigestDerivationDenial),
}

pub(in crate::domain_computation) fn start_read_graph_work_session<Basis>(
    plan: WorthQueryAdmittedGraphWorkPlan,
    basis: Basis,
    affinity: WorthQueryGraphWorkSessionAffinity,
) -> Result<
    WorthQueryManagedGraphWorkSession<WorthQueryReadGraphWorkLane, Basis>,
    WorthQueryGraphWorkSessionStartDenial,
> {
    use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkIntentKind;
    if !matches!(
        plan.intent().kind(),
        WorthQueryGraphWorkIntentKind::ApplicationQueryRead
            | WorthQueryGraphWorkIntentKind::ApplicationOperationRead
    ) {
        return Err(WorthQueryGraphWorkSessionStartDenial::WrongLane);
    }
    let (identity, session_work) = validate_session_start(&plan, &affinity)?;
    let attempt_identity = WorthQueryExecutionAttemptIdentity::graph_work(&identity);
    let provider_session = WorthQueryExecutionProviderSession::mint_graph_read(&attempt_identity);
    if provider_session.identity() != identity.render_hex() {
        return Err(WorthQueryGraphWorkSessionStartDenial::InvalidAffinity);
    }
    Ok(build_session(
        plan,
        basis,
        affinity,
        identity,
        session_work,
        Some(provider_session),
        None,
    ))
}

pub(in crate::domain_computation) fn start_mutation_graph_work_session<Basis>(
    mut plan: WorthQueryAdmittedGraphWorkPlan,
    basis: Basis,
    affinity: WorthQueryGraphWorkSessionAffinity,
    runtime: &crate::domain_computation::execution_runtime::WorthQueryExecutionRuntime,
    authority: &crate::domain_computation::operation_binding::WorthQueryExecutionBoundOperationAuthority,
) -> Result<
    WorthQueryManagedGraphWorkSession<WorthQueryMutationGraphWorkLane, Basis>,
    WorthQueryGraphWorkSessionStartDenial,
> {
    use worth_query_admission::facade::graph_obligation::WorthQueryGraphWorkIntentKind;
    if plan.intent().kind() != WorthQueryGraphWorkIntentKind::ApplicationOperationMutation {
        return Err(WorthQueryGraphWorkSessionStartDenial::WrongLane);
    }
    let (identity, session_work) = validate_session_start(&plan, &affinity)?;
    let capacity = plan
        .take_operation_capacity()
        .ok_or(WorthQueryGraphWorkSessionStartDenial::MissingOperationCapacity)?;
    let direct_attempt = runtime
        .start_reserved_direct_graph_work_attempt(authority, capacity, &identity)
        .map_err(|_| WorthQueryGraphWorkSessionStartDenial::ResourceAdmission)?;
    if direct_attempt.provider_session().identity() != identity.render_hex() {
        return Err(WorthQueryGraphWorkSessionStartDenial::InvalidAffinity);
    }
    Ok(build_session(
        plan,
        basis,
        affinity,
        identity,
        session_work,
        None,
        Some(direct_attempt),
    ))
}

fn validate_session_start(
    plan: &WorthQueryAdmittedGraphWorkPlan,
    affinity: &WorthQueryGraphWorkSessionAffinity,
) -> Result<
    (CanonicalDigestId, WorthQueryCanonicalWorkEvidence),
    WorthQueryGraphWorkSessionStartDenial,
> {
    if plan.identity() != &affinity.plan_identity
        || plan.obligation_identity() != &affinity.obligation_identity
    {
        return Err(WorthQueryGraphWorkSessionStartDenial::PlanMismatch);
    }
    if plan.binding_identity() != affinity.binding_identity() {
        return Err(WorthQueryGraphWorkSessionStartDenial::BindingMismatch);
    }
    derive_session_identity(plan.identity(), affinity)
        .map_err(WorthQueryGraphWorkSessionStartDenial::CanonicalIdentity)
}

fn build_session<Lane, Basis>(
    plan: WorthQueryAdmittedGraphWorkPlan,
    basis: Basis,
    affinity: WorthQueryGraphWorkSessionAffinity,
    identity: CanonicalDigestId,
    session_work: WorthQueryCanonicalWorkEvidence,
    read_provider_session: Option<WorthQueryExecutionProviderSession>,
    direct_attempt: Option<super::super::WorthQueryDirectExecutionResourceAttempt>,
) -> WorthQueryManagedGraphWorkSession<Lane, Basis> {
    let operation_resource_plan_identity = direct_attempt
        .as_ref()
        .map(|attempt| Arc::from(attempt.resources().identity()));
    let reserved_capacity_count = direct_attempt.as_ref().map_or_else(
        || plan.reservation_count(),
        |attempt| attempt.retained_capacity_reservation_count(),
    );
    WorthQueryManagedGraphWorkSession {
        identity,
        canonical_work: plan.canonical_work().combine(session_work),
        plan: Some(plan),
        basis: Some(basis),
        affinity,
        completed_owner_steps: BTreeSet::new(),
        read_provider_session,
        direct_attempt,
        operation_resource_plan_identity,
        reserved_capacity_count,
        _lane: PhantomData,
    }
}
