use crate::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationAspectRef, ApplicationEntityRef,
    ApplicationFieldPresence, ApplicationFieldRef, ApplicationOperationRef, ApplicationSchema,
    ApplicationSchemaAuthoringContext, ApplicationSchemaAuthoringDenialKind,
    ApplicationSchemaBindingIdentity, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaDeclarationDenial, ApplicationUnitMarker,
    DeclaredApplicationFieldValue, DeclaredApplicationUnit, EqualityPredicate, OperationCreates,
    OperationExpectsFact, ReadOnly, TypedApplicationValue, TypedOperationBuilder,
    TypedUnitApplicationValue,
};
use worth_foundational::facade::{AspectValue, ScalarAspectType};

struct Schema;
struct Entity;
struct Aspect;
struct Field;
struct CurrencyField;
struct Usd;
struct UsdCurrency;
struct CurrencyValue;
struct ProgramSchema;
struct ProgramEntity;
struct ProgramOperation;
struct SchemaOperation;
struct SchemaInput;
struct NamespacedSchema;
struct InvalidOwnerSchema;
struct DottedMemberSchema;
struct IdentifierEntity;

impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = Schema;
    type Entity = Entity;

    const IDENTIFIER: &'static str = "Aspect";
    const ASPECT_IDENTITY: worth_foundational::facade::AspectIdentity =
        worth_foundational::facade::AspectIdentity(0x9161_2301);
    const CONTRACT_REVISION: worth_foundational::facade::AspectContractRevision =
        worth_foundational::facade::AspectContractRevision(2);
}

impl DeclaredApplicationFieldValue for Field {
    type Value = u64;
    const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
}

impl DeclaredApplicationFieldValue for CurrencyField {
    type Value = CurrencyValue;
    const PRESENCE: ApplicationFieldPresence = ApplicationFieldPresence::Required;
}

crate::worth_query_application_schema! {
    schema MacroNamespacedSchema {
        owner: "bank.world",
        version: (1, 0),
        members: |schema| {
            schema.entity(MacroNamespacedEntity::reference())
        }
    }
}

crate::worth_query_entity!(MacroNamespacedEntity in MacroNamespacedSchema);

impl OperationCreates<ProgramOperation> for ProgramEntity {}
impl OperationExpectsFact<SchemaOperation> for Field {}

#[test]
fn application_schema_macro_accepts_the_canonical_namespace_qualified_owner() {
    let declaration = MacroNamespacedSchema::declaration().unwrap();
    assert_eq!(declaration.erased().owner(), "bank.world");
}

impl ApplicationUnitMarker<Usd> for UsdCurrency {
    const NAME: &'static str = "UsdCurrency";
}

impl TypedApplicationValue for CurrencyValue {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(0)
    }
}

impl TypedUnitApplicationValue for CurrencyValue {
    type Unit = Usd;
}

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "schema-test";
    const NAME: &'static str = "Schema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        declaration(false)
    }
}

impl ApplicationSchema for ProgramSchema {
    const OWNER: &'static str = "schema-test";
    const NAME: &'static str = "ProgramSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(program_entity())
            .operation(
                program_operation()
                    .definition()
                    .no_external_effect()
                    .no_aftermath()
                    .finish(),
            )
            .build()
    }
}

impl ApplicationSchema for NamespacedSchema {
    const OWNER: &'static str = "bank.world";
    const NAME: &'static str = "NamespacedSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(
                ApplicationEntityRef::<Self, IdentifierEntity>::from_schema_identifier(
                    "IdentifierEntity",
                ),
            )
            .build()
    }
}

impl ApplicationSchema for InvalidOwnerSchema {
    const OWNER: &'static str = "bank..world";
    const NAME: &'static str = "InvalidOwnerSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(
                ApplicationEntityRef::<Self, IdentifierEntity>::from_schema_identifier(
                    "IdentifierEntity",
                ),
            )
            .build()
    }
}

impl ApplicationSchema for DottedMemberSchema {
    const OWNER: &'static str = "bank.world";
    const NAME: &'static str = "DottedMemberSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema()
            .entity(
                ApplicationEntityRef::<Self, IdentifierEntity>::from_schema_identifier(
                    "Identifier.Entity",
                ),
            )
            .build()
    }
}

#[test]
fn namespace_qualified_owner_is_valid_but_empty_segments_and_dotted_members_are_not() {
    NamespacedSchema::declaration().unwrap();
    assert_eq!(
        InvalidOwnerSchema::declaration().unwrap_err(),
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );
    assert_eq!(
        DottedMemberSchema::declaration().unwrap_err(),
        ApplicationSchemaDeclarationDenial::InvalidIdentifier
    );
}

#[test]
fn application_schema_identity_converges_across_member_order() {
    let first = declaration(false).unwrap();
    let second = declaration(true).unwrap();
    assert_eq!(first.identity(), second.identity());
    assert_eq!(first.erased().members(), second.erased().members());
}

#[test]
fn field_without_its_exact_aspect_is_denied() {
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(entity())
        .field(entity(), field())
        .build()
        .unwrap_err();
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::MissingAspect);
}

#[test]
fn currency_field_without_its_declared_currency_is_denied() {
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(entity())
        .aspect(entity(), aspect())
        .field(entity(), currency_field())
        .build()
        .unwrap_err();
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::MissingUnit);
}

#[test]
fn compile_capability_without_installed_operation_edge_is_denied() {
    let declaration = ProgramSchema::declaration().unwrap();
    let binding = ApplicationSchemaBindingIdentity::from_installed_parts(
        1,
        1,
        worth_foundational::facade::CanonicalDigestId::new([1; 32]),
        worth_foundational::facade::CanonicalDigestId::new([2; 32]),
    );
    let context = ApplicationSchemaAuthoringContext::from_installed_declaration(
        binding,
        declaration.erased(),
    );
    let denial = TypedOperationBuilder::new(program_operation())
        .with_installed_context(context)
        .input(())
        .create(program_entity())
        .build()
        .unwrap_err();
    assert_eq!(
        denial.kind(),
        ApplicationSchemaAuthoringDenialKind::OperationProgramNotInstalled
    );
}

#[test]
fn operation_program_with_missing_target_member_is_denied() {
    let denial = ApplicationSchemaDeclarationBuilder::<ProgramSchema>::for_schema()
        .operation(
            program_operation()
                .definition()
                .no_external_effect()
                .no_aftermath()
                .finish(),
        )
        .operation_create(program_operation(), program_entity())
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency
    );
}

#[test]
fn mutation_precondition_without_the_exact_decision_read_is_denied() {
    let denial = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .entity(entity())
        .aspect(entity(), aspect())
        .field(entity(), field())
        .operation(
            schema_operation()
                .definition()
                .no_external_effect()
                .no_aftermath()
                .finish(),
        )
        .operation_expected_fact(schema_operation(), field())
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingOperationMutationPreconditionDependency
    );
}

fn declaration(
    reverse: bool,
) -> Result<ApplicationSchemaDeclaration<Schema>, ApplicationSchemaDeclarationDenial> {
    let builder = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema();
    if reverse {
        builder
            .field(entity(), field())
            .aspect(entity(), aspect())
            .entity(entity())
            .build()
    } else {
        builder
            .entity(entity())
            .aspect(entity(), aspect())
            .field(entity(), field())
            .build()
    }
}

fn entity() -> ApplicationEntityRef<Schema, Entity> {
    ApplicationEntityRef::from_schema_identifier("Entity")
}

fn aspect() -> ApplicationAspectRef<Schema, Entity, Aspect> {
    ApplicationAspectRef::from_schema_identifier("Aspect")
}

fn field() -> ApplicationFieldRef<Schema, Entity, Aspect, Field, u64, ReadOnly, EqualityPredicate> {
    ApplicationFieldRef::from_schema_identifiers("Entity", "Aspect", "Field")
}

fn schema_operation() -> ApplicationOperationRef<Schema, SchemaOperation, SchemaInput> {
    ApplicationOperationRef::from_schema_identifier("SchemaOperation")
}

fn currency_field() -> ApplicationFieldRef<
    Schema,
    Entity,
    Aspect,
    CurrencyField,
    CurrencyValue,
    ReadOnly,
    EqualityPredicate,
    DeclaredApplicationUnit<UsdCurrency, Usd>,
> {
    ApplicationFieldRef::from_schema_identifiers("Entity", "Aspect", "CurrencyField")
}

fn program_entity() -> ApplicationEntityRef<ProgramSchema, ProgramEntity> {
    ApplicationEntityRef::from_schema_identifier("ProgramEntity")
}

fn program_operation() -> ApplicationOperationRef<ProgramSchema, ProgramOperation, ()> {
    ApplicationOperationRef::from_schema_identifier("ProgramOperation")
}
