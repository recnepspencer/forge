use forge_relational::facade::schema::{RelationPayloadClass, SchemaId, SchemaVersionId};

use super::registry::bootstrap_schema_registry;
use super::schema_identity::{SCHEMA_ID, SCHEMA_VERSION_ID};
use crate::data::entities::EntityKind;
use crate::data::relations::RelationKind;

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
        assert_eq!(
            registration.payload_class,
            RelationPayloadClass::TopologyOnlyRelation
        );
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
