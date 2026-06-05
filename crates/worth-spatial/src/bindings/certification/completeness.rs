#[cfg(test)]
mod tests {
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
        PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use crate::bindings::authority::{
        attach_curve_to_edge, attach_pcurve_to_coedge, attach_surface_to_face,
        attach_vertex_geometry, CoedgeBindingSite, CoedgePCurveBindingSpec, EdgeBindingSite,
        EdgeCurveBindingSpec, FaceBindingSite, FaceSurfaceBindingSpec, SpatialBindingCompleteness,
        SpatialBindingIncompleteness, VertexBindingSite, VertexGeometryBindingSpec,
        VertexGeometryProvenanceKind, VertexToleranceRegime,
    };

    #[test]
    fn completeness_is_explicit_per_binding_family() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::ShellWithHole {
                outer_loop_edge_count: 4,
                hole_loop_edge_counts: vec![3],
            },
        );
        let empty_geometry = PrimitiveGeometryIdentityBundle::new(vec![], vec![]);
        let minimal_curve = PrimitiveGeometryIdentityBundle::new(
            vec![],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        );

        let face = attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1"),
            contract,
            empty_geometry.clone(),
        ))
        .expect("face binding");
        let edge = attach_curve_to_edge(EdgeCurveBindingSpec::new(
            EdgeBindingSite::new("edge-1"),
            contract,
            minimal_curve.clone(),
        ))
        .expect("edge binding");
        let coedge = attach_pcurve_to_coedge(CoedgePCurveBindingSpec::new(
            CoedgeBindingSite::new("coedge-1"),
            contract,
            minimal_curve,
        ))
        .expect("coedge binding");
        let vertex = attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1"),
            contract,
            empty_geometry,
            VertexGeometryProvenanceKind::CanonicalWitness,
            VertexToleranceRegime::ExactBits,
        ))
        .expect("vertex binding");

        assert_eq!(
            face.completeness(),
            &SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::MissingSupportPlane
            )
        );
        assert_eq!(
            edge.completeness(),
            &SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::CurveWitnessRequiresAtLeastTwoVertices
            )
        );
        assert_eq!(
            coedge.completeness(),
            &SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::PCurveWitnessRequiresPlanarSupport
            )
        );
        assert_eq!(
            vertex.completeness(),
            &SpatialBindingCompleteness::Incomplete(
                SpatialBindingIncompleteness::MissingVertexGeometry
            )
        );
    }

    #[test]
    fn unsupported_topology_birth_class_fails_before_completeness_folklore() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::WireBody { edge_count: 4 },
        );
        let geometry = PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "0".to_string(),
            )],
            vec![PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0])],
        );

        let face = attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1"),
            contract,
            geometry.clone(),
        ));
        let vertex = attach_vertex_geometry(VertexGeometryBindingSpec::new(
            VertexBindingSite::new("vertex-1"),
            contract,
            geometry,
            VertexGeometryProvenanceKind::RealizedVertex,
            VertexToleranceRegime::AdmittedTolerance,
        ));

        assert!(face.is_err());
        assert!(vertex.is_ok());
    }
}
