use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_binding_rebinding_candidate_fact, primitive_binding_rebinding_prior_binding_fact,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, ReplacementCandidate, ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    TopologyNeighborhoodReplacementFactReceipt,
};

use super::runtime_handles::{binding_handle, rebinding_handle};

pub(crate) fn local_neighborhood_receipt(
    world: &'static str,
) -> TopologyNeighborhoodReplacementFactReceipt {
    replacement_neighborhood_receipt(
        world,
        "face-local-rebuild-old",
        vec![
            (
                "successor-a",
                "face-local-rebuild-new-a",
                [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
            ),
            (
                "successor-b",
                "face-local-rebuild-new-b",
                [[0.0, 0.0, 0.0], [1.0, 1.0, 0.0]],
            ),
        ],
    )
}

pub(crate) fn single_candidate_local_neighborhood_receipt(
    world: &'static str,
) -> TopologyNeighborhoodReplacementFactReceipt {
    replacement_neighborhood_receipt(
        world,
        "face-local-rebuild-single-old",
        vec![(
            "single-successor",
            "face-local-rebuild-single-new",
            [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]],
        )],
    )
}

fn replacement_neighborhood_receipt(
    world: &'static str,
    prior_site: &'static str,
    successors: Vec<(&'static str, &'static str, [[f64; 3]; 2])>,
) -> TopologyNeighborhoodReplacementFactReceipt {
    let prior = surface_binding(prior_site, [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let prior_fact = primitive_binding_rebinding_prior_binding_fact(&prior, &binding_handle(world))
        .expect("prior rebinding fact");
    let candidates = successors
        .into_iter()
        .map(|(label, successor_site, vertices)| {
            let successor = surface_binding(successor_site, vertices);
            candidate(label, &successor, world)
        })
        .collect::<Vec<_>>();
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        prior_site,
        ReplacementCandidateSet::new(candidates).expect("candidate set"),
    )
    .expect("local neighborhood");
    let declaration = author_primitive_rebinding_declaration(
        AuthorPrimitiveRebindingIntent::replace_surface_binding(prior_fact, neighborhood),
    );
    let handle = rebinding_handle(world);
    let replacement_entry = topology_neighborhood_replacement_entry(
        primitive_rebinding_neighborhood_replacement_source(&declaration, &handle)
            .expect("replacement source"),
    );
    primitive_rebinding_neighborhood_replacement_facts(&replacement_entry, &handle)
        .expect("replacement facts")
}

fn candidate(
    label: &'static str,
    declaration: &PrimitiveBindingDeclarationEntry,
    world: &'static str,
) -> ReplacementCandidate {
    ReplacementCandidate::new(
        label,
        primitive_binding_rebinding_candidate_fact(declaration, &binding_handle(world))
            .expect("candidate fact"),
    )
    .expect("candidate")
}

fn surface_binding(
    site: &'static str,
    vertices: [[f64; 3]; 2],
) -> PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(site),
            PrimitiveConstructionFamilyContractRegistry::contract_for(
                &PrimitiveWitnessDescriptor::Orthotope,
            ),
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
            ),
        ),
    ))
}
