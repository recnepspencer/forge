use crate::bindings::authority::{
    CoedgePCurveBindingSpec, EdgeCurveBindingSpec, FaceSurfaceBindingSpec,
    SpatialBindingAuthorityError, SpatialBindingKind, VertexGeometryBindingSpec,
};
use crate::bindings::canonical_projection::SpatialCanonicalDeclarationField;
use crate::bindings::query_native::{
    PrimitiveBindingDeclarationFamily, PrimitiveBindingQueryDomain,
};
use crate::bindings::query_native_binding_projection_payload::{
    PrimitiveBindingProjectionPayload, PrimitiveBindingTargetIdentityPayload,
};
use crate::bindings::query_native_declared_target_identity_fact::{
    binding_declaration_fact, BindingDeclarationFact,
};
use crate::bindings::query_native_rebinding_candidate_fact::PrimitiveRebindingCandidateFact;
use crate::bindings::query_native_rebinding_declared_binding_fact::{
    declared_neighborhood_binding_fact_from_binding_parts, DeclaredNeighborhoodBindingFact,
};
use crate::bindings::query_native_rebinding_prior_fact::PrimitiveRebindingPriorBindingFact;
use forge_query::facade::{ForgeQueryDeclarationCanonicalEntry, ForgeQueryDeclarationInput};
#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveBindingIntent {
    AttachSurfaceToFace(FaceSurfaceBindingSpec),
    AttachCurveToEdge(EdgeCurveBindingSpec),
    AttachPCurveToCoedge(CoedgePCurveBindingSpec),
    AttachVertexGeometry(VertexGeometryBindingSpec),
}

impl AuthorPrimitiveBindingIntent {
    pub fn attach_surface_to_face(spec: FaceSurfaceBindingSpec) -> Self {
        Self::AttachSurfaceToFace(spec)
    }

    pub fn attach_curve_to_edge(spec: EdgeCurveBindingSpec) -> Self {
        Self::AttachCurveToEdge(spec)
    }

    pub fn attach_pcurve_to_coedge(spec: CoedgePCurveBindingSpec) -> Self {
        Self::AttachPCurveToCoedge(spec)
    }

    pub fn attach_vertex_geometry(spec: VertexGeometryBindingSpec) -> Self {
        Self::AttachVertexGeometry(spec)
    }
}

#[derive(Clone, Debug)]
pub struct PrimitiveBindingDeclarationEntry {
    intent: AuthorPrimitiveBindingIntent,
    binding_fact: Result<BindingDeclarationFact, PrimitiveBindingAuthoringError>,
    projection_payload: Result<PrimitiveBindingProjectionPayload, PrimitiveBindingAuthoringError>,
    target_identity_payload:
        Result<PrimitiveBindingTargetIdentityPayload, PrimitiveBindingAuthoringError>,
    neighborhood_binding_fact:
        Result<DeclaredNeighborhoodBindingFact, PrimitiveBindingAuthoringError>,
    rebinding_prior_binding_fact:
        Result<PrimitiveRebindingPriorBindingFact, PrimitiveBindingAuthoringError>,
    rebinding_candidate_fact:
        Result<PrimitiveRebindingCandidateFact, PrimitiveBindingAuthoringError>,
}

impl PrimitiveBindingDeclarationEntry {
    pub fn new(
        intent: AuthorPrimitiveBindingIntent,
    ) -> Result<Self, PrimitiveBindingAuthoringError> {
        let mut entry = Self {
            intent,
            binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
            projection_payload: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
            target_identity_payload: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
            neighborhood_binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
            rebinding_prior_binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
            rebinding_candidate_fact: Err(PrimitiveBindingAuthoringError::Spatial(
                SpatialBindingAuthorityError::Illegal(
                    crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                ),
            )),
        };
        entry.binding_fact = binding_declaration_fact(&entry).map_err(|error| match error {
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::BindingDeclarationDenied(inner) => inner,
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied(_) => {
                unreachable!("binding declaration fact cannot produce anchor denial")
            }
            crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::OutcomeNotBound { .. } => {
                unreachable!("binding declaration fact does not inspect ordinary outcomes")
            }
        });
        entry.projection_payload = entry
            .binding_fact
            .as_ref()
            .map(PrimitiveBindingProjectionPayload::from_binding_fact)
            .map_err(Clone::clone);
        entry.target_identity_payload = entry
            .binding_fact
            .as_ref()
            .map(PrimitiveBindingTargetIdentityPayload::from_binding_fact)
            .map_err(Clone::clone);
        entry.neighborhood_binding_fact = entry
            .binding_fact
            .as_ref()
            .map(|fact| declared_neighborhood_binding_fact_from_binding_parts(&entry.intent, fact))
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
            AuthorPrimitiveBindingIntent::AttachSurfaceToFace(_) => SpatialBindingKind::FaceSurface,
            AuthorPrimitiveBindingIntent::AttachCurveToEdge(_) => SpatialBindingKind::EdgeCurve,
            AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(_) => {
                SpatialBindingKind::CoedgePCurve
            }
            AuthorPrimitiveBindingIntent::AttachVertexGeometry(_) => {
                SpatialBindingKind::VertexGeometry
            }
        }
    }

    pub(crate) fn intent(&self) -> &AuthorPrimitiveBindingIntent {
        &self.intent
    }

    pub(crate) fn projection_payload(
        &self,
    ) -> Result<&PrimitiveBindingProjectionPayload, PrimitiveBindingAuthoringError> {
        self.projection_payload.as_ref().map_err(Clone::clone)
    }

    pub(crate) fn target_identity_payload(
        &self,
    ) -> Result<&PrimitiveBindingTargetIdentityPayload, PrimitiveBindingAuthoringError> {
        self.target_identity_payload.as_ref().map_err(Clone::clone)
    }

    pub(crate) fn rebinding_prior_binding_fact(
        &self,
    ) -> Result<&PrimitiveRebindingPriorBindingFact, PrimitiveBindingAuthoringError> {
        self.rebinding_prior_binding_fact
            .as_ref()
            .map_err(Clone::clone)
    }

    pub(crate) fn rebinding_candidate_fact(
        &self,
    ) -> Result<&PrimitiveRebindingCandidateFact, PrimitiveBindingAuthoringError> {
        self.rebinding_candidate_fact.as_ref().map_err(Clone::clone)
    }
}

impl PartialEq for PrimitiveBindingDeclarationEntry {
    fn eq(&self, other: &Self) -> bool {
        self.intent == other.intent
    }
}

impl ForgeQueryDeclarationInput<PrimitiveBindingQueryDomain> for PrimitiveBindingDeclarationEntry {
    type Family = PrimitiveBindingDeclarationFamily;

    fn canonical_declaration_entries(&self) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
        canonical_query_entries_for_intent(&self.intent)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveBindingAuthoringError {
    Spatial(SpatialBindingAuthorityError),
}

pub fn author_primitive_binding_declaration(
    intent: AuthorPrimitiveBindingIntent,
) -> PrimitiveBindingDeclarationEntry {
    let mut entry = PrimitiveBindingDeclarationEntry {
        intent,
        binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
        projection_payload: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
        target_identity_payload: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
        neighborhood_binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
        rebinding_prior_binding_fact: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
        rebinding_candidate_fact: Err(PrimitiveBindingAuthoringError::Spatial(
            SpatialBindingAuthorityError::Illegal(
                crate::bindings::authority::SpatialBindingIllegalityReason::MissingTopologyIdentity(
                    SpatialBindingKind::FaceSurface,
                ),
            ),
        )),
    };
    entry.binding_fact = binding_declaration_fact(&entry).map_err(|error| match error {
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::BindingDeclarationDenied(inner) => inner,
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied(_) => {
            unreachable!("binding declaration fact cannot produce anchor denial")
        }
        crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError::OutcomeNotBound { .. } => {
            unreachable!("binding declaration fact does not inspect ordinary outcomes")
        }
    });
    entry.projection_payload = entry
        .binding_fact
        .as_ref()
        .map(PrimitiveBindingProjectionPayload::from_binding_fact)
        .map_err(Clone::clone);
    entry.target_identity_payload = entry
        .binding_fact
        .as_ref()
        .map(PrimitiveBindingTargetIdentityPayload::from_binding_fact)
        .map_err(Clone::clone);
    entry.neighborhood_binding_fact = entry
        .binding_fact
        .as_ref()
        .map(|fact| declared_neighborhood_binding_fact_from_binding_parts(&entry.intent, fact))
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
    intent: &AuthorPrimitiveBindingIntent,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    match intent {
        AuthorPrimitiveBindingIntent::AttachSurfaceToFace(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachCurveToEdge(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachPCurveToCoedge(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
        AuthorPrimitiveBindingIntent::AttachVertexGeometry(spec) => {
            into_query_entries(spec.canonical_declaration_fields())
        }
    }
}

fn into_query_entries(
    fields: Vec<SpatialCanonicalDeclarationField>,
) -> Vec<ForgeQueryDeclarationCanonicalEntry> {
    fields
        .into_iter()
        .map(|field| ForgeQueryDeclarationCanonicalEntry::text(field.locus(), field.value()))
        .collect()
}
