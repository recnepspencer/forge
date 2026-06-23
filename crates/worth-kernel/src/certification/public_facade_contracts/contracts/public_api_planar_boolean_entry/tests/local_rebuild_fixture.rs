use forge_query::facade::ForgeQueryApplicationFacade;
use worth_primitives::{
    PrimitiveConstructionFamilyContractRegistry, PrimitiveGeometryIdentityBundle,
    PrimitiveSupportPlaneIdentity, PrimitiveVertexIdentity, PrimitiveWitnessDescriptor,
};
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    primitive_binding_rebinding_candidate_fact, primitive_binding_rebinding_prior_binding_fact,
    AuthorPrimitiveBindingIntent, AuthorPrimitiveRebindingIntent, FaceBindingSite,
    FaceSurfaceBindingSpec, LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily,
    PrimitiveBindingDeclarationEntry, PrimitiveBindingQueryDomain, PrimitiveBindingQueryWorld,
    PrimitiveRebindingQueryDomain, PrimitiveRebindingQueryWorld, ReplacementCandidate,
    ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
    TopologyNeighborhoodReplacementFactReceipt,
};
use worth_spatial::facade::planar_diagnostics::PlanarDiagnosticBundleReceipt;
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts,
    PlanarLocalRebuildParityQueryDomain, PlanarLocalRebuildParityQueryWorld,
    PlanarLocalRebuildScope, PlanarRebindingContinuityEvidence,
};
use worth_spatial::facade::planar_projection_consumption::ProjectionConsumedPlanarFactsReceipt;
use worth_spatial::facade::planar_recovery::PlanarRecoveryPostureReceipt;
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;

pub(crate) fn local_rebuild_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
    recovery: PlanarRecoveryPostureReceipt,
    diagnostics: PlanarDiagnosticBundleReceipt,
) -> worth_spatial::facade::planar_local_rebuild_parity::PlanarLocalRebuildParityReceipt {
    let neighborhood = local_neighborhood_receipt(world);
    let neighborhood_digest = neighborhood.fact_digest().to_string();

    PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "phase-1-boolean-local-rebuild",
    ))
    .local_neighborhood(neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "phase-1-boolean-rebinding-continuity",
        neighborhood_digest,
    ))
    .structural_identity(retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(retained.clone())
    .projection_consumed_planar_facts(projected)
    .motion_posture(retained.basis().motion_posture_receipt().clone())
    .topology_contract(retained.basis().topology_contract_receipt().clone())
    .recovery_posture(recovery)
    .diagnostics(diagnostics)
    .certify_same_planar_basis_across_views()
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(world),
    ))
    .expect("local rebuild parity plan")
    .certify()
    .expect("local rebuild parity receipt")
}

fn local_neighborhood_receipt(world: &'static str) -> TopologyNeighborhoodReplacementFactReceipt {
    let prior = surface_binding("phase1-planar-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let prior_fact = primitive_binding_rebinding_prior_binding_fact(&prior, &binding_handle(world))
        .expect("prior binding fact");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "phase1-planar-old",
        ReplacementCandidateSet::new(vec![
            candidate(
                "successor-a",
                &surface_binding("phase1-planar-new-a", [[0.0, 0.0, 0.0], [1.0, 1.0, 0.0]]),
                world,
            ),
            candidate(
                "successor-b",
                &surface_binding("phase1-planar-new-b", [[0.0, 0.0, 0.0], [1.0, -1.0, 0.0]]),
                world,
            ),
        ])
        .expect("replacement candidate set"),
    )
    .expect("replacement neighborhood");
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
            .expect("candidate binding fact"),
    )
    .expect("replacement candidate")
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

fn binding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveBindingQueryDomain,
    PrimitiveBindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveBindingQueryDomain)
        .with_operating_context(PrimitiveBindingQueryWorld::new(world))
        .validate()
        .expect("validated binding domain")
        .admit()
        .expect("admitted binding domain")
}

fn rebinding_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PrimitiveRebindingQueryDomain,
    PrimitiveRebindingQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PrimitiveRebindingQueryDomain)
        .with_operating_context(PrimitiveRebindingQueryWorld::new(world))
        .validate()
        .expect("validated rebinding domain")
        .admit()
        .expect("admitted rebinding domain")
}

fn local_rebuild_handle(
    world: &'static str,
) -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalRebuildParityQueryDomain,
    PlanarLocalRebuildParityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalRebuildParityQueryDomain)
        .with_operating_context(PlanarLocalRebuildParityQueryWorld::new(world))
        .validate()
        .expect("validated local rebuild domain")
        .admit()
        .expect("admitted local rebuild domain")
}
