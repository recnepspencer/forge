//! R8.2: pre-image demand must be covered by the operation's own decision reads.

use worth_query_declaration::facade::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract,
    DeclaredCorrectionMechanism, DeclaredLoweringCorrespondenceRef, DeclaredPreImageDemand,
    DeclaredRecordedInverse,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationOperationRef, ApplicationSchema, ApplicationSchemaDeclaration,
};

use super::*;

struct CoveredAftermathSchema;
struct UncoveredAftermathSchema;

impl ApplicationSchema for CoveredAftermathSchema {
    const OWNER: &'static str = "typed-test";
    const NAME: &'static str = "CoveredAftermathSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        aftermath_schema_members::<Self>("PrincipalIdentityField").build()
    }
}

impl ApplicationSchema for UncoveredAftermathSchema {
    const OWNER: &'static str = "typed-test";
    const NAME: &'static str = "UncoveredAftermathSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        aftermath_schema_members::<Self>("UncoveredSecretField").build()
    }
}

fn aftermath_schema_members<Schema>(
    preimage_field: &str,
) -> ApplicationSchemaDeclarationBuilder<Schema>
where
    Schema: ApplicationSchema,
{
    test_schema_members::<Schema>(Some(DeclaredApplicationAftermathContract::runtime_alone(
        DeclaredCorrectionMechanism::RecordedInverse(
            DeclaredRecordedInverse::new(
                "restore-prior",
                DeclaredLoweringCorrespondenceRef::new("test-inverse").unwrap(),
                DeclaredAftermathPostcondition::ExactPriorTruth,
                DeclaredPreImageDemand::new([preimage_field], 256).unwrap(),
            )
            .unwrap(),
        ),
    )))
}

fn index_for<Schema: ApplicationSchema>() -> WorthQueryInstalledPackageIndex {
    let package = WorthQueryPortableDomainPackage::new(WorthQueryPortableDomainIdentity::new(
        "typed-test",
        1,
        0,
    ))
    .application_schema(Schema::declaration().unwrap())
    .validate()
    .unwrap();
    let admitted = WorthQueryInstallationAdmissionProfile::new("support", "configuration")
        .admit(package)
        .unwrap();
    WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted],
    )
    .unwrap()
}

#[test]
fn covered_preimage_demand_installs_with_operation_decision_reads() {
    let index = index_for::<CoveredAftermathSchema>();
    let schema = index
        .bind_application_schema(CoveredAftermathSchema::declaration().unwrap())
        .unwrap();
    let installed = schema
        .installed_operation(ApplicationOperationRef::<
            CoveredAftermathSchema,
            TestOperation,
            TestInput,
        >::from_schema_identifier("TestOperation"))
        .expect("covered demand must install through operation compile");
    let aftermath = installed
        .contracts()
        .aftermath()
        .expect("aftermath compiles onto the operation");
    assert_eq!(
        aftermath.published_posture(),
        crate::facade::PublishedAftermathPosture::Reversible
    );
    let reads = installed.contracts().decision_reads();
    assert!(
        reads.iter().any(|read| {
            matches!(
                read,
                worth_query_declaration::facade::application_schema::ApplicationOperationDecisionReadTarget::Field {
                    field,
                    ..
                } if field == "PrincipalIdentityField"
            )
        }),
        "coverage must come from the operation's declared decision reads"
    );
}

#[test]
fn uncovered_preimage_demand_denies_operation_installation_by_name() {
    let index = index_for::<UncoveredAftermathSchema>();
    let schema = index
        .bind_application_schema(UncoveredAftermathSchema::declaration().unwrap())
        .unwrap();
    let denial = match schema.installed_operation(ApplicationOperationRef::<
        UncoveredAftermathSchema,
        TestOperation,
        TestInput,
    >::from_schema_identifier("TestOperation"))
    {
        Ok(_) => panic!("uncovered pre-image demand must deny operation installation"),
        Err(denial) => denial,
    };
    assert_eq!(
        denial.kind(),
        crate::facade::WorthQueryApplicationOperationInstallationDenialKind::AftermathInstallationDenied
    );
}
