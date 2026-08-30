use worth_store_recovery_physics::{
    admit_physical_page_facts, admit_physical_wal_tail, select_physical_recovery_sources,
    PhysicalCheckpointBase, PhysicalRecoveryResidue, PhysicalRootSlotObservation,
    PhysicalSourceSelection, SelectedCompactionProduct, SelectedPhysicalPageFacts,
    SelectedPhysicalRoot, SelectedPhysicalRootRole, SelectedPhysicalWalTail,
};

use crate::entry::{
    PhysicalRecoveryBlockEvidence, PhysicalRecoveryBlockKind, PhysicalRecoveryLimitDimension,
    PhysicalRecoveryLimitFailure, PhysicalRecoveryLimits, PhysicalRecoverySourceDenial,
};
use crate::orchestration::{
    BootstrapDiscovery, CheckpointDiscovery, ManifestFactsDiscovery, ManifestFactsState,
    WalDiscovery,
};

use super::PhysicalRecoveryDiscoveryCounters;

mod root;

pub(super) struct SelectionInput {
    pub(super) current: PhysicalRootSlotObservation,
    pub(super) previous: PhysicalRootSlotObservation,
    pub(super) bootstrap: BootstrapDiscovery,
    pub(super) current_manifest_facts: ManifestFactsDiscovery,
    pub(super) previous_manifest_facts: ManifestFactsDiscovery,
    pub(super) checkpoint: CheckpointDiscovery,
    pub(super) wal: WalDiscovery,
    pub(super) residue: Vec<PhysicalRecoveryResidue>,
    pub(super) root_protocol_denials: Vec<PhysicalRecoverySourceDenial>,
    pub(super) counters: PhysicalRecoveryDiscoveryCounters,
}

pub(super) struct SelectionOutput {
    pub(super) selection: PhysicalSourceSelection,
    pub(super) counters: PhysicalRecoveryDiscoveryCounters,
    pub(super) root_protocol_denials: Vec<PhysicalRecoverySourceDenial>,
    pub(super) integrity_trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace,
}

pub(super) struct SelectionFailure {
    pub(super) kind: PhysicalRecoveryBlockKind,
    pub(super) evidence: PhysicalRecoveryBlockEvidence,
}

struct ManifestSelectionInput {
    current: ManifestFactsState,
    previous: ManifestFactsState,
    limits: PhysicalRecoveryLimits,
}

struct FinalSelectionInput {
    root: SelectedPhysicalRoot,
    page_facts: SelectedPhysicalPageFacts,
    retained_previous_page_facts: Option<SelectedPhysicalPageFacts>,
    checkpoint: Option<PhysicalCheckpointBase>,
    wal_tail: SelectedPhysicalWalTail,
    compaction: Option<SelectedCompactionProduct>,
    residue: Vec<PhysicalRecoveryResidue>,
    counters: PhysicalRecoveryDiscoveryCounters,
    frontier: u64,
}
pub(super) fn select_sources(
    input: SelectionInput,
    limits: PhysicalRecoveryLimits,
) -> Result<SelectionOutput, SelectionFailure> {
    let mut counters = input.counters;
    let (current_manifest_facts, mut integrity_trace) = input.current_manifest_facts.into_parts();
    let (previous_manifest_facts, previous_integrity_trace) =
        input.previous_manifest_facts.into_parts();
    integrity_trace.append(previous_integrity_trace);
    let (root, root_protocol_denials) = root::select(
        input.current,
        input.previous,
        input.bootstrap,
        input.root_protocol_denials,
        counters,
    )?;
    let (page_facts, retained_previous_page_facts) = select_manifest_facts(
        &root,
        ManifestSelectionInput {
            current: current_manifest_facts,
            previous: previous_manifest_facts,
            limits,
        },
        &mut counters,
    )
    .map_err(|failure| {
        failure
            .with_root_protocol_denials(&root_protocol_denials)
            .with_integrity_trace(integrity_trace.clone())
    })?;
    let checkpoint = select_checkpoint(&root, input.checkpoint, counters).map_err(|failure| {
        failure
            .with_root_protocol_denials(&root_protocol_denials)
            .with_integrity_trace(integrity_trace.clone())
    })?;
    let frontier = checkpoint
        .as_ref()
        .map_or(0, |checkpoint| checkpoint.wal_tail_begin_lsn());
    let wal_tail = select_wal(&root, input.wal, frontier, &mut counters).map_err(|failure| {
        failure
            .with_root_protocol_denials(&root_protocol_denials)
            .with_integrity_trace(integrity_trace.clone())
    })?;
    let compaction = checkpoint.as_ref().map(SelectedCompactionProduct::admit);
    let selection = select_final_cut(FinalSelectionInput {
        root,
        page_facts,
        retained_previous_page_facts,
        checkpoint,
        wal_tail,
        compaction,
        residue: input.residue,
        counters,
        frontier,
    })
    .map_err(|failure| {
        failure
            .with_root_protocol_denials(&root_protocol_denials)
            .with_integrity_trace(integrity_trace.clone())
    })?;
    Ok(SelectionOutput {
        selection,
        counters,
        root_protocol_denials,
        integrity_trace,
    })
}

fn select_manifest_facts(
    root: &SelectedPhysicalRoot,
    input: ManifestSelectionInput,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<(SelectedPhysicalPageFacts, Option<SelectedPhysicalPageFacts>), SelectionFailure> {
    let (selected, retained_previous) = match root.role() {
        SelectedPhysicalRootRole::Current => (input.current, Some(input.previous)),
        SelectedPhysicalRootRole::PreviousFallback => (input.previous, None),
    };
    let generation = root.selected().selector().root_generation();
    let blocks = match selected {
        ManifestFactsState::Observed { blocks } => blocks,
        ManifestFactsState::Rejected(denial) => {
            return Err(SelectionFailure::new(
                PhysicalRecoveryBlockKind::SourceSelection,
                *counters,
                "records/root routing blocks",
            )
            .with_generation(generation)
            .with_source_denials(vec![
                PhysicalRecoverySourceDenial::ManifestObservation(denial),
            ]));
        }
        ManifestFactsState::Unavailable => {
            return Err(SelectionFailure::new(
                PhysicalRecoveryBlockKind::SourceSelection,
                *counters,
                "records/root routing blocks",
            )
            .with_generation(generation));
        }
    };
    let declaration = input.limits.declaration();
    let page_facts = admit_physical_page_facts(
        root.selected(),
        blocks,
        declaration.manifest_entries,
        declaration.distinct_pages_and_extents,
    )
    .map_err(|denial| manifest_failure(denial, generation, *counters, declaration))?;
    counters.selected_page_facts = page_facts.placements().len() as u64;
    counters.distinct_pages_and_extents = page_facts.distinct_pages_and_extents();
    let retained_previous_page_facts = match (root.retained_previous(), retained_previous) {
        (Some(previous), Some(ManifestFactsState::Observed { blocks })) => Some(
            admit_physical_page_facts(
                previous,
                blocks,
                declaration.manifest_entries,
                declaration.distinct_pages_and_extents,
            )
            .map_err(|denial| {
                manifest_failure(
                    denial,
                    previous.selector().root_generation(),
                    *counters,
                    declaration,
                )
            })?,
        ),
        (Some(previous), Some(ManifestFactsState::Rejected(denial))) => {
            return Err(SelectionFailure::new(
                PhysicalRecoveryBlockKind::SourceSelection,
                *counters,
                "records/retained-previous routing blocks",
            )
            .with_generation(previous.selector().root_generation())
            .with_source_denials(vec![
                PhysicalRecoverySourceDenial::ManifestObservation(denial),
            ]));
        }
        (Some(previous), Some(ManifestFactsState::Unavailable)) => {
            return Err(SelectionFailure::new(
                PhysicalRecoveryBlockKind::SourceSelection,
                *counters,
                "records/retained-previous routing blocks",
            )
            .with_generation(previous.selector().root_generation()));
        }
        (None, _) | (_, None) => None,
    };
    Ok((page_facts, retained_previous_page_facts))
}

fn select_checkpoint(
    root: &SelectedPhysicalRoot,
    checkpoint: CheckpointDiscovery,
    counters: PhysicalRecoveryDiscoveryCounters,
) -> Result<Option<PhysicalCheckpointBase>, SelectionFailure> {
    let generation = root.selected().selector().root_generation();
    match checkpoint {
        CheckpointDiscovery::Absent => Ok(None),
        CheckpointDiscovery::Rejected(denial) => Err(SelectionFailure::new(
            PhysicalRecoveryBlockKind::Checkpoint,
            counters,
            "families/checkpoint.current",
        )
        .with_generation(generation)
        .with_source_denials(vec![PhysicalRecoverySourceDenial::CheckpointFormat(denial)])),
        CheckpointDiscovery::Admitted(checkpoint) => {
            PhysicalCheckpointBase::admit(root, checkpoint)
                .map(Some)
                .map_err(|denial| {
                    SelectionFailure::new(
                        PhysicalRecoveryBlockKind::Checkpoint,
                        counters,
                        "families/checkpoint.current",
                    )
                    .with_generation(generation)
                    .with_source_denials(vec![
                        PhysicalRecoverySourceDenial::CheckpointBinding(denial),
                    ])
                })
        }
    }
}

fn select_wal(
    root: &SelectedPhysicalRoot,
    wal: WalDiscovery,
    frontier: u64,
    counters: &mut PhysicalRecoveryDiscoveryCounters,
) -> Result<SelectedPhysicalWalTail, SelectionFailure> {
    let generation = root.selected().selector().root_generation();
    let (candidates, rejected, corruptions) = wal.into_selection_parts();
    if rejected {
        let denials = corruptions
            .into_iter()
            .map(PhysicalRecoverySourceDenial::WalArtifact)
            .collect();
        return Err(SelectionFailure::new(
            PhysicalRecoveryBlockKind::WalInventory,
            *counters,
            "families/wal",
        )
        .with_generation(generation)
        .with_lsn(frontier)
        .with_source_denials(denials));
    }
    admit_physical_wal_tail(frontier, candidates).map_err(|denial| {
        counters.wal_missing_range_denials += 1;
        SelectionFailure::new(
            PhysicalRecoveryBlockKind::WalInventory,
            *counters,
            "families/wal",
        )
        .with_generation(generation)
        .with_lsn(frontier)
        .with_source_denials(vec![PhysicalRecoverySourceDenial::WalTail(denial)])
    })
}

fn select_final_cut(
    input: FinalSelectionInput,
) -> Result<PhysicalSourceSelection, SelectionFailure> {
    let generation = input.root.selected().selector().root_generation();
    select_physical_recovery_sources(
        input.root,
        input.page_facts,
        input.retained_previous_page_facts,
        input.checkpoint,
        input.wal_tail,
        input.compaction,
        input.residue,
    )
    .map_err(|denial| {
        SelectionFailure::new(
            PhysicalRecoveryBlockKind::SourceSelection,
            input.counters,
            "selected persisted-source cut",
        )
        .with_generation(generation)
        .with_lsn(input.frontier)
        .with_source_denials(vec![PhysicalRecoverySourceDenial::FinalSelection(denial)])
    })
}

fn manifest_failure(
    denial: worth_store_recovery_physics::PhysicalPageFactDenial,
    generation: u64,
    counters: PhysicalRecoveryDiscoveryCounters,
    limits: crate::entry::PhysicalRecoveryLimitDeclaration,
) -> SelectionFailure {
    let mut failure = SelectionFailure::new(
        PhysicalRecoveryBlockKind::SourceSelection,
        counters,
        "records/root routing blocks",
    )
    .with_generation(generation)
    .with_source_denials(vec![PhysicalRecoverySourceDenial::ManifestFacts(denial)]);
    match denial {
        worth_store_recovery_physics::PhysicalPageFactDenial::ManifestEntryLimit => {
            failure.kind = PhysicalRecoveryBlockKind::DiscoveryLimit;
            failure.evidence.limit = Some(PhysicalRecoveryLimitFailure {
                dimension: PhysicalRecoveryLimitDimension::ManifestEntries,
                observed: limits.manifest_entries + 1,
                admitted: limits.manifest_entries,
            });
        }
        worth_store_recovery_physics::PhysicalPageFactDenial::DistinctPageOrExtentLimit => {
            failure.kind = PhysicalRecoveryBlockKind::DiscoveryLimit;
            failure.evidence.limit = Some(PhysicalRecoveryLimitFailure {
                dimension: PhysicalRecoveryLimitDimension::DistinctPagesAndExtents,
                observed: limits.distinct_pages_and_extents + 1,
                admitted: limits.distinct_pages_and_extents,
            });
        }
        _ => {}
    }
    failure
}

impl SelectionFailure {
    fn new(
        kind: PhysicalRecoveryBlockKind,
        counters: PhysicalRecoveryDiscoveryCounters,
        artifact: &str,
    ) -> Self {
        Self {
            kind,
            evidence: PhysicalRecoveryBlockEvidence {
                counters,
                artifact: Some(artifact.to_owned()),
                ..PhysicalRecoveryBlockEvidence::default()
            },
        }
    }

    fn with_generation(mut self, generation: u64) -> Self {
        self.evidence.source_generation = Some(generation);
        self
    }

    fn with_lsn(mut self, lsn: u64) -> Self {
        self.evidence.lsn = Some(lsn);
        self
    }

    fn with_source_denials(mut self, denials: Vec<PhysicalRecoverySourceDenial>) -> Self {
        self.evidence.source_denials = denials;
        self
    }

    fn with_root_protocol_denials(mut self, denials: &[PhysicalRecoverySourceDenial]) -> Self {
        let mut combined = denials.to_vec();
        combined.append(&mut self.evidence.source_denials);
        self.evidence.source_denials = combined;
        self
    }

    fn with_integrity_trace(
        mut self,
        trace: crate::integrity_ingress::RecoveryIntegrityIngressTrace,
    ) -> Self {
        self.evidence.integrity_trace = trace;
        self
    }
}
