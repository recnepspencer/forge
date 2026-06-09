use forge_query::facade::{
    ForgeQueryApplicationFacade, ForgeQueryDeclarationRouteIntent,
    ForgeQueryDeclarationRoutePlanDenialCause, ForgeQueryDeclarationRoutePlanTerminalError,
};
use worth_geom::facade::{ParameterDomain, ParameterSpacePoint};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_anchor_binding_declaration, author_primitive_binding_declaration,
    author_primitive_rebinding_declaration, AnchorCarrierOwnership,
    AuthorPrimitiveAnchorBindingIntent, AuthorPrimitiveBindingIntent,
    CarrierOwnedParameterPointAnchorSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveAnchorBindingQueryDomain, PrimitiveAnchorBindingQueryWorld,
    PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld, PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld, ReplacementCandidateSet,
};

use super::support::{
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
};

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
fn primitive_binding_route_posture_fails_closed_for_bridge_only_intent() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let entry = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-route-binding"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new("binding-route-posture"))
        .validate()
        .expect("validated binding route handle")
        .admit()
        .expect("admitted binding route handle");
    let progressed = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("binding progression"));

    match handle.plan_routes_from_progressed_with_intent(
        progressed,
        ForgeQueryDeclarationRouteIntent::BridgeOnly,
    ) {
        Err(ForgeQueryDeclarationRoutePlanTerminalError::Denied(denial)) => {
            assert_eq!(
                denial.cause(),
                ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
            );
            assert_eq!(
                denial.route_contract().reason(),
                "the declaration lowers through one relational route"
            );
        }
        _ => panic!("bridge-only route intent should deny for relational-only binding"),
    }
}

#[test]
fn primitive_anchor_binding_route_posture_fails_closed_for_bridge_only_intent() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let entry = author_primitive_anchor_binding_declaration(
        AuthorPrimitiveAnchorBindingIntent::attach_parameter_space_point_to_face(
            FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-route-anchor"),
                contract,
                plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            ),
            CarrierOwnedParameterPointAnchorSpec::new(
                AnchorCarrierOwnership::for_face_surface(
                    "face-route-anchor",
                    ParameterDomain::plane(),
                )
                .expect("carrier ownership"),
                ParameterSpacePoint::try_new([0.25, 0.5]).expect("anchor point"),
            )
            .expect("anchor spec"),
        ),
    );
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveAnchorBindingQueryDomain)
        .with_operating_context(PrimitiveAnchorBindingQueryWorld::new(
            "anchor-route-posture",
        ))
        .validate()
        .expect("validated anchor route handle")
        .admit()
        .expect("admitted anchor route handle");
    let progressed = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("anchor progression"));

    match handle.plan_routes_from_progressed_with_intent(
        progressed,
        ForgeQueryDeclarationRouteIntent::BridgeOnly,
    ) {
        Err(ForgeQueryDeclarationRoutePlanTerminalError::Denied(denial)) => {
            assert_eq!(
                denial.cause(),
                ForgeQueryDeclarationRoutePlanDenialCause::NoAllowedRoutes
            );
            assert_eq!(
                denial.route_contract().reason(),
                "the declaration lowers through one relational route"
            );
        }
        _ => panic!("bridge-only route intent should deny for relational-only anchor binding"),
    }
}

#[test]
fn primitive_rebinding_route_posture_admits_bridge_only_intent() {
    let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
        &PrimitiveWitnessDescriptor::Orthotope,
    );
    let prior_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-route-rebinding-old"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let successor_declaration = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-route-rebinding-new"),
            contract,
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        )),
    );
    let entry = author_primitive_rebinding_declaration(
        crate::binding::tests::support::replace_surface_binding(
            rebinding_prior_fact_from_binding_declaration(
                &prior_declaration,
                "route-posture-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-route-rebinding-old",
                ReplacementCandidateSet::new(vec![rebinding_candidate_from_binding_declaration(
                    "successor",
                    &successor_declaration,
                    "route-posture-successor",
                )
                .expect("candidate")])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let handle = ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new("rebinding-route-posture"))
        .validate()
        .expect("validated rebinding route handle")
        .admit()
        .expect("admitted rebinding route handle");
    let progressed = handle
        .declare_review_and_progress(entry.clone())
        .unwrap_or_else(|_| panic!("rebinding progression"));

    match handle.plan_routes_from_progressed_with_intent(
        progressed,
        ForgeQueryDeclarationRouteIntent::BridgeOnly,
    ) {
        Ok(plan) => {
            assert_eq!(format!("{:?}", plan.class()), "BridgeOnly");
            assert_eq!(
                plan.route_families()
                    .iter()
                    .map(|family| family.as_str())
                    .collect::<Vec<_>>(),
                vec!["bridge"]
            );
            assert_eq!(
                plan.explain().route_contract_reason(),
                "the declaration may lower through both relational and bridge routes"
            );
        }
        Err(ForgeQueryDeclarationRoutePlanTerminalError::Denied(_)) => {
            panic!("bridge-only route intent should admit for rebinding bridge-capable routing")
        }
        Err(_) => panic!("unexpected rebinding route-plan result"),
    }
}
