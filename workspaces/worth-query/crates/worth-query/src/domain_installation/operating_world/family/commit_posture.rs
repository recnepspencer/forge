use super::super::{
    WorthQueryBoundCommitPosture, WorthQueryBoundGraphParticipation,
    WorthQueryOperationBindingCounters, WorthQueryOperationBindingDenial,
    WorthQueryOperationBindingDenialKind,
};
use crate::domain_installation::{
    InstalledCorrectionMechanism, PublishedAftermathPosture, WorthQueryGraphCommitPosture,
    WorthQueryOperationEffectContract, WorthQueryOperationGraphParticipation,
    WorthQueryOperationTouchContract,
};

pub(super) fn admit_commit_posture<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    graphs: &[WorthQueryBoundGraphParticipation],
    counters: &mut WorthQueryOperationBindingCounters,
) -> Result<WorthQueryBoundCommitPosture, WorthQueryOperationBindingDenial> {
    let semantics = operation.definition().semantics();
    let touched_roles = touched_roles(&semantics.touches);
    let primary_mutation = primary_graph_mutation(semantics, touched_roles, counters);
    let mutating_graphs = mutating_graphs(graphs, touched_roles, counters);
    if mutating_graphs.is_empty() {
        return Ok(if primary_mutation {
            WorthQueryBoundCommitPosture::Atomic
        } else {
            WorthQueryBoundCommitPosture::ReadOnly
        });
    }
    if primary_mutation {
        return require_compensation(operation, *counters);
    }
    let shared_atomic_authority = shared_atomic_authority(&mutating_graphs, counters);
    if !shared_atomic_authority {
        return require_compensation(operation, *counters);
    }
    Ok(WorthQueryBoundCommitPosture::Atomic)
}

fn primary_graph_mutation(
    semantics: &worth_query_installation::facade::WorthQueryDomainOperationSemanticClosure,
    touched_roles: &[String],
    counters: &mut WorthQueryOperationBindingCounters,
) -> bool {
    if (touched_roles.is_empty()
        || matches!(
            semantics.workflow,
            worth_query_installation::facade::WorthQueryOperationWorkflowContract::Declared(_)
        ))
        && matches!(
            semantics.effects,
            WorthQueryOperationEffectContract::Declared { .. }
        )
    {
        return true;
    }
    touched_roles.iter().any(|touched_role| {
        semantics.graph_reads.roles().iter().any(|read| {
            counters.graph_read_role_checks += 1;
            read.role == *touched_role
                && read.participation == WorthQueryOperationGraphParticipation::PrimaryLogicalGraph
        })
    })
}

fn mutating_graphs<'a>(
    graphs: &'a [WorthQueryBoundGraphParticipation],
    touched_roles: &[String],
    counters: &mut WorthQueryOperationBindingCounters,
) -> Vec<&'a WorthQueryBoundGraphParticipation> {
    graphs
        .iter()
        .filter(|graph| {
            counters.commit_graph_checks += 1;
            role_is_touched(&graph.role, touched_roles, counters)
        })
        .collect()
}

fn shared_atomic_authority(
    graphs: &[&WorthQueryBoundGraphParticipation],
    counters: &mut WorthQueryOperationBindingCounters,
) -> bool {
    let mut first_authority = None;
    let mut every_atomic = true;
    let mut authority_mismatch = false;
    for graph in graphs {
        counters.commit_graph_checks += 1;
        if graph.record.definition.contract.commit
            == WorthQueryGraphCommitPosture::CompensationRequired
        {
            return false;
        }
        every_atomic &= graph.record.definition.contract.commit
            == WorthQueryGraphCommitPosture::AtomicAuthorityRequired;
        counters.commit_authority_checks += 1;
        match (first_authority, graph.record.commit_authority.as_deref()) {
            (None, Some(authority)) => first_authority = Some(authority),
            (Some(first), Some(next)) => authority_mismatch |= !std::ptr::eq(first, next),
            (_, None) => every_atomic = false,
        }
    }
    every_atomic && first_authority.is_some() && !authority_mismatch
}

fn role_is_touched(
    role: &str,
    touched_roles: &[String],
    counters: &mut WorthQueryOperationBindingCounters,
) -> bool {
    touched_roles.iter().any(|touched| {
        counters.touched_graph_role_checks += 1;
        touched == role
    })
}

fn touched_roles(touches: &WorthQueryOperationTouchContract) -> &[String] {
    match touches {
        WorthQueryOperationTouchContract::Declared { graph_roles, .. } => graph_roles,
        WorthQueryOperationTouchContract::NotRequired => &[],
    }
}

fn require_compensation<D, O, F>(
    operation: &crate::domain_installation::WorthQueryInstalledDomainOperation<D, O, F>,
    counters: WorthQueryOperationBindingCounters,
) -> Result<WorthQueryBoundCommitPosture, WorthQueryOperationBindingDenial> {
    let aftermath = operation.definition().semantics().aftermath.as_ref();
    let compensatable = aftermath.is_some_and(|contract| {
        contract.published_posture() == PublishedAftermathPosture::Compensatable
            || matches!(
                contract.mechanism(),
                Some(InstalledCorrectionMechanism::Compensation(_))
            )
    });
    if compensatable {
        Ok(WorthQueryBoundCommitPosture::Compensated)
    } else {
        Err(WorthQueryOperationBindingDenial::new(
            WorthQueryOperationBindingDenialKind::CompensationUndeclared,
            "primary and separate mutations or separate commit authorities require compensation",
            counters,
        ))
    }
}
