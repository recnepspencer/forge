#![cfg(test)]

use forge_query::facade::ForgeQueryApplicationFacade;

use crate::bindings::authority::{
    CoedgeBindingSite, CoedgePCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
    VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
    VertexToleranceRegime,
};
use crate::bindings::query_native_rebinding::{
    PrimitiveRebindingQueryDomain, PrimitiveRebindingQueryWorld,
};
use crate::bindings::query_native_rebinding_authoring::{
    author_primitive_rebinding_declaration, AuthorPrimitiveRebindingIntent,
};
use crate::bindings::rebinding::{
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidateSet,
};
use crate::facade::bindings::{
    author_primitive_binding_declaration, primitive_rebinding_retained_fact_source,
    AuthorPrimitiveBindingIntent,
};
use crate::facade::recovery::{
    geometry_recovery_action_entry, primitive_rebinding_geometry_recovery_action,
    GeometryRecoveryAction, GeometryRecoveryTargetScope,
};
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};

fn admitted_rebinding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(world))
        .validate()
        .expect("rebinding query handle should validate")
        .admit()
        .expect("rebinding query handle should admit")
}

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

fn surface_binding_declaration(
    face_id: &str,
    name: &str,
    vertices: [[f64; 3]; 2],
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(face_id).with_persistent_name(name),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry(vertices),
        ),
    ))
}

fn pcurve_binding_declaration(
    coedge_id: &str,
    vertices: [[f64; 3]; 2],
    contract: worth_primitives::PrimitiveConstructionBirthSynopsisContract,
) -> crate::bindings::query_native_binding_authoring::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(
        CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new(coedge_id),
            contract,
            plane_geometry(vertices),
        ),
    ))
}

#[test]
fn geometry_recovery_action_keeps_ambiguous_rebinding_typed() {
    let prior = surface_binding_declaration(
        "face-old",
        "surface-alpha",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
    );
    let a =
        surface_binding_declaration("face-a", "surface-beta", [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]]);
    let b = surface_binding_declaration(
        "face-b",
        "surface-gamma",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
    );
    let entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            super::rebinding_prior_fact_from_binding_declaration(
                &prior,
                "recovery-ambiguous-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::FaceSurface,
                "face-old",
                ReplacementCandidateSet::new(vec![
                    super::rebinding_candidate_from_binding_declaration(
                        "a",
                        &a,
                        "recovery-ambiguous-a",
                    )
                    .expect("candidate a"),
                    super::rebinding_candidate_from_binding_declaration(
                        "b",
                        &b,
                        "recovery-ambiguous-b",
                    )
                    .expect("candidate b"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );

    let handle = admitted_rebinding_handle("recovery-ambiguous");
    let receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&entry, &handle)
                .expect("retained fact source"),
        ),
        &handle,
    )
    .expect("recovery receipt");

    assert_eq!(
        receipt.recovery_action_kind(),
        GeometryRecoveryAction::NarrowInput
    );
    assert_eq!(
        receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::InputNarrowing
    );
}

#[test]
fn geometry_recovery_action_keeps_orphaned_and_unsupported_rebinding_typed() {
    let prior = pcurve_binding_declaration(
        "coedge-old",
        [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 6,
                hole_loop_edge_counts: vec![3],
            },
        ),
    );
    let orphaned = pcurve_binding_declaration(
        "coedge-new",
        [[0.0, 0.0, 0.0], [2.0, 0.0, 0.0]],
        PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        ),
    );
    let orphaned_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_pcurve_binding(
            super::rebinding_prior_fact_from_binding_declaration(&prior, "recovery-orphaned-prior"),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::CoedgePCurve,
                "coedge-old",
                ReplacementCandidateSet::new(vec![
                    super::rebinding_candidate_from_binding_declaration(
                        "orphaned",
                        &orphaned,
                        "recovery-orphaned-candidate",
                    )
                    .expect("candidate"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let orphaned_handle = admitted_rebinding_handle("recovery-orphaned");
    let orphaned_receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&orphaned_entry, &orphaned_handle)
                .expect("retained fact source"),
        ),
        &orphaned_handle,
    )
    .expect("orphaned recovery receipt");
    assert_eq!(
        orphaned_receipt.recovery_action_kind(),
        GeometryRecoveryAction::RebindContext
    );
    assert_eq!(
        orphaned_receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::TruthContinuationContext
    );

    let vertex_prior = author_primitive_binding_declaration(
        AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-old"),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
            plane_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        )),
    );
    let unsupported_entry = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(
            super::rebinding_prior_fact_from_binding_declaration(
                &vertex_prior,
                "recovery-unsupported-prior",
            ),
            LocalTopologyReplacementNeighborhood::new(
                NeighborhoodBindingFamily::VertexGeometry,
                "vertex-old",
                ReplacementCandidateSet::new(vec![
                    super::rebinding_candidate_from_binding_declaration(
                        "vertex",
                        &vertex_prior,
                        "recovery-unsupported-candidate",
                    )
                    .expect("candidate"),
                ])
                .expect("candidate set"),
            )
            .expect("neighborhood"),
        ),
    );
    let unsupported_handle = admitted_rebinding_handle("recovery-unsupported");
    let unsupported_receipt = primitive_rebinding_geometry_recovery_action(
        &geometry_recovery_action_entry(
            primitive_rebinding_retained_fact_source(&unsupported_entry, &unsupported_handle)
                .expect("retained fact source"),
        ),
        &unsupported_handle,
    )
    .expect("unsupported recovery receipt");
    assert_eq!(
        unsupported_receipt.recovery_action_kind(),
        GeometryRecoveryAction::CheckSupport
    );
    assert_eq!(
        unsupported_receipt.recovery_target_scope(),
        GeometryRecoveryTargetScope::SupportReadiness
    );
}
