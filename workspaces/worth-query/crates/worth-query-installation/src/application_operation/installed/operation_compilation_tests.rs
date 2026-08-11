//! Hostile evidence for whole-candidate operation compilation.

use worth_foundational::facade::CanonicalDigestId;
use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredPreImageLocus, DeclaredRecordedInverse,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationAspectMarkerIdentity, ApplicationEntityMarkerIdentity,
    ApplicationFieldMarkerIdentity, ApplicationFieldRef, ApplicationOperationDecisionReadTarget,
    ApplicationOperationProgramTarget, ApplicationOperationRef, ApplicationSchema,
    ApplicationSchemaBindingIdentity, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaMember,
};

use super::operation_compilation::WorthQueryApplicationOperationCompilation;
use crate::application_operation::{
    WorthQueryApplicationOperationInstallationDenial,
    WorthQueryCompiledApplicationOperationContracts,
};

struct Schema;
struct Entity;
struct Aspect;
struct Field;

impl ApplicationSchema for Schema {
    const OWNER: &'static str = "worth-query-installation-tests";
    const NAME: &'static str = "OperationCompilationFixture";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        ApplicationSchemaDeclarationBuilder::<Self>::for_schema().build()
    }
}

impl ApplicationEntityMarkerIdentity for Entity {
    type Schema = Schema;
    const IDENTIFIER: &'static str = "Account";
}

impl ApplicationAspectMarkerIdentity for Aspect {
    type Schema = Schema;
    type Entity = Entity;
    const IDENTIFIER: &'static str = "State";
}

impl ApplicationFieldMarkerIdentity for Field {
    type Schema = Schema;
    type Entity = Entity;
    type Aspect = Aspect;
    const IDENTIFIER: &'static str = "balance";
}

#[test]
fn initial_installation_cannot_borrow_a_sibling_operations_exact_read() {
    let denied = compile(&members("sibling", "freeze", 64));
    assert!(
        denied.is_err(),
        "the exact demanded read belongs to sibling, not freeze"
    );

    let installed = compile(&members("freeze", "freeze", 64))
        .expect("moving that read onto the candidate operation installs");
    assert!(installed.aftermath().is_some());
}

#[test]
fn reinstallation_recompiles_only_the_presented_candidates_aftermath() {
    let baseline = compile(&members("freeze", "freeze", 64)).unwrap();
    let sibling_owned = compile(&members("freeze", "sibling", 64)).unwrap();
    assert!(sibling_owned.aftermath().is_none());
    assert_ne!(baseline, sibling_owned);

    let bound_drift = compile(&members("freeze", "freeze", 65)).unwrap();
    assert_ne!(baseline, bound_drift);
}

fn compile(
    members: &[ApplicationSchemaMember],
) -> Result<
    WorthQueryCompiledApplicationOperationContracts,
    WorthQueryApplicationOperationInstallationDenial,
> {
    WorthQueryApplicationOperationCompilation::resolve(
        ApplicationSchemaBindingIdentity::from_installed_parts(
            1,
            1,
            CanonicalDigestId::new([1; 32]),
            CanonicalDigestId::new([2; 32]),
        ),
        members,
        "freeze",
        "FixtureInput",
    )?
    .compile_contracts(Vec::new())
}

fn members(
    read_owner: &str,
    aftermath_owner: &'static str,
    bound: usize,
) -> Vec<ApplicationSchemaMember> {
    vec![
        operation("freeze"),
        operation("sibling"),
        ApplicationSchemaMember::OperationProgram {
            operation: "freeze".to_owned(),
            target: ApplicationOperationProgramTarget::Create {
                entity: "Audit".to_owned(),
            },
        },
        ApplicationSchemaMember::OperationDecisionRead {
            operation: read_owner.to_owned(),
            target: ApplicationOperationDecisionReadTarget::Field {
                entity: "Account".to_owned(),
                aspect: "State".to_owned(),
                field: "balance".to_owned(),
            },
        },
        ApplicationSchemaMember::OperationDecisionFactBudget {
            operation: "freeze".to_owned(),
            maximum_fact_count: 4,
        },
        ApplicationSchemaMember::OperationProjectionWorkBudget {
            operation: "freeze".to_owned(),
            maximum_work_units: 16,
        },
        recorded_inverse_member(aftermath_owner, bound),
    ]
}

fn operation(operation: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::Operation {
        operation: operation.to_owned(),
        input_type: "FixtureInput".to_owned(),
    }
}

fn recorded_inverse_member(operation: &'static str, bound: usize) -> ApplicationSchemaMember {
    let field = ApplicationFieldRef::<Schema, Entity, Aspect, Field, u64>::from_schema_types();
    let inverse = DeclaredRecordedInverse::new(
        "unfreeze",
        DeclaredLoweringCorrespondenceRef::new("inverse").unwrap(),
        DeclaredAftermathPostcondition::ExactPriorTruth,
        DeclaredPreImageDemand::new([DeclaredPreImageLocus::from_field(field)], bound).unwrap(),
    )
    .unwrap();
    let contract = DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(inverse),
    );
    let definition = ApplicationOperationRef::<Schema, (), ()>::from_schema_identifier(operation)
        .definition()
        .no_external_effect()
        .aftermath(contract)
        .finish();
    let declaration = ApplicationSchemaDeclarationBuilder::<Schema>::for_schema()
        .operation(definition)
        .build()
        .expect("the matching operation builder associates the aftermath");
    declaration
        .erased()
        .members()
        .iter()
        .find(|member| matches!(member, ApplicationSchemaMember::OperationAftermath { .. }))
        .expect("the matching operation emits its portable aftermath member")
        .clone()
}
