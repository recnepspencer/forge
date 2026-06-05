#[cfg(test)]
mod tests {
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
        PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use crate::bindings::authority::{
        attach_surface_to_face, FaceBindingSite, FaceSurfaceBindingSpec,
    };

    #[test]
    fn binding_identity_changes_with_geometry_not_topology_or_name() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        );
        let first_geometry = PrimitiveGeometryIdentityBundle::new(
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
        let second_geometry = PrimitiveGeometryIdentityBundle::new(
            vec![PrimitiveSupportPlaneIdentity::new(
                "0".to_string(),
                "0".to_string(),
                "1".to_string(),
                "1".to_string(),
            )],
            vec![
                PrimitiveVertexIdentity::from_position([0.0, 0.0, 0.0]),
                PrimitiveVertexIdentity::from_position([2.0, 0.0, 0.0]),
            ],
        );
        let site = FaceBindingSite::new("face-1").with_persistent_name("same-name");

        let first = attach_surface_to_face(FaceSurfaceBindingSpec::new(
            site.clone(),
            contract,
            first_geometry,
        ))
        .expect("first binding");
        let second =
            attach_surface_to_face(FaceSurfaceBindingSpec::new(site, contract, second_geometry))
                .expect("second binding");

        assert_ne!(first.identity().as_str(), second.identity().as_str());
        assert_eq!(first.site().persistent_name(), Some("same-name"));
        assert_eq!(second.site().persistent_name(), Some("same-name"));
    }

    #[test]
    fn equivalent_binding_meaning_is_order_insensitive() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
        );
        let geometry = PrimitiveGeometryIdentityBundle::new(
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

        let first = attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("alpha"),
            contract,
            geometry.clone(),
        ))
        .expect("first binding");
        let second = attach_surface_to_face(FaceSurfaceBindingSpec::new(
            FaceBindingSite::new("face-1").with_persistent_name("beta"),
            contract,
            geometry,
        ))
        .expect("second binding");

        assert_eq!(first.identity().as_str(), second.identity().as_str());
    }

    #[test]
    fn vertex_binding_identity_includes_provenance_and_tolerance_regime() {
        let contract = PrimitiveConstructionFamilyContractRegistry::contract_for(
            &PrimitiveWitnessDescriptor::Orthotope,
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

        let canonical = crate::bindings::authority::attach_vertex_geometry(
            crate::bindings::authority::VertexGeometryBindingSpec::new(
                crate::bindings::authority::VertexBindingSite::new("vertex-1")
                    .with_persistent_name("same-name"),
                contract,
                geometry.clone(),
                crate::bindings::authority::VertexGeometryProvenanceKind::CanonicalWitness,
                crate::bindings::authority::VertexToleranceRegime::ExactBits,
            ),
        )
        .expect("canonical vertex binding");
        let realized = crate::bindings::authority::attach_vertex_geometry(
            crate::bindings::authority::VertexGeometryBindingSpec::new(
                crate::bindings::authority::VertexBindingSite::new("vertex-1")
                    .with_persistent_name("same-name"),
                contract,
                geometry,
                crate::bindings::authority::VertexGeometryProvenanceKind::RealizedVertex,
                crate::bindings::authority::VertexToleranceRegime::AdmittedTolerance,
            ),
        )
        .expect("realized vertex binding");

        assert_ne!(canonical.identity().as_str(), realized.identity().as_str());
    }
}
