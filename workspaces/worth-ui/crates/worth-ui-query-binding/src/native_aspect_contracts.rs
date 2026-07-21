use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

pub(crate) const IDENTITY_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4901);
pub(crate) const MEASUREMENT_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4902);
const SIZE_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4903);

pub fn worth_ui_native_aspect_contracts() -> [AspectContract; 3] {
    [
        required_field_contract(
            "identity",
            IDENTITY_ASPECT_IDENTITY,
            "id",
            ScalarAspectType::String,
        ),
        required_field_contract(
            "measurement",
            MEASUREMENT_ASPECT_IDENTITY,
            "value",
            ScalarAspectType::Float32,
        ),
        required_field_contract(
            "size",
            SIZE_ASPECT_IDENTITY,
            "value",
            ScalarAspectType::Float32,
        ),
    ]
}

fn required_field_contract(
    aspect: &'static str,
    identity: AspectIdentity,
    field: &'static str,
    family: ScalarAspectType,
) -> AspectContract {
    let field = FieldDeclaration::new(
        FieldKey::new(field).expect("static Worth UI aspect field must admit"),
        family,
        FieldRequirement::Required,
        AbsenceLaw::Required,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .expect("required Worth UI field law must be coherent");
    let shape = StructAspectShape::new([field]).expect("Worth UI aspect fields must be unique");
    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("static Worth UI aspect key must admit"),
        identity,
        AspectContractRevision(1),
        shape,
    )
}
