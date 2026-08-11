use crate::application_aftermath::{
    DeclaredAftermathPostcondition, DeclaredApplicationAftermathContract, DeclaredCompensation,
    DeclaredCorrectionMechanism,
};

use super::operation_contract_cardinality::validate_operation_contract_cardinality;
use super::{
    ApplicationOperationRef, ApplicationSchema, ApplicationSchemaDeclaration,
    ApplicationSchemaDeclarationBuilder, ApplicationSchemaDeclarationDenial,
    ApplicationSchemaMember,
};

struct CardinalitySchema;
struct CardinalityOperation;

impl ApplicationSchema for CardinalitySchema {
    const OWNER: &'static str = "CardinalityOwner";
    const NAME: &'static str = "CardinalitySchema";
    const MAJOR: u32 = 1;
    const MINOR: u32 = 0;

    fn declaration(
    ) -> Result<ApplicationSchemaDeclaration<Self>, ApplicationSchemaDeclarationDenial> {
        ApplicationSchemaDeclarationBuilder::for_schema().build()
    }
}

#[test]
fn one_or_absent_operation_contracts_are_valid() {
    assert_eq!(validate_operation_contract_cardinality(&[]), Ok(()));
    assert_eq!(
        validate_operation_contract_cardinality(&[
            external_effect("Operation", "EffectA"),
            aftermath("Operation"),
            external_effect("OtherOperation", "EffectB"),
            aftermath("OtherOperation"),
        ]),
        Ok(())
    );
}

#[test]
fn different_external_effects_for_one_operation_have_a_semantic_denial() {
    let members = [
        external_effect("Operation", "EffectA"),
        external_effect("Operation", "EffectB"),
    ];
    assert_eq!(
        validate_operation_contract_cardinality(&members),
        Err(ApplicationSchemaDeclarationDenial::DuplicateOperationExternalEffect)
    );
}

#[test]
fn identical_external_effects_do_not_fall_through_to_generic_duplicate_member() {
    let member = external_effect("Operation", "EffectA");
    assert_eq!(
        validate_operation_contract_cardinality(&[member.clone(), member]),
        Err(ApplicationSchemaDeclarationDenial::DuplicateOperationExternalEffect)
    );
}

#[test]
fn aftermath_cardinality_has_its_own_semantic_denial() {
    assert_eq!(
        validate_operation_contract_cardinality(&[aftermath("Operation"), aftermath("Operation"),]),
        Err(ApplicationSchemaDeclarationDenial::DuplicateOperationAftermath)
    );
}

#[test]
fn corrupted_member_set_rejects_identical_external_effects_before_generic_duplicates() {
    let member = external_effect("Operation", "EffectA");
    let denial = ApplicationSchemaDeclarationBuilder::<CardinalitySchema>::for_schema()
        .push_member(member.clone())
        .push_member(member)
        .build()
        .expect_err("corrupted external-effect cardinality must deny before identity");
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::DuplicateOperationExternalEffect
    );
}

#[test]
fn corrupted_member_set_rejects_distinct_external_effects_before_identity() {
    let denial = ApplicationSchemaDeclarationBuilder::<CardinalitySchema>::for_schema()
        .push_member(external_effect("Operation", "EffectA"))
        .push_member(external_effect("Operation", "EffectB"))
        .build()
        .expect_err("different external effects for one operation must deny");
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::DuplicateOperationExternalEffect
    );
}

#[test]
fn corrupted_member_set_rejects_identical_aftermaths_before_generic_duplicates() {
    let member = aftermath("Operation");
    let denial = ApplicationSchemaDeclarationBuilder::<CardinalitySchema>::for_schema()
        .push_member(member.clone())
        .push_member(member)
        .build()
        .expect_err("corrupted aftermath cardinality must deny before identity");
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::DuplicateOperationAftermath
    );
}

#[test]
fn corrupted_member_set_rejects_distinct_aftermaths_before_identity() {
    let denial = ApplicationSchemaDeclarationBuilder::<CardinalitySchema>::for_schema()
        .push_member(aftermath("Operation"))
        .push_member(correctable_aftermath("Operation"))
        .build()
        .expect_err("different aftermath contracts for one operation must deny");
    assert_eq!(
        denial,
        ApplicationSchemaDeclarationDenial::DuplicateOperationAftermath
    );
}

fn external_effect(operation: &str, effect: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::OperationExternalEffect {
        operation: operation.to_owned(),
        effect: effect.to_owned(),
        rust_payload_type: "Payload".to_owned(),
        protocol: super::ApplicationExternalEffectProtocol::new(
            worth_foundational::facade::BoundaryProtocolIdentity::new("test.external-payload"),
            worth_foundational::facade::BoundaryProtocolVersion::new(1),
        ),
        maximum_payload_bytes: 64,
        correlation_family: "external-family".to_owned(),
    }
}

fn aftermath(operation: &'static str) -> ApplicationSchemaMember {
    aftermath_member(
        operation,
        DeclaredApplicationAftermathContract::<CardinalitySchema>::not_correctable(),
    )
}

fn correctable_aftermath(operation: &'static str) -> ApplicationSchemaMember {
    let compensation = DeclaredCompensation::new(
        "CompensateOperation",
        DeclaredAftermathPostcondition::InvariantRestored {
            invariant: "balance-restored".to_owned(),
        },
    )
    .expect("the compensation fixture is valid");
    aftermath_member(
        operation,
        DeclaredApplicationAftermathContract::<CardinalitySchema>::runtime_alone(
            DeclaredCorrectionMechanism::Compensation(compensation),
        ),
    )
}

fn aftermath_member(
    operation: &'static str,
    contract: DeclaredApplicationAftermathContract<CardinalitySchema>,
) -> ApplicationSchemaMember {
    let definition = ApplicationOperationRef::<CardinalitySchema, CardinalityOperation, ()>::
        from_schema_identifier(operation)
        .definition()
        .no_external_effect()
        .aftermath(contract)
        .finish();
    let declaration = ApplicationSchemaDeclarationBuilder::<CardinalitySchema>::for_schema()
        .operation(definition)
        .build()
        .expect("the owner builder accepts one aftermath");
    declaration
        .erased()
        .members()
        .iter()
        .find(|member| matches!(member, ApplicationSchemaMember::OperationAftermath { .. }))
        .expect("the owner builder emits the portable aftermath member")
        .clone()
}
