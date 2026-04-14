use std::collections::BTreeMap;

use crate::authoring::{AspectFieldKey, ScalarPredicateValue};
use crate::canonicalization::{
    CanonicalPredicateEntry, CanonicalPredicateFamily, CanonicalPredicateOperand,
};
use crate::schema_view::{QuerySchemaView, SchemaFieldKind};

use super::predicate_state::FieldPredicateState;
use super::{
    failure::ValidationFailureArtifact, QueryValidationCounters, QueryValidationError,
    ValidatedPredicateEntry, ValidationEvent, ValidationRejectionMatrix,
};

type LegalPredicate = (CanonicalPredicateEntry, SchemaFieldKind, &'static str);

pub(crate) fn validate_predicate_entries(
    predicates: &[CanonicalPredicateEntry],
    schema_view: &QuerySchemaView,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<(Vec<ValidatedPredicateEntry>, Vec<ValidationEvent>), ValidationFailureArtifact> {
    let mut legal_predicates = Vec::new();
    let mut events = Vec::new();

    for predicate in predicates {
        counters.record_schema_lookup();
        let Some(field) = schema_view.field(predicate.aspect.as_str(), predicate.field.as_str())
        else {
            counters.record_rejection();
            rejection_matrix.record_predicate_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnknownField {
                    aspect: predicate.aspect.to_string(),
                    field: predicate.field.to_string(),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        };

        let predicate_family = predicate_family_name(predicate.family);
        let value_kind = operand_kind_name(&predicate.operand);

        if matches!(field.kind(), SchemaFieldKind::StructuredContent) {
            counters.record_rejection();
            rejection_matrix.record_predicate_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::UnsupportedStructuredContentPredicate {
                    aspect: predicate.aspect.to_string(),
                    field: predicate.field.to_string(),
                    predicate_family,
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        if matches!(field.kind(), SchemaFieldKind::WorkflowState)
            && !field.is_workflow_predicate_queryable()
        {
            counters.record_rejection();
            rejection_matrix.record_predicate_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IllegalWorkflowPredicateCapabilityOrContextShape {
                    aspect: predicate.aspect.to_string(),
                    field: predicate.field.to_string(),
                    predicate_family,
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        if !predicate_capability_is_admitted(predicate, field) {
            counters.record_rejection();
            rejection_matrix.record_predicate_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IncompatiblePredicateFamily {
                    aspect: predicate.aspect.to_string(),
                    field: predicate.field.to_string(),
                    predicate_family,
                    field_kind: field_kind_name(field.kind()),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        if !predicate_operand_matches_field_kind(predicate, field.kind()) {
            counters.record_rejection();
            rejection_matrix.record_predicate_rejection();
            return Err(ValidationFailureArtifact::new(
                QueryValidationError::IncompatiblePredicateFamily {
                    aspect: predicate.aspect.to_string(),
                    field: predicate.field.to_string(),
                    predicate_family,
                    field_kind: field_kind_name(field.kind()),
                },
                counters.clone(),
                rejection_matrix.clone(),
            ));
        }

        counters.record_predicate_validated();
        legal_predicates.push((predicate.clone(), field.kind().clone(), value_kind));
        events.push(ValidationEvent::PredicateValidated {
            aspect: predicate.aspect.to_string(),
            field: predicate.field.to_string(),
            predicate_family,
            field_kind: format!("{:?}", field.kind()),
        });
    }

    let normalized = normalize_legal_predicates(legal_predicates, counters, rejection_matrix)?;
    counters.set_validated_predicate_count(normalized.len());

    Ok((normalized, events))
}

fn normalize_legal_predicates(
    legal_predicates: Vec<LegalPredicate>,
    counters: &mut QueryValidationCounters,
    rejection_matrix: &mut ValidationRejectionMatrix,
) -> Result<Vec<ValidatedPredicateEntry>, ValidationFailureArtifact> {
    let mut by_field: BTreeMap<AspectFieldKey, Vec<LegalPredicate>> = BTreeMap::new();

    for predicate in legal_predicates {
        by_field
            .entry(AspectFieldKey::from_parts(
                predicate.0.aspect.clone(),
                predicate.0.field.clone(),
            ))
            .or_default()
            .push(predicate);
    }

    let mut normalized = Vec::new();

    for (field_key, entries) in by_field {
        let mut state = FieldPredicateState::default();
        for entry in entries {
            state.ingest(
                entry,
                field_key.aspect().as_str(),
                field_key.field().as_str(),
                counters,
                rejection_matrix,
            )?;
        }
        normalized.extend(state.into_validated(
            field_key.aspect().as_str(),
            field_key.field().as_str(),
            counters,
            rejection_matrix,
        )?);
    }

    normalized.sort();
    Ok(normalized)
}

fn predicate_capability_is_admitted(
    predicate: &CanonicalPredicateEntry,
    field: &crate::schema_view::SchemaFieldView,
) -> bool {
    match predicate.family {
        CanonicalPredicateFamily::StringContains => field.is_text_predicate_queryable(),
        CanonicalPredicateFamily::ScalarMembership => field.is_membership_predicate_queryable(),
        CanonicalPredicateFamily::PresenceIsPresent => field.is_presence_predicate_queryable(),
        _ => true,
    }
}

fn predicate_operand_matches_field_kind(
    predicate: &CanonicalPredicateEntry,
    field_kind: &SchemaFieldKind,
) -> bool {
    match predicate.family {
        CanonicalPredicateFamily::Equality => matches!(
            (field_kind, &predicate.operand),
            (
                SchemaFieldKind::String,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(_))
            ) | (
                SchemaFieldKind::WorkflowState,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(_))
            ) | (
                SchemaFieldKind::Integer,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Integer(_))
            ) | (
                SchemaFieldKind::Boolean,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Boolean(_))
            )
        ),
        CanonicalPredicateFamily::IntegerGreaterThan
        | CanonicalPredicateFamily::IntegerLessThan => matches!(
            (field_kind, &predicate.operand),
            (
                SchemaFieldKind::Integer,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Integer(_))
            )
        ),
        CanonicalPredicateFamily::StringContains => matches!(
            (field_kind, &predicate.operand),
            (
                SchemaFieldKind::String,
                CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(_))
            )
        ),
        CanonicalPredicateFamily::ScalarMembership => match (field_kind, &predicate.operand) {
            (SchemaFieldKind::String, CanonicalPredicateOperand::ScalarSet(values)) => values
                .as_slice()
                .iter()
                .all(|value| matches!(value, ScalarPredicateValue::String(_))),
            (SchemaFieldKind::Integer, CanonicalPredicateOperand::ScalarSet(values)) => values
                .as_slice()
                .iter()
                .all(|value| matches!(value, ScalarPredicateValue::Integer(_))),
            (SchemaFieldKind::Boolean, CanonicalPredicateOperand::ScalarSet(values)) => values
                .as_slice()
                .iter()
                .all(|value| matches!(value, ScalarPredicateValue::Boolean(_))),
            _ => false,
        },
        CanonicalPredicateFamily::PresenceIsPresent => {
            matches!(&predicate.operand, CanonicalPredicateOperand::Presence(_))
        }
    }
}

fn predicate_family_name(family: CanonicalPredicateFamily) -> &'static str {
    match family {
        CanonicalPredicateFamily::Equality => "equality",
        CanonicalPredicateFamily::IntegerGreaterThan => "integer-greater-than",
        CanonicalPredicateFamily::IntegerLessThan => "integer-less-than",
        CanonicalPredicateFamily::StringContains => "string-contains",
        CanonicalPredicateFamily::ScalarMembership => "scalar-membership",
        CanonicalPredicateFamily::PresenceIsPresent => "presence-is-present",
    }
}

fn operand_kind_name(operand: &CanonicalPredicateOperand) -> &'static str {
    match operand {
        CanonicalPredicateOperand::Scalar(ScalarPredicateValue::String(_)) => "String",
        CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Integer(_)) => "Integer",
        CanonicalPredicateOperand::Scalar(ScalarPredicateValue::Boolean(_)) => "Boolean",
        CanonicalPredicateOperand::ScalarSet(values) => match values.first() {
            Some(ScalarPredicateValue::String(_)) => "Set<String>",
            Some(ScalarPredicateValue::Integer(_)) => "Set<Integer>",
            Some(ScalarPredicateValue::Boolean(_)) => "Set<Boolean>",
            None => "Set<Empty>",
        },
        CanonicalPredicateOperand::Presence(_) => "Presence",
    }
}

fn field_kind_name(field_kind: &SchemaFieldKind) -> &'static str {
    match field_kind {
        SchemaFieldKind::String => "String",
        SchemaFieldKind::Integer => "Integer",
        SchemaFieldKind::Boolean => "Boolean",
        SchemaFieldKind::StructuredContent => "StructuredContent",
        SchemaFieldKind::WorkflowState => "WorkflowState",
    }
}
