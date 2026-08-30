use worth_foundational::facade::{BoundaryProtocolIdentity, BoundaryProtocolVersion};

use super::{
    member_closure::validate_member_closure, ApplicationExternalEffectProtocol,
    ApplicationOperationProgramTarget, ApplicationSchemaDeclarationDenial, ApplicationSchemaMember,
    WorthQueryExternalEffectCorrelationFamily,
};

#[test]
fn external_effect_requires_exact_declared_effect_payload_and_emit_target() {
    assert_eq!(validate_member_closure(&closed_members()), Ok(()));

    for removal in [1_usize, 2] {
        let mut members = closed_members();
        members.remove(removal);
        assert_eq!(
            validate_member_closure(&members),
            Err(ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency)
        );
    }

    let mut wrong_payload = closed_members();
    let ApplicationSchemaMember::Effect { payload_type, .. } = &mut wrong_payload[1] else {
        unreachable!("fixture effect moved")
    };
    *payload_type =
        crate::portable_identity::WorthQueryPortableTypeIdentity::declared("WrongPayload");
    assert_eq!(
        validate_member_closure(&wrong_payload),
        Err(ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency)
    );
}

#[test]
fn external_effect_rejects_a_zero_wire_bound() {
    let mut members = closed_members();
    let ApplicationSchemaMember::OperationExternalEffect {
        maximum_payload_bytes,
        ..
    } = &mut members[3]
    else {
        unreachable!("fixture external contract moved")
    };
    *maximum_payload_bytes = 0;

    assert_eq!(
        validate_member_closure(&members),
        Err(ApplicationSchemaDeclarationDenial::MissingOperationProgramDependency)
    );
}

fn closed_members() -> Vec<ApplicationSchemaMember> {
    vec![
        ApplicationSchemaMember::Operation {
            operation: "Operation".to_owned(),
            input_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared("Input"),
        },
        ApplicationSchemaMember::Effect {
            effect: "ExternalEffect".to_owned(),
            payload_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "Payload",
            ),
        },
        ApplicationSchemaMember::OperationProgram {
            operation: "Operation".to_owned(),
            target: ApplicationOperationProgramTarget::Emit {
                effect: "ExternalEffect".to_owned(),
            },
        },
        ApplicationSchemaMember::OperationExternalEffect {
            operation: "Operation".to_owned(),
            effect: "ExternalEffect".to_owned(),
            rust_payload_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "Payload",
            ),
            protocol: ApplicationExternalEffectProtocol::new(
                BoundaryProtocolIdentity::new("test.external-payload"),
                BoundaryProtocolVersion::new(1),
            ),
            maximum_payload_bytes: 64,
            correlation_family: WorthQueryExternalEffectCorrelationFamily::new("external-family")
                .unwrap(),
        },
    ]
}
