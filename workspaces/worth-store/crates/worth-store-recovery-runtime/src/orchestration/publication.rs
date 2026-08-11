use worth_store::physical_runtime::{
    PhysicalRecoveryPublicationCandidate, PhysicalRecoveryPublicationCommand,
    PhysicalRecoveryPublicationCommandOutcome,
};

use crate::entry::{
    AdmittedPlatformAuthority, PhysicalRecoveryBlock, PhysicalRecoveryBlockEvidence,
    PhysicalRecoveryBlockKind, PhysicalRecoveryOutcome, PhysicalRecoveryPublicationCounters,
    PhysicalRecoveryPublicationDenial, PhysicalRecoveryPublicationIndeterminate,
    PhysicalRecoveryPublicationSettlement, PhysicalRecoveryPublicationSettlementLedger,
};
use crate::progression::{
    NamespaceDurablePhysicalRecovery, NamespaceDurableState, StagedPhysicalRecovery,
};

mod counters;

use counters::{completed_counters, denial_counters, indeterminate_counters};

pub(crate) fn publish_recovery(
    staged: StagedPhysicalRecovery,
) -> Result<NamespaceDurablePhysicalRecovery, PhysicalRecoveryOutcome> {
    let StagedPhysicalRecovery {
        authority,
        coordination,
        selection,
        discovery_counters,
        freshness,
        fates,
        planning_counters,
        base,
        publication,
        quiescence,
        closed,
        staging_counters,
        staging_settlements,
    } = staged;
    let planned_effects = quiescence.publication_commands();
    let (expectation, candidates) = publication.into_command_parts();
    let Some(candidates) = candidates
        .into_vec()
        .into_iter()
        .map(|candidate| {
            let (artifact, bytes, digest) = candidate.into_command_parts();
            PhysicalRecoveryPublicationCandidate::new(artifact, bytes, digest)
        })
        .collect::<Option<Vec<_>>>()
        .map(Vec::into_boxed_slice)
    else {
        return Err(block_invalid(
            authority,
            coordination,
            discovery_counters,
            planning_counters,
            staging_counters,
            staging_settlements,
            planned_effects,
        ));
    };
    if candidates.is_empty() {
        let selected = selection.root().selected();
        if planned_effects != 0
            || closed.artifact_count() != 0
            || closed.byte_count() != 0
            || expectation.current_selector() != selected.selector()
            || expectation.recovered_root() != selected.manifest()
            || !coordination.is_ready()
        {
            return Err(block_invalid(
                authority,
                coordination,
                discovery_counters,
                planning_counters,
                staging_counters,
                staging_settlements,
                planned_effects,
            ));
        }
        return Ok(NamespaceDurablePhysicalRecovery::new(
            NamespaceDurableState {
                authority,
                coordination,
                selection,
                discovery_counters,
                freshness,
                fates,
                planning_counters,
                base,
                quiescence,
                closed,
                staging_counters,
                staging_settlements,
            },
            expectation,
            PhysicalRecoveryPublicationCounters::default(),
            PhysicalRecoveryPublicationSettlementLedger::new(
                PhysicalRecoveryPublicationSettlement::PreexistingNamespaceDurable,
            ),
        ));
    }
    let Some(command) = PhysicalRecoveryPublicationCommand::new(
        expectation.plan_identity(),
        expectation.staging_generation(),
        candidates,
        expectation.root_protocol(),
    ) else {
        return Err(block_invalid(
            authority,
            coordination,
            discovery_counters,
            planning_counters,
            staging_counters,
            staging_settlements,
            planned_effects,
        ));
    };
    match coordination
        .owner()
        .execute_publication_command(&authority.media, command)
    {
        PhysicalRecoveryPublicationCommandOutcome::Completed(completed) => {
            let counters = completed_counters(planned_effects, &completed);
            if !coordination.is_ready() {
                return Err(indeterminate(
                    authority,
                    coordination,
                    counters,
                    PhysicalRecoveryPublicationSettlement::Completed(completed),
                ));
            }
            Ok(NamespaceDurablePhysicalRecovery::new(
                NamespaceDurableState {
                    authority,
                    coordination,
                    selection,
                    discovery_counters,
                    freshness,
                    fates,
                    planning_counters,
                    base,
                    quiescence,
                    closed,
                    staging_counters,
                    staging_settlements,
                },
                expectation,
                counters,
                PhysicalRecoveryPublicationSettlementLedger::new(
                    PhysicalRecoveryPublicationSettlement::Completed(completed),
                ),
            ))
        }
        PhysicalRecoveryPublicationCommandOutcome::DeniedBeforeEffect(denial) => {
            let counters = denial_counters(planned_effects, &denial);
            let escaped = !denial.candidates().is_empty()
                || denial.candidate_materialization().is_some()
                || denial.root_protocol().is_some();
            let settlement = PhysicalRecoveryPublicationSettlement::DeniedBeforeEffect(denial);
            if escaped {
                Err(indeterminate(authority, coordination, counters, settlement))
            } else {
                Err(block_settlement(
                    authority,
                    coordination,
                    discovery_counters,
                    planning_counters,
                    staging_counters,
                    staging_settlements,
                    counters,
                    settlement,
                ))
            }
        }
        PhysicalRecoveryPublicationCommandOutcome::Indeterminate(outcome) => {
            let counters = indeterminate_counters(planned_effects, &outcome);
            Err(indeterminate(
                authority,
                coordination,
                counters,
                PhysicalRecoveryPublicationSettlement::Indeterminate(outcome),
            ))
        }
    }
}

fn block_invalid(
    authority: AdmittedPlatformAuthority,
    coordination: super::RecoveryCoordination,
    discovery: crate::progression::PhysicalRecoveryDiscoveryCounters,
    planning: worth_store_recovery_physics::RecoveryPlanningCounters,
    staging: crate::entry::PhysicalRecoveryStagingCounters,
    staging_settlements: crate::entry::PhysicalRecoveryStagingSettlementLedger,
    planned_effects: u64,
) -> PhysicalRecoveryOutcome {
    assert!(coordination.shutdown_is_quiescent());
    let store = authority.media.store_identity();
    let session_identity = authority.session.identity();
    let recovery_effects = authority.media.recovery_effect_count();
    let AdmittedPlatformAuthority { media, session, .. } = authority;
    drop(media);
    session.block();
    PhysicalRecoveryOutcome::Blocked(PhysicalRecoveryBlock::new(
        PhysicalRecoveryBlockKind::Publication,
        store,
        session_identity,
        PhysicalRecoveryBlockEvidence {
            counters: discovery,
            planning_counters: Some(planning),
            staging_counters: Some(staging),
            staging_settlements: Some(staging_settlements),
            publication_counters: Some(PhysicalRecoveryPublicationCounters {
                planned_effects,
                ..PhysicalRecoveryPublicationCounters::default()
            }),
            publication_denial: Some(PhysicalRecoveryPublicationDenial::InvalidPlan),
            ..PhysicalRecoveryBlockEvidence::default()
        },
        recovery_effects,
    ))
}

#[allow(clippy::too_many_arguments)]
fn block_settlement(
    authority: AdmittedPlatformAuthority,
    coordination: super::RecoveryCoordination,
    discovery: crate::progression::PhysicalRecoveryDiscoveryCounters,
    planning: worth_store_recovery_physics::RecoveryPlanningCounters,
    staging: crate::entry::PhysicalRecoveryStagingCounters,
    staging_settlements: crate::entry::PhysicalRecoveryStagingSettlementLedger,
    counters: PhysicalRecoveryPublicationCounters,
    settlement: PhysicalRecoveryPublicationSettlement,
) -> PhysicalRecoveryOutcome {
    assert!(coordination.shutdown_is_quiescent());
    let store = authority.media.store_identity();
    let session_identity = authority.session.identity();
    let recovery_effects = authority.media.recovery_effect_count();
    let AdmittedPlatformAuthority { media, session, .. } = authority;
    drop(media);
    session.block();
    PhysicalRecoveryOutcome::Blocked(PhysicalRecoveryBlock::new(
        PhysicalRecoveryBlockKind::Publication,
        store,
        session_identity,
        PhysicalRecoveryBlockEvidence {
            counters: discovery,
            planning_counters: Some(planning),
            staging_counters: Some(staging),
            staging_settlements: Some(staging_settlements),
            publication_counters: Some(counters),
            publication_settlements: Some(PhysicalRecoveryPublicationSettlementLedger::new(
                settlement,
            )),
            ..PhysicalRecoveryBlockEvidence::default()
        },
        recovery_effects,
    ))
}

fn indeterminate(
    authority: AdmittedPlatformAuthority,
    coordination: super::RecoveryCoordination,
    counters: PhysicalRecoveryPublicationCounters,
    settlement: PhysicalRecoveryPublicationSettlement,
) -> PhysicalRecoveryOutcome {
    assert!(coordination.shutdown_is_quiescent());
    let store = authority.media.store_identity();
    let session_identity = authority.session.identity();
    let recovery_effects = authority.media.recovery_effect_count();
    let AdmittedPlatformAuthority { media, session, .. } = authority;
    drop(media);
    session.publication_indeterminate();
    PhysicalRecoveryOutcome::PublicationIndeterminate(
        PhysicalRecoveryPublicationIndeterminate::new(
            store,
            session_identity,
            counters,
            PhysicalRecoveryPublicationSettlementLedger::new(settlement),
            recovery_effects,
        ),
    )
}
