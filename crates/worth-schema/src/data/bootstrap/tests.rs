use forge_relational::facade::schema::{RelationPayloadClass, SchemaId, SchemaVersionId};

use crate::data::bootstrap::{
    worth_bootstrap_invariant_plan, worth_bootstrap_schema_registry, worth_bootstrap_tracing_plan,
    WORTH_SCHEMA_ID, WORTH_SCHEMA_VERSION_ID,
};
use crate::data::entities::WorthEntityKind;
use crate::data::relations::WorthRelationKind;
use crate::data::{aspects::WorthAspect, invariants::WorthInvariantGroup};

#[test]
fn worth_bootstrap_schema_registry_registers_all_bootstrap_kinds() {
    let registry = worth_bootstrap_schema_registry().expect("worth bootstrap schema");

    for kind in WorthEntityKind::ALL {
        let registration = registry
            .entity_registration(kind.kind_id())
            .expect("registered worth entity kind");
        assert_eq!(registration.kind_name, kind.kind_name());
    }

    for kind in WorthRelationKind::ALL {
        let registration = registry
            .relation_registration(kind.kind_id())
            .expect("registered worth relation kind");
        assert_eq!(registration.kind_name, kind.kind_name());
        assert_eq!(
            registration.payload_class,
            RelationPayloadClass::TopologyOnlyRelation
        );
        assert!(registration.relation_integrity.contract_count() >= 3);
    }
}

#[test]
fn worth_bootstrap_schema_registry_has_single_authoritative_schema_basis() {
    let registry = worth_bootstrap_schema_registry().expect("worth bootstrap schema");

    assert_eq!(
        registry
            .authoritative_schema_basis()
            .expect("single schema basis"),
        Some((
            SchemaId(WORTH_SCHEMA_ID.to_string()),
            SchemaVersionId(WORTH_SCHEMA_VERSION_ID),
        ))
    );
}

#[test]
fn worth_bootstrap_invariant_plan_covers_domain_groups_including_naming() {
    let plan = worth_bootstrap_invariant_plan();
    let all_groups = plan.all_groups();

    assert!(all_groups.contains(&WorthInvariantGroup::Naming(
        crate::data::invariants::WorthNamingInvariantGroup::PersistentNameStability,
    )));
    assert!(all_groups.contains(&WorthInvariantGroup::Naming(
        crate::data::invariants::WorthNamingInvariantGroup::PersistentNameUniqueness,
    )));
}

#[test]
fn worth_bootstrap_tracing_plan_covers_domain_aspects_including_persistent_naming() {
    let plan = worth_bootstrap_tracing_plan();
    let all_aspects = plan.all_aspects();

    assert!(all_aspects.contains(&WorthAspect::Naming(
        crate::data::aspects::WorthNamingAspect::PersistentName,
    )));
}
