use crate::bindings::anchors::{
    CarrierOwnedParameterDirectionAnchorSpec, CarrierOwnedParameterPointAnchorSpec,
    SpatialAnchorAuthorityError,
};
use crate::bindings::authority::{
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec, SpatialBindingKind,
};
use crate::bindings::canonical_projection::SpatialCanonicalDeclarationField;
use crate::bindings::query_native::{
    PrimitiveAnchorBindingDeclarationFamily, PrimitiveAnchorBindingQueryDomain,
};
use crate::bindings::query_native_binding_projection_payload::{
    PrimitiveAnchorBindingProjectionPayload, PrimitiveAnchorBindingTargetIdentityPayload,
};
use crate::bindings::query_native_declared_target_identity_fact::{
    anchor_binding_declaration_fact, AnchorBindingDeclarationFact,
};
use crate::bindings::query_native_rebinding_candidate_fact::PrimitiveRebindingCandidateFact;
use crate::bindings::query_native_rebinding_declared_binding_fact::{
    declared_neighborhood_binding_fact_from_anchor_parts, DeclaredNeighborhoodBindingFact,
};
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;
use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveAnchorBindingIntent {
    AttachParameterSpacePointToFace(FaceSurfaceBindingSpec, CarrierOwnedParameterPointAnchorSpec),
    AttachParameterSpacePointToEdge(EdgeCurveBindingSpec, CarrierOwnedParameterPointAnchorSpec),
    AttachParameterSpacePointToCoedge(
        CoedgePCurveBindingSpec,
        CarrierOwnedParameterPointAnchorSpec,
    ),
    AttachParameterSpaceDirectionToFace(
        FaceSurfaceBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
    AttachParameterSpaceDirectionToEdge(
        EdgeCurveBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
    AttachParameterSpaceDirectionToCoedge(
        CoedgePCurveBindingSpec,
        CarrierOwnedParameterDirectionAnchorSpec,
    ),
}

impl AuthorPrimitiveAnchorBindingIntent {
    pub fn attach_parameter_space_point_to_face(
        binding_spec: FaceSurfaceBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToFace(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_point_to_edge(
        binding_spec: EdgeCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToEdge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_point_to_coedge(
        binding_spec: CoedgePCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterPointAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpacePointToCoedge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_face(
        binding_spec: FaceSurfaceBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToFace(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_edge(
        binding_spec: EdgeCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToEdge(binding_spec, anchor_spec)
    }

    pub fn attach_parameter_space_direction_to_coedge(
        binding_spec: CoedgePCurveBindingSpec,
        anchor_spec: CarrierOwnedParameterDirectionAnchorSpec,
    ) -> Self {
        Self::AttachParameterSpaceDirectionToCoedge(binding_spec, anchor_spec)
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveAnchorBindingDeclarationEntry {
    intent: AuthorPrimitiveAnchorBindingIntent,
    binding_fact: Result<AnchorBindingDeclarationFact, PrimitiveAnchorBindingAuthoringError>,
    projection_payload:
        Result<PrimitiveAnchorBindingProjectionPayload, PrimitiveAnchorBindingAuthoringError>,
    target_identity_payload:
        Result<PrimitiveAnchorBindingTargetIdentityPayload, PrimitiveAnchorBindingAuthoringError>,
    neighborhood_binding_fact:
        Result<DeclaredNeighborhoodBindingFact, PrimitiveAnchorBindingAuthoringError>,
    rebinding_prior_binding_fact:
        Result<PrimitiveRebindingPriorBindingFact, PrimitiveAnchorBindingAuthoringError>,
    rebinding_candidate_fact:
        Result<PrimitiveRebindingCandidateFact, PrimitiveAnchorBindingAuthoringError>,
}

impl PrimitiveAnchorBindingDeclarationEntry {
    pub fn new(
        intent: AuthorPrimitiveAnchorBindingIntent,
    ) -> Result<Self, PrimitiveAnchorBindingAuthoringError> {
        let mut entry = Self {
            intent,
            binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
            projection_payload: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
            target_identity_payload: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
            neighborhood_binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
            rebinding_prior_binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
            rebinding_candidate_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
                SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                    expected: String::new(),
                    found: String::new(),
                },
            )),
        };
        entry.binding_fact = anchor_binding_declaration_fact(&entry).map_err(|error| match error {
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied(inner) => inner,
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::BindingDeclarationDenied(_) => {
                unreachable!("anchor declaration fact cannot produce binding denial")
            }
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::OutcomeNotBound { .. } => {
                unreachable!("anchor declaration fact does not inspect ordinary outcomes")
            }
        });
        entry.projection_payload = entry
            .binding_fact
            .as_ref()
            .map(PrimitiveAnchorBindingProjectionPayload::from_binding_fact)
            .map_err(Clone::clone);
        entry.target_identity_payload = entry
            .binding_fact
            .as_ref()
            .map(PrimitiveAnchorBindingTargetIdentityPayload::from_binding_fact)
            .map_err(Clone::clone);
        entry.neighborhood_binding_fact = entry
            .binding_fact
            .as_ref()
            .map(|fact| declared_neighborhood_binding_fact_from_anchor_parts(&entry.intent, fact))
            .map_err(Clone::clone);
        entry.rebinding_prior_binding_fact = entry
            .neighborhood_binding_fact
            .as_ref()
            .map(PrimitiveRebindingPriorBindingFact::from_neighborhood_binding_fact)
            .map_err(Clone::clone);
        entry.rebinding_candidate_fact = entry
            .neighborhood_binding_fact
            .as_ref()
            .map(PrimitiveRebindingCandidateFact::from_neighborhood_binding_fact)
            .map_err(Clone::clone);
        entry.binding_fact.as_ref().map_err(Clone::clone)?;
        Ok(entry)
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        match &self.intent {
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(_, _)
            | AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(_, _) => {
                SpatialBindingKind::FaceSurface
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(_, _)
            | AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(_, _) => {
                SpatialBindingKind::EdgeCurve
            }
            AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(_, _)
            | AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(_, _) => {
                SpatialBindingKind::CoedgePCurve
            }
        }
    }

    pub(crate) fn intent(&self) -> &AuthorPrimitiveAnchorBindingIntent {
        &self.intent
    }

    pub(crate) fn projection_payload(
        &self,
    ) -> Result<&PrimitiveAnchorBindingProjectionPayload, PrimitiveAnchorBindingAuthoringError>
    {
        self.projection_payload.as_ref().map_err(Clone::clone)
    }

    pub(crate) fn target_identity_payload(
        &self,
    ) -> Result<&PrimitiveAnchorBindingTargetIdentityPayload, PrimitiveAnchorBindingAuthoringError>
    {
        self.target_identity_payload.as_ref().map_err(Clone::clone)
    }

    pub(crate) fn rebinding_prior_binding_fact(
        &self,
    ) -> Result<&PrimitiveRebindingPriorBindingFact, PrimitiveAnchorBindingAuthoringError> {
        self.rebinding_prior_binding_fact
            .as_ref()
            .map_err(Clone::clone)
    }

    pub(crate) fn rebinding_candidate_fact(
        &self,
    ) -> Result<&PrimitiveRebindingCandidateFact, PrimitiveAnchorBindingAuthoringError> {
        self.rebinding_candidate_fact.as_ref().map_err(Clone::clone)
    }
}

impl PartialEq for PrimitiveAnchorBindingDeclarationEntry {
    fn eq(&self, other: &Self) -> bool {
        self.intent == other.intent
    }
}

impl ForgeQueryDeclarationInput<PrimitiveAnchorBindingQueryDomain>
    for PrimitiveAnchorBindingDeclarationEntry
{
    type Family = PrimitiveAnchorBindingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveAnchorBindingAuthoringError {
    Anchor(SpatialAnchorAuthorityError),
}

pub fn author_primitive_anchor_binding_declaration(
    intent: AuthorPrimitiveAnchorBindingIntent,
) -> PrimitiveAnchorBindingDeclarationEntry {
    let mut entry = PrimitiveAnchorBindingDeclarationEntry {
        intent,
        binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
        projection_payload: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
        target_identity_payload: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
        neighborhood_binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
        rebinding_prior_binding_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
        rebinding_candidate_fact: Err(PrimitiveAnchorBindingAuthoringError::Anchor(
            SpatialAnchorAuthorityError::CarrierIdentityMismatch {
                expected: String::new(),
                found: String::new(),
            },
        )),
    };
    entry.binding_fact = anchor_binding_declaration_fact(&entry).map_err(|error| match error {
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied(inner) => inner,
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::BindingDeclarationDenied(_) => {
            unreachable!("anchor declaration fact cannot produce binding denial")
        }
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::OutcomeNotBound { .. } => {
            unreachable!("anchor declaration fact does not inspect ordinary outcomes")
        }
    });
    entry.projection_payload = entry
        .binding_fact
        .as_ref()
        .map(PrimitiveAnchorBindingProjectionPayload::from_binding_fact)
        .map_err(Clone::clone);
    entry.target_identity_payload = entry
        .binding_fact
        .as_ref()
        .map(PrimitiveAnchorBindingTargetIdentityPayload::from_binding_fact)
        .map_err(Clone::clone);
    entry.neighborhood_binding_fact = entry
        .binding_fact
        .as_ref()
        .map(|fact| declared_neighborhood_binding_fact_from_anchor_parts(&entry.intent, fact))
        .map_err(Clone::clone);
    entry.rebinding_prior_binding_fact = entry
        .neighborhood_binding_fact
        .as_ref()
        .map(PrimitiveRebindingPriorBindingFact::from_neighborhood_binding_fact)
        .map_err(Clone::clone);
    entry.rebinding_candidate_fact = entry
        .neighborhood_binding_fact
        .as_ref()
        .map(PrimitiveRebindingCandidateFact::from_neighborhood_binding_fact)
        .map_err(Clone::clone);
    entry
}

fn canonical_query_entries_for_intent(
    intent: &AuthorPrimitiveAnchorBindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    match intent {
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToFace(spec, anchor_spec) => {
            extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToEdge(spec, anchor_spec) => {
            extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec)
        }
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpacePointToCoedge(
            spec,
            anchor_spec,
        ) => extend_with_point_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToFace(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToEdge(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
        AuthorPrimitiveAnchorBindingIntent::AttachParameterSpaceDirectionToCoedge(
            spec,
            anchor_spec,
        ) => extend_with_direction_anchor_entries(spec.canonical_declaration_fields(), anchor_spec),
    }
}

fn extend_with_point_anchor_entries(
    mut fields: Vec<SpatialCanonicalDeclarationField>,
    anchor_spec: &CarrierOwnedParameterPointAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields.extend(anchor_spec.canonical_declaration_fields());
    into_query_entries(fields)
}

fn extend_with_direction_anchor_entries(
    mut fields: Vec<SpatialCanonicalDeclarationField>,
    anchor_spec: &CarrierOwnedParameterDirectionAnchorSpec,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields.extend(anchor_spec.canonical_declaration_fields());
    into_query_entries(fields)
}

fn into_query_entries(
    fields: Vec<SpatialCanonicalDeclarationField>,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields
        .into_iter()
        .map(|field| ForgeQueryDeclarationCanonicalEntry::text(field.locus(), field.value()))
        .collect()
}
