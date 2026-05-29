mod declarations;
mod lowering;
mod merge_policy_declarations;
mod relation_integrity;
mod revision;

use smallvec::SmallVec;

use crate::schema::data::{
    AspectPlanCatalog, KindAspectDeclarations, LoweredAspectBinding, LoweredAspectPlan,
    RelationIntegrityPlanCatalog, RelationKindRegistration, RelationalSchemaRegistry,
    SchemaRegistryError,
};

use declarations::{canonicalize_kind_aspect_declarations, RegistrationDomain};
use lowering::lower_binding;
use relation_integrity::{
    canonicalize_relation_integrity_declarations, lower_relation_integrity_plan,
};
use revision::derive_plan_revision;

pub(crate) fn canonicalize_entity_registration(
    mut registration: crate::schema::data::EntityKindRegistration,
) -> Result<crate::schema::data::EntityKindRegistration, SchemaRegistryError> {
    registration.aspect_declarations = canonicalize_kind_aspect_declarations(
        registration.kind_id,
        registration.aspect_declarations,
        RegistrationDomain::Entity,
    )?;
    Ok(registration)
}

pub(crate) fn canonicalize_relation_registration(
    mut registration: RelationKindRegistration,
) -> Result<RelationKindRegistration, SchemaRegistryError> {
    registration.aspect_declarations = canonicalize_kind_aspect_declarations(
        registration.kind_id,
        registration.aspect_declarations,
        RegistrationDomain::Relation,
    )?;
    registration.relation_integrity = canonicalize_relation_integrity_declarations(
        registration.kind_id,
        registration.relation_integrity,
    )?;
    Ok(registration)
}

pub(crate) fn lower_aspect_plans(registry: &RelationalSchemaRegistry) -> AspectPlanCatalog {
    let entity_plans = registry
        .entity_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_kind_plan(*kind_id, &registration.aspect_declarations),
            )
        })
        .collect();
    let relation_plans = registry
        .relation_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_kind_plan(*kind_id, &registration.aspect_declarations),
            )
        })
        .collect();
    AspectPlanCatalog {
        entity_plans,
        relation_plans,
    }
}

pub(crate) fn lower_relation_integrity_plans(
    registry: &RelationalSchemaRegistry,
) -> RelationIntegrityPlanCatalog {
    let relation_plans = registry
        .relation_kinds
        .iter()
        .map(|(kind_id, registration)| {
            (
                *kind_id,
                lower_relation_integrity_plan(
                    *kind_id,
                    &registration.relation_integrity,
                    registration.cascade_delete_policy,
                ),
            )
        })
        .collect();
    RelationIntegrityPlanCatalog { relation_plans }
}

fn lower_kind_plan(
    kind_id: crate::identity::data::KindId,
    declarations: &KindAspectDeclarations,
) -> LoweredAspectPlan {
    let executable_bindings = declarations
        .aspects
        .iter()
        .map(|aspect| lower_binding(kind_id, declarations.plan_revision, aspect))
        .collect::<SmallVec<[LoweredAspectBinding; 8]>>();
    LoweredAspectPlan {
        kind_id,
        plan_revision: declarations.plan_revision,
        executable_bindings,
    }
}

fn derive_kind_plan_revision(
    aspects: &[crate::schema::data::DeclaredAspect],
    identity_declarations: &[crate::merge::data::IdentityBasisDeclaration],
    merge_policy_declarations: &[crate::merge::data::AspectMergePolicyDeclaration],
) -> crate::schema::data::AspectPlanRevision {
    derive_plan_revision(aspects, identity_declarations, merge_policy_declarations)
}
