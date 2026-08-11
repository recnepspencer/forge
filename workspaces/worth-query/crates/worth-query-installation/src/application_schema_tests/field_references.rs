//! Schema-owned marker family for the installation schema fixtures.

use std::marker::PhantomData;

use worth_query_declaration::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldPresence, DeclaredApplicationFieldValue,
    OperationCreates, OperationExpectsFact, OperationReads,
};
use worth_query_declaration::facade::authentication::{
    WorthQueryExternalPrincipalIdentity, WorthQueryPrincipalMappingStatus,
};

use super::{TestOperation, TestSchema};

pub(super) struct FixtureEntity<Schema>(PhantomData<fn() -> Schema>);
pub(super) struct FixtureIdentityAspect<Schema>(PhantomData<fn() -> Schema>);
pub(super) struct FixtureExternalIdentityField<Schema>(PhantomData<fn() -> Schema>);
pub(super) struct FixtureMappingStatusField<Schema>(PhantomData<fn() -> Schema>);
pub(super) struct FixturePrincipalIdentityField<Schema>(PhantomData<fn() -> Schema>);

pub(super) type TestEntity = FixtureEntity<TestSchema>;

impl<Schema> ApplicationEntityMarkerIdentity for FixtureEntity<Schema> {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "TestEntity";
}

impl<Schema> ApplicationAspectMarkerIdentity for FixtureIdentityAspect<Schema> {
    type Schema = Schema;
    type Entity = FixtureEntity<Schema>;
    const IDENTIFIER: &'static str = "IdentityAspect";
}

macro_rules! field_marker_identity {
    ($marker:ident, $identifier:literal) => {
        impl<Schema> ApplicationFieldMarkerIdentity for $marker<Schema> {
            type Schema = Schema;
            type Entity = FixtureEntity<Schema>;
            type Aspect = FixtureIdentityAspect<Schema>;
            const IDENTIFIER: &'static str = $identifier;
        }
    };
}

field_marker_identity!(FixtureExternalIdentityField, "ExternalIdentityField");
field_marker_identity!(FixtureMappingStatusField, "MappingStatusField");
field_marker_identity!(FixturePrincipalIdentityField, "PrincipalIdentityField");

macro_rules! required_field {
    ($field:ident, $value:ty) => {
        impl<Schema> DeclaredApplicationFieldValue for $field<Schema> {
            type Value = $value;
            const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
        }
    };
}

required_field!(
    FixtureExternalIdentityField,
    WorthQueryExternalPrincipalIdentity
);
required_field!(FixtureMappingStatusField, WorthQueryPrincipalMappingStatus);
required_field!(FixturePrincipalIdentityField, u64);

impl<Schema> OperationCreates<TestOperation> for FixtureEntity<Schema> {}
impl<Schema> OperationReads<TestOperation> for FixturePrincipalIdentityField<Schema> {}
impl<Schema> OperationExpectsFact<TestOperation> for FixturePrincipalIdentityField<Schema> {}
