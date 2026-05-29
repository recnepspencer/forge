use forge_relational::facade::schema::{SchemaId, SchemaVersionId};

use crate::data::bootstrap::{
    bootstrap_invariant_plan, bootstrap_schema_registry, bootstrap_tracing_plan, SCHEMA_ID,
    SCHEMA_VERSION_ID,
};
use crate::data::entities::EntityKind;
use crate::data::relations::RelationKind;
use crate::data::{aspects::Aspect, invariants::InvariantGroup};

#[test]
fn bootstrap_schema_registry_registers_all_bootstrap_kinds() {
    let registry = bootstrap_schema_registry().expect(" bootstrap schema");

    for kind in EntityKind::ALL {
        let registration = registry
            .entity_registration(kind.kind_id())
            .expect("registered  entity kind");
        assert_eq!(registration.kind_name, kind.kind_name());
    }

    for kind in RelationKind::ALL {
        let registration = registry
            .relation_registration(kind.kind_id())
            .expect("registered  relation kind");
        assert_eq!(registration.kind_name, kind.kind_name());
        assert!(registration.relation_integrity.contract_count() >= 3);
    }
}

#[test]
fn bootstrap_schema_registry_has_single_authoritative_schema_basis() {
    let registry = bootstrap_schema_registry().expect(" bootstrap schema");

    assert_eq!(
        registry
            .authoritative_schema_basis()
            .expect("single schema basis"),
        Some((
            SchemaId(SCHEMA_ID.to_string()),
            SchemaVersionId(SCHEMA_VERSION_ID),
        ))
    );
}

#[test]
fn bootstrap_invariant_plan_covers_domain_groups_including_naming() {
    let plan = bootstrap_invariant_plan();
    let all_groups = plan.all_groups();

    assert!(all_groups.contains(&InvariantGroup::Naming(
        crate::data::invariants::NamingInvariantGroup::PersistentNameStability,
    )));
    assert!(all_groups.contains(&InvariantGroup::Naming(
        crate::data::invariants::NamingInvariantGroup::PersistentNameUniqueness,
    )));
}

#[test]
fn bootstrap_tracing_plan_covers_domain_aspects_including_persistent_naming() {
    let plan = bootstrap_tracing_plan();
    let all_aspects = plan.all_aspects();

    assert!(all_aspects.contains(&Aspect::Naming(
        crate::data::aspects::NamingAspect::PersistentName,
    )));
}
