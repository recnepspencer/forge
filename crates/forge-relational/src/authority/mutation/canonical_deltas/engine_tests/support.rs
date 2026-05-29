use std::collections::BTreeMap;

use forge_foundational::{
    aspects, validate_aspect_value, AspectContract, AspectContractRevision, AspectIdentity,
    AspectKey as FoundationalAspectKey, AspectValue as FoundationalAspectValue,
    InternedString as FoundationalInternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;

use crate::authority::mutation::MutationWorkspace;
use crate::config::data::{
    AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CrossContextPolicy,
};
use crate::identity::data::{KindId, VersionId};
use crate::schema::data::{
    AspectBinding, AspectPlanCatalog, AspectPlanRevision, LoweredAspectBinding, LoweredAspectPlan,
    RelationalSchemaRegistry,
};
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;

pub(super) fn mutation_config() -> crate::config::data::MutationConfig {
    crate::config::data::MutationConfig {
        cascade_delete_policy: CascadeDeletePolicy::CascadeDeleteRelations,
        adjacency_policy: AdjacencyPolicy {
            backend: AdjacencyBackend::InlineSmallDegreeAdjacency,
            small_degree_inline_capacity: 4,
        },
        cross_context_policy: CrossContextPolicy::AllowExplicit,
        execution_model: crate::logic::planning::RelationalExecutionModel::SerialAuthority,
    }
}

pub(super) fn empty_workspace<'a>(
    state: &'a mut WorkingState,
    symbols: &'a mut StringInterner,
    aspect_plans: &'a AspectPlanCatalog,
    config: &'a crate::config::data::MutationConfig,
    schema: &'a RelationalSchemaRegistry,
) -> MutationWorkspace<'a> {
    MutationWorkspace::new(
        state,
        symbols,
        config,
        schema,
        aspect_plans,
        VersionId(1),
        crate::authority::mutation::BranchLocalDeleteAllowance::default(),
    )
}

pub(super) fn catalog_with_entity_binding(
    kind_id: KindId,
    contract: AspectContract,
    binding: AspectBinding,
) -> AspectPlanCatalog {
    let mut catalog = AspectPlanCatalog::empty();
    catalog.entity_plans.insert(
        kind_id,
        LoweredAspectPlan {
            kind_id,
            plan_revision: AspectPlanRevision(7),
            executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                contract,
                target: binding,
            }],
        },
    );
    catalog
}

pub(super) fn catalog_with_relation_binding(
    kind_id: KindId,
    contract: AspectContract,
    binding: AspectBinding,
) -> AspectPlanCatalog {
    let mut catalog = AspectPlanCatalog::empty();
    catalog.relation_plans.insert(
        kind_id,
        LoweredAspectPlan {
            kind_id,
            plan_revision: AspectPlanRevision(3),
            executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                contract,
                target: binding,
            }],
        },
    );
    catalog
}

pub(super) fn assert_authoritative_whole_aspect_locator(
    locator: &forge_foundational::facade::AspectValueLocator,
    expected_aspect_key: &str,
) {
    let forge_foundational::facade::AspectValueLocator::WholeAspect(aspect_locator) = locator
    else {
        panic!("expected authoritative whole-aspect value locator");
    };
    assert_eq!(
        aspect_locator.authority(),
        forge_foundational::facade::LocatorAuthority::Authoritative
    );
    assert_eq!(
        aspect_locator.aspect_key(),
        &FoundationalAspectKey::new(expected_aspect_key).expect("expected aspect key")
    );
}

pub(super) fn scalar_string_contract(
    aspect_key: FoundationalAspectKey,
    identity: u64,
    revision: u64,
) -> AspectContract {
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(AspectIdentity(identity))
        .at_revision(AspectContractRevision(revision))
        .scalar(ScalarAspectType::String)
}

pub(super) fn summary_struct_contract(aspect_key: FoundationalAspectKey) -> AspectContract {
    let shape = aspects()
        .struct_fields()
        .required("title", ScalarAspectType::String)
        .finish()
        .expect("valid summary struct shape");
    aspects()
        .contract()
        .for_key(aspect_key)
        .identified_by(AspectIdentity(41))
        .at_revision(AspectContractRevision(7))
        .struct_aspect(shape)
}

pub(super) fn authoritative_string_patch(
    contract: &AspectContract,
    value: &str,
) -> forge_foundational::facade::AuthoritativeRecordAspectPatch {
    let TransitionOutcome::Success(validated) = validate_aspect_value(
        contract,
        FoundationalAspectValue::String(FoundationalInternedString::Raw(value.to_string())).into(),
    ) else {
        panic!("test value should validate against scalar string contract");
    };
    let TransitionOutcome::Success(patch) =
        forge_foundational::facade::AuthoritativeRecordAspectPatch::whole_aspect([validated], [])
    else {
        panic!("test patch should construct one whole-aspect set");
    };
    patch
}

pub(super) fn authoritative_summary_patch(
    contract: &AspectContract,
    title: &str,
) -> forge_foundational::facade::AuthoritativeRecordAspectPatch {
    let value = forge_foundational::facade::StructAspectValue::new([(
        forge_foundational::facade::FieldKey::new("title").expect("valid field"),
        FoundationalAspectValue::String(title.into()),
    )])
    .expect("valid summary struct value");
    let TransitionOutcome::Success(validated) = validate_aspect_value(contract, value.into())
    else {
        panic!("test value should validate against struct contract");
    };
    let TransitionOutcome::Success(patch) =
        forge_foundational::facade::AuthoritativeRecordAspectPatch::whole_aspect([validated], [])
    else {
        panic!("test patch should construct one whole-aspect set");
    };
    patch
}

pub(super) fn empty_working_state(config: &crate::config::data::MutationConfig) -> WorkingState {
    WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone())
}
