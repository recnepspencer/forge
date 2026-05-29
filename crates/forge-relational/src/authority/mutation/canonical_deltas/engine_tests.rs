use std::collections::BTreeMap;

use forge_foundational::{
    aspects, validate_aspect_value, AspectContract, AspectContractRevision, AspectIdentity,
    AspectKey as FoundationalAspectKey, AspectValue as FoundationalAspectValue,
    InternedString as FoundationalInternedString, ScalarAspectType,
};
use forge_proof::TransitionOutcome;

use crate::authority::mutation::outcomes::RecordMutation;
use crate::authority::mutation::MutationWorkspace;
use crate::config::data::{
    AdjacencyBackend, AdjacencyPolicy, CascadeDeletePolicy, CrossContextPolicy,
};
use crate::identity::data::{EntityId, KindId, PartitionId, RelationId, VersionId};
use crate::publication::patch::data::{AspectKey, CanonicalAspectSet};
use crate::schema::data::{
    AspectPlanCatalog, AspectPlanRevision, LoweredAspectBinding, LoweredAspectPlan,
    LoweredExecutableAspectBindingKind, RelationalSchemaRegistry,
};
use crate::storage::overlay::WorkingState;
use crate::symbols::data::StringInterner;
use crate::transactions::data::{AspectDeltaFailureFields, AspectFieldPatchTarget};

use super::data::{AuthoritativeDeltaPatchOperation, CanonicalAspectDeltaEvidence};
use super::{canonical_delta_for_mutation, CanonicalDeltaError};

fn mutation_config() -> crate::config::data::MutationConfig {
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

fn empty_workspace<'a>(
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

#[test]
fn missing_entity_aspect_plan_returns_typed_error() {
    let config = mutation_config();
    let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
    let mut symbols = StringInterner::default();
    let catalog = AspectPlanCatalog::empty();
    let schema = RelationalSchemaRegistry::new();
    let mutation = RecordMutation::EntityCreated {
        entity_id: EntityId::new(PartitionId(1), 0, 1),
        kind_id: KindId(999),
        authoritative_patch: None,
    };

    let error = canonical_delta_for_mutation(
        &mutation,
        &empty_workspace(&mut state, &mut symbols, &catalog, &config, &schema),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        CanonicalDeltaError::MissingEntityAspectPlan {
            kind_id: KindId(999)
        }
    ));
}

#[test]
fn missing_relation_aspect_plan_returns_typed_error() {
    let config = mutation_config();
    let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
    let mut symbols = StringInterner::default();
    let catalog = AspectPlanCatalog::empty();
    let schema = RelationalSchemaRegistry::new();
    let source = EntityId::new(PartitionId(1), 0, 1);
    let target = EntityId::new(PartitionId(1), 1, 1);
    let mutation = RecordMutation::RelationCreated {
        relation_id: RelationId::new(PartitionId(2), 0, 1),
        kind_id: KindId(777),
        source,
        target,
        authoritative_patch: None,
    };

    let error = canonical_delta_for_mutation(
        &mutation,
        &empty_workspace(&mut state, &mut symbols, &catalog, &config, &schema),
    )
    .unwrap_err();

    assert!(matches!(
        error,
        CanonicalDeltaError::MissingRelationAspectPlan {
            kind_id: KindId(777)
        }
    ));
}

#[test]
fn entity_field_patch_evidence_denial_carries_typed_target() {
    let target = AspectFieldPatchTarget::single(
        AspectKey::new("name").expect("valid aspect key"),
        forge_foundational::facade::FieldKey::new("name").expect("valid field key"),
    );
    let conflict = CanonicalDeltaError::EntityFieldBindingRequiresAuthoritativePatchEvidence {
        target: target.clone(),
    }
    .to_commit_conflict();

    let crate::transactions::data::ConflictClass::AspectDeltaFailure { fields, detail } =
        conflict.class
    else {
        panic!("expected aspect delta failure conflict");
    };
    assert!(detail.contains("requires authoritative patch evidence"));
    assert!(matches!(
        fields,
        AspectDeltaFailureFields::EntityFieldBindingRequiresAuthoritativePatchEvidence {
            target: actual
        } if actual == target
    ));
}

#[test]
fn entity_field_delta_requires_authoritative_state() {
    let config = mutation_config();
    let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
    let mut symbols = StringInterner::default();
    let schema = RelationalSchemaRegistry::new();
    let mut catalog = AspectPlanCatalog::empty();
    catalog.entity_plans.insert(
        KindId(1),
        LoweredAspectPlan {
            kind_id: KindId(1),
            plan_revision: AspectPlanRevision(7),
            executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                aspect_key: AspectKey::new("name").unwrap(),
                contract: aspects()
                    .contract()
                    .for_key(
                        FoundationalAspectKey::new("name")
                            .expect("foundational key for lowered aspect binding"),
                    )
                    .identified_by(AspectIdentity(1))
                    .at_revision(AspectContractRevision(7))
                    .scalar(ScalarAspectType::String),
                binding_kind: LoweredExecutableAspectBindingKind::EntityFieldScalar {
                    field: forge_foundational::facade::FieldKey::new("name").expect("valid field"),
                },
            }],
        },
    );
    let mutation = RecordMutation::EntityCreated {
        entity_id: EntityId::new(PartitionId(1), 0, 1),
        kind_id: KindId(1),
        authoritative_patch: None,
    };

    let delta = canonical_delta_for_mutation(
        &mutation,
        &empty_workspace(&mut state, &mut symbols, &catalog, &config, &schema),
    )
    .expect("missing authoritative state should leave absent aspect evidence");

    assert!(delta.changed_aspects.is_empty());
    let CanonicalAspectDeltaEvidence::ScalarAspectValueTransition {
        locator,
        old_present,
        new_present,
        old_value,
        new_value,
    } = &delta.evaluated_bindings[0].evidence
    else {
        panic!("expected absent scalar evidence without authoritative state");
    };
    assert_authoritative_whole_aspect_locator(locator, "name");
    assert!(!old_present);
    assert!(!new_present);
    assert_eq!(old_value, &None);
    assert_eq!(new_value, &None);
}

#[test]
fn entity_field_delta_materializes_authoritative_aspect_patch() {
    let config = mutation_config();
    let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
    let mut symbols = StringInterner::default();
    let schema = RelationalSchemaRegistry::new();
    let contract = scalar_string_contract("name", 1, 7);
    let authoritative_patch = authoritative_string_patch(&contract, "native-authority");
    let mut catalog = AspectPlanCatalog::empty();
    catalog.entity_plans.insert(
        KindId(1),
        LoweredAspectPlan {
            kind_id: KindId(1),
            plan_revision: AspectPlanRevision(7),
            executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                aspect_key: AspectKey::new("name").unwrap(),
                contract: contract.clone(),
                binding_kind: LoweredExecutableAspectBindingKind::EntityFieldScalar {
                    field: forge_foundational::facade::FieldKey::new("name").expect("valid field"),
                },
            }],
        },
    );
    let mutation = RecordMutation::EntityCreated {
        entity_id: EntityId::new(PartitionId(1), 0, 1),
        kind_id: KindId(1),
        authoritative_patch: Some(authoritative_patch),
    };

    let delta = canonical_delta_for_mutation(
        &mutation,
        &empty_workspace(&mut state, &mut symbols, &catalog, &config, &schema),
    )
    .expect("entity field evidence should come from authoritative patch");

    assert_eq!(
        delta.changed_aspects,
        CanonicalAspectSet::new([AspectKey::new("name").unwrap()])
    );
    let CanonicalAspectDeltaEvidence::AuthoritativePatchOperation {
        locator,
        operation:
            AuthoritativeDeltaPatchOperation::WholeAspectSet {
                value: Some(FoundationalAspectValue::String(actual)),
            },
    } = &delta.evaluated_bindings[0].evidence
    else {
        panic!("expected scalar authoritative patch evidence");
    };
    assert_authoritative_whole_aspect_locator(locator, "name");
    assert_eq!(
        actual,
        &FoundationalInternedString::Raw("native-authority".to_string())
    );
}

#[test]
fn relation_field_delta_materializes_authoritative_aspect_patch() {
    let config = mutation_config();
    let mut state = WorkingState::new(BTreeMap::new(), config.adjacency_policy.clone());
    let mut symbols = StringInterner::default();
    let schema = RelationalSchemaRegistry::new();
    let contract = scalar_string_contract("relation.label", 11, 3);
    let mut catalog = AspectPlanCatalog::empty();
    catalog.relation_plans.insert(
        KindId(2),
        LoweredAspectPlan {
            kind_id: KindId(2),
            plan_revision: AspectPlanRevision(3),
            executable_bindings: smallvec::smallvec![LoweredAspectBinding {
                aspect_key: AspectKey::new("relation.label").unwrap(),
                contract: contract.clone(),
                binding_kind: LoweredExecutableAspectBindingKind::RelationFieldScalar {
                    field: forge_foundational::facade::FieldKey::new("label").expect("valid field"),
                },
            }],
        },
    );
    let authoritative_patch = authoritative_string_patch(&contract, "native-authority");
    let source = EntityId::new(PartitionId(1), 0, 1);
    let target = EntityId::new(PartitionId(1), 1, 1);
    let mutation = RecordMutation::RelationCreated {
        relation_id: RelationId::new(PartitionId(2), 0, 1),
        kind_id: KindId(2),
        source,
        target,
        authoritative_patch: Some(authoritative_patch),
    };

    let delta = canonical_delta_for_mutation(
        &mutation,
        &empty_workspace(&mut state, &mut symbols, &catalog, &config, &schema),
    )
    .expect("relation field evidence should come from authoritative aspect patch");

    let CanonicalAspectDeltaEvidence::AuthoritativePatchOperation {
        locator,
        operation:
            AuthoritativeDeltaPatchOperation::WholeAspectSet {
                value: Some(FoundationalAspectValue::String(actual)),
            },
    } = &delta.evaluated_bindings[0].evidence
    else {
        panic!("expected scalar authoritative relation patch evidence");
    };
    assert_authoritative_whole_aspect_locator(locator, "relation.label");
    assert_eq!(
        actual,
        &FoundationalInternedString::Raw("native-authority".to_string())
    );
}

fn assert_authoritative_whole_aspect_locator(
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

fn scalar_string_contract(key: &str, identity: u64, revision: u64) -> AspectContract {
    aspects()
        .contract()
        .for_key(FoundationalAspectKey::new(key).expect("foundational key"))
        .identified_by(AspectIdentity(identity))
        .at_revision(AspectContractRevision(revision))
        .scalar(ScalarAspectType::String)
}

fn authoritative_string_patch(
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
