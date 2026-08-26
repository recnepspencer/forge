use super::*;

#[test]
fn capability_context_and_provenance_must_be_declared() {
    let mut without_context = members(contract(false, false, true));
    without_context.retain(|member| {
        !matches!(
            member,
            ApplicationSchemaMember::ApplicationCapabilityContext { .. }
        )
    });
    assert_eq!(
        build_from_members(without_context),
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityContextDependency)
    );

    let mut without_provenance = members(contract(false, false, true));
    without_provenance.retain(|member| {
        !matches!(
            member,
            ApplicationSchemaMember::ApplicationCapabilityProvenance { .. }
        )
    });
    assert_eq!(
        build_from_members(without_provenance),
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityProvenanceDependency)
    );
}

#[test]
fn capability_context_and_provenance_names_cannot_alias_marker_types() {
    let mut duplicate_context = members(contract(false, false, true));
    duplicate_context.push(ApplicationSchemaMember::ApplicationCapabilityContext {
        context: "Context".to_string(),
        context_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
            "foreign::Context",
        ),
    });
    assert_eq!(
        build_from_members(duplicate_context),
        Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityContext)
    );

    let mut duplicate_provenance = members(contract(false, false, true));
    duplicate_provenance.push(ApplicationSchemaMember::ApplicationCapabilityProvenance {
        provenance: "Provenance".to_string(),
        provenance_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
            "foreign::Provenance",
        ),
    });
    assert_eq!(
        build_from_members(duplicate_provenance),
        Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityProvenance)
    );
}

#[test]
fn capability_context_slots_require_declared_context_and_unique_identity() {
    let mut duplicate_slot = members(contract(false, false, true));
    duplicate_slot.push(
        ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
            context: "Context".to_string(),
            context_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "Context",
            ),
            slot: "ResourceSlot".to_string(),
            slot_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "foreign::ResourceSlot",
            ),
            entity: "Resource".to_string(),
        },
    );
    assert_eq!(
        build_from_members(duplicate_slot),
        Err(ApplicationSchemaDeclarationDenial::DuplicateApplicationCapabilityContextSlot)
    );

    let mut foreign_context_slot = members(contract(false, false, true));
    foreign_context_slot.push(
        ApplicationSchemaMember::ApplicationCapabilityContextEntitySlot {
            context: "ForeignContext".to_string(),
            context_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "foreign::Context",
            ),
            slot: "ForeignSlot".to_string(),
            slot_type: crate::portable_identity::WorthQueryPortableTypeIdentity::declared(
                "foreign::Slot",
            ),
            entity: "Resource".to_string(),
        },
    );
    assert_eq!(
        build_from_members(foreign_context_slot),
        Err(ApplicationSchemaDeclarationDenial::MissingApplicationCapabilityContextDependency)
    );
}
