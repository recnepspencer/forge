use std::collections::BTreeMap;

use forge_foundational::facade::{
    AuthoritativeRecordAspectState, ContractValidatedAspectValueView, FieldKey,
};

use crate::capabilities::AspectPlanSource;
use crate::logic::runtime::RelationalRuntime;
use crate::schema::data::{LoweredAspectPlan, LoweredExecutableAspectBindingKind};
use crate::storage::data::{
    authoritative_aspect_value_field_comparison_key, AuthoritativeFieldComparisonKey,
};

pub(super) fn authoritative_entity_field_comparison_keys(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
) -> BTreeMap<FieldKey, AuthoritativeFieldComparisonKey> {
    authoritative_record_field_comparison_keys(
        runtime.entity_aspect_plan(kind_id),
        authoritative_state,
        FieldBindingDomain::Entity,
    )
}

pub(super) fn authoritative_relation_field_comparison_keys(
    runtime: &RelationalRuntime,
    kind_id: crate::identity::data::KindId,
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
) -> BTreeMap<FieldKey, AuthoritativeFieldComparisonKey> {
    authoritative_record_field_comparison_keys(
        runtime.relation_aspect_plan(kind_id),
        authoritative_state,
        FieldBindingDomain::Relation,
    )
}

#[derive(Debug, Clone, Copy)]
enum FieldBindingDomain {
    Entity,
    Relation,
}

fn authoritative_record_field_comparison_keys(
    plan: Option<&LoweredAspectPlan>,
    authoritative_state: Option<&AuthoritativeRecordAspectState>,
    domain: FieldBindingDomain,
) -> BTreeMap<FieldKey, AuthoritativeFieldComparisonKey> {
    let Some(authoritative_state) = authoritative_state else {
        return BTreeMap::new();
    };
    let Some(plan) = plan else {
        return BTreeMap::new();
    };
    let mut comparison_keys = BTreeMap::new();
    for binding in &plan.executable_bindings {
        match field_binding_target(&binding.binding_kind, domain) {
            Some(FieldBindingTarget::Scalar(field)) => {
                let Some(entry) = authoritative_state.get(&binding.aspect_key) else {
                    continue;
                };
                if let ContractValidatedAspectValueView::Scalar(value) = entry.view() {
                    comparison_keys.insert(
                        field.clone(),
                        authoritative_aspect_value_field_comparison_key(value),
                    );
                }
            }
            Some(FieldBindingTarget::Struct) => {
                let Some(entry) = authoritative_state.get(&binding.aspect_key) else {
                    continue;
                };
                if let ContractValidatedAspectValueView::Struct(struct_value) = entry.view() {
                    for (field, value) in struct_value.fields() {
                        comparison_keys.insert(
                            field.clone(),
                            authoritative_aspect_value_field_comparison_key(value),
                        );
                    }
                }
            }
            None => {}
        }
    }
    comparison_keys
}

enum FieldBindingTarget<'a> {
    Scalar(&'a FieldKey),
    Struct,
}

fn field_binding_target(
    binding_kind: &LoweredExecutableAspectBindingKind,
    domain: FieldBindingDomain,
) -> Option<FieldBindingTarget<'_>> {
    match (domain, binding_kind) {
        (
            FieldBindingDomain::Entity,
            LoweredExecutableAspectBindingKind::EntityFieldScalar { field },
        )
        | (
            FieldBindingDomain::Relation,
            LoweredExecutableAspectBindingKind::RelationFieldScalar { field },
        ) => Some(FieldBindingTarget::Scalar(field)),
        (
            FieldBindingDomain::Entity,
            LoweredExecutableAspectBindingKind::EntityFieldStruct { .. },
        )
        | (
            FieldBindingDomain::Relation,
            LoweredExecutableAspectBindingKind::RelationFieldStruct { .. },
        ) => Some(FieldBindingTarget::Struct),
        _ => None,
    }
}
