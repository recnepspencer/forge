use std::collections::BTreeSet;

use super::super::conditional_node::validate_conditional_nodes;
use super::super::*;
use super::validate_text_sequence;

pub(super) fn validate_workflow(
    contract: &WorthQueryOperationWorkflowContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationWorkflowContract::Declared(workflow) = contract else {
        return Ok(());
    };
    if workflow.entry_stage().trim().is_empty() {
        return Err("empty-workflow-entry-stage");
    }
    if workflow.stages().is_empty() {
        return Err("empty-workflow-stage-set");
    }
    let stages = workflow.stages();
    let mut identities = BTreeSet::new();
    for stage in stages {
        if stage.identity().trim().is_empty() {
            return Err("empty-workflow-stage-identity");
        }
        if !identities.insert(stage.identity()) {
            return Err("duplicate-workflow-stage-identity");
        }
        validate_text_sequence(stage.predecessors(), "empty-workflow-predecessor")?;
    }
    let entry = stages
        .iter()
        .find(|stage| stage.identity() == workflow.entry_stage())
        .ok_or("missing-workflow-entry-stage")?;
    if !entry.predecessors().is_empty() {
        return Err("workflow-entry-has-predecessors");
    }
    if stages
        .iter()
        .any(|stage| stage.identity() != workflow.entry_stage() && stage.predecessors().is_empty())
    {
        return Err("workflow-non-entry-root");
    }
    if !stages.iter().any(|stage| stage.is_terminal()) {
        return Err("workflow-has-no-terminal-stage");
    }
    for stage in stages {
        if stage
            .predecessors()
            .iter()
            .any(|predecessor| !identities.contains(predecessor.as_str()))
        {
            return Err("missing-workflow-predecessor");
        }
        if stage.is_terminal()
            && stages.iter().any(|candidate| {
                candidate
                    .predecessors()
                    .iter()
                    .any(|predecessor| predecessor == stage.identity())
            })
        {
            return Err("workflow-terminal-has-successor");
        }
    }
    let mut reachable = BTreeSet::from([workflow.entry_stage()]);
    loop {
        let before = reachable.len();
        for stage in stages {
            if stage
                .predecessors()
                .iter()
                .all(|predecessor| reachable.contains(predecessor.as_str()))
            {
                reachable.insert(stage.identity());
            }
        }
        if reachable.len() == before {
            break;
        }
    }
    if reachable.len() != stages.len() {
        return Err("cyclic-or-unreachable-workflow-stage");
    }
    let mut reaches_terminal = stages
        .iter()
        .filter(|stage| stage.is_terminal())
        .map(|stage| stage.identity())
        .collect::<BTreeSet<_>>();
    loop {
        let before = reaches_terminal.len();
        for stage in stages {
            if stages.iter().any(|successor| {
                reaches_terminal.contains(successor.identity())
                    && successor
                        .predecessors()
                        .iter()
                        .any(|predecessor| predecessor == stage.identity())
            }) {
                reaches_terminal.insert(stage.identity());
            }
        }
        if reaches_terminal.len() == before {
            break;
        }
    }
    if reaches_terminal.len() != stages.len() {
        return Err("incomplete-workflow-terminal-path");
    }
    Ok(())
}

pub(super) fn validate_workflow_closure(
    operation: &WorthQueryDomainOperationSemanticClosure,
) -> Result<(), &'static str> {
    let WorthQueryOperationWorkflowContract::Declared(workflow) = &operation.workflow else {
        return Ok(());
    };
    let publishable_count = workflow
        .stages()
        .iter()
        .filter(|stage| stage.is_publishable())
        .count();
    let operation_publishes = !matches!(
        operation.publication,
        WorthQueryOperationPublicationContract::NotRequired
    );
    if publishable_count != usize::from(operation_publishes) {
        return Err("workflow-publication-stage-count-mismatch");
    }
    for stage in workflow.stages() {
        let semantics = stage.semantics();
        validate_conditional_nodes(&semantics.conditional_nodes)?;
        if stage
            .required_capabilities()
            .iter()
            .any(|required| !operation.required_capabilities.contains(required))
        {
            return Err("workflow-stage-references-undeclared-capability");
        }
        if semantics
            .required_domain_roles
            .iter()
            .any(|role| !operation.required_domains.contains(role))
        {
            return Err("workflow-stage-references-undeclared-required-domain");
        }
        validate_text_sequence(
            &semantics.graph_read_roles,
            "empty-workflow-graph-read-role",
        )?;
        validate_text_sequence(&semantics.touch_roles, "empty-workflow-touch-role")?;
        if semantics
            .effect_roles
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err("duplicate-workflow-effect-role");
        }
        validate_text_sequence(&semantics.invariant_roles, "empty-workflow-invariant-role")?;
        if semantics
            .cost_roles
            .windows(2)
            .any(|pair| pair[0] == pair[1])
        {
            return Err("duplicate-workflow-cost-role");
        }
        if semantics.graph_read_roles.iter().any(|role| {
            !operation
                .graph_reads
                .roles()
                .iter()
                .any(|read| &read.role == role)
        }) {
            return Err("workflow-stage-references-undeclared-graph-read");
        }
        if semantics.touch_roles.iter().any(|role| !matches!(&operation.touches, WorthQueryOperationTouchContract::Declared { graph_roles, .. } if graph_roles.contains(role))) {
            return Err("workflow-stage-references-undeclared-touch");
        }
        if semantics.effect_roles.iter().any(|role| !matches!(&operation.effects, WorthQueryOperationEffectContract::Declared { effect_families } if effect_families.contains(role))) {
            return Err("workflow-stage-references-undeclared-effect");
        }
        if semantics.invariant_roles.iter().any(|role| !matches!(&operation.invariants, WorthQueryOperationInvariantContract::Declared { invariant_slots } if invariant_slots.contains(role))) {
            return Err("workflow-stage-references-undeclared-invariant");
        }
        if stage.is_terminal() && semantics.terminal_result_states.is_empty() {
            return Err("workflow-terminal-missing-result-state");
        }
        if !stage.is_terminal() && !semantics.terminal_result_states.is_empty() {
            return Err("nonterminal-workflow-stage-declares-result-state");
        }
        if semantics
            .terminal_result_states
            .iter()
            .any(|state| !operation.terminal.result_states.contains(state))
        {
            return Err("workflow-stage-references-undeclared-result-state");
        }
        if semantics
            .failure_classes
            .iter()
            .any(|failure| !operation.terminal.failure_classes.contains(failure))
        {
            return Err("workflow-stage-references-undeclared-failure-class");
        }
        if stage.is_publishable() && semantics.output != WorthQueryWorkflowValueContract::Projection
        {
            return Err("workflow-publishable-stage-output-is-not-projection");
        }
    }
    let stage_graph_reads = workflow
        .stages()
        .iter()
        .flat_map(|stage| stage.semantics().graph_read_roles.iter())
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let operation_graph_reads = operation
        .graph_reads
        .roles()
        .iter()
        .map(|read| read.role.as_str())
        .collect::<BTreeSet<_>>();
    if stage_graph_reads != operation_graph_reads {
        return Err("workflow-graph-read-closure-mismatch");
    }
    let stage_touches = workflow
        .stages()
        .iter()
        .flat_map(|stage| stage.semantics().touch_roles.iter())
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let operation_touches = match &operation.touches {
        WorthQueryOperationTouchContract::NotRequired => BTreeSet::new(),
        WorthQueryOperationTouchContract::Declared { graph_roles, .. } => {
            graph_roles.iter().map(String::as_str).collect()
        }
    };
    if stage_touches != operation_touches {
        return Err("workflow-touch-closure-mismatch");
    }
    let stage_effects = workflow
        .stages()
        .iter()
        .flat_map(|stage| stage.semantics().effect_roles.iter().copied())
        .collect::<BTreeSet<_>>();
    let operation_effects = match &operation.effects {
        WorthQueryOperationEffectContract::NotRequired => BTreeSet::new(),
        WorthQueryOperationEffectContract::Declared { effect_families } => {
            effect_families.iter().copied().collect()
        }
    };
    if stage_effects != operation_effects {
        return Err("workflow-effect-closure-mismatch");
    }
    let stage_invariants = workflow
        .stages()
        .iter()
        .flat_map(|stage| stage.semantics().invariant_roles.iter())
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    let operation_invariants = match &operation.invariants {
        WorthQueryOperationInvariantContract::NotRequired => BTreeSet::new(),
        WorthQueryOperationInvariantContract::Declared { invariant_slots } => {
            invariant_slots.iter().map(String::as_str).collect()
        }
    };
    if stage_invariants != operation_invariants {
        return Err("workflow-invariant-closure-mismatch");
    }
    Ok(())
}
