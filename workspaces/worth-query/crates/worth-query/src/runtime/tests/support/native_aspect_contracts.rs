use worth_foundational::facade::{
    AbsenceLaw, AspectContract, AspectContractRevision, AspectEvolutionPolicy, AspectIdentity,
    AspectKey, FieldDeclaration, FieldKey, FieldRequirement, ScalarAspectType, StructAspectShape,
};

pub(in crate::runtime::tests) fn stateful_bridge_aspect_contracts() -> Vec<AspectContract> {
    let mut contracts = vec![string_struct_contract(
        "identity",
        0x5751_3001,
        "id",
        FieldRequirement::Required,
        AbsenceLaw::Required,
    )];
    contracts.extend(
        [
            ("title", 0x5751_3002, "value"),
            ("status", 0x5751_3003, "value"),
            ("summary", 0x5751_3004, "value"),
            ("description", 0x5751_3005, "value"),
            ("kind", 0x5751_3006, "value"),
            ("role", 0x5751_3007, "value"),
            ("position", 0x5751_3008, "ordinal"),
            ("profile", 0x5751_3010, "display_name"),
        ]
        .into_iter()
        .map(|(aspect, identity, field)| {
            string_struct_contract(
                aspect,
                identity,
                field,
                FieldRequirement::Optional,
                AbsenceLaw::Optional,
            )
        }),
    );
    contracts.extend([
        entity_reference_contract("source", 0x5751_3009, "id"),
        entity_reference_contract("target", 0x5751_300A, "id"),
        entity_reference_contract("vertex", 0x5751_300B, "id"),
        entity_reference_contract("face", 0x5751_300C, "id"),
        entity_reference_contract("half_edge", 0x5751_300D, "id"),
        entity_reference_contract("loop", 0x5751_300E, "id"),
        edge_contract(),
    ]);
    contracts
}

fn entity_reference_contract(aspect: &str, identity: u64, field: &str) -> AspectContract {
    struct_contract(aspect, identity, [(field, ScalarAspectType::EntityRef)])
}

fn edge_contract() -> AspectContract {
    struct_contract(
        "edge",
        0x5751_300F,
        [
            ("id", ScalarAspectType::EntityRef),
            ("kind", ScalarAspectType::String),
            ("source_identity", ScalarAspectType::EntityRef),
            ("target_identity", ScalarAspectType::EntityRef),
        ],
    )
}

fn string_struct_contract(
    aspect: &str,
    identity: u64,
    field: &str,
    requirement: FieldRequirement,
    absence: AbsenceLaw,
) -> AspectContract {
    let field = field_declaration(field, ScalarAspectType::String, requirement, absence);
    struct_contract_from_declarations(aspect, identity, [field])
}

fn struct_contract<const N: usize>(
    aspect: &str,
    identity: u64,
    fields: [(&str, ScalarAspectType); N],
) -> AspectContract {
    struct_contract_from_declarations(
        aspect,
        identity,
        fields.map(|(field, value_type)| {
            field_declaration(
                field,
                value_type,
                FieldRequirement::Optional,
                AbsenceLaw::Optional,
            )
        }),
    )
}

fn field_declaration(
    field: &str,
    value_type: ScalarAspectType,
    requirement: FieldRequirement,
    absence: AbsenceLaw,
) -> FieldDeclaration {
    FieldDeclaration::new(
        FieldKey::new(field).expect("test contract field must admit"),
        value_type,
        requirement,
        absence,
        AspectEvolutionPolicy::ExplicitBreakRequired,
    )
    .expect("test contract field law must be coherent")
}

fn struct_contract_from_declarations(
    aspect: &str,
    identity: u64,
    fields: impl IntoIterator<Item = FieldDeclaration>,
) -> AspectContract {
    AspectContract::struct_aspect(
        AspectKey::new(aspect).expect("test contract aspect must admit"),
        AspectIdentity(identity),
        AspectContractRevision(1),
        StructAspectShape::new(fields).expect("test contract fields must be unique"),
    )
}
