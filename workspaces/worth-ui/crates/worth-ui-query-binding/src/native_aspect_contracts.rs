use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

pub(crate) const IDENTITY_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4901);
pub(crate) const MEASUREMENT_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4902);
const SIZE_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4903);
pub(crate) const QUERY_TEXT_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4904);
pub(crate) const QUERY_REVISION_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4905);
pub(crate) const COLLECTION_ITEM_ASPECT_IDENTITY: AspectIdentity = AspectIdentity(0x5755_4906);

pub fn worth_ui_native_aspect_contracts() -> [AspectContract; 6] {
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
        required_field_contract(
            "query_text",
            QUERY_TEXT_ASPECT_IDENTITY,
            "status",
            ScalarAspectType::String,
        ),
        required_field_contract(
            "query_revision",
            QUERY_REVISION_ASPECT_IDENTITY,
            "value",
            ScalarAspectType::UInt64,
        ),
        collection_item_contract(),
    ]
}

fn collection_item_contract() -> AspectContract {
    let fields = [
        FieldDeclaration::new(
            FieldKey::new("status").expect("static collection status field must admit"),
            ScalarAspectType::String,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("required collection status field law must be coherent"),
        FieldDeclaration::new(
            FieldKey::new("key").expect("static collection key field must admit"),
            ScalarAspectType::UInt64,
            FieldRequirement::Required,
            AbsenceLaw::Required,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
        .expect("required collection key field law must be coherent"),
    ];
    AspectContract::struct_aspect(
        AspectKey::new("collection_item").expect("static collection aspect key must admit"),
        COLLECTION_ITEM_ASPECT_IDENTITY,
        AspectContractRevision(1),
        StructAspectShape::new(fields).expect("collection item fields must be unique"),
    )
}

pub(crate) fn worth_ui_native_aspect_contract(key: &str) -> Option<AspectContract> {
    worth_ui_native_aspect_contracts()
        .into_iter()
        .find(|contract| contract.key().as_str() == key)
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
