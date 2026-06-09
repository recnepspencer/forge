#[cfg(test)]
mod tests {
    use worth_primitives::{
        PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
        PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
    };

    use crate::bindings::authority::{FaceBindingSite, FaceSurfaceBindingSpec};
    use crate::bindings::query_native_binding_authoring::{
        author_primitive_binding_declaration, AuthorPrimitiveBindingIntent,
    };
    use crate::bindings::query_native_declared_target_identity_fact::binding_declaration_fact;

    #[test]
    fn binding_identity_diverges_from_topology_and_naming_when_geometry_changes() {
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

        let first = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                site.clone(),
                contract,
                first_geometry,
            )),
        ))
        .expect("first binding fact");
        let second = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                site,
                contract,
                second_geometry,
            )),
        ))
        .expect("second binding fact");

        assert_ne!(
            first.binding_identity().as_str(),
            second.binding_identity().as_str()
        );
        assert_eq!(first.site_identity(), "face-1");
        assert_eq!(second.site_identity(), "face-1");
    }

    #[test]
    fn binding_identity_is_stable_under_equivalent_authoring_order_variation() {
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

        let first = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("alpha"),
                contract,
                geometry.clone(),
            )),
        ))
        .expect("first binding fact");
        let second = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_surface_to_face(FaceSurfaceBindingSpec::new(
                FaceBindingSite::new("face-1").with_persistent_name("beta"),
                contract,
                geometry,
            )),
        ))
        .expect("second binding fact");

        assert_eq!(
            first.binding_identity().as_str(),
            second.binding_identity().as_str()
        );
    }

    #[test]
    fn vertex_binding_identity_diverges_when_semantic_identity_inputs_change() {
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

        let canonical = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_vertex_geometry(
                crate::bindings::authority::VertexGeometryBindingSpec::new(
                    crate::bindings::authority::VertexBindingSite::new("vertex-1")
                        .with_persistent_name("same-name"),
                    contract,
                    geometry.clone(),
                    crate::bindings::authority::VertexGeometryProvenanceKind::CanonicalWitness,
                    crate::bindings::authority::VertexToleranceRegime::ExactBits,
                ),
            ),
        ))
        .expect("canonical vertex binding fact");
        let realized = binding_declaration_fact(&author_primitive_binding_declaration(
            AuthorPrimitiveBindingIntent::attach_vertex_geometry(
                crate::bindings::authority::VertexGeometryBindingSpec::new(
                    crate::bindings::authority::VertexBindingSite::new("vertex-1")
                        .with_persistent_name("same-name"),
                    contract,
                    geometry,
                    crate::bindings::authority::VertexGeometryProvenanceKind::RealizedVertex,
                    crate::bindings::authority::VertexToleranceRegime::AdmittedTolerance,
                ),
            ),
        ))
        .expect("realized vertex binding fact");

        assert_ne!(
            canonical.binding_identity().as_str(),
            realized.binding_identity().as_str()
        );
    }
}
