use worth_foundational::facade::CanonicalDigestId;
use worth_query_installation::facade::{
    WorthQueryInstalledGraphAuthorizationRequirement, WorthQueryInstalledGraphObligation,
    WorthQueryInstalledGraphObligationKind, WorthQueryInstalledGraphObligationOwner,
    WorthQueryInstalledGraphObligationTerminalRequirement,
};

use super::WorthQueryManagedGraphWorkSession;

pub(in crate::domain_computation) struct WorthQueryGraphOwnerCompletion {
    session_identity: CanonicalDigestId,
    obligation_slot: u32,
    owner_ordinal: usize,
    owner: WorthQueryInstalledGraphObligationOwner,
    terminal: WorthQueryInstalledGraphObligationTerminalRequirement,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::domain_computation) enum WorthQueryGraphOwnerCompletionDenial {
    ForeignSession,
    UnknownObligation,
    OwnerRouteMismatch,
    TerminalMismatch,
    DuplicateCompletion,
}

impl<Lane, Basis> WorthQueryManagedGraphWorkSession<Lane, Basis> {
    pub(in crate::domain_computation) fn record_owner_completion(
        &mut self,
        completion: WorthQueryGraphOwnerCompletion,
    ) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
        if completion.session_identity != self.identity {
            return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
        }
        let row = self
            .plan()
            .required_obligations()
            .iter()
            .find(|row| row.identity().slot() == completion.obligation_slot)
            .ok_or(WorthQueryGraphOwnerCompletionDenial::UnknownObligation)?;
        if row.owner_progression().get(completion.owner_ordinal) != Some(&completion.owner) {
            return Err(WorthQueryGraphOwnerCompletionDenial::OwnerRouteMismatch);
        }
        if row.terminal_requirement() != completion.terminal {
            return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
        }
        if !self
            .completed_owner_steps
            .insert((completion.obligation_slot, completion.owner_ordinal))
        {
            return Err(WorthQueryGraphOwnerCompletionDenial::DuplicateCompletion);
        }
        Ok(())
    }
}

pub(in crate::domain_computation) fn record_principal_authorization_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    principal: &crate::domain_computation::authorization::WorthQueryPrincipalCurrentnessDependency,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if principal.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, principal.branch_id())?;
    let row = authorization_row_matching(session, |requirement| {
        matches!(
            requirement,
            WorthQueryInstalledGraphAuthorizationRequirement::Principal
        )
    })?;
    record_exact_route(session, row)
}

pub(in crate::domain_computation) fn record_ability_authorization_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    decisions: &[crate::domain_computation::authorization::WorthQueryAuthorizationDecisionFact],
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    let row = authorization_row_matching(session, |requirement| {
        matches!(
            requirement,
            WorthQueryInstalledGraphAuthorizationRequirement::Abilities(_)
        )
    })?;
    let Some(WorthQueryInstalledGraphAuthorizationRequirement::Abilities(requirements)) =
        row.authorization_requirement()
    else {
        return Err(WorthQueryGraphOwnerCompletionDenial::OwnerRouteMismatch);
    };
    if decisions.len() != requirements.len() {
        return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
    }
    if decisions
        .iter()
        .any(|decision| decision.session_identity() != session.identity())
    {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    if decisions
        .iter()
        .any(|decision| decision.branch_id() != session.branch_affinity().relational_branch())
    {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    record_exact_route(session, row)
}

pub(in crate::domain_computation) fn record_capability_authorization_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    authorization: &crate::domain_computation::authorization::WorthQueryRetainedCapabilityAuthorization,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    let row = authorization_row_matching(session, |requirement| {
        matches!(
            requirement,
            WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements)
                if requirements.iter().any(|requirement| {
                    requirement.identity().bytes() == authorization.capability_identity()
                })
        )
    })?;
    let Some(WorthQueryInstalledGraphAuthorizationRequirement::Capabilities(requirements)) =
        row.authorization_requirement()
    else {
        return Err(WorthQueryGraphOwnerCompletionDenial::OwnerRouteMismatch);
    };
    if authorization.exact_fact_count() == 0
        || !requirements.iter().any(|requirement| {
            requirement.identity().bytes() == authorization.capability_identity()
        })
    {
        return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
    }
    if !authorization.belongs_to_session(session.identity()) {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    if !authorization.belongs_to_branch(session.branch_affinity().relational_branch()) {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    record_exact_route(session, row)
}

pub(in crate::domain_computation) fn record_operation_graph_read_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    evidence: crate::domain_computation::primary_graph::WorthQueryOperationGraphReadCompletion,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if evidence.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, evidence.branch_id())?;
    let rows = obligation_rows(session, WorthQueryInstalledGraphObligationKind::GraphRead);
    record_routes(session, rows)
}

pub(in crate::domain_computation) fn record_application_query_graph_read_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    evidence: crate::domain_computation::primary_graph::WorthQueryApplicationQueryGraphReadCompletion,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if evidence.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, evidence.branch_id())?;
    let rows = obligation_rows(session, WorthQueryInstalledGraphObligationKind::GraphRead);
    record_routes(session, rows)
}

pub(in crate::domain_computation) fn record_operation_mutation_touch_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    evidence: crate::domain_computation::primary_graph::WorthQueryOperationMutationTouchCompletion,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if evidence.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, evidence.branch_id())?;
    let rows = obligation_rows(
        session,
        WorthQueryInstalledGraphObligationKind::MutationTouch,
    );
    if !rows.is_empty() && evidence.realized_effect_count() == 0 {
        return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
    }
    record_routes(session, rows)
}

pub(in crate::domain_computation) fn record_operation_invariant_execution_completion<
    Lane,
    Basis,
>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    evidence: crate::domain_computation::primary_graph::WorthQueryOperationInvariantExecutionCompletion,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if evidence.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, evidence.branch_id())?;
    let rows = obligation_rows(
        session,
        WorthQueryInstalledGraphObligationKind::InvariantExecution,
    );
    if evidence.receipt_count() != rows.len() {
        return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
    }
    record_routes(session, rows)
}

pub(in crate::domain_computation) fn record_operation_effect_application_completion<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    evidence: crate::domain_computation::primary_graph::WorthQueryOperationEffectApplicationCompletion,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    if evidence.session_identity() != session.identity() {
        return Err(WorthQueryGraphOwnerCompletionDenial::ForeignSession);
    }
    require_branch(session, evidence.branch_id())?;
    let rows = obligation_rows(
        session,
        WorthQueryInstalledGraphObligationKind::EffectApplication,
    );
    if rows.is_empty()
        || evidence.provider_runtime_instance_id() == 0
        || evidence.commit_id().0 == 0
    {
        return Err(WorthQueryGraphOwnerCompletionDenial::TerminalMismatch);
    }
    record_routes(session, rows)
}

fn require_branch<Lane, Basis>(
    session: &WorthQueryManagedGraphWorkSession<Lane, Basis>,
    branch_id: &worth_relational::facade::history::BranchId,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    (branch_id == session.branch_affinity().relational_branch())
        .then_some(())
        .ok_or(WorthQueryGraphOwnerCompletionDenial::ForeignSession)
}

fn authorization_row_matching<Lane, Basis>(
    session: &WorthQueryManagedGraphWorkSession<Lane, Basis>,
    matches_requirement: impl Fn(&WorthQueryInstalledGraphAuthorizationRequirement) -> bool,
) -> Result<WorthQueryInstalledGraphObligation, WorthQueryGraphOwnerCompletionDenial> {
    session
        .plan()
        .required_obligations()
        .iter()
        .find(|row| {
            row.kind() == WorthQueryInstalledGraphObligationKind::AuthorizationObservation
                && row
                    .authorization_requirement()
                    .is_some_and(|requirement| matches_requirement(&requirement))
        })
        .cloned()
        .ok_or(WorthQueryGraphOwnerCompletionDenial::UnknownObligation)
}

fn record_exact_route<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    row: WorthQueryInstalledGraphObligation,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    for (owner_ordinal, owner) in row.owner_progression().iter().copied().enumerate() {
        session.record_owner_completion(WorthQueryGraphOwnerCompletion {
            session_identity: *session.identity(),
            obligation_slot: row.identity().slot(),
            owner_ordinal,
            owner,
            terminal: row.terminal_requirement(),
        })?;
    }
    Ok(())
}

fn obligation_rows<Lane, Basis>(
    session: &WorthQueryManagedGraphWorkSession<Lane, Basis>,
    kind: WorthQueryInstalledGraphObligationKind,
) -> Vec<WorthQueryInstalledGraphObligation> {
    session
        .plan()
        .required_obligations()
        .iter()
        .filter(|row| row.kind() == kind)
        .cloned()
        .collect()
}

fn record_routes<Lane, Basis>(
    session: &mut WorthQueryManagedGraphWorkSession<Lane, Basis>,
    rows: Vec<WorthQueryInstalledGraphObligation>,
) -> Result<(), WorthQueryGraphOwnerCompletionDenial> {
    for row in rows {
        record_exact_route(session, row)?;
    }
    Ok(())
}
