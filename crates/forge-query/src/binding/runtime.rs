use crate::identity::BindingFulfillmentDigest;
use crate::validation::ValidatedQueryBundle;

use super::{QueryBindingSlot, QueryBindingSubject};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BindingResolutionError {
    MissingBindingSlot { slot: String },
    ExtraBindingSlot { slot: String },
    ConflictingBindingSubjects {
        slot: String,
        expected: QueryBindingSubject,
        actual: QueryBindingSubject,
    },
    DuplicateBindingSlot { slot: String },
}

impl BindingResolutionError {
    pub fn failure_digest(&self) -> String {
        match self {
            Self::MissingBindingSlot { slot } => format!("missing-binding-slot:{slot}"),
            Self::ExtraBindingSlot { slot } => format!("extra-binding-slot:{slot}"),
            Self::ConflictingBindingSubjects {
                slot,
                expected,
                actual,
            } => format!(
                "conflicting-binding-subjects:{slot}:{expected:?}:{actual:?}"
            ),
            Self::DuplicateBindingSlot { slot } => format!("duplicate-binding-slot:{slot}"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingRequirement {
    slot: QueryBindingSlot,
    subject: QueryBindingSubject,
    identity_bearing: bool,
}

impl BindingRequirement {
    pub(crate) fn new(
        slot: QueryBindingSlot,
        subject: QueryBindingSubject,
        identity_bearing: bool,
    ) -> Self {
        Self {
            slot,
            subject,
            identity_bearing,
        }
    }

    pub fn slot(&self) -> &QueryBindingSlot {
        &self.slot
    }

    pub fn subject(&self) -> &QueryBindingSubject {
        &self.subject
    }

    pub fn identity_bearing(&self) -> bool {
        self.identity_bearing
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingRequirements {
    requirements: Vec<BindingRequirement>,
}

impl BindingRequirements {
    pub(crate) fn new(requirements: Vec<BindingRequirement>) -> Self {
        Self { requirements }
    }

    pub fn requirements(&self) -> &[BindingRequirement] {
        &self.requirements
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct BoundBinding {
    slot: QueryBindingSlot,
    subject: QueryBindingSubject,
    value: String,
}

impl BoundBinding {
    pub fn new(
        slot: QueryBindingSlot,
        subject: QueryBindingSubject,
        value: impl Into<String>,
    ) -> Self {
        Self {
            slot,
            subject,
            value: value.into(),
        }
    }

    pub fn slot(&self) -> &QueryBindingSlot {
        &self.slot
    }

    pub fn subject(&self) -> &QueryBindingSubject {
        &self.subject
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    fn digest_fragment(&self) -> String {
        format!(
            "binding:{}:{:?}:{}",
            self.slot.as_str(),
            self.subject,
            self.value
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundBindings {
    bindings: Vec<BoundBinding>,
}

impl BoundBindings {
    #[allow(dead_code)]
    pub(crate) fn new(bindings: Vec<BoundBinding>) -> Self {
        Self { bindings }
    }

    pub fn bindings(&self) -> &[BoundBinding] {
        &self.bindings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BindingResolution {
    requirements: BindingRequirements,
    bindings: BoundBindings,
    digest: BindingFulfillmentDigest,
}

impl BindingResolution {
    pub fn requirements(&self) -> &BindingRequirements {
        &self.requirements
    }

    pub fn bindings(&self) -> &BoundBindings {
        &self.bindings
    }

    pub fn digest(&self) -> &BindingFulfillmentDigest {
        &self.digest
    }

    pub(crate) fn new(requirements: BindingRequirements, bindings: BoundBindings) -> Self {
        let mut parts: Vec<String> = requirements
            .requirements()
            .iter()
            .map(|requirement| {
                format!(
                    "requirement:{}:{:?}:{}",
                    requirement.slot().as_str(),
                    requirement.subject(),
                    requirement.identity_bearing()
                )
            })
            .collect();
        let mut binding_parts: Vec<String> = bindings
            .bindings()
            .iter()
            .map(BoundBinding::digest_fragment)
            .collect();
        binding_parts.sort();
        parts.extend(binding_parts);

        Self {
            requirements,
            bindings,
            digest: BindingFulfillmentDigest::from_parts(&parts),
        }
    }
}

pub fn derive_binding_requirements(bundle: &ValidatedQueryBundle) -> BindingRequirements {
    let requirements = bundle
        .query()
        .identity_bindings()
        .iter()
        .map(|binding| {
            BindingRequirement::new(
                binding.slot().clone(),
                binding.subject().clone(),
                true,
            )
        })
        .collect();
    BindingRequirements::new(requirements)
}

pub fn resolve_bindings(
    requirements: BindingRequirements,
    bindings: BoundBindings,
) -> Result<BindingResolution, BindingResolutionError> {
    let mut seen_slots = std::collections::BTreeSet::new();
    for binding in bindings.bindings() {
        let slot = binding.slot().as_str().to_string();
        if !seen_slots.insert(slot.clone()) {
            return Err(BindingResolutionError::DuplicateBindingSlot { slot });
        }
    }

    for requirement in requirements.requirements() {
        let bound = bindings
            .bindings()
            .iter()
            .find(|binding| binding.slot() == requirement.slot())
            .ok_or_else(|| BindingResolutionError::MissingBindingSlot {
                slot: requirement.slot().as_str().to_string(),
            })?;

        if bound.subject() != requirement.subject() {
            return Err(BindingResolutionError::ConflictingBindingSubjects {
                slot: requirement.slot().as_str().to_string(),
                expected: requirement.subject().clone(),
                actual: bound.subject().clone(),
            });
        }
    }

    for binding in bindings.bindings() {
        let declared = requirements
            .requirements()
            .iter()
            .any(|requirement| requirement.slot() == binding.slot());
        if !declared {
            return Err(BindingResolutionError::ExtraBindingSlot {
                slot: binding.slot().as_str().to_string(),
            });
        }
    }

    Ok(BindingResolution::new(requirements, bindings))
}
