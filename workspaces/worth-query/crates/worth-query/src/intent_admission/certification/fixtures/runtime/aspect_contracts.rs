use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

pub(super) fn certification_aspect_contracts() -> Vec<AspectContract> {
    vec![
        string_contract(
            "identity",
            0x5751_4101,
            "id",
            FieldRequirement::Required,
            AbsenceLaw::Required,
        ),
        string_contract(
            "title",
            0x5751_4102,
            "value",
            FieldRequirement::Optional,
            AbsenceLaw::Optional,
        ),
        string_contract(
            "status",
            0x5751_4103,
            "value",
            FieldRequirement::Optional,
            AbsenceLaw::Optional,
        ),
    ]
}

fn string_contract(
    aspect: &str,
    identity: u64,
    field: &str,
    requirement: FieldRequirement,
    absence: AbsenceLaw,
) -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new(field).expect("certification field must admit"),
        ScalarAspectType::String,
        requirement,
        absence,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .expect("certification field law must be coherent");

    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("certification aspect must admit"),
        AspectIdentity(identity),
        AspectContractRevision(1),
        StructAspectShape::new([field]).expect("certification field must be unique"),
    )
}
