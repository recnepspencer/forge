use super::{AbsenceLaw, AspectEquivalenceBasis, AspectShape};
use crate::aspects::evolution::{
    classify_struct_evolution, scalar_widens, AspectEvolutionKind, AspectEvolutionPolicy,
    AspectEvolutionVerdict,
};
use crate::aspects::identity::{AspectContractRevision, AspectIdentity};
use crate::aspects::keys::AspectKey;
use crate::aspects::masks::{
    AspectMask, AspectMaskContract, DiagnosticMask, MaskAdmissibilityDenial, MutationMask,
    ProjectionMask,
};
use crate::aspects::structs::StructAspectShape;
use crate::values::ScalarAspectType;

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
            return AspectEvolutionVerdict::new(
                AspectEvolutionKind::Incompatible,
                "aspect identity or key changed",
            );
        }

        match (&self.shape, &next.shape) {
            (AspectShape::Scalar(left), AspectShape::Scalar(right)) if left == right => {
                AspectEvolutionVerdict::new(
                    AspectEvolutionKind::Unchanged,
                    "scalar shape unchanged",
                )
            }
            (AspectShape::Scalar(left), AspectShape::Scalar(right))
                if scalar_widens(*left, *right) =>
            {
                AspectEvolutionVerdict::new(AspectEvolutionKind::Widening, "scalar shape widened")
            }
            (AspectShape::Scalar(_), AspectShape::Scalar(_)) => AspectEvolutionVerdict::new(
                AspectEvolutionKind::Narrowing,
                "scalar shape narrowed or changed incompatibly",
            ),
            (AspectShape::Struct(left), AspectShape::Struct(right)) => {
                classify_struct_evolution(left, right)
            }
            _ => AspectEvolutionVerdict::new(
                AspectEvolutionKind::Incompatible,
                "aspect shape family changed",
            ),
        }
    }

    pub fn admits_projection_mask(
        &self,
        mask: &AspectMask<ProjectionMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Projection)
    }

    pub fn admits_mutation_mask(
        &self,
        mask: &AspectMask<MutationMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Mutation)
    }

    pub fn admits_diagnostic_mask(
        &self,
        mask: &AspectMask<DiagnosticMask>,
    ) -> Result<(), MaskAdmissibilityDenial> {
        self.masks
            .admit_paths_for_shape(mask.paths(), self.shape(), MaskModeAdmission::Diagnostic)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MaskModeAdmission {
    Projection,
    Mutation,
    Diagnostic,
}
