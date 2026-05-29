use crate::merge::data::{
    AspectMergePolicyDeclaration, AspectMergePolicyKind, IdentityBasisDeclaration,
    IdentityBasisKind, IdentityBasisScope,
};
use crate::publication::patch::data::AspectKey;
use crate::schema::data::{AspectBinding, AspectPlanRevision, DeclaredAspect};
use forge_foundational::{
    AspectShape, FieldDeclaration, FieldKey, ReferenceAspectType, ScalarAspectType,
};

pub(super) fn derive_plan_revision(
    aspects: &[DeclaredAspect],
    identity_declarations: &[IdentityBasisDeclaration],
    merge_policy_declarations: &[AspectMergePolicyDeclaration],
) -> AspectPlanRevision {
    const FNV_OFFSET: u128 = 0x6c62272e07bb014262b821756295c58d;
    const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;

    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }

    fn mix_string(hash: &mut u128, value: &str) {
        mix_bytes(hash, value.as_bytes());
    }

    fn mix_aspect_key(hash: &mut u128, value: &AspectKey) {
        mix_bytes(hash, b"foundational");
        mix_string(hash, value.as_str());
    }

    fn mix_field_key(hash: &mut u128, value: &FieldKey) {
        mix_bytes(hash, b"field_key");
        mix_string(hash, value.as_str());
    }

    let mut hash = FNV_OFFSET;
    for aspect in aspects {
        mix_aspect_key(&mut hash, &aspect.aspect_key());
        match &aspect.binding {
            AspectBinding::EntityField { field } => {
                mix_bytes(&mut hash, b"entity_field");
                mix_field_key(&mut hash, field);
            }
            AspectBinding::RelationField { field } => {
                mix_bytes(&mut hash, b"relation_field");
                mix_field_key(&mut hash, field);
            }
            AspectBinding::RelationSourceEndpoint => mix_bytes(&mut hash, b"source_endpoint"),
            AspectBinding::RelationTargetEndpoint => mix_bytes(&mut hash, b"target_endpoint"),
            AspectBinding::LifecycleTransition => mix_bytes(&mut hash, b"lifecycle"),
        }
        mix_foundational_contract(&mut hash, &aspect.contract);
    }
    for declaration in identity_declarations {
        match &declaration.scope {
            IdentityBasisScope::EntityKind(kind_id) => {
                mix_bytes(&mut hash, b"identity_scope_entity_kind");
                mix_bytes(&mut hash, &kind_id.0.to_le_bytes());
            }
            IdentityBasisScope::RelationKind(kind_id) => {
                mix_bytes(&mut hash, b"identity_scope_relation_kind");
                mix_bytes(&mut hash, &kind_id.0.to_le_bytes());
            }
            IdentityBasisScope::AspectKey(aspect_key) => {
                mix_bytes(&mut hash, b"identity_scope_aspect_key");
                mix_aspect_key(&mut hash, aspect_key);
            }
        }
        match &declaration.basis {
            IdentityBasisKind::StorageIdentity => mix_bytes(&mut hash, b"identity_basis_storage"),
            IdentityBasisKind::LineageIdentity => mix_bytes(&mut hash, b"identity_basis_lineage"),
            IdentityBasisKind::StructuralFingerprint => {
                mix_bytes(&mut hash, b"identity_basis_structural")
            }
            IdentityBasisKind::DeclaredKeySet(keys) => {
                mix_bytes(&mut hash, b"identity_basis_declared_key_set");
                for key in keys.iter() {
                    mix_aspect_key(&mut hash, key);
                }
            }
            IdentityBasisKind::Custom(custom) => {
                mix_bytes(&mut hash, b"identity_basis_custom");
                mix_bytes(&mut hash, custom.name.as_bytes());
                mix_bytes(&mut hash, &custom.semantic_version.to_le_bytes());
            }
        }
    }
    for declaration in merge_policy_declarations {
        mix_bytes(&mut hash, b"merge_policy_declaration");
        mix_aspect_key(&mut hash, &declaration.aspect_key);
        match &declaration.policy {
            AspectMergePolicyKind::FailOnConflict => mix_bytes(&mut hash, b"merge_policy_fail"),
            AspectMergePolicyKind::LastWriterWins => mix_bytes(&mut hash, b"merge_policy_lww"),
            AspectMergePolicyKind::MonotonicCounter => {
                mix_bytes(&mut hash, b"merge_policy_monotonic_counter")
            }
            AspectMergePolicyKind::AdditiveSet => {
                mix_bytes(&mut hash, b"merge_policy_additive_set")
            }
            AspectMergePolicyKind::PreferRicher => {
                mix_bytes(&mut hash, b"merge_policy_prefer_richer")
            }
            AspectMergePolicyKind::Custom(custom) => {
                mix_bytes(&mut hash, b"merge_policy_custom");
                mix_bytes(&mut hash, custom.name.as_bytes());
                mix_bytes(&mut hash, &custom.semantic_version.to_le_bytes());
            }
        }
    }
    AspectPlanRevision(hash)
}

fn mix_foundational_contract(hash: &mut u128, contract: &forge_foundational::AspectContract) {
    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }

    mix_bytes(hash, b"contract");
    mix_bytes(hash, contract.key().as_str().as_bytes());
    mix_bytes(hash, &contract.identity().0.to_le_bytes());
    mix_bytes(hash, &contract.revision().0.to_le_bytes());
    mix_bytes(
        hash,
        match contract.absence() {
            forge_foundational::AbsenceLaw::Required => b"required",
            forge_foundational::AbsenceLaw::Optional => b"optional",
            forge_foundational::AbsenceLaw::Defaulted => b"defaulted",
        },
    );
    mix_bytes(
        hash,
        match contract.equivalence() {
            forge_foundational::AspectEquivalenceBasis::ExactCanonicalValue => b"exact",
            forge_foundational::AspectEquivalenceBasis::DeclaredStructFields => b"declared_fields",
            forge_foundational::AspectEquivalenceBasis::OpaqueIdentity => b"opaque_identity",
            forge_foundational::AspectEquivalenceBasis::ReferenceIdentity => b"reference_identity",
            forge_foundational::AspectEquivalenceBasis::ContentIdentity => b"content_identity",
        },
    );
    mix_bytes(
        hash,
        match contract.evolution() {
            forge_foundational::AspectEvolutionPolicy::Frozen => b"frozen",
            forge_foundational::AspectEvolutionPolicy::AdditiveFieldsAllowed => b"additive",
            forge_foundational::AspectEvolutionPolicy::WideningAllowed => b"widening",
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired => b"explicit_break",
        },
    );
    match contract.shape() {
        AspectShape::Scalar(scalar) => mix_bytes(hash, scalar_name(*scalar).as_bytes()),
        AspectShape::Reference(ReferenceAspectType::Entity) => mix_bytes(hash, b"reference_entity"),
        AspectShape::Content => mix_bytes(hash, b"content_ref"),
        AspectShape::Opaque(_) => mix_bytes(hash, b"opaque_token"),
        AspectShape::Struct(shape) => {
            mix_bytes(hash, b"struct");
            for field in shape.fields() {
                mix_struct_field(hash, field);
            }
            mix_bytes(hash, b"struct_complete");
        }
    }
}

fn mix_struct_field(hash: &mut u128, field: &FieldDeclaration) {
    fn mix_bytes(hash: &mut u128, bytes: &[u8]) {
        const FNV_PRIME: u128 = 0x0000000001000000000000000000013B;
        for byte in bytes {
            *hash ^= *byte as u128;
            *hash = hash.wrapping_mul(FNV_PRIME);
        }
        *hash ^= 0xff_u128;
        *hash = hash.wrapping_mul(FNV_PRIME);
    }

    mix_bytes(hash, field.key().as_str().as_bytes());
    mix_bytes(hash, scalar_name(field.value_type()).as_bytes());
    mix_bytes(
        hash,
        match field.requirement() {
            forge_foundational::FieldRequirement::Required => b"required",
            forge_foundational::FieldRequirement::Optional => b"optional",
            forge_foundational::FieldRequirement::Defaulted => b"defaulted",
        },
    );
    mix_bytes(
        hash,
        match field.absence() {
            forge_foundational::AbsenceLaw::Required => b"required",
            forge_foundational::AbsenceLaw::Optional => b"optional",
            forge_foundational::AbsenceLaw::Defaulted => b"defaulted",
        },
    );
    mix_bytes(
        hash,
        match field.evolution() {
            forge_foundational::AspectEvolutionPolicy::Frozen => b"frozen",
            forge_foundational::AspectEvolutionPolicy::AdditiveFieldsAllowed => b"additive",
            forge_foundational::AspectEvolutionPolicy::WideningAllowed => b"widening",
            forge_foundational::AspectEvolutionPolicy::ExplicitBreakRequired => b"explicit_break",
        },
    );
}

fn scalar_name(scalar: ScalarAspectType) -> &'static str {
    match scalar {
        ScalarAspectType::Null => "null",
        ScalarAspectType::Bool => "bool",
        ScalarAspectType::Int8 => "int8",
        ScalarAspectType::Int16 => "int16",
        ScalarAspectType::Int32 => "int32",
        ScalarAspectType::Int64 => "int64",
        ScalarAspectType::UInt8 => "uint8",
        ScalarAspectType::UInt16 => "uint16",
        ScalarAspectType::UInt32 => "uint32",
        ScalarAspectType::UInt64 => "uint64",
        ScalarAspectType::Float32 => "float32",
        ScalarAspectType::Float64 => "float64",
        ScalarAspectType::Decimal => "decimal",
        ScalarAspectType::BigInt => "bigint",
        ScalarAspectType::Rational => "rational",
        ScalarAspectType::String => "string",
        ScalarAspectType::Bytes => "bytes",
        ScalarAspectType::Uuid => "uuid",
        ScalarAspectType::Date => "date",
        ScalarAspectType::Time => "time",
        ScalarAspectType::Timestamp => "timestamp",
        ScalarAspectType::TimestampTz => "timestamp_tz",
        ScalarAspectType::EntityRef => "entity_ref",
        ScalarAspectType::ContentRef => "content_ref",
    }
}
