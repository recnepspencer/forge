use super::*;

pub(super) fn build_closeout_counter_report(
    seeded_bootstrap: &MilestoneOneCertificationReport,
    primitive_corpus: &PrimitiveCorpusReport,
    illegal_topology_rejection_report: &IllegalTopologyRejectionReport,
) -> MilestoneOneCounters {
    let mut counter_report = seeded_bootstrap.counters.clone();
    counter_report.commit_boundary_rejection_count = illegal_topology_rejection_report.case_count;
    for case in &primitive_corpus.cases {
        counter_report.topology_entity_upsert_count +=
            case.certification.counters.topology_entity_upsert_count;
        counter_report.topology_relation_upsert_count +=
            case.certification.counters.topology_relation_upsert_count;
        counter_report.topology_relation_remove_count +=
            case.certification.counters.topology_relation_remove_count;
        counter_report.commit_boundary_validator_count +=
            case.certification.counters.commit_boundary_validator_count;
        counter_report.derived_topology_interpretation_count += case
            .certification
            .counters
            .derived_topology_interpretation_count;
        counter_report.derived_topology_full_fallback_count += case
            .certification
            .counters
            .derived_topology_full_fallback_count;
        counter_report.naming_target_lookup_count +=
            case.certification.counters.naming_target_lookup_count;
        counter_report.primitive_family_member_count +=
            case.certification.counters.primitive_family_member_count;
        counter_report.replay_history_length += case.certification.counters.replay_history_length;
        counter_report.replay_interpretation_rerun_count += case
            .certification
            .counters
            .replay_interpretation_rerun_count;
    }
    counter_report
}

pub(super) fn build_closeout_digest(
    seeded_bootstrap: &MilestoneOneCertificationReport,
    primitive_corpus: &PrimitiveCorpusReport,
    select: impl Fn(&MilestoneOneCertificationReport) -> DeterministicDigest,
) -> DeterministicDigest {
    digest_rows(
        std::iter::once(("seeded_bootstrap".to_string(), select(seeded_bootstrap)))
            .chain(
                primitive_corpus
                    .cases
                    .iter()
                    .map(|case| (case.stem.clone(), select(&case.certification))),
            )
            .map(|(source, digest)| {
                format!(
                    "{source}:{}:{}:{}",
                    digest.algorithm, digest.digest_hex, digest.row_count
                )
            }),
    )
}

pub(super) fn build_closeout_validation_report(
    seeded_bootstrap: &MilestoneOneCertificationReport,
    primitive_corpus: &PrimitiveCorpusReport,
) -> MilestoneOneValidationAggregateReport {
    let mut rows = Vec::new();
    rows.extend(
        seeded_bootstrap
            .topology_validation_report
            .rows
            .iter()
            .map(|row| MilestoneOneValidationAggregateRow {
                source: "seeded_bootstrap".to_string(),
                family: "SeededBootstrap".to_string(),
                validator: row.validator.clone(),
                status: row.status.clone(),
            }),
    );
    rows.push(MilestoneOneValidationAggregateRow {
        source: "seeded_bootstrap".to_string(),
        family: "SeededBootstrap".to_string(),
        validator: "naming".to_string(),
        status: if seeded_bootstrap.named_truth_validated {
            "passed".to_string()
        } else {
            "failed".to_string()
        },
    });
    rows.extend(primitive_corpus.cases.iter().flat_map(|case| {
        case.certification
            .topology_validation_report
            .rows
            .iter()
            .map(move |row| MilestoneOneValidationAggregateRow {
                source: case.stem.clone(),
                family: case.family.clone(),
                validator: row.validator.clone(),
                status: row.status.clone(),
            })
    }));
    rows.extend(
        primitive_corpus
            .cases
            .iter()
            .map(|case| MilestoneOneValidationAggregateRow {
                source: case.stem.clone(),
                family: case.family.clone(),
                validator: "naming".to_string(),
                status: if case.certification.named_truth_validated {
                    "passed".to_string()
                } else {
                    "failed".to_string()
                },
            }),
    );
    MilestoneOneValidationAggregateReport { rows }
}

pub(super) fn build_closeout_localization_report(
    seeded_bootstrap: &MilestoneOneCertificationReport,
    primitive_corpus: &PrimitiveCorpusReport,
) -> TopologyLocalizationAggregateReport {
    let mut topology_entities = Vec::new();
    let mut topology_relations = Vec::new();
    topology_entities.extend(
        seeded_bootstrap
            .topology_localization_report
            .topology_entities
            .iter()
            .map(|row| TopologyLocalizationAggregateEntityRow {
                source: "seeded_bootstrap".to_string(),
                entity_id: row.entity_id,
                kind_name: row.kind_name.clone(),
            }),
    );
    topology_relations.extend(
        seeded_bootstrap
            .topology_localization_report
            .topology_relations
            .iter()
            .map(|row| TopologyLocalizationAggregateRelationRow {
                source: "seeded_bootstrap".to_string(),
                relation_id: row.relation_id,
                kind_name: row.kind_name.clone(),
            }),
    );
    for case in &primitive_corpus.cases {
        topology_entities.extend(
            case.certification
                .topology_localization_report
                .topology_entities
                .iter()
                .map(|row| TopologyLocalizationAggregateEntityRow {
                    source: case.stem.clone(),
                    entity_id: row.entity_id,
                    kind_name: row.kind_name.clone(),
                }),
        );
        topology_relations.extend(
            case.certification
                .topology_localization_report
                .topology_relations
                .iter()
                .map(|row| TopologyLocalizationAggregateRelationRow {
                    source: case.stem.clone(),
                    relation_id: row.relation_id,
                    kind_name: row.kind_name.clone(),
                }),
        );
    }
    TopologyLocalizationAggregateReport {
        topology_entities,
        topology_relations,
    }
}

pub(super) fn build_closeout_naming_attachment_report(
    seeded_bootstrap: &MilestoneOneCertificationReport,
    primitive_corpus: &PrimitiveCorpusReport,
) -> NamingAttachmentAggregateReport {
    let mut attachments = Vec::new();
    let mut orphan_persistent_name_ids = BTreeSet::new();
    attachments.extend(
        seeded_bootstrap
            .naming_attachment_report
            .attachments
            .iter()
            .map(|row| NamingAttachmentAggregateRow {
                source: "seeded_bootstrap".to_string(),
                topology_entity_id: row.topology_entity_id,
                topology_kind_name: row.topology_kind_name.clone(),
                attached_persistent_name_ids: row.attached_persistent_name_ids.clone(),
            }),
    );
    orphan_persistent_name_ids.extend(
        seeded_bootstrap
            .naming_attachment_report
            .orphan_persistent_name_ids
            .iter()
            .copied(),
    );
    for case in &primitive_corpus.cases {
        attachments.extend(
            case.certification
                .naming_attachment_report
                .attachments
                .iter()
                .map(|row| NamingAttachmentAggregateRow {
                    source: case.stem.clone(),
                    topology_entity_id: row.topology_entity_id,
                    topology_kind_name: row.topology_kind_name.clone(),
                    attached_persistent_name_ids: row.attached_persistent_name_ids.clone(),
                }),
        );
        orphan_persistent_name_ids.extend(
            case.certification
                .naming_attachment_report
                .orphan_persistent_name_ids
                .iter()
                .copied(),
        );
    }
    NamingAttachmentAggregateReport {
        fully_named: orphan_persistent_name_ids.is_empty(),
        orphan_persistent_name_ids: orphan_persistent_name_ids.into_iter().collect(),
        attachments,
    }
}

pub(super) fn build_closeout_validator_coverage_report(
    aggregate: &MilestoneOneValidationAggregateReport,
) -> MilestoneOneValidatorCoverageReport {
    let mut rows = BTreeMap::<(String, String), MilestoneOneValidatorCoverageRow>::new();
    for row in &aggregate.rows {
        let entry = rows
            .entry((row.family.clone(), row.validator.clone()))
            .or_insert_with(|| MilestoneOneValidatorCoverageRow {
                family: row.family.clone(),
                validator: row.validator.clone(),
                passed_count: 0,
                source_count: 0,
            });
        entry.source_count += 1;
        if row.status == "passed" {
            entry.passed_count += 1;
        }
    }
    MilestoneOneValidatorCoverageReport {
        rows: rows.into_values().collect(),
    }
}




