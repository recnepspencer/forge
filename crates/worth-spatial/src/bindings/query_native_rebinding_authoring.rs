use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};

use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native_rebinding::{
    PrimitiveRebindingDeclarationFamily, PrimitiveRebindingQueryDomain,
};
use crate::bindings::query_native_rebinding_declaration_support::canonical_query_entries_for_intent;
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;
use crate::bindings::query_native_rebinding_projection_logic::projection_receipt_from_intent;
use crate::bindings::rebinding::{
    BindingMotionSemanticsInput, LocalTopologyReplacementNeighborhood,
    PrimitiveRebindingFactReceipt, SpatialRebindingAuthorityError,
};

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveRebindingIntent {
    ReplaceSurfaceBinding {
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    },
    ReplaceCurveBinding {
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    },
    ReplacePCurveBinding {
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    },
    ReplaceGeometryBinding {
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    },
}

impl AuthorPrimitiveRebindingIntent {
    pub fn replace_surface_binding(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceSurfaceBinding {
            prior_binding,
            neighborhood,
            motion: BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
        }
    }

    pub fn replace_surface_binding_with_motion(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    ) -> Self {
        Self::ReplaceSurfaceBinding {
            prior_binding,
            neighborhood,
            motion,
        }
    }

    pub fn replace_curve_binding(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceCurveBinding {
            prior_binding,
            neighborhood,
            motion: BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
        }
    }

    pub fn replace_curve_binding_with_motion(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    ) -> Self {
        Self::ReplaceCurveBinding {
            prior_binding,
            neighborhood,
            motion,
        }
    }

    pub fn replace_pcurve_binding(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplacePCurveBinding {
            prior_binding,
            neighborhood,
            motion: BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
        }
    }

    pub fn replace_pcurve_binding_with_motion(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    ) -> Self {
        Self::ReplacePCurveBinding {
            prior_binding,
            neighborhood,
            motion,
        }
    }

    pub fn replace_geometry_binding(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceGeometryBinding {
            prior_binding,
            neighborhood,
            motion: BindingMotionSemanticsInput::unresolved_without_motion_workflow(),
        }
    }

    pub fn replace_geometry_binding_with_motion(
        prior_binding: PrimitiveRebindingPriorBindingFact,
        neighborhood: LocalTopologyReplacementNeighborhood,
        motion: BindingMotionSemanticsInput,
    ) -> Self {
        Self::ReplaceGeometryBinding {
            prior_binding,
            neighborhood,
            motion,
        }
    }

    pub fn prior_binding_fact(&self) -> &PrimitiveRebindingPriorBindingFact {
        match self {
            Self::ReplaceSurfaceBinding { prior_binding, .. }
            | Self::ReplaceCurveBinding { prior_binding, .. }
            | Self::ReplacePCurveBinding { prior_binding, .. }
            | Self::ReplaceGeometryBinding { prior_binding, .. } => prior_binding,
        }
    }

    pub fn neighborhood(&self) -> &LocalTopologyReplacementNeighborhood {
        match self {
            Self::ReplaceSurfaceBinding { neighborhood, .. }
            | Self::ReplaceCurveBinding { neighborhood, .. }
            | Self::ReplacePCurveBinding { neighborhood, .. }
            | Self::ReplaceGeometryBinding { neighborhood, .. } => neighborhood,
        }
    }

    pub fn motion(&self) -> BindingMotionSemanticsInput {
        match self {
            Self::ReplaceSurfaceBinding { motion, .. }
            | Self::ReplaceCurveBinding { motion, .. }
            | Self::ReplacePCurveBinding { motion, .. }
            | Self::ReplaceGeometryBinding { motion, .. } => *motion,
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.prior_binding_fact().binding_kind()
    }

    pub fn rebinding_kind_label(&self) -> &'static str {
        match self {
            Self::ReplaceSurfaceBinding { .. } => "surface_rebinding",
            Self::ReplaceCurveBinding { .. } => "curve_rebinding",
            Self::ReplacePCurveBinding { .. } => "pcurve_rebinding",
            Self::ReplaceGeometryBinding { .. } => "geometry_rebinding",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveRebindingDeclarationEntry {
    intent: AuthorPrimitiveRebindingIntent,
    neighborhood_replacement_seed: PrimitiveRebindingNeighborhoodReplacementSeed,
}

impl PrimitiveRebindingDeclarationEntry {
    pub fn new(
        intent: AuthorPrimitiveRebindingIntent,
    ) -> Result<Self, PrimitiveRebindingAuthoringError> {
        projection_receipt_from_intent(&intent)
            .map_err(|error| PrimitiveRebindingAuthoringError::Spatial(error.clone()))?;
        let neighborhood_replacement_seed =
            PrimitiveRebindingNeighborhoodReplacementSeed::from_intent(&intent);
        Ok(Self {
            intent,
            neighborhood_replacement_seed,
        })
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.intent.binding_kind()
    }

    pub(crate) fn projection_receipt(
        &self,
    ) -> Result<PrimitiveRebindingFactReceipt, PrimitiveRebindingAuthoringError> {
        projection_receipt_from_intent(&self.intent)
            .map_err(|error| PrimitiveRebindingAuthoringError::Spatial(error.clone()))
    }

    pub(crate) fn neighborhood_replacement_seed(
        &self,
    ) -> &PrimitiveRebindingNeighborhoodReplacementSeed {
        &self.neighborhood_replacement_seed
    }
}

impl ForgeQueryDeclarationInput<PrimitiveRebindingQueryDomain>
    for PrimitiveRebindingDeclarationEntry
{
    type Family = PrimitiveRebindingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveRebindingAuthoringError {
    Spatial(SpatialRebindingAuthorityError),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PrimitiveRebindingNeighborhoodReplacementSeed {
    neighborhood_family: &'static str,
    prior_binding_identity: String,
    prior_site_identity: String,
    affected_target_identities: Vec<String>,
    candidate_frontier: Vec<String>,
    candidate_labels: Vec<String>,
}

impl PrimitiveRebindingNeighborhoodReplacementSeed {
    pub(crate) fn from_intent(intent: &AuthorPrimitiveRebindingIntent) -> Self {
        Self {
            neighborhood_family: intent.neighborhood().family().rebinding_kind_label(),
            prior_binding_identity: intent
                .prior_binding_fact()
                .prior_binding_identity()
                .to_string(),
            prior_site_identity: intent.neighborhood().prior_site_identity().to_string(),
            affected_target_identities: intent
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.binding_identity().to_string())
                .collect(),
            candidate_frontier: intent
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.site_identity().to_string())
                .collect(),
            candidate_labels: intent
                .neighborhood()
                .candidates()
                .iter()
                .map(|candidate| candidate.label().to_string())
                .collect(),
        }
    }

    pub(crate) fn neighborhood_family(&self) -> &'static str {
        self.neighborhood_family
    }

    pub(crate) fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub(crate) fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub(crate) fn affected_target_identities(&self) -> &[String] {
        &self.affected_target_identities
    }

    pub(crate) fn candidate_frontier(&self) -> &[String] {
        &self.candidate_frontier
    }

    pub(crate) fn candidate_labels(&self) -> &[String] {
        &self.candidate_labels
    }
}

pub fn author_primitive_rebinding_declaration(
    intent: AuthorPrimitiveRebindingIntent,
) -> PrimitiveRebindingDeclarationEntry {
    let neighborhood_replacement_seed =
        PrimitiveRebindingNeighborhoodReplacementSeed::from_intent(&intent);
    PrimitiveRebindingDeclarationEntry {
        intent,
        neighborhood_replacement_seed,
    }
}
