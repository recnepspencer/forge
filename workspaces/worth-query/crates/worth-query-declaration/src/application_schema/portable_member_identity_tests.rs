use super::*;
use crate::application_query::{
    ApplicationQueryBasisSupport, ApplicationQueryCardinality, ApplicationQueryDefinitionBuilder,
    ApplicationQueryDependencyCeiling, ApplicationQueryDisclosureContract,
    ApplicationQueryLaneEligibility, ApplicationQueryResultShapeBuilder,
};

pub(super) struct Schema;
struct FirstInput;
struct SecondInput;
struct BlankInput;
struct FirstOperation;
struct SecondOperation;
struct BlankOperation;
struct FirstPayload;
struct SecondPayload;
struct FirstEffect;
struct SecondEffect;
struct QueryEntity;
struct QueryParameters;

crate::worth_query_entity!(PortableEntity in Schema);
crate::worth_query_capability_context!(
    FirstContext in Schema,
    identity "worth.query.test.colliding-context.v1"
);
crate::worth_query_capability_context!(
    SecondContext in Schema,
    identity "worth.query.test.colliding-context.v1"
);
crate::worth_query_capability_context!(BlankContext in Schema, identity "");
crate::worth_query_capability_context_entity_slot!(
    FirstSlot in Schema,
    FirstContext => PortableEntity,
    identity "worth.query.test.colliding-slot.v1"
);
crate::worth_query_capability_context_entity_slot!(
    BlankSlot in Schema,
    FirstContext => PortableEntity,
    identity ""
);
crate::worth_query_capability_context_entity_slot!(
    SecondSlot in Schema,
    FirstContext => PortableEntity,
    identity "worth.query.test.colliding-slot.v1"
);
crate::worth_query_capability_provenance!(
    FirstProvenance in Schema,
    identity "worth.query.test.colliding-provenance.v1"
);
crate::worth_query_capability_provenance!(BlankProvenance in Schema, identity "");
crate::worth_query_capability_provenance!(
    SecondProvenance in Schema,
    identity "worth.query.test.colliding-provenance.v1"
);

crate::worth_query_application_query!(
    FirstQuery in Schema,
    identity "worth.query.test.colliding-query.v1",
    parameters QueryParameters => "worth.query.test.query-parameters.v1",
    result () => "worth.rust.unit",
    scope QueryEntity => "worth.query.test.query-scope.v1",
    name "first_query"
);
crate::worth_query_application_query!(
    SecondQuery in Schema,
    identity "worth.query.test.colliding-query.v1",
    parameters QueryParameters => "worth.query.test.query-parameters.v1",
    result () => "worth.rust.unit",
    scope QueryEntity => "worth.query.test.query-scope.v1",
    name "second_query"
);
crate::worth_query_application_query!(
    BlankQuery in Schema,
    identity "",
    parameters QueryParameters => "worth.query.test.query-parameters.v1",
    result () => "worth.rust.unit",
    scope QueryEntity => "worth.query.test.query-scope.v1",
    name "blank_query"
);

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "WORTH.tests";
    const NAME: &'static str = "portable-member-identity";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema().build()
    }
}

worth_query_portable_type!(FirstInput => "worth.query.test.first-input.v1");
worth_query_portable_type!(SecondInput => "worth.query.test.second-input.v1");
worth_query_portable_type!(BlankInput => "");
worth_query_portable_type!(FirstPayload => "worth.query.test.first-payload.v1");
worth_query_portable_type!(SecondPayload => "worth.query.test.second-payload.v1");

macro_rules! operation_identity {
    ($operation:ty, $input:ty, $identifier:literal) => {
        impl ApplicationOperationMarkerIdentity for $operation {
            type Schema = Schema;
            type Input = $input;
            const IDENTIFIER: &'static str = $identifier;
        }
    };
}

operation_identity!(FirstOperation, FirstInput, "SameOperation");
operation_identity!(SecondOperation, SecondInput, "SameOperation");
operation_identity!(BlankOperation, BlankInput, "BlankOperation");

macro_rules! effect_identity {
    ($effect:ty, $payload:ty) => {
        impl ApplicationEffectMarkerIdentity for $effect {
            type Schema = Schema;
            type Payload = $payload;
            const IDENTIFIER: &'static str = "SameEffect";
        }
        impl ApplicationEffectPayload for $payload {
            fn retained_bytes(&self) -> u64 {
                0
            }
        }
    };
}

effect_identity!(FirstEffect, FirstPayload);
effect_identity!(SecondEffect, SecondPayload);

#[test]
fn one_operation_name_cannot_redeclare_a_different_input_identity() {
    let first = ApplicationOperationRef::<Schema, FirstOperation, FirstInput>::from_declaration();
    let second =
        ApplicationOperationRef::<Schema, SecondOperation, SecondInput>::from_declaration();
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .operation(
            first
                .definition()
                .no_external_effect()
                .no_aftermath()
                .finish(),
        )
        .operation(
            second
                .definition()
                .no_external_effect()
                .no_aftermath()
                .finish(),
        )
        .build()
        .unwrap_err();
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::DuplicateMember);
}

#[test]
fn one_effect_name_cannot_redeclare_a_different_payload_identity() {
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .effect(ApplicationEffectRef::<Schema, FirstEffect, FirstPayload>::from_declaration())
        .effect(ApplicationEffectRef::<Schema, SecondEffect, SecondPayload>::from_declaration())
        .build()
        .unwrap_err();
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::DuplicateMember);
}

#[test]
fn blank_portable_type_identity_is_rejected_before_canonical_meaning() {
    let operation =
        ApplicationOperationRef::<Schema, BlankOperation, BlankInput>::from_declaration();
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .operation(
            operation
                .definition()
                .no_external_effect()
                .no_aftermath()
                .finish(),
        )
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );
}

#[test]
fn two_query_markers_cannot_claim_one_portable_query_identity() {
    let entity = ApplicationEntityRef::<Schema, QueryEntity>::from_schema_identifier("QueryEntity");
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(entity)
        .application_query(query_definition(FirstQuery::reference(), entity).unwrap())
        .application_query(query_definition(SecondQuery::reference(), entity).unwrap())
        .build()
        .unwrap_err();
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::DuplicateMember);
}

#[test]
fn blank_query_identity_is_rejected_before_schema_canonicalization() {
    let entity = ApplicationEntityRef::<Schema, QueryEntity>::from_schema_identifier("QueryEntity");
    let denial = match query_definition(BlankQuery::reference(), entity) {
        Ok(_) => panic!("blank query identity must be rejected"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial,
        crate::application_query::ApplicationQueryDefinitionDenial::InvalidPortableIdentity
    );
}

#[test]
fn capability_marker_identity_collisions_are_rejected() {
    let context_denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .capability_context(FirstContext::reference())
        .capability_context(SecondContext::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        context_denial,
        ApplicationSchemaDeclarationDenial::DuplicateMember
    );

    let slot_denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(PortableEntity::reference())
        .capability_context(FirstContext::reference())
        .capability_context_entity_slot(FirstSlot::reference())
        .capability_context_entity_slot(SecondSlot::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        slot_denial,
        ApplicationSchemaDeclarationDenial::DuplicateMember
    );

    let provenance_denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .capability_provenance(FirstProvenance::reference())
        .capability_provenance(SecondProvenance::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        provenance_denial,
        ApplicationSchemaDeclarationDenial::DuplicateMember
    );
}

#[test]
fn blank_capability_marker_identity_is_rejected() {
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .capability_context(BlankContext::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );

    let slot_denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(PortableEntity::reference())
        .capability_context(FirstContext::reference())
        .capability_context_entity_slot(BlankSlot::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        slot_denial,
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );

    let provenance_denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .capability_provenance(BlankProvenance::reference())
        .build()
        .unwrap_err();
    assert_eq!(
        provenance_denial,
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );
}

#[test]
fn capability_marker_identity_survives_a_rust_module_move() {
    mod original_location {
        crate::worth_query_capability!(
            pub(super) Capability in super::Schema,
            identity "worth.query.test.module-move-capability.v1"
        );
    }
    mod moved_location {
        crate::worth_query_capability!(
            pub(super) Capability in super::Schema,
            identity "worth.query.test.module-move-capability.v1"
        );
    }

    assert_eq!(
        original_location::Capability::reference().marker_identity(),
        moved_location::Capability::reference().marker_identity()
    );
    assert_ne!(
        std::any::type_name::<original_location::Capability>(),
        std::any::type_name::<moved_location::Capability>()
    );
}

#[test]
fn capability_dimension_schema_identity_survives_rust_module_moves() {
    mod original_location {
        crate::worth_query_entity!(pub(super) Entity in super::Schema);
        crate::worth_query_capability_context!(
            pub(super) Context in super::Schema,
            identity "worth.query.test.moved-context.v1"
        );
        crate::worth_query_capability_context_entity_slot!(
            pub(super) Slot in super::Schema,
            Context => Entity,
            identity "worth.query.test.moved-slot.v1"
        );
        crate::worth_query_capability_provenance!(
            pub(super) Provenance in super::Schema,
            identity "worth.query.test.moved-provenance.v1"
        );
    }
    mod moved_location {
        crate::worth_query_entity!(pub(super) Entity in super::Schema);
        crate::worth_query_capability_context!(
            pub(super) Context in super::Schema,
            identity "worth.query.test.moved-context.v1"
        );
        crate::worth_query_capability_context_entity_slot!(
            pub(super) Slot in super::Schema,
            Context => Entity,
            identity "worth.query.test.moved-slot.v1"
        );
        crate::worth_query_capability_provenance!(
            pub(super) Provenance in super::Schema,
            identity "worth.query.test.moved-provenance.v1"
        );
    }

    let original = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(original_location::Entity::reference())
        .capability_context(original_location::Context::reference())
        .capability_context_entity_slot(original_location::Slot::reference())
        .capability_provenance(original_location::Provenance::reference())
        .build()
        .unwrap();
    let moved = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(moved_location::Entity::reference())
        .capability_context(moved_location::Context::reference())
        .capability_context_entity_slot(moved_location::Slot::reference())
        .capability_provenance(moved_location::Provenance::reference())
        .build()
        .unwrap();

    assert_eq!(original.identity(), moved.identity());
}

fn query_definition<Query>(
    reference: crate::application_query::ApplicationQueryReference<
        Schema,
        Query,
        QueryParameters,
        (),
        QueryEntity,
    >,
    entity: ApplicationEntityRef<Schema, QueryEntity>,
) -> Result<
    crate::application_query::ApplicationQueryDefinition<
        Schema,
        Query,
        QueryParameters,
        (),
        QueryEntity,
    >,
    crate::application_query::ApplicationQueryDefinitionDenial,
>
where
    Query: crate::application_query::ApplicationQueryMarkerIdentity,
{
    ApplicationQueryDefinitionBuilder::declare(reference)
        .root(entity)
        .scope(entity)
        .result_shape(
            ApplicationQueryResultShapeBuilder::<Schema, Query, QueryEntity, ()>::new(entity)
                .build(),
        )
        .cardinality(ApplicationQueryCardinality::ExactlyOne)
        .dependency_ceiling(ApplicationQueryDependencyCeiling::bounded(0, 0, 0))
        .disclosure(ApplicationQueryDisclosureContract::public())
        .basis_support(ApplicationQueryBasisSupport::current_and_pinned())
        .lanes(ApplicationQueryLaneEligibility::one_shot())
        .public()
        .build()
}
