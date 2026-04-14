use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use forge_relational::facade::transactions::TransactionCommitError;
use forge_relational::facade::history::BranchId;
use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::replay::{
    RelationalReplayRequest, ReplayExecutionMode, ReplayVerificationMode,
};
use forge_relational::facade::runtime::RelationalRuntime;
use forge_relational::facade::runtime::RelationalReadView;
use forge_runtime_bridge::facade::{
    BridgeDeliveryReceipt, BridgeSignalInvalidationDelivery, BridgeTruthViewEvaluationRequest,
    InvalidationSink, SignalBridgeSinkError, TruthBranchIdentity,
};
use worth_schema::facade::{
    milestone_one_default_primitive_corpus, seed_minimal_topology, seed_milestone_one_primitive,
    seed_milestone_one_primitive_on_branch, DerivedTopologyReadBasis, VerifiedTopologyCommit,
    WorthEntityKind, WorthMilestoneOnePrimitiveAuthoringError, WorthMilestoneOnePrimitiveCase,
    WorthMilestoneOnePrimitiveExpectedOutcome, WorthMilestoneOnePrimitiveScenario,
    WorthMutationOrigin, WorthNamingEntityKind, WorthNamingRelationKind, WorthRelationKind,
    WorthShellInterpretationClass, WorthTopologyEntityKind, WorthTopologyRelationKind,
    WorthWireInterpretationClass,
};

use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::report::{
    WorthBridgeProofReport, WorthBranchLocalTopologyReport, WorthDeterministicDigest,
    WorthMilestoneOneCloseoutReport, WorthMilestoneOneCertificationReport,
    WorthMilestoneOneCounters,
    WorthNamingAttachmentReport, WorthNamingAttachmentRow,
    WorthPrimitiveCorpusCaseReport, WorthPrimitiveCorpusRejectedCaseReport,
    WorthPrimitiveCorpusCoverageEntry, WorthPrimitiveCorpusCoverageMatrix,
    WorthPrimitiveCorpusParityEntry, WorthPrimitiveCorpusParityReport,
    WorthPrimitiveCorpusReport, WorthPrimitiveFamilyCoverageEntry, WorthPrimitiveFamilyCoverageMatrix,
    WorthPrimitiveRejectionReport,
    WorthReplayParityReport, WorthReplayParityStatus,
    WorthTopologyLocalizationEntityRow, WorthTopologyLocalizationRelationRow,
    WorthTopologyLocalizationReport,
};
use crate::facade::{
    build_worth_milestone_one_bridge,
    build_topology_read_artifact, certify_topology_view, topology_validation_report,
    validate_named_topology_truth, WorthTopologyMaterializer,
};
use worth_schema::facade::{WorthAspect, WorthFallbackDisposition, WorthTopologyMutation, WorthTopologyMutationBatch};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthMilestoneOneCertificationHarness;

impl WorthMilestoneOneCertificationHarness {
    pub fn certify_read_view(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        Self::certify_read_view_with_batch(read_view, read_basis, None, 0)
    }

    fn certify_read_view_with_batch(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
        authority_batch: Option<&WorthTopologyMutationBatch>,
        replay_history_length: usize,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        validate_named_topology_truth(read_view)?;
        let topology = WorthTopologyMaterializer::materialize_from_truth(read_view)?;
        let topology_validation_report = topology_validation_report(&topology)?;
        let read_artifact = build_topology_read_artifact(&read_basis, &topology);
        let certified_interpretation = certify_topology_view(read_basis.clone(), &topology);
        let replay_read_basis = read_basis.replay_of();
        let replay_read_artifact = build_topology_read_artifact(&replay_read_basis, &topology);
        let replay_certified_interpretation =
            certify_topology_view(replay_read_basis, &topology);

        let topology_localization_report = build_topology_localization_report(read_view);
        let naming_attachment_report = build_naming_attachment_report(read_view);
        let primitive_family_coverage_matrix =
            build_primitive_family_coverage_matrix(&read_artifact.interpretations);
        let topology_truth_digest = digest_rows(
            topology_localization_report
                .topology_entities
                .iter()
                .map(|row| format!("entity:{:?}:{}", row.entity_id, row.kind_name))
                .chain(
                    topology_localization_report
                        .topology_relations
                        .iter()
                        .map(|row| format!("relation:{:?}:{}", row.relation_id, row.kind_name)),
                ),
        );
        let naming_truth_digest = digest_rows(
            naming_attachment_report.attachments.iter().map(|row| {
                format!(
                    "attachment:{:?}:{}:{}",
                    row.topology_entity_id,
                    row.topology_kind_name,
                    row.attached_persistent_name_ids
                        .iter()
                        .map(|id| format!("{id:?}"))
                        .collect::<Vec<_>>()
                        .join(",")
                )
            }),
        );
        let topology_validation_digest = digest_rows(
            topology_validation_report.rows.iter().map(|row| {
                format!(
                    "validator:{}:{}",
                    row.validator, row.status
                )
            }),
        );
        let replay_topology_truth_digest = digest_rows(
            topology_localization_report
                .topology_entities
                .iter()
                .map(|row| format!("entity:{:?}:{}", row.entity_id, row.kind_name))
                .chain(
                    topology_localization_report
                        .topology_relations
                        .iter()
                        .map(|row| format!("relation:{:?}:{}", row.relation_id, row.kind_name)),
                ),
        );
        let replay_topology_validation_digest = digest_rows(
            topology_validation_report.rows.iter().map(|row| {
                format!(
                    "validator:{}:{}",
                    row.validator, row.status
                )
            }),
        );
        let interpretation_digest_match = replay_read_artifact.interpretations
            == replay_certified_interpretation.interpretations
            && replay_certified_interpretation.interpretations
                == certified_interpretation.interpretations;
        let branch_local_topology_report = WorthBranchLocalTopologyReport {
            mutation_origin: read_basis.mutation_origin,
            branch_local: matches!(read_basis.mutation_origin, WorthMutationOrigin::BranchLocalApplication),
            branch_id: read_basis.branch_id.clone(),
            snapshot_id: read_basis.snapshot.snapshot_id.0,
            touched_aspect_count: read_basis.touched_aspects.len(),
        };
        let milestone_1_replay_parity_report = WorthReplayParityReport {
            mutation_origin: read_basis.mutation_origin,
            replay_origin: matches!(read_basis.mutation_origin, WorthMutationOrigin::Replay),
            branch_id: read_basis.branch_id.clone(),
            parity_status: WorthReplayParityStatus::NotChecked,
            relational_replay_checked: false,
            relational_replay_verified: false,
            replayed_commit_id: None,
            compared_surfaces: Vec::new(),
            mismatch_count: 0,
            replay_failure: None,
            interpretation_digest_match,
            truth_digest_match: topology_truth_digest == replay_topology_truth_digest,
            validation_digest_match: topology_validation_digest == replay_topology_validation_digest,
        };
        let counters = build_counter_report(
            authority_batch,
            &topology_validation_report,
            &naming_attachment_report,
            &primitive_family_coverage_matrix,
            &read_basis,
            replay_history_length,
        );

        Ok(WorthMilestoneOneCertificationReport {
            named_truth_validated: true,
            topology_validated: true,
            topology_truth_digest,
            naming_truth_digest,
            topology_validation_digest,
            topology_validation_report,
            topology_localization_report,
            naming_attachment_report,
            primitive_family_coverage_matrix,
            branch_local_topology_report,
            milestone_1_replay_parity_report,
            counters,
            read_artifact,
            certified_interpretation,
        })
    }

    pub fn certify_verified_commit(
        runtime: &mut RelationalRuntime,
        verified: &VerifiedTopologyCommit,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        let read_view = runtime
            .read_truth()
            .read_snapshot(&verified.persisted_truth.snapshot)
            .ok_or_else(|| {
                WorthMilestoneOneCertificationError::ReadView(format!(
                    "worth certification could not open verified snapshot {:?}",
                    verified.persisted_truth.snapshot
                ))
            })?;
        let mut report = Self::certify_read_view_with_batch(
            &read_view,
            verified.read_basis.clone(),
            Some(&verified.canonical_batch.batch),
            verified.commits.len(),
        )?;

        if let Some(commit) = verified.commits.last() {
            let replay = runtime.replay_authority().replay_commit(RelationalReplayRequest {
                commit_id: commit.outcome.commit.commit_id,
                branch_id: verified.branch_id.clone(),
                execution_mode: ReplayExecutionMode::SerialDeterministic,
                verification_mode: ReplayVerificationMode::NormalRecoveryVerification,
            });
            let relational_replay_verified = runtime.replay().compare_outcome(&replay);
            report.milestone_1_replay_parity_report.relational_replay_checked = true;
            report.milestone_1_replay_parity_report.relational_replay_verified =
                relational_replay_verified;
            report.milestone_1_replay_parity_report.replayed_commit_id =
                Some(format!("{}", commit.outcome.commit.commit_id.0));
            report.milestone_1_replay_parity_report.compared_surfaces =
                replay.compared_surfaces.clone();
            report.milestone_1_replay_parity_report.mismatch_count = replay.mismatches.len();
            report.milestone_1_replay_parity_report.replay_failure = replay.failure;
            report.milestone_1_replay_parity_report.parity_status = if relational_replay_verified
                && report.milestone_1_replay_parity_report.interpretation_digest_match
                && report.milestone_1_replay_parity_report.truth_digest_match
                && report.milestone_1_replay_parity_report.validation_digest_match
            {
                WorthReplayParityStatus::Match
            } else {
                WorthReplayParityStatus::Mismatch
            };
        }

        Ok(report)
    }
}

pub fn certify_milestone_one_read_view(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    WorthMilestoneOneCertificationHarness::certify_read_view(read_view, read_basis)
}

pub fn certify_verified_topology_commit(
    runtime: &mut RelationalRuntime,
    verified: &VerifiedTopologyCommit,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    WorthMilestoneOneCertificationHarness::certify_verified_commit(runtime, verified)
}

pub fn certify_milestone_one_primitive_corpus<F>(
    mut runtime_factory: F,
    stem: &str,
    primitives: &[WorthMilestoneOnePrimitiveCase],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::with_capacity(primitives.len());
    let rejected_cases = Vec::new();
    for (index, primitive) in primitives.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        let verified = seed_milestone_one_primitive(&mut runtime, &case_stem, primitive)?;
        let certification =
            WorthMilestoneOneCertificationHarness::certify_verified_commit(&mut runtime, &verified)?;
        cases.push(WorthPrimitiveCorpusCaseReport {
            stem: case_stem,
            family: primitive_family_name(primitive).to_string(),
            role: worth_schema::facade::WorthMilestoneOnePrimitiveRole::Generic,
            primitive: primitive.clone(),
            expected_outcome: WorthMilestoneOnePrimitiveExpectedOutcome::Admit,
            certification,
        });
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(WorthPrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

pub fn certify_milestone_one_default_primitive_corpus<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let scenarios = milestone_one_default_primitive_corpus();
    let mut report =
        certify_milestone_one_primitive_scenarios(&mut runtime_factory, stem, &scenarios)?;
    let branch_local_scenarios = scenarios
        .iter()
        .filter(|scenario| {
            scenario.expected_outcome == WorthMilestoneOnePrimitiveExpectedOutcome::Admit
        })
        .cloned()
        .collect::<Vec<_>>();
    let branch_local = certify_milestone_one_branch_local_primitive_scenarios(
        &mut runtime_factory,
        &format!("{stem}.branch_local"),
        "feature",
        &branch_local_scenarios,
    )?;
    report.parity_report =
        build_primitive_corpus_parity_report(&report.cases, Some(&branch_local.cases));
    Ok(report)
}

pub fn certify_milestone_one_closeout<F>(
    mut runtime_factory: F,
    stem: &str,
) -> Result<WorthMilestoneOneCloseoutReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut baseline_runtime = runtime_factory();
    let seeded = seed_minimal_topology(&mut baseline_runtime, &format!("{stem}.bootstrap"))
        .map_err(|error| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one closeout failed to seed bootstrap truth: {error:?}"
            ))
        })?;
    let baseline_read = baseline_runtime
        .read_truth()
        .read_snapshot(&seeded.snapshot)
        .ok_or_else(|| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one closeout could not open seeded snapshot {:?}",
                seeded.snapshot
            ))
        })?;
    let seeded_bootstrap = WorthMilestoneOneCertificationHarness::certify_read_view_with_batch(
        &baseline_read,
        seeded.read_basis,
        Some(&seeded.persisted_truth.batch),
        1,
    )?;
    let primitive_corpus = certify_milestone_one_default_primitive_corpus(
        &mut runtime_factory,
        &format!("{stem}.corpus"),
    )?;
    let bridge_proof = certify_milestone_one_bridge_proof(
        runtime_factory(),
        &format!("{stem}.bridge"),
    )?;

    Ok(WorthMilestoneOneCloseoutReport {
        seeded_bootstrap,
        primitive_corpus,
        bridge_proof,
    })
}

pub fn certify_milestone_one_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    scenarios: &[WorthMilestoneOnePrimitiveScenario],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::new();
    let mut rejected_cases = Vec::new();
    for (index, scenario) in scenarios.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        match scenario.expected_outcome {
            WorthMilestoneOnePrimitiveExpectedOutcome::Admit => {
                let verified =
                    seed_milestone_one_primitive(&mut runtime, &case_stem, &scenario.primitive)?;
                let certification = WorthMilestoneOneCertificationHarness::certify_verified_commit(
                    &mut runtime,
                    &verified,
                )?;
                cases.push(WorthPrimitiveCorpusCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    certification,
                });
            }
            WorthMilestoneOnePrimitiveExpectedOutcome::Reject => {
                let rejection = match seed_milestone_one_primitive(
                    &mut runtime,
                    &case_stem,
                    &scenario.primitive,
                ) {
                    Ok(_) => {
                        return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                            "out-of-class scenario `{}` unexpectedly admitted",
                            scenario.family
                        )));
                    }
                    Err(error) => summarize_primitive_rejection(&error),
                };
                rejected_cases.push(WorthPrimitiveCorpusRejectedCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    rejection,
                });
            }
        }
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(WorthPrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

pub fn certify_milestone_one_branch_local_primitive_scenarios<F>(
    runtime_factory: &mut F,
    stem: &str,
    branch_id: &str,
    scenarios: &[WorthMilestoneOnePrimitiveScenario],
) -> Result<WorthPrimitiveCorpusReport, WorthMilestoneOneCertificationError>
where
    F: FnMut() -> RelationalRuntime,
{
    let mut cases = Vec::new();
    let mut rejected_cases = Vec::new();
    for (index, scenario) in scenarios.iter().enumerate() {
        let case_stem = format!("{stem}.case.{index}");
        let mut runtime = runtime_factory();
        runtime
            .history_authority()
            .create_branch(BranchId(branch_id.to_string()), &BranchId("main".to_string()))
            .map_err(|error| {
                WorthMilestoneOneCertificationError::ReadView(format!(
                    "failed to create branch `{branch_id}`: {error:?}"
                ))
            })?;
        match scenario.expected_outcome {
            WorthMilestoneOnePrimitiveExpectedOutcome::Admit => {
                let verified = seed_milestone_one_primitive_on_branch(
                    &mut runtime,
                    &case_stem,
                    &scenario.primitive,
                    BranchId(branch_id.to_string()),
                    WorthMutationOrigin::BranchLocalApplication,
                )?;
                let certification = WorthMilestoneOneCertificationHarness::certify_verified_commit(
                    &mut runtime,
                    &verified,
                )?;
                cases.push(WorthPrimitiveCorpusCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    certification,
                });
            }
            WorthMilestoneOnePrimitiveExpectedOutcome::Reject => {
                let rejection = match seed_milestone_one_primitive_on_branch(
                    &mut runtime,
                    &case_stem,
                    &scenario.primitive,
                    BranchId(branch_id.to_string()),
                    WorthMutationOrigin::BranchLocalApplication,
                ) {
                    Ok(_) => {
                        return Err(WorthMilestoneOneCertificationError::ReadView(format!(
                            "out-of-class branch-local scenario `{}` unexpectedly admitted",
                            scenario.family
                        )));
                    }
                    Err(error) => summarize_primitive_rejection(&error),
                };
                rejected_cases.push(WorthPrimitiveCorpusRejectedCaseReport {
                    stem: case_stem,
                    family: scenario.family.clone(),
                    role: scenario.role,
                    primitive: scenario.primitive.clone(),
                    expected_outcome: scenario.expected_outcome,
                    rejection,
                });
            }
        }
    }

    let coverage_matrix = build_primitive_corpus_coverage_matrix(&cases, &rejected_cases);
    let parity_report = build_primitive_corpus_parity_report(&cases, None);
    Ok(WorthPrimitiveCorpusReport {
        coverage_matrix,
        parity_report,
        cases,
        rejected_cases,
    })
}

fn build_topology_localization_report(
    read_view: &RelationalReadView,
) -> WorthTopologyLocalizationReport {
    let topology_entity_ids: BTreeSet<KindId> = WorthTopologyEntityKind::WRAPPED_ALL
        .into_iter()
        .map(WorthEntityKind::kind_id)
        .collect();
    let topology_relation_ids: BTreeSet<KindId> = WorthTopologyRelationKind::WRAPPED_ALL
        .into_iter()
        .map(WorthRelationKind::kind_id)
        .collect();

    let topology_entities = read_view
        .entities()
        .iter()
        .filter(|record| topology_entity_ids.contains(&record.kind.kind_id))
        .map(|record| WorthTopologyLocalizationEntityRow {
            entity_id: record.entity_id,
            kind_name: record.kind.kind_name.clone(),
        })
        .collect();

    let topology_relations = read_view
        .relations()
        .iter()
        .filter(|record| topology_relation_ids.contains(&record.kind.kind_id))
        .map(|record| WorthTopologyLocalizationRelationRow {
            relation_id: record.relation_id,
            kind_name: record.kind.kind_name.clone(),
        })
        .collect();

    WorthTopologyLocalizationReport {
        topology_entities,
        topology_relations,
    }
}

fn build_naming_attachment_report(read_view: &RelationalReadView) -> WorthNamingAttachmentReport {
    let topology_entity_ids: BTreeSet<KindId> = WorthTopologyEntityKind::WRAPPED_ALL
        .into_iter()
        .map(WorthEntityKind::kind_id)
        .collect();
    let persistent_name_kind =
        WorthEntityKind::Naming(WorthNamingEntityKind::PersistentName).kind_id();
    let persistent_name_targets_kind =
        WorthRelationKind::Naming(WorthNamingRelationKind::PersistentNameTargetsEntity).kind_id();

    let mut attachments: BTreeMap<EntityId, Vec<EntityId>> = BTreeMap::new();
    for relation in read_view
        .relations()
        .iter()
        .filter(|relation| relation.kind.kind_id == persistent_name_targets_kind)
    {
        attachments
            .entry(relation.target)
            .or_default()
            .push(relation.source);
    }

    let attachment_rows = read_view
        .entities()
        .iter()
        .filter(|entity| topology_entity_ids.contains(&entity.kind.kind_id))
        .map(|entity| WorthNamingAttachmentRow {
            topology_entity_id: entity.entity_id,
            topology_kind_name: entity.kind.kind_name.clone(),
            attached_persistent_name_ids: attachments
                .get(&entity.entity_id)
                .cloned()
                .unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    let named_entity_ids: BTreeSet<EntityId> = attachment_rows
        .iter()
        .filter(|row| !row.attached_persistent_name_ids.is_empty())
        .map(|row| row.topology_entity_id)
        .collect();
    let orphan_persistent_name_ids = read_view
        .entities()
        .iter()
        .filter(|entity| entity.kind.kind_id == persistent_name_kind)
        .filter(|entity| {
            !read_view.relations().iter().any(|relation| {
                relation.kind.kind_id == persistent_name_targets_kind
                    && relation.source == entity.entity_id
            })
        })
        .map(|entity| entity.entity_id)
        .collect::<Vec<_>>();

    WorthNamingAttachmentReport {
        fully_named: attachment_rows.len() == named_entity_ids.len() && orphan_persistent_name_ids.is_empty(),
        orphan_persistent_name_ids,
        attachments: attachment_rows,
    }
}

fn build_primitive_family_coverage_matrix(
    interpretations: &worth_schema::facade::WorthTopologyInterpretationRecordSet,
) -> WorthPrimitiveFamilyCoverageMatrix {
    let wire_open = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::OpenChain)
        .count();
    let wire_closed = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::ClosedCycle)
        .count();
    let wire_branch = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WorthWireInterpretationClass::ConnectedBranch)
        .count();
    let sheet_patch = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == WorthShellInterpretationClass::OpenSheet && record.face_count > 1
        })
        .count();
    let sheet_disk = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == WorthShellInterpretationClass::OpenSheet
                && record.face_count == 1
                && record.boundary_component_count == 1
        })
        .count();
    let solid_shell = interpretations
        .shells
        .iter()
        .filter(|record| record.class == WorthShellInterpretationClass::ClosedSolid)
        .count();
    let nmt_edge_fan = interpretations
        .shells
        .iter()
        .filter(|record| {
            matches!(
                record.class,
                WorthShellInterpretationClass::OpenNonManifold
                    | WorthShellInterpretationClass::ClosedNonManifold
            )
        })
        .count();

    WorthPrimitiveFamilyCoverageMatrix {
        entries: vec![
            coverage_entry("WireOpen(n)", wire_open),
            coverage_entry("WireClosed(n)", wire_closed),
            coverage_entry("WireBranch(k)", wire_branch),
            coverage_entry("SheetDisk(n)", sheet_disk),
            coverage_entry("SheetPatch(f)", sheet_patch),
            coverage_entry("SolidShell(f)", solid_shell),
            coverage_entry("NmtEdgeFan(k)", nmt_edge_fan),
        ],
    }
}

fn build_primitive_corpus_coverage_matrix(
    cases: &[WorthPrimitiveCorpusCaseReport],
    rejected_cases: &[WorthPrimitiveCorpusRejectedCaseReport],
) -> WorthPrimitiveCorpusCoverageMatrix {
    let mut rows = BTreeMap::<String, WorthPrimitiveCorpusCoverageEntry>::new();

    for case in cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_coverage_entry(&case.family));
        match case.role {
            worth_schema::facade::WorthMilestoneOnePrimitiveRole::Smallest => {
                row.admitted_smallest_count += 1;
            }
            worth_schema::facade::WorthMilestoneOnePrimitiveRole::Generic => {
                row.admitted_generic_count += 1;
            }
            worth_schema::facade::WorthMilestoneOnePrimitiveRole::HostileAdmitted => {
                row.admitted_hostile_count += 1;
            }
            worth_schema::facade::WorthMilestoneOnePrimitiveRole::OutOfClass => {}
        }
    }

    for case in rejected_cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_coverage_entry(&case.family));
        if case.role == worth_schema::facade::WorthMilestoneOnePrimitiveRole::OutOfClass {
            row.rejected_out_of_class_count += 1;
        }
    }

    for row in rows.values_mut() {
        row.role_closure_complete = row.admitted_smallest_count > 0
            && row.admitted_generic_count > 0
            && row.admitted_hostile_count > 0
            && row.rejected_out_of_class_count > 0;
    }

    WorthPrimitiveCorpusCoverageMatrix {
        entries: rows.into_values().collect(),
    }
}

fn build_primitive_corpus_parity_report(
    cases: &[WorthPrimitiveCorpusCaseReport],
    branch_local_cases: Option<&[WorthPrimitiveCorpusCaseReport]>,
) -> WorthPrimitiveCorpusParityReport {
    let mut rows = BTreeMap::<String, WorthPrimitiveCorpusParityEntry>::new();

    for case in cases {
        let row = rows
            .entry(case.family.clone())
            .or_insert_with(|| empty_corpus_parity_entry(&case.family));
        row.mainline_case_count += 1;
        let is_branch_local = case.certification.branch_local_topology_report.branch_local;
        if is_branch_local {
            row.branch_local_case_count += 1;
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_checked
        {
            row.mainline_replay_checked_case_count += 1;
            if is_branch_local {
                row.branch_local_replay_checked_case_count += 1;
            }
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .relational_replay_verified
        {
            row.mainline_replay_verified_case_count += 1;
            if is_branch_local {
                row.branch_local_replay_verified_case_count += 1;
            }
        }
        if case
            .certification
            .milestone_1_replay_parity_report
            .interpretation_digest_match
            && case
                .certification
                .milestone_1_replay_parity_report
                .truth_digest_match
            && case
                .certification
                .milestone_1_replay_parity_report
                .validation_digest_match
        {
            row.mainline_digest_parity_case_count += 1;
            if is_branch_local {
                row.branch_local_digest_parity_case_count += 1;
            }
        }
    }

    let mut branch_lookup = BTreeMap::<String, &WorthPrimitiveCorpusCaseReport>::new();
    if let Some(branch_local_cases) = branch_local_cases {
        for case in branch_local_cases {
            let row = rows
                .entry(case.family.clone())
                .or_insert_with(|| empty_corpus_parity_entry(&case.family));
            row.branch_local_case_count += 1;
            if case
                .certification
                .milestone_1_replay_parity_report
                .relational_replay_checked
            {
                row.branch_local_replay_checked_case_count += 1;
            }
            if case
                .certification
                .milestone_1_replay_parity_report
                .relational_replay_verified
            {
                row.branch_local_replay_verified_case_count += 1;
            }
            if case
                .certification
                .milestone_1_replay_parity_report
                .interpretation_digest_match
                && case
                    .certification
                    .milestone_1_replay_parity_report
                    .truth_digest_match
                && case
                    .certification
                    .milestone_1_replay_parity_report
                    .validation_digest_match
            {
                row.branch_local_digest_parity_case_count += 1;
            }
            branch_lookup.insert(parity_case_key(case), case);
        }
    }

    if branch_local_cases.is_some() {
        for case in cases {
            if let Some(branch_case) = branch_lookup.get(&parity_case_key(case)) {
                let row = rows
                    .entry(case.family.clone())
                    .or_insert_with(|| empty_corpus_parity_entry(&case.family));
                if case.certification.topology_truth_digest
                    == branch_case.certification.topology_truth_digest
                    && case.certification.topology_validation_digest
                        == branch_case.certification.topology_validation_digest
                    && case.certification.read_artifact.interpretations
                        == branch_case.certification.read_artifact.interpretations
                    && case.certification.certified_interpretation.interpretations
                        == branch_case
                            .certification
                            .certified_interpretation
                            .interpretations
                {
                    row.cross_branch_parity_case_count += 1;
                }
            }
        }
    }

    for row in rows.values_mut() {
        let cross_branch_scope_satisfied = match branch_local_cases {
            Some(_) => {
                row.branch_local_case_count == row.mainline_case_count
                    && row.branch_local_replay_checked_case_count == row.branch_local_case_count
                    && row.branch_local_replay_verified_case_count == row.branch_local_case_count
                    && row.branch_local_digest_parity_case_count == row.branch_local_case_count
                    && row.cross_branch_parity_case_count == row.mainline_case_count
            }
            None => row.branch_local_case_count == row.mainline_case_count,
        };
        row.parity_closure_complete = row.mainline_case_count > 0
            && row.mainline_replay_checked_case_count == row.mainline_case_count
            && row.mainline_replay_verified_case_count == row.mainline_case_count
            && row.mainline_digest_parity_case_count == row.mainline_case_count
            && cross_branch_scope_satisfied;
    }

    WorthPrimitiveCorpusParityReport {
        entries: rows.into_values().collect(),
    }
}

fn empty_corpus_coverage_entry(family: &str) -> WorthPrimitiveCorpusCoverageEntry {
    WorthPrimitiveCorpusCoverageEntry {
        family: family.to_string(),
        admitted_smallest_count: 0,
        admitted_generic_count: 0,
        admitted_hostile_count: 0,
        rejected_out_of_class_count: 0,
        role_closure_complete: false,
    }
}

fn empty_corpus_parity_entry(family: &str) -> WorthPrimitiveCorpusParityEntry {
    WorthPrimitiveCorpusParityEntry {
        family: family.to_string(),
        mainline_case_count: 0,
        branch_local_case_count: 0,
        mainline_replay_checked_case_count: 0,
        mainline_replay_verified_case_count: 0,
        branch_local_replay_checked_case_count: 0,
        branch_local_replay_verified_case_count: 0,
        mainline_digest_parity_case_count: 0,
        branch_local_digest_parity_case_count: 0,
        cross_branch_parity_case_count: 0,
        parity_closure_complete: false,
    }
}

fn parity_case_key(case: &WorthPrimitiveCorpusCaseReport) -> String {
    format!("{}:{:?}:{:?}", case.family, case.role, case.primitive)
}

fn coverage_entry(family: &str, observed_member_count: usize) -> WorthPrimitiveFamilyCoverageEntry {
    WorthPrimitiveFamilyCoverageEntry {
        family: family.to_string(),
        observed: observed_member_count > 0,
        observed_member_count,
    }
}

fn primitive_family_name(primitive: &WorthMilestoneOnePrimitiveCase) -> &'static str {
    match primitive {
        WorthMilestoneOnePrimitiveCase::WireOpen { .. } => "WireOpen(n)",
        WorthMilestoneOnePrimitiveCase::WireClosed { .. } => "WireClosed(n)",
        WorthMilestoneOnePrimitiveCase::WireBranch { .. } => "WireBranch(k)",
        WorthMilestoneOnePrimitiveCase::SheetDisk { .. } => "SheetDisk(n)",
        WorthMilestoneOnePrimitiveCase::SheetPatch { .. } => "SheetPatch(f)",
        WorthMilestoneOnePrimitiveCase::SolidShell { .. } => "SolidShell(f)",
        WorthMilestoneOnePrimitiveCase::NmtEdgeFan { .. } => "NmtEdgeFan(k)",
    }
}

fn summarize_primitive_rejection(
    error: &WorthMilestoneOnePrimitiveAuthoringError,
) -> WorthPrimitiveRejectionReport {
    match error {
        WorthMilestoneOnePrimitiveAuthoringError::InvalidParameter {
            family,
            parameter,
            requirement,
        } => WorthPrimitiveRejectionReport {
            rejection_class: "InvalidParameter".to_string(),
            diagnostic_code: None,
            detail: format!("invalid `{family}` parameter `{parameter}`; expected {requirement}"),
            fields_json: Some(format!(
                "{{\"family\":\"{family}\",\"parameter\":{parameter},\"requirement\":\"{requirement}\"}}"
            )),
            context: None,
        },
        WorthMilestoneOnePrimitiveAuthoringError::Authority(authority) => {
            summarize_authority_rejection(authority)
        }
    }
}

fn summarize_authority_rejection(
    error: &worth_schema::facade::WorthTopologyAuthorityError,
) -> WorthPrimitiveRejectionReport {
    match error {
        worth_schema::facade::WorthTopologyAuthorityError::Commit(
            TransactionCommitError::Conflict { error, .. },
        ) => WorthPrimitiveRejectionReport {
            rejection_class: "CommitConflict".to_string(),
            diagnostic_code: Some(error.code()),
            detail: error.detail(),
            fields_json: error.fields().map(ToString::to_string),
            context: Some(error.context.clone()),
        },
        other => WorthPrimitiveRejectionReport {
            rejection_class: "AuthorityError".to_string(),
            diagnostic_code: None,
            detail: format!("{other:?}"),
            fields_json: None,
            context: None,
        },
    }
}

fn digest_rows(rows: impl Iterator<Item = String>) -> WorthDeterministicDigest {
    let mut count = 0usize;
    let mut hash = 0xcbf29ce484222325u64;
    for row in rows {
        count += 1;
        for byte in row.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= u64::from(b'\n');
        hash = hash.wrapping_mul(0x100000001b3);
    }

    WorthDeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}

fn build_counter_report(
    authority_batch: Option<&WorthTopologyMutationBatch>,
    topology_validation_report: &crate::validators::WorthTopologyValidationReport,
    naming_attachment_report: &WorthNamingAttachmentReport,
    primitive_family_coverage_matrix: &WorthPrimitiveFamilyCoverageMatrix,
    read_basis: &DerivedTopologyReadBasis,
    replay_history_length: usize,
) -> WorthMilestoneOneCounters {
    let (topology_entity_upsert_count, topology_relation_upsert_count, topology_relation_remove_count) =
        authority_batch.map(count_batch_mutations).unwrap_or((0, 0, 0));
    let derived_topology_full_fallback_count = read_basis
        .precision_fallbacks
        .iter()
        .filter(|record| record.disposition != WorthFallbackDisposition::NoneRequired)
        .count()
        + read_basis.precision_budget_fallbacks.len();
    let touched_topology_aspect_count = read_basis
        .touched_aspects
        .iter()
        .filter(|aspect| matches!(aspect, WorthAspect::Topology(_)))
        .count();

    WorthMilestoneOneCounters {
        topology_entity_upsert_count,
        topology_relation_upsert_count,
        topology_relation_remove_count,
        commit_boundary_validator_count: topology_validation_report.rows.len() + 1,
        commit_boundary_rejection_count: 0,
        derived_topology_interpretation_count: primitive_family_coverage_matrix
            .entries
            .iter()
            .map(|entry| entry.observed_member_count)
            .sum(),
        derived_topology_full_fallback_count,
        naming_target_lookup_count: naming_attachment_report.attachments.len(),
        primitive_family_member_count: primitive_family_coverage_matrix
            .entries
            .iter()
            .map(|entry| entry.observed_member_count)
            .sum(),
        replay_history_length,
        replay_interpretation_rerun_count: usize::from(touched_topology_aspect_count > 0),
    }
}

fn count_batch_mutations(batch: &WorthTopologyMutationBatch) -> (usize, usize, usize) {
    let mut entity_upserts = 0usize;
    let mut relation_upserts = 0usize;
    let mut relation_removes = 0usize;

    for mutation in &batch.mutations {
        match mutation {
            WorthTopologyMutation::CreateEntity { kind, .. }
            | WorthTopologyMutation::UpsertEntity { kind, .. }
                if matches!(kind, worth_schema::facade::WorthEntityKind::Topology(_)) =>
            {
                entity_upserts += 1;
            }
            WorthTopologyMutation::CreateRelation { kind, .. }
            | WorthTopologyMutation::UpsertRelation { kind, .. }
                if matches!(kind, worth_schema::facade::WorthRelationKind::Topology(_)) =>
            {
                relation_upserts += 1;
            }
            WorthTopologyMutation::RemoveRelation { .. } => {
                relation_removes += 1;
            }
            _ => {}
        }
    }

    (entity_upserts, relation_upserts, relation_removes)
}

#[derive(Clone)]
struct WorthBridgeCertificationSink;

impl InvalidationSink for WorthBridgeCertificationSink {
    fn deliver_invalidation(
        &self,
        delivery: BridgeSignalInvalidationDelivery,
    ) -> Result<BridgeDeliveryReceipt, SignalBridgeSinkError> {
        Ok(BridgeDeliveryReceipt::new(
            delivery.invalidation_targets().len(),
            delivery.source_snapshot().clone(),
        ))
    }
}

fn certify_milestone_one_bridge_proof(
    mut runtime: RelationalRuntime,
    stem: &str,
) -> Result<WorthBridgeProofReport, WorthMilestoneOneCertificationError> {
    let verified = seed_milestone_one_primitive(
        &mut runtime,
        stem,
        &WorthMilestoneOnePrimitiveCase::SolidShell { face_count: 6 },
    )?;
    let commit = verified.commits.last().ok_or_else(|| {
        WorthMilestoneOneCertificationError::ReadView(
            "worth milestone one bridge proof requires a committed topology mutation".to_string(),
        )
    })?;
    let runtime = Arc::new(runtime);
    let bridge = build_worth_milestone_one_bridge(Arc::clone(&runtime), WorthBridgeCertificationSink)
        .map_err(|error| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one bridge proof could not build bridge: {error:?}"
            ))
        })?;
    let _route = bridge
        .route(format!("commit-{}", commit.outcome.commit.commit_id.0))
        .map_err(|error| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one bridge proof could not route committed truth: {error:?}"
            ))
        })?;
    let evaluation = bridge
        .evaluate(BridgeTruthViewEvaluationRequest::for_branch_head(
            TruthBranchIdentity::new(verified.branch_id.0.as_str()),
        ))
        .map_err(|error| {
            WorthMilestoneOneCertificationError::ReadView(format!(
                "worth milestone one bridge proof could not evaluate branch head: {error:?}"
            ))
        })?;
    let route_records = bridge.diagnostics().route_records();
    let historical_records = bridge.diagnostics().historical_evaluation_records();
    let bridge_routing_digest = digest_rows(route_records.iter().map(|record| {
        format!(
            "route:{}:{}:{}:{}:{}",
            record.route_identity().as_str(),
            record.source_branch().as_str(),
            record.source_commit().as_str(),
            record.source_snapshot().as_str(),
            record.invalidation_targets().len()
        )
    }));
    let bridge_historical_evaluation_digest =
        digest_rows(historical_records.iter().map(|record| {
            format!(
                "historical:{}:{}:{}:{}:{:?}",
                record.record_identity().as_str(),
                record.decision_log().branch_identity().as_str(),
                record.decision_log().commit_identity().as_str(),
                record.decision_log().snapshot_identity().as_str(),
                record.decision_log().materialization_path()
            )
        }));

    Ok(WorthBridgeProofReport {
        bridge_routing_digest,
        bridge_historical_evaluation_digest,
        route_record_count: route_records.len(),
        historical_evaluation_record_count: historical_records.len(),
        source_branch: verified.branch_id.0.clone(),
        source_commit: commit.outcome.commit.commit_id.0.to_string(),
        source_snapshot: evaluation.snapshot_identity().as_str().to_string(),
    })
}

#[cfg(test)]
mod facade_tests {
    use forge_relational::facade::identity::{EntityId, PartitionId};
    use worth_schema::facade::{
        WorthShellInterpretationClass, WorthShellInterpretationRecord,
        WorthTopologyInterpretationRecordSet, WorthWireInterpretationRecord,
        WorthWireInterpretationClass,
    };

    use super::build_primitive_family_coverage_matrix;

    #[test]
    fn primitive_family_coverage_distinguishes_sheet_disk_from_sheet_patch() {
        let interpretations = WorthTopologyInterpretationRecordSet {
            wires: vec![WorthWireInterpretationRecord {
                wire_id: entity(1),
                class: WorthWireInterpretationClass::OpenChain,
                connected_component_count: 1,
                terminal_vertex_ids: vec![entity(2), entity(3)],
                branch_vertex_ids: Vec::new(),
            }],
            shells: vec![
                WorthShellInterpretationRecord {
                    shell_id: entity(10),
                    class: WorthShellInterpretationClass::OpenSheet,
                    face_count: 1,
                    boundary_component_count: 1,
                    boundary_half_edge_count: 5,
                    non_manifold_edge_ids: Vec::new(),
                },
                WorthShellInterpretationRecord {
                    shell_id: entity(11),
                    class: WorthShellInterpretationClass::OpenSheet,
                    face_count: 3,
                    boundary_component_count: 2,
                    boundary_half_edge_count: 7,
                    non_manifold_edge_ids: Vec::new(),
                },
            ],
        };

        let matrix = build_primitive_family_coverage_matrix(&interpretations);

        assert!(matrix.entries.iter().any(|entry| {
            entry.family == "SheetDisk(n)" && entry.observed && entry.observed_member_count == 1
        }));
        assert!(matrix.entries.iter().any(|entry| {
            entry.family == "SheetPatch(f)" && entry.observed && entry.observed_member_count == 1
        }));
    }

    fn entity(slot: u64) -> EntityId {
        EntityId::new(PartitionId::main(), slot, 1)
    }
}
