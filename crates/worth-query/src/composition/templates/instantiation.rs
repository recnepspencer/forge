use std::collections::BTreeMap;

use crate::authoring::{
    PredicateSelector, RawAuthoredQuery, RawAuthoredResultShape, WorthQueryPredicateOperand,
};
use crate::composition::counters::CompositionCounters;
use crate::composition::digests::TemplateBindingDigest;
use crate::composition::errors::{QueryCompositionAdmissionFailureClass, QueryCompositionError};
use crate::composition::TemplateFamily;

use crate::composition::scopes::BasisScopeEvidence;

use super::binding_set::{TemplateBindingSet, TemplateBindingValue};
use super::slot::{TemplateParameterSlot, TemplateParameterSlotKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TemplateInstantiationArtifact {
    template_family: TemplateFamily,
    binding_digest: TemplateBindingDigest,
    basis_evidence: Option<BasisScopeEvidence>,
    counters: CompositionCounters,
}

impl TemplateInstantiationArtifact {
    pub fn template_family(&self) -> TemplateFamily {
        self.template_family
    }

    pub fn binding_digest(&self) -> &TemplateBindingDigest {
        &self.binding_digest
    }

    pub fn basis_evidence(&self) -> Option<&BasisScopeEvidence> {
        self.basis_evidence.as_ref()
    }

    pub fn counters(&self) -> &CompositionCounters {
        &self.counters
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct TemplateInstantiationResult {
    pub(crate) query: RawAuthoredQuery,
    pub(crate) result_shape: RawAuthoredResultShape,
    pub(crate) artifact: TemplateInstantiationArtifact,
}

pub(crate) fn instantiate_template(
    family: TemplateFamily,
    mut query: RawAuthoredQuery,
    result_shape: RawAuthoredResultShape,
    slots: &[TemplateParameterSlot],
    bindings: &TemplateBindingSet,
    basis_evidence: Option<BasisScopeEvidence>,
) -> Result<TemplateInstantiationResult, QueryCompositionError> {
    deny_if_deferred_family(family, slots.len(), bindings.bindings().len())?;

    let slot_index = index_slots(family, slots)?;
    let bound_values = index_bindings(family, bindings, &slot_index)?;

    for slot in slots {
        let binding = bound_values.get(slot.name()).ok_or_else(|| {
            QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::MissingTemplateBinding,
                CompositionCounters::for_template_instantiation(
                    slots.len(),
                    bindings.bindings().len(),
                ),
                format!("template slot '{}' is missing a binding", slot.name()),
            )
        })?;
        query = apply_binding(
            query,
            family,
            slot,
            binding,
            slots.len(),
            bindings.bindings().len(),
        )?;
    }

    let binding_digest = TemplateBindingDigest::from_parts(
        &slots
            .iter()
            .map(|slot| {
                let binding = bound_values
                    .get(slot.name())
                    .expect("slot bindings are validated before digest construction");
                format!(
                    "{}:{}:{}",
                    slot.digest_part(family),
                    binding.kind_name(),
                    binding.digest_fragment()
                )
            })
            .collect::<Vec<_>>(),
    );
    let counters =
        CompositionCounters::for_template_instantiation(slots.len(), bindings.bindings().len());

    Ok(TemplateInstantiationResult {
        query,
        result_shape,
        artifact: TemplateInstantiationArtifact {
            template_family: family,
            binding_digest,
            basis_evidence,
            counters,
        },
    })
}

fn deny_if_deferred_family(
    family: TemplateFamily,
    slot_count: usize,
    binding_width: usize,
) -> Result<(), QueryCompositionError> {
    let counters = CompositionCounters::for_template_instantiation(slot_count, binding_width);
    match family {
        TemplateFamily::DetailTemplate
        | TemplateFamily::CollectionTemplate
        | TemplateFamily::GroupedCollectionTemplate => Ok(()),
        TemplateFamily::ObservedInspectorDetailTemplate
        | TemplateFamily::FocusedInspectorDetailTemplate => {
            Err(QueryCompositionError::unsupported_template(
                family,
                QueryCompositionAdmissionFailureClass::DeferredTemplateFamily,
                counters,
                "template family remains explicit deferred debt until later composition phases",
            ))
        }
        #[cfg(test)]
        TemplateFamily::UnsupportedTemplate => Err(QueryCompositionError::unsupported_template(
            family,
            QueryCompositionAdmissionFailureClass::UnsupportedTemplateFamily,
            counters,
            "unsupported template family remains denied in Phase 1",
        )),
    }
}

fn index_slots(
    family: TemplateFamily,
    slots: &[TemplateParameterSlot],
) -> Result<BTreeMap<String, TemplateParameterSlot>, QueryCompositionError> {
    let mut index = BTreeMap::new();
    for slot in slots {
        if index
            .insert(slot.name().to_string(), slot.clone())
            .is_some()
        {
            return Err(QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::TemplateBindingMismatch,
                CompositionCounters::for_template_instantiation(slots.len(), 0),
                format!("template slot '{}' is declared more than once", slot.name()),
            ));
        }
    }
    Ok(index)
}

fn index_bindings<'a>(
    family: TemplateFamily,
    bindings: &'a TemplateBindingSet,
    slot_index: &BTreeMap<String, TemplateParameterSlot>,
) -> Result<BTreeMap<String, &'a TemplateBindingValue>, QueryCompositionError> {
    let mut bound = BTreeMap::new();
    for entry in bindings.bindings() {
        let Some(slot) = slot_index.get(entry.slot.name()) else {
            return Err(QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::TemplateBindingMismatch,
                CompositionCounters::for_template_instantiation(
                    slot_index.len(),
                    bindings.bindings().len(),
                ),
                format!(
                    "binding supplied for undeclared template slot '{}'",
                    entry.slot.name()
                ),
            ));
        };
        if slot.kind() != entry.slot.kind() {
            return Err(QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::TemplateBindingMismatch,
                CompositionCounters::for_template_instantiation(
                    slot_index.len(),
                    bindings.bindings().len(),
                ),
                format!(
                    "binding kind mismatch for slot '{}': expected '{}' but received '{}'",
                    slot.name(),
                    slot.kind().as_str(),
                    entry.slot.kind().as_str()
                ),
            ));
        }
        if bound
            .insert(slot.name().to_string(), &entry.value)
            .is_some()
        {
            return Err(QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::DuplicateTemplateBinding,
                CompositionCounters::for_template_instantiation(
                    slot_index.len(),
                    bindings.bindings().len(),
                ),
                format!(
                    "duplicate binding supplied for template slot '{}'",
                    slot.name()
                ),
            ));
        }
    }

    Ok(bound)
}

fn apply_binding(
    mut query: RawAuthoredQuery,
    family: TemplateFamily,
    slot: &TemplateParameterSlot,
    binding: &TemplateBindingValue,
    slot_count: usize,
    binding_width: usize,
) -> Result<RawAuthoredQuery, QueryCompositionError> {
    match (slot.kind(), binding) {
        (TemplateParameterSlotKind::Predicate, TemplateBindingValue::Predicate(predicate)) => {
            query = query.with_predicate(predicate.clone());
        }
        (TemplateParameterSlotKind::Ordering, TemplateBindingValue::Ordering(ordering)) => {
            query = query.with_ordering(ordering.clone());
        }
        (TemplateParameterSlotKind::Projection, TemplateBindingValue::Projection(projection)) => {
            query = query.with_projection(projection.clone());
        }
        (TemplateParameterSlotKind::Traversal, TemplateBindingValue::Traversal(traversal)) => {
            query = query.with_traversal(traversal.clone());
        }
        (expected_kind, value) => {
            return Err(QueryCompositionError::invalid_template(
                family,
                QueryCompositionAdmissionFailureClass::TemplateBindingMismatch,
                CompositionCounters::for_template_instantiation(slot_count, binding_width),
                format!(
                    "slot '{}' expects '{}' but received '{}'",
                    slot.name(),
                    expected_kind.as_str(),
                    value.kind_name()
                ),
            ));
        }
    }

    Ok(query)
}

impl TemplateBindingValue {
    fn kind_name(&self) -> &'static str {
        match self {
            Self::Predicate(_) => "predicate",
            Self::Ordering(_) => "ordering",
            Self::Projection(_) => "projection",
            Self::Traversal(_) => "traversal",
        }
    }

    fn digest_fragment(&self) -> String {
        match self {
            Self::Predicate(predicate) => predicate_binding_digest_fragment(predicate),
            Self::Ordering(ordering) => format!(
                "{}:{}:{:?}",
                ordering.source_field_key().aspect().as_str(),
                ordering.source_field_key().field().as_str(),
                ordering.direction()
            ),
            Self::Projection(projection) => {
                format!("{}:{}", projection.aspect(), projection.field())
            }
            Self::Traversal(traversal) => {
                format!(
                    "{}:{}",
                    traversal.terminal_relation_projection_for_boundary(),
                    traversal.depth()
                )
            }
        }
    }
}

fn predicate_binding_digest_fragment(predicate: &PredicateSelector) -> String {
    match predicate {
        PredicateSelector::Equality(predicate) => format!(
            "equality:{}:{}:{}",
            predicate.aspect(),
            predicate.field(),
            scalar_predicate_value_digest_fragment(predicate.value())
        ),
        PredicateSelector::NativeComparison(predicate) => format!(
            "native_comparison:{}:{}:{:?}:{}",
            predicate.aspect(),
            predicate.field(),
            predicate.operator(),
            scalar_predicate_value_digest_fragment(predicate.value())
        ),
        PredicateSelector::StringContains(predicate) => format!(
            "string_contains:{}:{}:{}",
            predicate.aspect(),
            predicate.field(),
            predicate.value()
        ),
        PredicateSelector::SetMembership(predicate) => format!(
            "set_membership:{}:{}:{}",
            predicate.aspect(),
            predicate.field(),
            predicate
                .values()
                .iter()
                .map(scalar_predicate_value_digest_fragment)
                .collect::<Vec<_>>()
                .join("|")
        ),
        PredicateSelector::Presence(predicate) => format!(
            "presence:{}:{}:{}",
            predicate.aspect(),
            predicate.field(),
            predicate.kind().digest_key()
        ),
    }
}

fn scalar_predicate_value_digest_fragment(value: &WorthQueryPredicateOperand) -> String {
    worth_foundational::facade::prepare_aspect_value_identity_basis(value.as_native())
        .as_str()
        .to_owned()
}
