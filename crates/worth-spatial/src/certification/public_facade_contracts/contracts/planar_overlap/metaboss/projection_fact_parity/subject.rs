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
use worth_spatial::facade::planar_diagnostics::{
    PlanarDiagnosticBundle, PlanarDiagnosticBundleContracts, PlanarDiagnosticBundleReceipt,
    PlanarDiagnosticSubject,
};
use worth_spatial::facade::planar_local_rebuild_parity::{
    PlanarLocalRebuildParity, PlanarLocalRebuildParityContracts, PlanarLocalRebuildParityReceipt,
    PlanarLocalRebuildScope, PlanarRebindingContinuityEvidence,
};
use worth_spatial::facade::planar_projection_consumption::{
    ProjectionConsumedPlanarFacts, ProjectionConsumedPlanarFactsContracts,
    ProjectionConsumedPlanarFactsReceipt,
};
use worth_spatial::facade::planar_recovery::{
    PlanarRecoveryPosture, PlanarRecoveryPostureContracts, PlanarRecoveryPostureReceipt,
    PlanarRecoverySource,
};
use worth_spatial::facade::planar_retained_facts::RetainedPlanarFactsReceipt;
use worth_spatial::facade::projection_fact_parity::{
    ProjectionFactParityEvidenceBasis, ProjectionFactParityLane, ProjectionFactParityLaneStatus,
    ProjectionFactParityWorkload,
};
use worth_spatial::facade::user_response::{
    WorthUserOutcome, WorthUserResponseSource, WorthUserResponseWorkload,
};
use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;

use crate::public_api_planar_projection_consumption::contract_subject::projection_consumed_planar_parts;
use crate::public_api_planar_projection_consumption::runtime_handles::projection_consumption_handle;

use super::catalog::{projection_parity_workload_ledger, ProjectionParityCatalog};
use super::runtime_handles::{
    binding_handle, diagnostic_handle, local_rebuild_handle, rebinding_handle, recovery_handle,
};

pub(crate) struct ProjectionFactParitySubject {
    pub(crate) receipt: worth_spatial::facade::projection_fact_parity::ProjectionFactParityReceipt,
    pub(crate) user_outcome: WorthUserOutcome,
}

pub(crate) fn certify_projection_fact_parity(world: &'static str) -> ProjectionFactParitySubject {
    certify_projection_fact_parity_for_catalog(world, ProjectionParityCatalog::CoplanarOverlapStorm)
}

pub(crate) fn certify_projection_fact_parity_for_catalog(
    world: &'static str,
    catalog: ProjectionParityCatalog,
) -> ProjectionFactParitySubject {
    let parts = real_parity_parts(world, catalog);
    let basis = admitted_basis(&parts);
    let receipt = ProjectionFactParityWorkload::from_evidence_basis(basis)
        .declared(format!("MB-M6-7 projection fact parity {world}"))
        .compare_lanes()
        .certify()
        .expect("projection fact parity should certify");
    let user_outcome = WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_projection_fact_parity(&receipt),
    )
    .declared("explain projection fact parity")
    .respond()
    .expect("projection fact parity response")
    .outcome()
    .clone();
    ProjectionFactParitySubject {
        receipt,
        user_outcome,
    }
}

pub(crate) fn denied_parity_outcome(world: &'static str) -> WorthUserOutcome {
    let parts = real_parity_parts(world, ProjectionParityCatalog::CoplanarOverlapStorm);
    let basis =
        admitted_basis(&parts).with_all_lane_statuses(ProjectionFactParityLaneStatus::Denied);
    let receipt = ProjectionFactParityWorkload::from_evidence_basis(basis)
        .declared(format!("MB-M6-7 denied projection parity {world}"))
        .compare_lanes()
        .certify()
        .expect("denied parity should remain denied");
    WorthUserResponseWorkload::from_source(WorthUserResponseSource::from_projection_fact_parity(
        &receipt,
    ))
    .declared("explain denied projection parity")
    .respond()
    .expect("denied parity response")
    .outcome()
    .clone()
}

pub(crate) fn mismatch_outcome(
    world: &'static str,
    lane: ProjectionFactParityLane,
) -> WorthUserOutcome {
    let parts = real_parity_parts(world, ProjectionParityCatalog::CoplanarOverlapStorm);
    let foreign = real_parity_parts(
        "projection-parity-foreign",
        ProjectionParityCatalog::CoplanarOverlapStorm,
    );
    let basis = mismatch_basis(&parts, &foreign, lane);
    let denial = ProjectionFactParityWorkload::from_evidence_basis(basis)
        .declared(format!("MB-M6-7 mismatched {} {world}", lane.human_name()))
        .compare_lanes()
        .certify()
        .expect_err("foreign real basis must deny parity");
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_projection_fact_parity_denial(&denial),
    )
    .declared("explain projection parity mismatch")
    .respond()
    .expect("projection mismatch response")
    .outcome()
    .clone()
}

pub(crate) fn denied_upgrade_outcome(world: &'static str) -> WorthUserOutcome {
    let parts = real_parity_parts(world, ProjectionParityCatalog::CoplanarOverlapStorm);
    let basis = admitted_basis(&parts)
        .with_all_lane_statuses(ProjectionFactParityLaneStatus::Denied)
        .with_lane_status(
            ProjectionFactParityLane::Recovered,
            ProjectionFactParityLaneStatus::Admitted,
        );
    let denial = ProjectionFactParityWorkload::from_evidence_basis(basis)
        .declared(format!("MB-M6-7 denied upgrade {world}"))
        .compare_lanes()
        .certify()
        .expect_err("denied parity cannot upgrade in recovery");
    WorthUserResponseWorkload::from_source(
        WorthUserResponseSource::from_projection_fact_parity_denial(&denial),
    )
    .declared("explain denied projection upgrade")
    .respond()
    .expect("denied upgrade response")
    .outcome()
    .clone()
}

pub(crate) struct RealParityParts {
    pub(crate) ledger: CompleteWorkloadEvidenceLedger,
    pub(crate) retained: RetainedPlanarFactsReceipt,
    pub(crate) projected: ProjectionConsumedPlanarFactsReceipt,
    pub(crate) recovery: PlanarRecoveryPostureReceipt,
    pub(crate) diagnostics: PlanarDiagnosticBundleReceipt,
    pub(crate) local_rebuild: PlanarLocalRebuildParityReceipt,
}

pub(crate) fn admitted_basis(parts: &RealParityParts) -> ProjectionFactParityEvidenceBasis {
    let replay = parts
        .retained
        .historical_replay(&parts.retained.replay_subject())
        .expect("historical replay from retained fact receipt");
    ProjectionFactParityEvidenceBasis::from_evidence_ledger(parts.ledger.clone())
        .with_live_lane_from_ledger()
        .with_projected_lane_from_ledger()
        .with_projection_consumed_facts(&parts.projected)
        .with_retained_workload(&parts.retained)
        .with_replay(&replay)
        .with_transformed_lane_from_ledger()
        .with_recovery(&parts.recovery)
        .with_local_rebuild(&parts.local_rebuild)
        .with_diagnostics(&parts.diagnostics)
}

pub(crate) fn real_parity_parts(
    world: &'static str,
    catalog: ProjectionParityCatalog,
) -> RealParityParts {
    let projection_parts = projection_consumed_planar_parts(world);
    let projected = ProjectionConsumedPlanarFacts::from_retained_planar_facts(
        projection_parts.retained.clone(),
    )
    .consume_bundle_projection_receipts(projection_parts.projections.clone())
    .compile(&ProjectionConsumedPlanarFactsContracts::new(
        projection_consumption_handle(world),
    ))
    .expect("projection-consumed parity plan")
    .consume()
    .expect("projection-consumed parity receipt");
    let recovery = recovery_receipt(world, projection_parts.retained.clone(), projected.clone());
    let diagnostics = diagnostic_receipt(world);
    let neighborhood = local_neighborhood_receipt(world);
    let local_rebuild = local_rebuild_receipt(
        world,
        projection_parts.retained.clone(),
        projected.clone(),
        recovery.clone(),
        diagnostics.clone(),
        neighborhood,
    );
    RealParityParts {
        ledger: projection_parity_workload_ledger(world, catalog),
        retained: projection_parts.retained,
        projected,
        recovery,
        diagnostics,
        local_rebuild,
    }
}

fn mismatch_basis(
    primary: &RealParityParts,
    foreign: &RealParityParts,
    lane: ProjectionFactParityLane,
) -> ProjectionFactParityEvidenceBasis {
    let replay_source = if lane == ProjectionFactParityLane::Replayed {
        &foreign.retained
    } else {
        &primary.retained
    };
    let replay = replay_source
        .historical_replay(&replay_source.replay_subject())
        .expect("historical replay from selected retained fact receipt");
    let basis = ProjectionFactParityEvidenceBasis::from_evidence_ledger(primary.ledger.clone())
        .with_live_lane_from_ledger()
        .with_projected_lane_from_ledger()
        .with_projection_consumed_facts(if lane == ProjectionFactParityLane::ProjectionConsumed {
            &foreign.projected
        } else {
            &primary.projected
        })
        .with_retained_workload(if lane == ProjectionFactParityLane::Retained {
            &foreign.retained
        } else {
            &primary.retained
        })
        .with_replay(&replay)
        .with_transformed_lane_from_ledger()
        .with_recovery(if lane == ProjectionFactParityLane::Recovered {
            &foreign.recovery
        } else {
            &primary.recovery
        })
        .with_local_rebuild(if lane == ProjectionFactParityLane::LocalRebuild {
            &foreign.local_rebuild
        } else {
            &primary.local_rebuild
        })
        .with_diagnostics(if lane == ProjectionFactParityLane::Diagnostics {
            &foreign.diagnostics
        } else {
            &primary.diagnostics
        });
    if matches!(
        lane,
        ProjectionFactParityLane::Live
            | ProjectionFactParityLane::Projected
            | ProjectionFactParityLane::Transformed
    ) {
        basis.with_adversarial_foreign_ledger_basis_for_lane(lane, &foreign.ledger)
    } else {
        basis
    }
}

fn recovery_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
) -> PlanarRecoveryPostureReceipt {
    PlanarRecoveryPosture::from_blocked_planar_source(
        PlanarRecoverySource::from_retained_or_projection_basis_denial(format!(
            "projection-parity-recovery:{world}"
        )),
    )
    .with_retained_planar_facts(retained)
    .with_projection_consumed_facts(projected)
    .prepare_next_step()
    .compile(&PlanarRecoveryPostureContracts::new(recovery_handle(world)))
    .expect("recovery parity plan")
    .certify()
    .expect("recovery parity receipt")
}

fn diagnostic_receipt(world: &'static str) -> PlanarDiagnosticBundleReceipt {
    PlanarDiagnosticBundle::explain_planar_failure(PlanarDiagnosticSubject::binding_failure(
        format!("projection-parity-diagnostic:{world}"),
    ))
    .inspect_failure_locality()
    .compile(&PlanarDiagnosticBundleContracts::new(diagnostic_handle(
        world,
    )))
    .expect("diagnostic parity plan")
    .certify()
    .expect("diagnostic parity receipt")
}

pub(crate) fn local_rebuild_receipt(
    world: &'static str,
    retained: RetainedPlanarFactsReceipt,
    projected: ProjectionConsumedPlanarFactsReceipt,
    recovery: PlanarRecoveryPostureReceipt,
    diagnostics: PlanarDiagnosticBundleReceipt,
    neighborhood: TopologyNeighborhoodReplacementFactReceipt,
) -> PlanarLocalRebuildParityReceipt {
    let neighborhood_digest = neighborhood.fact_digest().to_string();
    PlanarLocalRebuildParity::for_local_rebuild(PlanarLocalRebuildScope::named(
        "projection-parity-local-rebuild",
    ))
    .local_neighborhood(neighborhood)
    .rebinding_continuity(PlanarRebindingContinuityEvidence::from_query_continuation(
        "projection-parity-rebinding-continuity",
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

pub(crate) fn local_neighborhood_receipt(
    world: &'static str,
) -> TopologyNeighborhoodReplacementFactReceipt {
    let prior = surface_binding("projection-parity-old", [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0]]);
    let prior_fact = primitive_binding_rebinding_prior_binding_fact(&prior, &binding_handle(world))
        .expect("prior binding fact");
    let candidates = vec![candidate(
        "projection-parity-successor",
        &surface_binding("projection-parity-new", [[0.0, 0.0, 0.0], [1.0, 1.0, 0.0]]),
        world,
    )];
    let neighborhood = LocalTopologyReplacementNeighborhood::new(
        NeighborhoodBindingFamily::FaceSurface,
        "projection-parity-old",
        ReplacementCandidateSet::new(candidates).expect("candidate set"),
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
