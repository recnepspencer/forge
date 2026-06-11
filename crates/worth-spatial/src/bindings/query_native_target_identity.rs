use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDeclarationEnvelope,
    ForgeQueryDomainOperatingContext, ForgeQueryOrdinaryNextStep, ForgeQueryOrdinaryOutcome,
    ForgeQueryOrdinaryPosture, ForgeQueryOrdinaryPostureKind,
};
use worth_primitives::{truth_digest_parts, TruthDigestScope};

use crate::bindings::query_native::{
    PrimitiveAnchorBindingQueryDomain, PrimitiveBindingQueryDomain,
};
use crate::bindings::query_native_anchor_binding_authoring::{
    PrimitiveAnchorBindingAuthoringError, PrimitiveAnchorBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_authoring::{
    PrimitiveBindingAuthoringError, PrimitiveBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_projection_payload::{
    CanonicalGeometryTargetKind, PrimitiveAnchorBindingTargetIdentityPayload,
    PrimitiveBindingTargetIdentityPayload,
};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryTargetKind {
    FaceSurface,
    EdgeCurve,
    CoedgePCurve,
    VertexGeometry,
    FaceSurfacePointAnchor,
    EdgeCurvePointAnchor,
    CoedgePCurvePointAnchor,
    FaceSurfaceDirectionAnchor,
    EdgeCurveDirectionAnchor,
    CoedgePCurveDirectionAnchor,
}

impl GeometryTargetKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FaceSurface => "face_surface",
            Self::EdgeCurve => "edge_curve",
            Self::CoedgePCurve => "coedge_pcurve",
            Self::VertexGeometry => "vertex_geometry",
            Self::FaceSurfacePointAnchor => "face_surface_point_anchor",
            Self::EdgeCurvePointAnchor => "edge_curve_point_anchor",
            Self::CoedgePCurvePointAnchor => "coedge_pcurve_point_anchor",
            Self::FaceSurfaceDirectionAnchor => "face_surface_direction_anchor",
            Self::EdgeCurveDirectionAnchor => "edge_curve_direction_anchor",
            Self::CoedgePCurveDirectionAnchor => "coedge_pcurve_direction_anchor",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GeometryTargetSourceAuthority {
    PrimitiveBindingDeclarationEnvelope,
    PrimitiveAnchorBindingDeclarationEnvelope,
}

impl GeometryTargetSourceAuthority {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PrimitiveBindingDeclarationEnvelope => "primitive_binding_declaration_envelope",
            Self::PrimitiveAnchorBindingDeclarationEnvelope => {
                "primitive_anchor_binding_declaration_envelope"
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeometryTargetIdentityFactReceipt {
    target_identity: String,
    target_kind: GeometryTargetKind,
    source_authority: GeometryTargetSourceAuthority,
    declaration_digest: String,
    alias_identities: Vec<String>,
    fact_digest: String,
}

impl GeometryTargetIdentityFactReceipt {
    pub(crate) fn new(
        target_identity: String,
        target_kind: GeometryTargetKind,
        source_authority: GeometryTargetSourceAuthority,
        declaration_digest: String,
        alias_identities: Vec<String>,
    ) -> Self {
        let fact_digest = digest_parts(&[
            target_identity.clone(),
            target_kind.as_str().to_string(),
            source_authority.as_str().to_string(),
            declaration_digest.clone(),
            format!("{alias_identities:?}"),
        ]);
        Self {
            target_identity,
            target_kind,
            source_authority,
            declaration_digest,
            alias_identities,
            fact_digest,
        }
    }

    pub fn target_identity(&self) -> &str {
        &self.target_identity
    }

    pub fn target_kind(&self) -> GeometryTargetKind {
        self.target_kind
    }

    pub fn source_authority(&self) -> GeometryTargetSourceAuthority {
        self.source_authority
    }

    pub fn declaration_digest(&self) -> &str {
        &self.declaration_digest
    }

    pub fn alias_identities(&self) -> &[String] {
        &self.alias_identities
    }

    pub fn fact_digest(&self) -> &str {
        &self.fact_digest
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum GeometryTargetIdentityFactError {
    BindingDeclarationDenied(PrimitiveBindingAuthoringError),
    AnchorBindingDeclarationDenied(PrimitiveAnchorBindingAuthoringError),
    OutcomeNotBound {
        kind: ForgeQueryOrdinaryPostureKind,
        reason: String,
        next_step: ForgeQueryOrdinaryNextStep,
    },
}

impl GeometryTargetIdentityFactError {
    fn outcome_not_bound(posture: &ForgeQueryOrdinaryPosture) -> Self {
        Self::OutcomeNotBound {
            kind: posture.kind(),
            reason: posture.reason().to_string(),
            next_step: posture.next_step(),
        }
    }
}

pub(crate) fn binding_target_identity_from_envelope(
    payload: &PrimitiveBindingTargetIdentityPayload,
    envelope: &ForgeQueryDeclarationEnvelope<
        PrimitiveBindingQueryDomain,
        PrimitiveBindingDeclarationEntry,
    >,
) -> Result<GeometryTargetIdentityFactReceipt, GeometryTargetIdentityFactError> {
    Ok(GeometryTargetIdentityFactReceipt::new(
        payload.target_identity().to_string(),
        geometry_target_kind(payload.target_kind()),
        GeometryTargetSourceAuthority::PrimitiveBindingDeclarationEnvelope,
        envelope.declaration_digest().to_string(),
        payload.alias_identities().to_vec(),
    ))
}

pub(crate) fn anchor_binding_target_identity_from_envelope(
    payload: &PrimitiveAnchorBindingTargetIdentityPayload,
    envelope: &ForgeQueryDeclarationEnvelope<
        PrimitiveAnchorBindingQueryDomain,
        PrimitiveAnchorBindingDeclarationEntry,
    >,
) -> Result<GeometryTargetIdentityFactReceipt, GeometryTargetIdentityFactError> {
    Ok(GeometryTargetIdentityFactReceipt::new(
        payload.target_identity().to_string(),
        geometry_target_kind(payload.target_kind()),
        GeometryTargetSourceAuthority::PrimitiveAnchorBindingDeclarationEnvelope,
        envelope.declaration_digest().to_string(),
        payload.alias_identities().to_vec(),
    ))
}

pub fn primitive_binding_geometry_target_identity<C>(
    declaration: &PrimitiveBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
) -> Result<GeometryTargetIdentityFactReceipt, GeometryTargetIdentityFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => declaration
            .target_identity_payload()
            .map_err(GeometryTargetIdentityFactError::BindingDeclarationDenied)
            .and_then(|payload| binding_target_identity_from_envelope(payload, &envelope)),
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(GeometryTargetIdentityFactError::outcome_not_bound(&posture))
        }
    }
}

pub fn primitive_anchor_binding_geometry_target_identity<C>(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
) -> Result<GeometryTargetIdentityFactReceipt, GeometryTargetIdentityFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(envelope) => declaration
            .target_identity_payload()
            .map_err(GeometryTargetIdentityFactError::AnchorBindingDeclarationDenied)
            .and_then(|payload| anchor_binding_target_identity_from_envelope(payload, &envelope)),
        ForgeQueryOrdinaryOutcome::Ambiguous(posture)
        | ForgeQueryOrdinaryOutcome::AspectConflict(posture)
        | ForgeQueryOrdinaryOutcome::AuthorityMismatch(posture)
        | ForgeQueryOrdinaryOutcome::BasisMismatch(posture)
        | ForgeQueryOrdinaryOutcome::Deferred(posture)
        | ForgeQueryOrdinaryOutcome::Denied(posture)
        | ForgeQueryOrdinaryOutcome::ExplicitNarrowingRequired(posture)
        | ForgeQueryOrdinaryOutcome::Failed(posture)
        | ForgeQueryOrdinaryOutcome::MissingRequiredAspect(posture)
        | ForgeQueryOrdinaryOutcome::RebindRequired(posture)
        | ForgeQueryOrdinaryOutcome::Refused(posture)
        | ForgeQueryOrdinaryOutcome::Stale(posture)
        | ForgeQueryOrdinaryOutcome::Unavailable(posture)
        | ForgeQueryOrdinaryOutcome::Unsupported(posture)
        | ForgeQueryOrdinaryOutcome::WrongHandle(posture)
        | ForgeQueryOrdinaryOutcome::WrongWorld(posture) => {
            Err(GeometryTargetIdentityFactError::outcome_not_bound(&posture))
        }
    }
}

fn digest_parts(parts: &[String]) -> String {
    truth_digest_parts(TruthDigestScope::ArtifactIdentity, parts)
}

fn geometry_target_kind(kind: CanonicalGeometryTargetKind) -> GeometryTargetKind {
    match kind {
        CanonicalGeometryTargetKind::FaceSurface => GeometryTargetKind::FaceSurface,
        CanonicalGeometryTargetKind::EdgeCurve => GeometryTargetKind::EdgeCurve,
        CanonicalGeometryTargetKind::CoedgePCurve => GeometryTargetKind::CoedgePCurve,
        CanonicalGeometryTargetKind::VertexGeometry => GeometryTargetKind::VertexGeometry,
        CanonicalGeometryTargetKind::FaceSurfacePointAnchor => {
            GeometryTargetKind::FaceSurfacePointAnchor
        }
        CanonicalGeometryTargetKind::EdgeCurvePointAnchor => {
            GeometryTargetKind::EdgeCurvePointAnchor
        }
        CanonicalGeometryTargetKind::CoedgePCurvePointAnchor => {
            GeometryTargetKind::CoedgePCurvePointAnchor
        }
        CanonicalGeometryTargetKind::FaceSurfaceDirectionAnchor => {
            GeometryTargetKind::FaceSurfaceDirectionAnchor
        }
        CanonicalGeometryTargetKind::EdgeCurveDirectionAnchor => {
            GeometryTargetKind::EdgeCurveDirectionAnchor
        }
        CanonicalGeometryTargetKind::CoedgePCurveDirectionAnchor => {
            GeometryTargetKind::CoedgePCurveDirectionAnchor
        }
    }
}
