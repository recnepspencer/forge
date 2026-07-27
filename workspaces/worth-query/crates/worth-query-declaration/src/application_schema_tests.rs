use crate::facade::application_schema::{
    ApplicationAspectRef, ApplicationCurrencyMarker, ApplicationEntityRef, ApplicationFieldRef,
    ApplicationOperationRef, ApplicationSchema, ApplicationSchemaAuthoringContext,
    ApplicationSchemaAuthoringDenialKind, ApplicationSchemaBindingIdentity,
    ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder,
    ApplicationSchemaDeclarationDenial, DeclaredApplicationCurrency, EqualityPredicate,
    OperationCreates, ReadOnly, TypedApplicationValue, TypedCurrencyApplicationValue,
    TypedOperationBuilder,
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

impl OperationCreates<ProgramOperation> for ProgramEntity {}

impl ApplicationCurrencyMarker<Usd> for UsdCurrency {
    const NAME: &'static str = "UsdCurrency";
}

impl TypedApplicationValue for CurrencyValue {
    const SCALAR_FAMILY: ScalarAspectType = ScalarAspectType::Int64;

    fn into_foundational_value(self) -> AspectValue {
        AspectValue::Int64(0)
    }
}

impl TypedCurrencyApplicationValue for CurrencyValue {
    type Currency = Usd;
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
            .operation(program_operation())
            .build()
    }
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
    assert_eq!(denial, ApplicationSchemaDeclarationDenial::MissingCurrency);
}

#[test]
fn compile_capability_without_installed_operation_edge_is_denied() {
    let declaration = ProgramSchema::declaration().unwrap();
    let binding = ApplicationSchemaBindingIdentity::from_installed_parts(
        1,
        1,
        "package",
        declaration.identity().clone(),
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
        .operation(program_operation())
        .operation_create(program_operation(), program_entity())
        .build()
        .unwrap_err();
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency
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

fn currency_field() -> ApplicationFieldRef<
    Schema,
    Entity,
    Aspect,
    CurrencyField,
    CurrencyValue,
    ReadOnly,
    EqualityPredicate,
    DeclaredApplicationCurrency<UsdCurrency, Usd>,
> {
    ApplicationFieldRef::from_schema_identifiers("Entity", "Aspect", "CurrencyField")
}

fn program_entity() -> ApplicationEntityRef<ProgramSchema, ProgramEntity> {
    ApplicationEntityRef::from_schema_identifier("ProgramEntity")
}

fn program_operation() -> ApplicationOperationRef<ProgramSchema, ProgramOperation, ()> {
    ApplicationOperationRef::from_schema_identifier("ProgramOperation")
}
