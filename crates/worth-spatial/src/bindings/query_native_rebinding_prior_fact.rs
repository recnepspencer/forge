use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native::{
    PrimitiveAnchorBindingQueryDomain, PrimitiveBindingQueryDomain,
};
use crate::bindings::query_native_anchor_binding_authoring::{
    PrimitiveAnchorBindingAuthoringError, PrimitiveAnchorBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_authoring::{
    PrimitiveBindingAuthoringError, PrimitiveBindingDeclarationEntry,
};
use crate::bindings::query_native_rebinding_declared_binding_fact::DeclaredNeighborhoodBindingFact;
use crate::bindings::rebinding::binding_snapshot::BindingSnapshot;
use crate::bindings::rebinding::NeighborhoodBindingFamily;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryOutcome,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveRebindingPriorBindingFact {
    binding_kind: SpatialBindingKind,
    prior_binding_identity: String,
    prior_site_identity: String,
    family: NeighborhoodBindingFamily,
    snapshot: BindingSnapshot,
}

impl PrimitiveRebindingPriorBindingFact {
    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn prior_binding_identity(&self) -> &str {
        &self.prior_binding_identity
    }

    pub fn prior_site_identity(&self) -> &str {
        &self.prior_site_identity
    }

    pub fn family(&self) -> NeighborhoodBindingFamily {
        self.family
    }

    pub(crate) fn snapshot(&self) -> &BindingSnapshot {
        &self.snapshot
    }

    pub(crate) fn from_neighborhood_binding_fact(fact: &DeclaredNeighborhoodBindingFact) -> Self {
        Self {
            binding_kind: fact.binding_kind(),
            prior_binding_identity: fact.binding_identity().to_string(),
            prior_site_identity: fact.site_identity().to_string(),
            family: fact.family(),
            snapshot: fact.snapshot().clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveRebindingPriorBindingFactError {
    Binding(PrimitiveBindingAuthoringError),
    Anchor(PrimitiveAnchorBindingAuthoringError),
    QueryNotBound,
}

pub fn primitive_binding_rebinding_prior_binding_fact<C>(
    declaration: &PrimitiveBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
) -> Result<PrimitiveRebindingPriorBindingFact, PrimitiveRebindingPriorBindingFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(_) => declaration
            .rebinding_prior_binding_fact()
            .map_err(PrimitiveRebindingPriorBindingFactError::Binding)
            .cloned(),
        _ => Err(PrimitiveRebindingPriorBindingFactError::QueryNotBound),
    }
}

pub fn primitive_anchor_binding_rebinding_prior_binding_fact<C>(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
) -> Result<PrimitiveRebindingPriorBindingFact, PrimitiveRebindingPriorBindingFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(_) => declaration
            .rebinding_prior_binding_fact()
            .map_err(PrimitiveRebindingPriorBindingFactError::Anchor)
            .cloned(),
        _ => Err(PrimitiveRebindingPriorBindingFactError::QueryNotBound),
    }
}

#[cfg(test)]
mod tests {
    use forge_query::facade::ForgeQueryApplicationFacade;
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
        PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use super::*;
    use crate::bindings::anchors::{AnchorCarrierOwnership, CarrierOwnedParameterPointAnchorSpec};
    use crate::bindings::authority::{
        EdgeBindingSite, EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    };
    use crate::bindings::query_native::{
        PrimitiveAnchorBindingQueryWorld, PrimitiveBindingQueryWorld,
    };
    use crate::bindings::query_native_anchor_binding_authoring::{
        author_primitive_anchor_binding_declaration, AuthorPrimitiveAnchorBindingIntent,
    };
    use crate::bindings::query_native_binding_authoring::{
        author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
    };
    use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};

    fn plane_geometry(vertices: [[f64; 3]; 2]) -> PrimitiveGeometryIdentityBundle {
        PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vertices
                .into_iter()
                .map(PrimitiveVertexIdentity::from_position)
                .collect(),
        )
    }

    #[test]
    fn binding_query_handle_builds_rebinding_prior_binding_fact() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        );
        let declaration = author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-rebinding-prior"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            )),
        );
        let handle = ForgeQueryApplicationFacade::runtime_backed_default()
            .domain(PrimitiveBindingQueryDomain)
            .with_operating_context(PrimitiveBindingQueryWorld::new(
                "rebinding-prior-binding-fact",
            ))
            .validate()
            .expect("validated handle")
            .admit()
            .expect("admitted handle");

        let fact = primitive_binding_rebinding_prior_binding_fact(&declaration, &handle)
            .expect("prior fact");

        assert_eq!(fact.binding_kind(), SpatialBindingKind::FaceSurface);
        assert_eq!(fact.prior_site_identity(), "face-rebinding-prior");
        assert_eq!(fact.family(), NeighborhoodBindingFamily::FaceSurface);
    }

    #[test]
    fn anchor_query_handle_builds_rebinding_prior_binding_fact() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        );
        let declaration = author_primitive_anchor_binding_declaration(
            AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_edge(
                EdgeCurveBindingSpec::new(
                    EdgeBindingSite::new("edge-rebinding-prior"),
                    contract,
                    plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
                ),
                CarrierOwnedParameterPointAnchorSpec::new(
                    AnchorCarrierOwnership::for_edge_curve(
                        "edge-rebinding-prior",
                        ParameterDomain::plane(),
                    )
                    .expect("ownership"),
                    ParameterSpacePoint::try_new([0.25, 0.0]).expect("parameter"),
                )
                .expect("anchor"),
            ),
        );
        let handle = ForgeQueryApplicationFacade::runtime_backed_default()
            .domain(PrimitiveAnchorBindingQueryDomain)
            .with_operating_context(PrimitiveAnchorBindingQueryWorld::new(
                "rebinding-prior-anchor-fact",
            ))
            .validate()
            .expect("validated handle")
            .admit()
            .expect("admitted handle");

        let fact = primitive_anchor_binding_rebinding_prior_binding_fact(&declaration, &handle)
            .expect("prior fact");

        assert_eq!(fact.binding_kind(), SpatialBindingKind::EdgeCurve);
        assert_eq!(fact.prior_site_identity(), "edge-rebinding-prior");
        assert_eq!(
            fact.family(),
            NeighborhoodBindingFamily::EdgeCurvePointAnchor
        );
    }
}
