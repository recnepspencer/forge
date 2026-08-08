use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};
use worth_query_declaration::facade::application_aftermath::DeclaredApplicationAftermathContract;
use worth_query_declaration::facade::application_schema::{
    ApplicationExternalEffectProtocol, ApplicationSchemaMember,
};

use super::{
    operation_aftermath, operation_external_effect, WorthQueryOperationContractCardinalityDenial,
};

#[test]
fn external_effect_resolution_rejects_ambiguity_instead_of_selecting_first() {
    let members = [
        external_effect("Operation", "EffectA"),
        external_effect("Operation", "EffectB"),
    ];
    assert_eq!(
        operation_external_effect(&members, "Operation"),
        Err(WorthQueryOperationContractCardinalityDenial::AmbiguousExternalEffect)
    );
}

#[test]
fn aftermath_resolution_rejects_ambiguity_instead_of_selecting_first() {
    let members = [aftermath("Operation"), aftermath("Operation")];
    assert_eq!(
        operation_aftermath(&members, "Operation"),
        Err(WorthQueryOperationContractCardinalityDenial::AmbiguousAftermath)
    );
}

#[test]
fn unrelated_operation_contracts_do_not_create_false_ambiguity() {
    let members = [
        external_effect("Operation", "EffectA"),
        external_effect("OtherOperation", "EffectB"),
        aftermath("Operation"),
        aftermath("OtherOperation"),
    ];
    assert!(operation_external_effect(&members, "Operation").is_ok());
    assert!(matches!(
        operation_aftermath(&members, "Operation"),
        Ok(Some(_))
    ));
}

fn external_effect(operation: &str, effect: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::OperationExternalEffect {
        operation: operation.to_owned(),
        effect: effect.to_owned(),
        rust_payload_type: "Payload".to_owned(),
        protocol: ApplicationExternalEffectProtocol::new(
            BoundaryProtocolIdentity::new("test.external-payload"),
            BoundaryProtocolVersion::new(1),
        ),
        maximum_payload_bytes: 64,
        correlation_family: "external-family".to_owned(),
    }
}

fn aftermath(operation: &str) -> ApplicationSchemaMember {
    ApplicationSchemaMember::OperationAftermath {
        operation: operation.to_owned(),
        contract: DeclaredApplicationAftermathContract::not_correctable(),
    }
}
