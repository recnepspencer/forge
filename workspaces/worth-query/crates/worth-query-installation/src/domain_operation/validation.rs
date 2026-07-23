use std::collections::BTreeSet;

use super::*;

mod conditional_graph_closure;
mod workflow;

use super::conditional_node::validate_conditional_nodes;
use conditional_graph_closure::validate_conditional_graph_closure;
use workflow::{validate_workflow, validate_workflow_closure};

pub(super) fn validate_domain_operation_meaning(
    operation: &WorthQueryPortableDomainOperationDefinition,
) -> Result<(), &'static str> {
    if operation.identity().name().trim().is_empty() {
        return Err("empty-operation-name");
    }
    if operation.identity().version() == 0 {
        return Err("zero-operation-version");
    }
    let semantics = operation.semantics();
    semantics
        .canonical_query
        .check_invariants()
        .map_err(|_| "invalid-canonical-query-bundle")?;
    validate_parameters(&semantics.parameters)?;
    validate_collection(&semantics.collection, &semantics.canonical_query)?;
    if semantics
        .required_domains
        .iter()
        .enumerate()
        .any(|(index, role)| {
            semantics.required_domains[..index].contains(role) || role.as_str().trim().is_empty()
        })
    {
        return Err("duplicate-or-empty-required-domain-role");
    }
    validate_workflow(&semantics.workflow)?;
    validate_conditional_nodes(&semantics.conditional_nodes)?;
    validate_conditional_graph_closure(semantics)?;
    validate_workflow_closure(semantics)?;
    validate_graph_reads(&semantics.graph_reads)?;
    validate_touches(&semantics.touches)?;
    validate_touch_graph_roles(&semantics.touches, &semantics.graph_reads)?;
    validate_effects(&semantics.effects)?;
    validate_invariants(&semantics.invariants)?;
    validate_reversal(&semantics.reversal)?;
    validate_publication(&semantics.publication)?;
    validate_projection_consumption(&semantics.publication, semantics.projection_consumption)?;
    if semantics.terminal.result_states.is_empty() {
        return Err("empty-terminal-result-state-set");
    }
    if semantics.lowering.family.trim().is_empty() {
        return Err("empty-lowering-family");
    }
    Ok(())
}

fn validate_parameters(
    contract: &WorthQueryOperationParameterContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationParameterContract::Declared { fields } = contract else {
        return Ok(());
    };
    let mut names = BTreeSet::new();
    for field in fields {
        if field.name.trim().is_empty() {
            return Err("empty-parameter-name");
        }
        if !names.insert(field.name.as_str()) {
            return Err("duplicate-parameter-name");
        }
    }
    Ok(())
}

fn validate_collection(
    contract: &WorthQueryOperationCollectionContract,
    canonical_query: &worth_query_declaration::facade::canonicalization::CanonicalQueryBundle,
) -> Result<(), &'static str> {
    let WorthQueryOperationCollectionContract::Collection {
        row_identity_field,
        ordering_fields,
        grouping,
        window,
        continuation,
        ..
    } = contract
    else {
        return Ok(());
    };
    if ordering_fields.is_empty() {
        return Err("empty-collection-ordering");
    }
    if ordering_fields.iter().collect::<BTreeSet<_>>().len() != ordering_fields.len() {
        return Err("duplicate-ordering-field");
    }
    validate_collection_query_contract(
        row_identity_field,
        ordering_fields,
        grouping,
        canonical_query,
    )?;
    if let WorthQueryOperationGroupingContract::Grouped { grouping_fields } = grouping {
        if grouping_fields.is_empty() {
            return Err("empty-collection-grouping");
        }
        if grouping_fields.iter().collect::<BTreeSet<_>>().len() != grouping_fields.len() {
            return Err("duplicate-grouping-field");
        }
    }
    match (continuation, window) {
        (
            WorthQueryOperationContinuationPosture::NotRequired,
            WorthQueryOperationWindowPolicy::CompleteCollection,
        )
        | (
            WorthQueryOperationContinuationPosture::SnapshotCursor
            | WorthQueryOperationContinuationPosture::LiveCursor,
            WorthQueryOperationWindowPolicy::ContinuationBounded,
        ) => Ok(()),
        _ => Err("collection-window-continuation-mismatch"),
    }
}

fn validate_collection_query_contract(
    row_identity_field: &WorthQueryOperationCollectionField,
    ordering_fields: &[WorthQueryOperationCollectionField],
    grouping: &WorthQueryOperationGroupingContract,
    canonical_query: &worth_query_declaration::facade::canonicalization::CanonicalQueryBundle,
) -> Result<(), &'static str> {
    use worth_query_declaration::facade::authoring::QueryFamily;

    let query = canonical_query.query();
    if query.family() != &QueryFamily::Collection {
        return Err("collection-contract-requires-collection-query");
    }
    if !query
        .projection()
        .iter()
        .any(|entry| collection_field_matches(row_identity_field, entry.field_key()))
    {
        return Err("collection-row-identity-not-projected");
    }
    if ordering_fields.len() != query.ordering().len()
        || ordering_fields
            .iter()
            .zip(query.ordering())
            .any(|(declared, canonical)| !collection_field_matches(declared, canonical.field_key()))
    {
        return Err("collection-ordering-canonical-query-mismatch");
    }
    if let WorthQueryOperationGroupingContract::Grouped { grouping_fields } = grouping {
        if grouping_fields.iter().any(|grouping_field| {
            !query
                .projection()
                .iter()
                .any(|entry| collection_field_matches(grouping_field, entry.field_key()))
        }) {
            return Err("collection-grouping-field-not-projected");
        }
    }
    Ok(())
}

fn collection_field_matches(
    collection_field: &WorthQueryOperationCollectionField,
    query_field: &worth_query_declaration::facade::authoring::AspectFieldKey,
) -> bool {
    let fields = collection_field.field_path().fields();
    fields.len() == 1
        && collection_field.aspect_key() == &query_field.native_aspect_key()
        && fields.first() == Some(&query_field.native_field_key())
}

fn validate_graph_reads(
    contract: &WorthQueryOperationGraphReadContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationGraphReadContract::Declared { roles } = contract else {
        return Ok(());
    };
    if roles.is_empty() {
        return Err("empty-graph-read-role-set");
    }
    for (index, role) in roles.iter().enumerate() {
        if role.role.trim().is_empty() {
            return Err("empty-graph-read-role");
        }
        if let WorthQueryOperationGraphParticipation::SeparateAuthority {
            role: participation_role,
        } = &role.participation
        {
            if participation_role.trim().is_empty() {
                return Err("empty-separate-graph-role");
            }
            if participation_role != &role.role {
                return Err("separate-graph-role-mismatch");
            }
        }
        if roles[..index].iter().any(|prior| prior.role == role.role) {
            return Err("duplicate-graph-read-role");
        }
    }
    Ok(())
}

fn validate_touch_graph_roles(
    touches: &WorthQueryOperationTouchContract,
    reads: &WorthQueryOperationGraphReadContract,
) -> Result<(), &'static str> {
    let WorthQueryOperationTouchContract::Declared { graph_roles, .. } = touches else {
        return Ok(());
    };
    if graph_roles
        .iter()
        .any(|role| !reads.roles().iter().any(|read| &read.role == role))
    {
        return Err("touch-references-undeclared-graph-role");
    }
    Ok(())
}

fn validate_touches(contract: &WorthQueryOperationTouchContract) -> Result<(), &'static str> {
    if let WorthQueryOperationTouchContract::Declared {
        graph_roles,
        scopes,
    } = contract
    {
        if graph_roles.is_empty() || scopes.is_empty() {
            return Err("empty-touch-contract");
        }
        validate_text_sequence(graph_roles, "empty-touch-graph-role")?;
        validate_text_sequence(scopes, "empty-touch-scope")?;
    }
    Ok(())
}

fn validate_effects(contract: &WorthQueryOperationEffectContract) -> Result<(), &'static str> {
    if let WorthQueryOperationEffectContract::Declared { effect_families } = contract {
        if effect_families.is_empty() {
            return Err("empty-effect-family-set");
        }
        if effect_families.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("duplicate-effect-family");
        }
    }
    Ok(())
}

fn validate_invariants(
    contract: &WorthQueryOperationInvariantContract,
) -> Result<(), &'static str> {
    if let WorthQueryOperationInvariantContract::Declared { invariant_slots } = contract {
        if invariant_slots.is_empty() {
            return Err("empty-invariant-slot-set");
        }
        validate_text_sequence(invariant_slots, "empty-invariant-slot")?;
    }
    Ok(())
}

fn validate_reversal(contract: &WorthQueryOperationReversalContract) -> Result<(), &'static str> {
    let subject = match contract {
        WorthQueryOperationReversalContract::ExactInverse { lowering_family } => lowering_family,
        WorthQueryOperationReversalContract::Compensation { .. } => return Ok(()),
        WorthQueryOperationReversalContract::ExactInverseWithPostcondition {
            operation,
            lowering_family,
            postcondition,
        } => {
            validate_text_sequence(
                &[
                    operation.slot(),
                    lowering_family.clone(),
                    aftermath_identity(postcondition).into(),
                ],
                "empty-aftermath-contract",
            )?;
            return Ok(());
        }
        WorthQueryOperationReversalContract::CompensationWithPostcondition {
            operation,
            postcondition,
        } => {
            validate_text_sequence(
                &[operation.slot(), aftermath_identity(postcondition).into()],
                "empty-aftermath-contract",
            )?;
            return Ok(());
        }
        WorthQueryOperationReversalContract::RebuildRequired { recovery_family } => recovery_family,
        WorthQueryOperationReversalContract::Irreversible
        | WorthQueryOperationReversalContract::ProvisionalDiscard => return Ok(()),
    };
    if subject.trim().is_empty() {
        return Err("empty-reversal-subject");
    }
    Ok(())
}

fn aftermath_identity(postcondition: &WorthQueryAftermathPostcondition) -> &str {
    match postcondition {
        WorthQueryAftermathPostcondition::ExactPriorTruth => "exact-prior-truth",
        WorthQueryAftermathPostcondition::InvariantRestored { invariant } => invariant,
        WorthQueryAftermathPostcondition::BusinessPostcondition { identity } => identity,
    }
}

fn validate_publication(
    contract: &WorthQueryOperationPublicationContract,
) -> Result<(), &'static str> {
    if matches!(
        contract,
        WorthQueryOperationPublicationContract::DerivedProjection { projection_role }
            if projection_role.as_str().trim().is_empty()
    ) {
        return Err("empty-publication-role");
    }
    Ok(())
}

fn validate_projection_consumption(
    publication: &WorthQueryOperationPublicationContract,
    consumption: WorthQueryOperationProjectionConsumptionContract,
) -> Result<(), &'static str> {
    match (publication, consumption) {
        (
            WorthQueryOperationPublicationContract::NotRequired,
            WorthQueryOperationProjectionConsumptionContract::NotRequired,
        )
        | (
            WorthQueryOperationPublicationContract::DerivedProjection { .. },
            WorthQueryOperationProjectionConsumptionContract::QueryReadAuthority,
        ) => Ok(()),
        _ => Err("publication-projection-consumption-contract-mismatch"),
    }
}

fn validate_text_sequence(values: &[String], denial: &'static str) -> Result<(), &'static str> {
    if values.iter().any(|value| value.trim().is_empty()) {
        return Err(denial);
    }
    Ok(())
}
