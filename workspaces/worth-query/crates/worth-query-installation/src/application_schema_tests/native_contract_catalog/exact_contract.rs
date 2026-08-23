use std::collections::BTreeSet;

use worth_foundational::facade::{
    aspects, canonical_basis_sequence_material, prepare_aspect_contract_for_canonical_basis,
    AspectContractRevision, AspectIdentity, AspectKey, CanonicalizationRuleVersion, FieldKey,
    ScalarAspectType,
};
use worth_query_declaration::facade::application_schema::{
    ApplicationSchema, ApplicationSchemaDeclaration, ApplicationSchemaDeclarationBuilder,
};

use super::{admitted_package, WorthQueryInstallationGeneration, WorthQueryInstalledPackageIndex};
use crate::facade::WorthQueryInstallationRuntimeIdentity;

struct ExactContractSchema;
worth_query_declaration::worth_query_entity!(ExactEntity in ExactContractSchema);
worth_query_declaration::worth_query_aspect!(
    ExactAspect in ExactContractSchema, ExactEntity;
    identity = AspectIdentity(0x9161_2100),
    revision = AspectContractRevision(7),
);
worth_query_declaration::worth_query_field!(
    RequiredCount in ExactContractSchema, ExactEntity, ExactAspect: u64, read_only, equality
);
worth_query_declaration::worth_query_field!(
    OptionalLabel in ExactContractSchema, ExactEntity, ExactAspect:
    optional String, read_only, no_equality
);
worth_query_declaration::worth_query_field!(
    RequiredActive in ExactContractSchema, ExactEntity, ExactAspect: bool, read_only, no_equality
);

impl ApplicationSchema for ExactContractSchema {
    const OWNER: &'static str = "native-contract-exact";
    const NAME: &'static str = "ExactContractSchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration() -> Result<
        ApplicationSchemaDeclaration<Self>,
        worth_query_declaration::facade::application_schema::ApplicationSchemaDeclarationDenial,
    > {
        ApplicationSchemaDeclarationBuilder::for_schema()
            .entity(ExactEntity::reference())
            .aspect(ExactEntity::reference(), ExactAspect::reference())
            .field(ExactEntity::reference(), RequiredCount::reference())
            .field(ExactEntity::reference(), OptionalLabel::reference())
            .field(ExactEntity::reference(), RequiredActive::reference())
            .build()
    }
}

#[test]
fn catalog_contract_equals_an_independently_authored_foundational_contract() {
    let declaration = ExactContractSchema::declaration().unwrap();
    let index = WorthQueryInstalledPackageIndex::build(
        WorthQueryInstallationRuntimeIdentity::fresh(),
        WorthQueryInstallationGeneration::initial(),
        [admitted_package(declaration.clone())],
    )
    .unwrap();
    let schema = index.bind_application_schema(declaration).unwrap();
    let installed = schema
        .native_contracts()
        .aspect("ExactEntity", "ExactAspect")
        .unwrap();
    let expected = expected_contract();

    assert_eq!(installed.contract(), &expected);
    assert_eq!(installed.contract().absence(), expected.absence());
    assert_eq!(installed.contract().evolution(), expected.evolution());
    assert_eq!(
        installed.fields().cloned().collect::<BTreeSet<_>>(),
        ["OptionalLabel", "RequiredActive", "RequiredCount"]
            .map(|field| FieldKey::new(field).unwrap())
            .into_iter()
            .collect()
    );
    let expected_basis = prepare_aspect_contract_for_canonical_basis(
        CanonicalizationRuleVersion::new("worth-query-native-contract-v1").unwrap(),
        expected,
    )
    .into_result()
    .unwrap();
    let expected_material = canonical_basis_sequence_material(expected_basis.payload());
    assert_eq!(installed.canonical_contract_basis(), &expected_basis);
    assert_eq!(installed.canonical_contract_material(), expected_material);
}

fn expected_contract() -> worth_foundational::facade::AspectContract {
    let shape = aspects()
        .struct_fields()
        .optional("OptionalLabel", ScalarAspectType::String)
        .required("RequiredActive", ScalarAspectType::Bool)
        .required("RequiredCount", ScalarAspectType::UInt64)
        .finish()
        .unwrap();
    aspects()
        .contract()
        .for_key(AspectKey::new("ExactAspect").unwrap())
        .identified_by(AspectIdentity(0x9161_2100))
        .at_revision(AspectContractRevision(7))
        .struct_aspect(shape)
}
