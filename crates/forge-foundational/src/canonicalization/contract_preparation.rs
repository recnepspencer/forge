use forge_proof::{Artifact, TransitionOutcome};

use crate::aspects::{
    AbsenceLaw, AspectContract, AspectEquivalenceBasis, AspectEvolutionPolicy, AspectShape,
    FieldRequirement, OpaqueAspectType, ReferenceAspectType,
};
use crate::values::ScalarAspectType;

use super::{
    prepare_canonical_basis_sequence, CanonicalBasisDomain, CanonicalBasisEntry,
    CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisReadyArtifact, CanonicalBasisValue,
    CanonicalDigestAspectShapeKind, CanonicalDigestMaskMode, CanonicalDigestPreparationEntry,
    CanonicalizationRuleVersion, DigestPreparationReadyAspectContract,
    DigestPreparationReadyAspectContractArtifact,
};

pub fn prepare_aspect_contract_for_digest(
    contract: AspectContract,
) -> TransitionOutcome<DigestPreparationReadyAspectContractArtifact> {
    let basis = digest_basis_for_aspect_contract(&contract);

    TransitionOutcome::success(Artifact::new(DigestPreparationReadyAspectContract::new(
        contract, basis,
    )))
}

pub fn aspect_contract_digest_preparation_basis(
    ready: &DigestPreparationReadyAspectContractArtifact,
) -> &[CanonicalDigestPreparationEntry] {
    ready.payload().basis()
}

pub fn prepare_aspect_contract_for_canonical_basis(
    version: CanonicalizationRuleVersion,
    contract: AspectContract,
) -> forge_proof::TransitionOutcome<
    CanonicalBasisReadyArtifact,
    super::CanonicalBasisConstructionDenial,
> {
    prepare_canonical_basis_sequence(
        version,
        CanonicalBasisDomain::AspectContract,
        canonical_basis_for_aspect_contract(&contract),
    )
}

fn canonical_basis_for_aspect_contract(contract: &AspectContract) -> Vec<CanonicalBasisEntry> {
    digest_basis_for_aspect_contract(contract)
        .into_iter()
        .flat_map(canonical_entries_for_contract_digest_entry)
        .collect()
}

fn canonical_entries_for_contract_digest_entry(
    entry: CanonicalDigestPreparationEntry,
) -> Vec<CanonicalBasisEntry> {
    match entry {
        CanonicalDigestPreparationEntry::ContractHeader {
            key,
            identity,
            revision,
        } => vec![
            contract_entry(
                key.as_str(),
                "header.identity",
                CanonicalBasisEntryKind::Header,
                unsigned_64(identity.0),
            ),
            contract_entry(
                key.as_str(),
                "header.revision",
                CanonicalBasisEntryKind::Header,
                unsigned_64(revision.0),
            ),
        ],
        CanonicalDigestPreparationEntry::ContractShape { key, shape } => vec![contract_entry(
            key.as_str(),
            "shape",
            CanonicalBasisEntryKind::Shape,
            exact_text(digest_shape_name(shape)),
        )],
        CanonicalDigestPreparationEntry::ContractScalarShape { key, scalar } => {
            vec![contract_entry(
                key.as_str(),
                "shape.scalar",
                CanonicalBasisEntryKind::Shape,
                exact_text(scalar_aspect_type_name(scalar)),
            )]
        }
        CanonicalDigestPreparationEntry::ContractOpaqueShape { key, opaque } => {
            vec![contract_entry(
                key.as_str(),
                "shape.opaque",
                CanonicalBasisEntryKind::Shape,
                exact_text(opaque_aspect_type_name(opaque)),
            )]
        }
        CanonicalDigestPreparationEntry::ContractReferenceShape { key, reference } => {
            vec![contract_entry(
                key.as_str(),
                "shape.reference",
                CanonicalBasisEntryKind::Shape,
                exact_text(reference_aspect_type_name(reference)),
            )]
        }
        CanonicalDigestPreparationEntry::ContractStructField {
            key,
            field,
            value_type,
            requirement,
            absence,
            evolution,
        } => vec![
            contract_entry(
                key.as_str(),
                format!("field.{}.value_type", field.as_str()),
                CanonicalBasisEntryKind::Field,
                exact_text(scalar_aspect_type_name(value_type)),
            ),
            contract_entry(
                key.as_str(),
                format!("field.{}.requirement", field.as_str()),
                CanonicalBasisEntryKind::Field,
                exact_text(field_requirement_name(requirement)),
            ),
            contract_entry(
                key.as_str(),
                format!("field.{}.absence", field.as_str()),
                CanonicalBasisEntryKind::Field,
                exact_text(absence_law_name(absence)),
            ),
            contract_entry(
                key.as_str(),
                format!("field.{}.evolution", field.as_str()),
                CanonicalBasisEntryKind::Field,
                exact_text(aspect_evolution_policy_name(evolution)),
            ),
        ],
        CanonicalDigestPreparationEntry::ContractMaskMode { key, mode, allowed } => {
            vec![contract_entry(
                key.as_str(),
                format!("mask.{}", digest_mask_mode_name(mode)),
                CanonicalBasisEntryKind::Mask,
                CanonicalBasisValue::Bool(allowed),
            )]
        }
        CanonicalDigestPreparationEntry::ContractAbsenceLaw { key, absence } => {
            vec![contract_entry(
                key.as_str(),
                "absence",
                CanonicalBasisEntryKind::Header,
                exact_text(absence_law_name(absence)),
            )]
        }
        CanonicalDigestPreparationEntry::ContractEquivalenceBasis { key, equivalence } => {
            vec![contract_entry(
                key.as_str(),
                "equivalence",
                CanonicalBasisEntryKind::Header,
                exact_text(aspect_equivalence_basis_name(equivalence)),
            )]
        }
        CanonicalDigestPreparationEntry::ContractEvolutionPolicy { key, evolution } => {
            vec![contract_entry(
                key.as_str(),
                "evolution",
                CanonicalBasisEntryKind::Header,
                exact_text(aspect_evolution_policy_name(evolution)),
            )]
        }
        _ => unreachable!("contract canonical basis only consumes contract digest entries"),
    }
}

fn contract_entry(
    aspect_key: &str,
    locus: impl Into<String>,
    kind: CanonicalBasisEntryKind,
    value: CanonicalBasisValue,
) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        CanonicalBasisDomain::AspectContract,
        CanonicalBasisLocus::Named(format!("{aspect_key}.{}", locus.into()).into()),
        kind,
        value,
    )
}

fn exact_text(value: &'static str) -> CanonicalBasisValue {
    CanonicalBasisValue::ExactText(value.into())
}

fn unsigned_64(value: u64) -> CanonicalBasisValue {
    CanonicalBasisValue::UnsignedInteger {
        width: super::CanonicalIntegerWidth::Bits64,
        value: u128::from(value),
    }
}

fn digest_shape_name(shape: CanonicalDigestAspectShapeKind) -> &'static str {
    match shape {
        CanonicalDigestAspectShapeKind::Scalar => "scalar",
        CanonicalDigestAspectShapeKind::Struct => "struct",
        CanonicalDigestAspectShapeKind::Opaque => "opaque",
        CanonicalDigestAspectShapeKind::Reference => "reference",
        CanonicalDigestAspectShapeKind::Content => "content",
    }
}

fn digest_mask_mode_name(mode: CanonicalDigestMaskMode) -> &'static str {
    match mode {
        CanonicalDigestMaskMode::Projection => "projection",
        CanonicalDigestMaskMode::Mutation => "mutation",
        CanonicalDigestMaskMode::Diagnostic => "diagnostic",
    }
}

fn scalar_aspect_type_name(value: ScalarAspectType) -> &'static str {
    match value {
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
        ScalarAspectType::BigInt => "big_int",
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

fn opaque_aspect_type_name(value: OpaqueAspectType) -> &'static str {
    match value {
        OpaqueAspectType::Token => "token",
    }
}

fn reference_aspect_type_name(value: ReferenceAspectType) -> &'static str {
    match value {
        ReferenceAspectType::Entity => "entity",
    }
}

fn field_requirement_name(value: FieldRequirement) -> &'static str {
    match value {
        FieldRequirement::Required => "required",
        FieldRequirement::Optional => "optional",
        FieldRequirement::Defaulted => "defaulted",
    }
}

fn absence_law_name(value: AbsenceLaw) -> &'static str {
    match value {
        AbsenceLaw::Required => "required",
        AbsenceLaw::Optional => "optional",
        AbsenceLaw::Defaulted => "defaulted",
    }
}

fn aspect_evolution_policy_name(value: AspectEvolutionPolicy) -> &'static str {
    match value {
        AspectEvolutionPolicy::Frozen => "frozen",
        AspectEvolutionPolicy::AdditiveFieldsAllowed => "additive_fields_allowed",
        AspectEvolutionPolicy::WideningAllowed => "widening_allowed",
        AspectEvolutionPolicy::ExplicitBreakRequired => "explicit_break_required",
    }
}

fn aspect_equivalence_basis_name(value: AspectEquivalenceBasis) -> &'static str {
    match value {
        AspectEquivalenceBasis::ExactCanonicalValue => "exact_canonical_value",
        AspectEquivalenceBasis::DeclaredStructFields => "declared_struct_fields",
        AspectEquivalenceBasis::OpaqueIdentity => "opaque_identity",
        AspectEquivalenceBasis::ReferenceIdentity => "reference_identity",
        AspectEquivalenceBasis::ContentIdentity => "content_identity",
    }
}

fn digest_basis_for_aspect_contract(
    contract: &AspectContract,
) -> Vec<CanonicalDigestPreparationEntry> {
    let mut basis = vec![
        CanonicalDigestPreparationEntry::ContractHeader {
            key: contract.key().clone(),
            identity: contract.identity(),
            revision: contract.revision(),
        },
        CanonicalDigestPreparationEntry::ContractAbsenceLaw {
            key: contract.key().clone(),
            absence: contract.absence(),
        },
        CanonicalDigestPreparationEntry::ContractEquivalenceBasis {
            key: contract.key().clone(),
            equivalence: contract.equivalence(),
        },
        CanonicalDigestPreparationEntry::ContractEvolutionPolicy {
            key: contract.key().clone(),
            evolution: contract.evolution(),
        },
    ];

    basis.extend(digest_basis_for_contract_masks(contract));
    basis.extend(digest_basis_for_contract_shape(contract));
    basis.sort();
    basis
}

fn digest_basis_for_contract_masks(
    contract: &AspectContract,
) -> [CanonicalDigestPreparationEntry; 3] {
    [
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: contract.key().clone(),
            mode: CanonicalDigestMaskMode::Projection,
            allowed: contract.masks().projection_allowed(),
        },
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: contract.key().clone(),
            mode: CanonicalDigestMaskMode::Mutation,
            allowed: contract.masks().mutation_allowed(),
        },
        CanonicalDigestPreparationEntry::ContractMaskMode {
            key: contract.key().clone(),
            mode: CanonicalDigestMaskMode::Diagnostic,
            allowed: contract.masks().diagnostic_allowed(),
        },
    ]
}

fn digest_basis_for_contract_shape(
    contract: &AspectContract,
) -> Vec<CanonicalDigestPreparationEntry> {
    match contract.shape() {
        AspectShape::Scalar(scalar) => vec![
            CanonicalDigestPreparationEntry::ContractShape {
                key: contract.key().clone(),
                shape: CanonicalDigestAspectShapeKind::Scalar,
            },
            CanonicalDigestPreparationEntry::ContractScalarShape {
                key: contract.key().clone(),
                scalar: *scalar,
            },
        ],
        AspectShape::Struct(shape) => {
            let mut basis = vec![CanonicalDigestPreparationEntry::ContractShape {
                key: contract.key().clone(),
                shape: CanonicalDigestAspectShapeKind::Struct,
            }];
            basis.extend(shape.fields().iter().map(|field| {
                CanonicalDigestPreparationEntry::ContractStructField {
                    key: contract.key().clone(),
                    field: field.key().clone(),
                    value_type: field.value_type(),
                    requirement: field.requirement(),
                    absence: field.absence(),
                    evolution: field.evolution(),
                }
            }));
            basis
        }
        AspectShape::Opaque(opaque) => vec![
            CanonicalDigestPreparationEntry::ContractShape {
                key: contract.key().clone(),
                shape: CanonicalDigestAspectShapeKind::Opaque,
            },
            CanonicalDigestPreparationEntry::ContractOpaqueShape {
                key: contract.key().clone(),
                opaque: *opaque,
            },
        ],
        AspectShape::Reference(reference) => vec![
            CanonicalDigestPreparationEntry::ContractShape {
                key: contract.key().clone(),
                shape: CanonicalDigestAspectShapeKind::Reference,
            },
            CanonicalDigestPreparationEntry::ContractReferenceShape {
                key: contract.key().clone(),
                reference: *reference,
            },
        ],
        AspectShape::Content => vec![CanonicalDigestPreparationEntry::ContractShape {
            key: contract.key().clone(),
            shape: CanonicalDigestAspectShapeKind::Content,
        }],
    }
}
