use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

const IDENTITY_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x4841_4401);
const COLORABILITY_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x4841_4402);

pub fn hadwiger_native_aspect_contracts() -> [AspectContract; 2] {
    [
        required_string_field_contract("identity", IDENTITY_ASPECT_IDENTITY, "id"),
        required_string_field_contract("colorability", COLORABILITY_ASPECT_IDENTITY, "lower_bound"),
    ]
}

fn required_string_field_contract(
    aspect: &'static str,
    identity: AspectIdentity,
    field: &'static str,
) -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new(field).expect("static Hadwiger aspect field must admit"),
        ScalarAspectType::String,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .expect("required Hadwiger field law must be coherent");
    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("static Hadwiger aspect key must admit"),
        identity,
        AspectContractRevision(1),
        StructAspectShape::new([field]).expect("Hadwiger aspect fields must be unique"),
    )
}
