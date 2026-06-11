use forge_query::facade::ForgeQueryApplicationFacade;
use worth_spatial::facade::bindings::{
    author_primitive_binding_declaration, author_primitive_rebinding_declaration,
    AuthorPrimitiveBindingIntent, FaceBindingSite, FaceSurfaceBindingSpec,
    LocalTopologyReplacementNeighborhood, NeighborhoodBindingFamily, ReplacementCandidateSet,
};
use worth_spatial::facade::neighborhood::{
    primitive_rebinding_neighborhood_replacement_facts,
    primitive_rebinding_neighborhood_replacement_source, topology_neighborhood_replacement_entry,
};
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld, PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts,
    PlanarLocalRebuildParityDenialKind, PlanarLocalRebuildParityQueryDomain,
    PlanarLocalRebuildParityQueryWorld, PlanarLocalRebuildScope, PlanarRebindingContinuityEvidence,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsQueryDomain, ProjectionConsumedPlanarFactsQueryWorld,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld, PlanarRecoverySource,
};

use super::super::bundle_closeout::contract_bundle::readiness_receipt;
use super::super::retained_views::projection_consumption::retained_planar_facts;
use crate::binding::tests::support::{
    admitted_rebinding_handle, canonical_geometry, orthotope_contract,
    rebinding_candidate_from_binding_declaration, rebinding_prior_fact_from_binding_declaration,
    replace_surface_binding,
};

#[test]
fn kernel_consumes_planar_local_rebuild_parity_without_reclassification() {
    let receipt = local_rebuild_receipt("kernel-rebinding-continuation");

    assert_eq!(receipt.counters().parity_views_compared(), 7);
    assert_eq!(receipt.counters().source_receipts_consumed(), 8);
    assert_eq!(receipt.counters().denied_substitute_rows(), 0);
    assert_eq!(receipt.parity_rows().len(), 7);
    assert!(!receipt.query_receipt_digest().is_empty());
}

#[test]
fn kernel_summary_cannot_substitute_for_rebinding_continuity() {
    let denial = match build_local_rebuild(PlanarRebindingContinuityEvidence::kernel_summary(
        "kernel-summary:local-rebuild-parity",
    ))
    .compile(&PlanarLocalRebuildParityContracts::new(
        local_rebuild_handle(),
    )) {
        Ok(_) => panic!("kernel summary must not substitute for rebinding continuity"),
        Err(error) => error,
    };

    assert_eq!(
        denial.kind(),
        PlanarLocalRebuildParityDenialKind::KernelSummaryNotAuthority
    );
}

fn local_rebuild_receipt(
    continuity_digest: &'static str,
) -> worth_spatial::facade::planar_local_rebuild_parity::PlanarLocalRebuildParityReceipt {
    build_local_rebuild_for_continuity(continuity_digest)
        .compile(&PlanarLocalRebuildParityContracts::new(
            local_rebuild_handle(),
        ))
        .expect("kernel local rebuild parity plan")
        .certify()
        .expect("kernel local rebuild parity receipt")
}

fn build_local_rebuild_for_continuity(continuity_digest: &'static str) -> PlanarLocalRebuildParity {
    let neighborhood = local_neighborhood_receipt();
    let neighborhood_digest = neighborhood.fact_digest().to_string();
    build_local_rebuild_with_neighborhood(
        neighborhood,
        PlanarRebindingContinuityEvidence::from_query_continuation(
            continuity_digest,
            neighborhood_digest,
        ),
    )
}

fn build_local_rebuild(rebinding: PlanarRebindingContinuityEvidence) -> PlanarLocalRebuildParity {
    build_local_rebuild_with_neighborhood(local_neighborhood_receipt(), rebinding)
}

fn build_local_rebuild_with_neighborhood(
    neighborhood: worth_spatial::facade::neighborhood::TopologyNeighborhoodReplacementFactReceipt,
    rebinding: PlanarRebindingContinuityEvidence,
) -> PlanarLocalRebuildParity {
    let readiness = readiness_receipt();
    let retained = retained_planar_facts(readiness.clone());
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(retained.clone())
        .consume_bundle_projection_receipts(readiness.basis().projection_receipts().to_vec())
        .compile(&ProjectionConsumedPlanarFactsContracts::new(
            projection_consumption_handle(),
        ))
        .expect("kernel projection-consumed plan")
        .consume()
        .expect("kernel projection-consumed receipt");
    let recovery = PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_retained_or_projection_basis_denial(
            "kernel-local-rebuild-recovery",
        ),
    )
    .with_retained_planar_facts(retained.clone())
    .with_projection_consumed_facts(projected.clone())
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle()))
    .expect("kernel recovery plan")
    .certify()
    .expect("kernel recovery receipt");
    let diagnostics = PlanarDiagnosticBundle::explain_planar_failure(
        PlanarDiagnosticSubject::binding_failure("kernel-local-rebuild-binding"),
    )
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle()))
    .expect("kernel diagnostic plan")
    .certify()
    .expect("kernel diagnostic receipt");

    PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "kernel-local-rebuild",
    ))
    .local_neighborhood(neighborhood)
    .rebinding_continuity(rebinding)
    .structural_identity(retained.basis().structural_identity_receipt().clone())
    .retained_planar_facts(retained.clone())
    .projection_consumed_planar_facts(projected)
    .motion_posture(retained.basis().motion_posture_receipt().clone())
    .topology_contract(retained.basis().topology_contract_receipt().clone())
    .recovery_posture(recovery)
    .diagnostics(diagnostics)
    .certify_same_planar_basis_across_views()
}

fn local_neighborhood_receipt(
) -> worth_spatial::facade::neighborhood::TopologyNeighborhoodReplacementFactReceipt {
    let prior = surface_binding("kernel-local-rebuild-old");
    let successor_a = surface_binding("kernel-local-rebuild-new-a");
    let successor_b = surface_binding("kernel-local-rebuild-new-b");
    let prior_fact =
        rebinding_prior_fact_from_binding_declaration(&prior, "kernel-local-rebuild-prior");
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "kernel-local-rebuild-old",
        ReplacementCandidateSet::new(vec![
            rebinding_candidate_from_binding_declaration(
                "successor-a",
                &successor_a,
                "kernel-local-rebuild-successor-a",
            )
            .expect("successor a"),
            rebinding_candidate_from_binding_declaration(
                "successor-b",
                &successor_b,
                "kernel-local-rebuild-successor-b",
            )
            .expect("successor b"),
        ])
        .expect("candidate set"),
    )
    .expect("local neighborhood");
    let declaration =
        author_primitive_rebinding_declaration(replace_surface_binding(prior_fact, neighborhood));
    let handle = admitted_rebinding_handle("kernel-local-rebuild-rebinding");
    let entry = topology_neighborhood_replacement_entry(
        primitive_rebinding_neighborhood_replacement_source(&declaration, &handle)
            .expect("kernel replacement source"),
    );
    primitive_rebinding_neighborhood_replacement_facts(&entry, &handle)
        .expect("kernel replacement facts")
}

fn surface_binding(
    site: &'static str,
) -> worth_spatial::facade::bindings::PrimitiveBindingDeclarationEntry {
    author_primitive_binding_declaration(AuthorPrimitiveBindingIntent::attach_surface_to_face(
        FaceSurfaceBindingSpec::new(
            FaceBindingSite::new(site),
            orthotope_contract(),
            canonical_geometry([[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]),
        ),
    ))
}

fn local_rebuild_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarLocalRebuildParityQueryDomain,
    PlanarLocalRebuildParityQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarLocalRebuildParityQueryDomain)
        .with_operating_context(PlanarLocalRebuildParityQueryWorld::new(
            "kernel-local-rebuild-parity",
        ))
        .validate()
        .expect("validated local rebuild domain")
        .admit()
        .expect("admitted local rebuild domain")
}

fn projection_consumption_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    ProjectionConsumedPlanarFactsQueryDomain,
    ProjectionConsumedPlanarFactsQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(ProjectionConsumedPlanarFactsQueryDomain)
        .with_operating_context(ProjectionConsumedPlanarFactsQueryWorld::new(
            "kernel-local-rebuild-parity",
        ))
        .validate()
        .expect("validated projection-consumed domain")
        .admit()
        .expect("admitted projection-consumed domain")
}

fn recovery_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarRecoveryPostureQueryDomain,
    PlanarRecoveryPostureQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarRecoveryPostureQueryDomain)
        .with_operating_context(PlanarRecoveryPostureQueryWorld::new(
            "kernel-local-rebuild-parity",
        ))
        .validate()
        .expect("validated recovery domain")
        .admit()
        .expect("admitted recovery domain")
}

fn diagnostic_handle() -> forge_query::facade::ForgeQueryAdmittedConfiguredDomainHandle<
    PlanarDiagnosticBundleQueryDomain,
    PlanarDiagnosticBundleQueryWorld,
> {
    ForgeQueryApplicationFacade::runtime_backed_default()
        .domain(PlanarDiagnosticBundleQueryDomain)
        .with_operating_context(PlanarDiagnosticBundleQueryWorld::new(
            "kernel-local-rebuild-parity",
        ))
        .validate()
        .expect("validated diagnostic domain")
        .admit()
        .expect("admitted diagnostic domain")
}
