use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use forge_proof::{Artifact, PhaseMarker, TransitionOutcome};

use crate::facade::ResponsibilityArea;
use crate::values::{AspectValue, ScalarAspectType};

pub fn responsibility() -> ResponsibilityArea {
    ResponsibilityArea::new(
        "aspect_state_and_patches",
        "aspect contracts, state-map vocabulary, mask law, and patch vocabulary",
        "domain-owned truth mutation or persistence engines",
    )
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectKey(String);

impl AspectKey {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.trim().is_empty() || value.contains(char::is_whitespace) {
            None
        } else {
            Some(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectIdentity(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct AspectContractRevision(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AbsenceLaw {
    Required,
    Optional,
    Defaulted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEquivalenceBasis {
    ExactCanonicalValue,
    DeclaredStructFields,
    OpaqueIdentity,
    ReferenceIdentity,
    ContentIdentity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEvolutionPolicy {
    Frozen,
    AdditiveFieldsAllowed,
    WideningAllowed,
    ExplicitBreakRequired,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectEvolutionKind {
    Unchanged,
    Additive,
    Widening,
    Narrowing,
    Incompatible,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectEvolutionVerdict {
    kind: AspectEvolutionKind,
    reason: &'static str,
}

impl AspectEvolutionVerdict {
    pub fn kind(&self) -> AspectEvolutionKind {
        self.kind
    }

    pub fn reason(&self) -> &'static str {
        self.reason
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OpaqueAspectType {
    Token,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ReferenceAspectType {
    Entity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AspectShape {
    Scalar(ScalarAspectType),
    Struct(StructAspectShape),
    Opaque(OpaqueAspectType),
    Reference(ReferenceAspectType),
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructAspectShape {
    fields: Vec<FieldDeclaration>,
}

impl StructAspectShape {
    pub fn new(fields: impl IntoIterator<Item = FieldDeclaration>) -> Option<Self> {
        let mut fields: Vec<_> = fields.into_iter().collect();
        fields.sort_by(|left, right| left.key.cmp(&right.key));

        let mut seen = BTreeSet::new();
        for field in &fields {
            if !seen.insert(field.key.clone()) {
                return None;
            }
        }

        Some(Self { fields })
    }

    pub fn fields(&self) -> &[FieldDeclaration] {
        &self.fields
    }

    pub fn field(&self, key: &FieldKey) -> Option<&FieldDeclaration> {
        self.fields.iter().find(|field| &field.key == key)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldKey(String);

impl FieldKey {
    pub fn new(value: impl Into<String>) -> Option<Self> {
        let value = value.into();
        if value.trim().is_empty() || value.contains('.') || value.contains(char::is_whitespace) {
            None
        } else {
            Some(Self(value))
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum FieldRequirement {
    Required,
    Optional,
    Defaulted,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration {
    key: FieldKey,
    value_type: ScalarAspectType,
    requirement: FieldRequirement,
    absence: AbsenceLaw,
    evolution: AspectEvolutionPolicy,
}

impl FieldDeclaration {
    pub fn new(
        key: FieldKey,
        value_type: ScalarAspectType,
        requirement: FieldRequirement,
        absence: AbsenceLaw,
        evolution: AspectEvolutionPolicy,
    ) -> Self {
        Self {
            key,
            value_type,
            requirement,
            absence,
            evolution,
        }
    }

    pub fn key(&self) -> &FieldKey {
        &self.key
    }

    pub fn value_type(&self) -> ScalarAspectType {
        self.value_type
    }

    pub fn requirement(&self) -> FieldRequirement {
        self.requirement
    }

    pub fn absence(&self) -> AbsenceLaw {
        self.absence
    }

    pub fn evolution(&self) -> AspectEvolutionPolicy {
        self.evolution
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructAspectValue {
    fields: BTreeMap<FieldKey, AspectValue>,
}

impl StructAspectValue {
    pub fn new(fields: impl IntoIterator<Item = (FieldKey, AspectValue)>) -> Self {
        Self {
            fields: fields.into_iter().collect(),
        }
    }

    pub fn fields(&self) -> impl Iterator<Item = (&FieldKey, &AspectValue)> {
        self.fields.iter()
    }

    pub fn get(&self, key: &FieldKey) -> Option<&AspectValue> {
        self.fields.get(key)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectionMask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MutationMask;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagnosticMask;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CanonicalFieldPath(Vec<FieldKey>);

impl CanonicalFieldPath {
    pub fn new(fields: impl IntoIterator<Item = FieldKey>) -> Option<Self> {
        let fields: Vec<_> = fields.into_iter().collect();
        if fields.is_empty() {
            None
        } else {
            Some(Self(fields))
        }
    }

    pub fn single(field: FieldKey) -> Self {
        Self(vec![field])
    }

    pub fn fields(&self) -> &[FieldKey] {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectMask<Mode> {
    paths: Vec<CanonicalFieldPath>,
    mode: PhantomData<Mode>,
}

impl<Mode> AspectMask<Mode> {
    pub fn new(paths: impl IntoIterator<Item = CanonicalFieldPath>) -> Self {
        let mut paths: Vec<_> = paths.into_iter().collect();
        paths.sort();
        paths.dedup();
        Self {
            paths,
            mode: PhantomData,
        }
    }

    pub fn whole_aspect() -> Self {
        Self {
            paths: Vec::new(),
            mode: PhantomData,
        }
    }

    pub fn paths(&self) -> &[CanonicalFieldPath] {
        &self.paths
    }

    pub fn is_whole_aspect(&self) -> bool {
        self.paths.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectMaskContract {
    projection: bool,
    mutation: bool,
    diagnostic: bool,
}

impl AspectMaskContract {
    pub const fn scalar() -> Self {
        Self {
            projection: true,
            mutation: true,
            diagnostic: true,
        }
    }

    pub const fn struct_fields() -> Self {
        Self {
            projection: true,
            mutation: true,
            diagnostic: true,
        }
    }

    pub const fn opaque_diagnostic_only() -> Self {
        Self {
            projection: false,
            mutation: false,
            diagnostic: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AspectContract {
    key: AspectKey,
    identity: AspectIdentity,
    revision: AspectContractRevision,
    shape: AspectShape,
    masks: AspectMaskContract,
    absence: AbsenceLaw,
    equivalence: AspectEquivalenceBasis,
    evolution: AspectEvolutionPolicy,
}

impl AspectContract {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        key: AspectKey,
        identity: AspectIdentity,
        revision: AspectContractRevision,
        shape: AspectShape,
        masks: AspectMaskContract,
        absence: AbsenceLaw,
        equivalence: AspectEquivalenceBasis,
        evolution: AspectEvolutionPolicy,
    ) -> Self {
        Self {
            key,
            identity,
            revision,
            shape,
            masks,
            absence,
            equivalence,
            evolution,
        }
    }

    pub fn scalar(
        key: AspectKey,
        identity: AspectIdentity,
        revision: AspectContractRevision,
        scalar: ScalarAspectType,
    ) -> Self {
        Self::new(
            key,
            identity,
            revision,
            AspectShape::Scalar(scalar),
            AspectMaskContract::scalar(),
            AbsenceLaw::Required,
            AspectEquivalenceBasis::ExactCanonicalValue,
            AspectEvolutionPolicy::ExplicitBreakRequired,
        )
    }

    pub fn struct_aspect(
        key: AspectKey,
        identity: AspectIdentity,
        revision: AspectContractRevision,
        shape: StructAspectShape,
    ) -> Self {
        Self::new(
            key,
            identity,
            revision,
            AspectShape::Struct(shape),
            AspectMaskContract::struct_fields(),
            AbsenceLaw::Required,
            AspectEquivalenceBasis::DeclaredStructFields,
            AspectEvolutionPolicy::AdditiveFieldsAllowed,
        )
    }

    pub fn key(&self) -> &AspectKey {
        &self.key
    }

    pub fn revision(&self) -> AspectContractRevision {
        self.revision
    }

    pub fn shape(&self) -> &AspectShape {
        &self.shape
    }

    pub fn equivalence(&self) -> AspectEquivalenceBasis {
        self.equivalence
    }

    pub fn classify_evolution_to(&self, next: &Self) -> AspectEvolutionVerdict {
        if self.identity != next.identity || self.key != next.key {
            return AspectEvolutionVerdict {
                kind: AspectEvolutionKind::Incompatible,
                reason: "aspect identity or key changed",
            };
        }

        match (&self.shape, &next.shape) {
            (AspectShape::Scalar(left), AspectShape::Scalar(right)) if left == right => {
                AspectEvolutionVerdict {
                    kind: AspectEvolutionKind::Unchanged,
                    reason: "scalar shape unchanged",
                }
            }
            (AspectShape::Scalar(left), AspectShape::Scalar(right))
                if scalar_widens(*left, *right) =>
            {
                AspectEvolutionVerdict {
                    kind: AspectEvolutionKind::Widening,
                    reason: "scalar shape widened",
                }
            }
            (AspectShape::Scalar(_), AspectShape::Scalar(_)) => AspectEvolutionVerdict {
                kind: AspectEvolutionKind::Narrowing,
                reason: "scalar shape narrowed or changed incompatibly",
            },
            (AspectShape::Struct(left), AspectShape::Struct(right)) => {
                classify_struct_evolution(left, right)
            }
            _ => AspectEvolutionVerdict {
                kind: AspectEvolutionKind::Incompatible,
                reason: "aspect shape family changed",
            },
        }
    }

    pub fn admits_projection_mask(
        &self,
        mask: &AspectMask<ProjectionMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.admits_mask_paths(mask.paths(), self.masks.projection)
    }

    pub fn admits_mutation_mask(
        &self,
        mask: &AspectMask<MutationMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.admits_mask_paths(mask.paths(), self.masks.mutation)
    }

    pub fn admits_diagnostic_mask(
        &self,
        mask: &AspectMask<DiagnosticMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.admits_mask_paths(mask.paths(), self.masks.diagnostic)
    }

    fn admits_mask_paths(
        &self,
        paths: &[CanonicalFieldPath],
        mode_allowed: bool,
    ) -> Result<(), MaskAdmissibilityDenial> {
        if !mode_allowed {
            return Err(MaskAdmissibilityDenial::ModeNotAllowed);
        }

        match &self.shape {
            AspectShape::Struct(shape) => {
                for path in paths {
                    if path.fields().len() != 1 || shape.field(&path.fields()[0]).is_none() {
                        return Err(MaskAdmissibilityDenial::UnknownField);
                    }
                }
                Ok(())
            }
            _ if paths.is_empty() => Ok(()),
            _ => Err(MaskAdmissibilityDenial::FieldMaskRequiresStruct),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MaskAdmissibilityDenial {
    ModeNotAllowed,
    FieldMaskRequiresStruct,
    UnknownField,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidatedAspectValue {
    Scalar {
        key: AspectKey,
        value: AspectValue,
        contract_revision: AspectContractRevision,
    },
    Struct {
        key: AspectKey,
        value: StructAspectValue,
        contract_revision: AspectContractRevision,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContractValidated;

impl PhaseMarker for ContractValidated {}

pub type ContractValidatedAspectArtifact =
    Artifact<ContractValidated, ContractValidatedAspectValue>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidationDenial {
    ScalarTypeMismatch {
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
    StructValueRequired,
    ScalarValueRequired,
    MissingRequiredField(FieldKey),
    UnknownField(FieldKey),
    FieldTypeMismatch {
        field: FieldKey,
        expected: ScalarAspectType,
        found: ScalarAspectType,
    },
}

pub fn validate_aspect_value(
    contract: &AspectContract,
    value: ContractValidationInput,
) -> TransitionOutcome<ContractValidatedAspectArtifact, ContractValidationDenial> {
    match (contract.shape(), value) {
        (AspectShape::Scalar(expected), ContractValidationInput::Scalar(value)) => {
            let found = value.value_family();
            if *expected == found {
                TransitionOutcome::success(Artifact::new(ContractValidatedAspectValue::Scalar {
                    key: contract.key().clone(),
                    value,
                    contract_revision: contract.revision(),
                }))
            } else {
                TransitionOutcome::denied(ContractValidationDenial::ScalarTypeMismatch {
                    expected: *expected,
                    found,
                })
            }
        }
        (AspectShape::Struct(shape), ContractValidationInput::Struct(value)) => {
            validate_struct_value(contract, shape, value)
        }
        (AspectShape::Struct(_), ContractValidationInput::Scalar(_)) => {
            TransitionOutcome::denied(ContractValidationDenial::StructValueRequired)
        }
        (
            AspectShape::Reference(ReferenceAspectType::Entity),
            ContractValidationInput::Scalar(value),
        ) => validate_scalar_family(contract, value, ScalarAspectType::EntityRef),
        (AspectShape::Content, ContractValidationInput::Scalar(value)) => {
            validate_scalar_family(contract, value, ScalarAspectType::ContentRef)
        }
        (AspectShape::Opaque(_), ContractValidationInput::Scalar(_)) => {
            TransitionOutcome::denied(ContractValidationDenial::ScalarValueRequired)
        }
        (_, ContractValidationInput::Struct(_)) => {
            TransitionOutcome::denied(ContractValidationDenial::ScalarValueRequired)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContractValidationInput {
    Scalar(AspectValue),
    Struct(StructAspectValue),
}

impl From<AspectValue> for ContractValidationInput {
    fn from(value: AspectValue) -> Self {
        Self::Scalar(value)
    }
}

impl From<StructAspectValue> for ContractValidationInput {
    fn from(value: StructAspectValue) -> Self {
        Self::Struct(value)
    }
}

fn validate_struct_value(
    contract: &AspectContract,
    shape: &StructAspectShape,
    value: StructAspectValue,
) -> TransitionOutcome<ContractValidatedAspectArtifact, ContractValidationDenial> {
    for field in shape.fields() {
        if matches!(field.requirement(), FieldRequirement::Required)
            && value.get(field.key()).is_none()
        {
            return TransitionOutcome::denied(ContractValidationDenial::MissingRequiredField(
                field.key().clone(),
            ));
        }
    }

    for (key, field_value) in value.fields() {
        let Some(field) = shape.field(key) else {
            return TransitionOutcome::denied(ContractValidationDenial::UnknownField(key.clone()));
        };
        let found = field_value.value_family();
        if found != field.value_type() {
            return TransitionOutcome::denied(ContractValidationDenial::FieldTypeMismatch {
                field: key.clone(),
                expected: field.value_type(),
                found,
            });
        }
    }

    TransitionOutcome::success(Artifact::new(ContractValidatedAspectValue::Struct {
        key: contract.key().clone(),
        value,
        contract_revision: contract.revision(),
    }))
}

fn validate_scalar_family(
    contract: &AspectContract,
    value: AspectValue,
    expected: ScalarAspectType,
) -> TransitionOutcome<ContractValidatedAspectArtifact, ContractValidationDenial> {
    let found = value.value_family();
    if found == expected {
        TransitionOutcome::success(Artifact::new(ContractValidatedAspectValue::Scalar {
            key: contract.key().clone(),
            value,
            contract_revision: contract.revision(),
        }))
    } else {
        TransitionOutcome::denied(ContractValidationDenial::ScalarTypeMismatch { expected, found })
    }
}

fn scalar_widens(left: ScalarAspectType, right: ScalarAspectType) -> bool {
    matches!(
        (left, right),
        (ScalarAspectType::Int8, ScalarAspectType::Int16)
            | (ScalarAspectType::Int8, ScalarAspectType::Int32)
            | (ScalarAspectType::Int8, ScalarAspectType::Int64)
            | (ScalarAspectType::Int16, ScalarAspectType::Int32)
            | (ScalarAspectType::Int16, ScalarAspectType::Int64)
            | (ScalarAspectType::Int32, ScalarAspectType::Int64)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt16)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt32)
            | (ScalarAspectType::UInt8, ScalarAspectType::UInt64)
            | (ScalarAspectType::UInt16, ScalarAspectType::UInt32)
            | (ScalarAspectType::UInt16, ScalarAspectType::UInt64)
            | (ScalarAspectType::UInt32, ScalarAspectType::UInt64)
            | (ScalarAspectType::Float32, ScalarAspectType::Float64)
    )
}

fn classify_struct_evolution(
    left: &StructAspectShape,
    right: &StructAspectShape,
) -> AspectEvolutionVerdict {
    let left_keys: BTreeSet<_> = left.fields().iter().map(FieldDeclaration::key).collect();
    let right_keys: BTreeSet<_> = right.fields().iter().map(FieldDeclaration::key).collect();

    if left_keys == right_keys {
        for key in left_keys {
            let left_field = left.field(key).expect("left key from left shape");
            let right_field = right.field(key).expect("right key from equal set");
            if left_field.value_type() != right_field.value_type() {
                return if scalar_widens(left_field.value_type(), right_field.value_type()) {
                    AspectEvolutionVerdict {
                        kind: AspectEvolutionKind::Widening,
                        reason: "struct field widened",
                    }
                } else {
                    AspectEvolutionVerdict {
                        kind: AspectEvolutionKind::Narrowing,
                        reason: "struct field narrowed or changed incompatibly",
                    }
                };
            }
        }
        AspectEvolutionVerdict {
            kind: AspectEvolutionKind::Unchanged,
            reason: "struct shape unchanged",
        }
    } else if left_keys.is_subset(&right_keys) {
        AspectEvolutionVerdict {
            kind: AspectEvolutionKind::Additive,
            reason: "struct fields were added",
        }
    } else {
        AspectEvolutionVerdict {
            kind: AspectEvolutionKind::Incompatible,
            reason: "struct fields were removed or renamed",
        }
    }
}
