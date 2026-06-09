#[cfg(test)]
mod tests {
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveCurvedSupportIdentity,
        PrimitiveGeometryIdentityBundle, PrimitiveSupportPlaneIdentity,
        PrimitiveTriaxialEllipsoidIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use crate::bindings::authority::{
        AdmittedPartialBindingPosture, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
        EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec,
        SpatialBindingAuthorityError, SpatialBindingCompleteness, SpatialBindingIllegalityReason,
        SpatialBindingIncompleteness, SpatialBindingKind, SpatialBindingUnsupportedReason,
        VertexBindingSite, VertexGeometryBindingSpec, VertexGeometryProvenanceKind,
        VertexToleranceRegime,
    };
    use crate::bindings::query_native_binding_authoring::{
        author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
        PrimitiveBindingAuthoringError,
    };
    use crate::bindings::query_native_declared_target_identity_fact::binding_declaration_fact;
    use crate::bindings::query_native_target_identity::GeometryTargetIdentityFactError;

    #[test]
    fn completeness_policy_distinguishes_complete_partial_unsupported_and_illegal() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![3],
            },
        );
        let unsupported_contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::WireBody { edge_count: 4 },
        );
        let planar_but_vertexless = PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![],
        );
        let single_vertex_curve = PrimitiveGeometryIdentityBundle::new(
            vec![],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        );
        let complete_geometry = PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![
                PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
                PrimitiveVertexIdentity::from_position([1.0, 0.0, 0.0]),
            ],
        );
        let asymmetric_surface_geometry = PrimitiveGeometryIdentityBundle::with_curved_support(
            vec![],
            vec![PrimitiveCurvedSupportIdentity::TriaxialEllipsoid(
                PrimitiveTriaxialEllipsoidIdentity::new(
                    [0.0, 0.0, 0.0],
                    [1.0, 0.0, 0.0],
                    [0.0, 1.0, 0.0],
                    [0.0, 0.0, 1.0],
                    5.0,
                    3.0,
                    2.0,
                ),
            )],
            vec![
                PrimitiveVertexIdentity::from_position([0.0, 0.0, 2.0]),
                PrimitiveVertexIdentity::from_position([5.0, 0.0, 0.0]),
            ],
        );

        let face = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
                contract,
                planar_but_vertexless.clone(),
            )),
        ))
        .expect("face binding fact");
        let edge = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_curve_to_edge(EdgeCurveBindingSpec::new(
                EdgeBindingSite::new("edge-1"),
                contract,
                single_vertex_curve.clone(),
            )),
        ))
        .expect("edge binding fact");
        let coedge = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-1"),
                contract,
                PrimitiveGeometryIdentityBundle::new(
                    vec![PrimitiveSupportPlaneIdentity::new(
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "0".to_string(),
                    )],
                    vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
                ),
            )),
        ))
        .expect("coedge binding fact");
        let vertex = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_vertex_geometry(VertexGeometryBindingSpec::new(
                VertexBindingSite::new("vertex-1"),
                contract,
                complete_geometry,
                VertexGeometryProvenanceKind::CanonicalWitness,
                VertexToleranceRegime::ExactBits,
            )),
        ))
        .expect("vertex binding fact");
        let asymmetric_face = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-asymmetric"),
                contract,
                asymmetric_surface_geometry,
            )),
        ))
        .expect("asymmetric face binding fact");
        let unsupported = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-unsupported"),
                unsupported_contract,
                planar_but_vertexless,
            )),
        ));
        let illegal = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new(""),
                contract,
                PrimitiveGeometryIdentityBundle::new(vec![], vec![]),
            )),
        ));

        assert_eq!(
            face.completeness(),
            SpatialBindingCompleteness::AdmittedPartial(
                AdmittedPartialBindingPosture::FaceSurfaceMissingVertexGeometry,
            )
        );
        assert_eq!(
            edge.completeness(),
            SpatialBindingCompleteness::AdmittedPartial(
                AdmittedPartialBindingPosture::EdgeCurveSingleVertexWitness,
            )
        );
        assert_eq!(
            coedge.completeness(),
            SpatialBindingCompleteness::AdmittedPartial(
                AdmittedPartialBindingPosture::CoedgePCurveSingleVertexWitness,
            )
        );
        assert_eq!(
            asymmetric_face.completeness(),
            SpatialBindingCompleteness::Complete
        );
        assert_eq!(vertex.completeness(), SpatialBindingCompleteness::Complete);
        assert!(matches!(
            unsupported,
            Err(GeometryTargetIdentityFactError::BindingDeclarationDenied(
                PrimitiveBindingAuthoringError::Spatial(SpatialBindingAuthorityError::Unsupported(
                    SpatialBindingUnsupportedReason::TopologyBirthClassDoesNotAdmitBindingKind {
                        binding_kind: SpatialBindingKind::FaceSurface,
                        topology_birth_class: "planar_wire_body",
                    },
                )),
            ))
        ));
        assert!(matches!(
            illegal,
            Err(GeometryTargetIdentityFactError::BindingDeclarationDenied(
                PrimitiveBindingAuthoringError::Spatial(SpatialBindingAuthorityError::Illegal(
                    SpatialBindingIllegalityReason::MissingTopologyIdentity(
                        SpatialBindingKind::FaceSurface,
                    ),
                )),
            ))
        ));
    }

    #[test]
    fn completeness_denial_is_typed_before_it_can_be_upgraded() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![3],
            },
        );

        let face = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1"),
                contract,
                PrimitiveGeometryIdentityBundle::new(
                    vec![],
                    vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
                ),
            )),
        ))
        .expect("face binding fact");
        let coedge = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
                CoedgeBindingSite::new("coedge-1"),
                contract,
                PrimitiveGeometryIdentityBundle::new(
                    vec![PrimitiveSupportPlaneIdentity::new(
                        "0".to_string(),
                        "0".to_string(),
                        "1".to_string(),
                        "0".to_string(),
                    )],
                    vec![],
                ),
            )),
        ))
        .expect("coedge binding fact");

        assert_eq!(
            face.completeness(),
            SpatialBindingCompleteness::DeniedIncomplete(
                SpatialBindingIncompleteness::FaceSurfaceMissingSupportCarrier,
            )
        );
        assert_eq!(
            coedge.completeness(),
            SpatialBindingCompleteness::DeniedIncomplete(
                SpatialBindingIncompleteness::CoedgePCurveMissingCurveWitnessVertices,
            )
        );
    }
}
