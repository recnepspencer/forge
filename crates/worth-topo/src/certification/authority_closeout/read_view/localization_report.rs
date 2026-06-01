use super::*;

pub(super) fn build_topology_localization_report_from_query_rows(
    entity_rows: &[ForgeQueryEntity],
    relation_rows: &[ForgeQueryEntity],
) -> Result<TopologyLocalizationReport, MilestoneOneCertificationError> {
    let topology_entities = entity_rows
        .iter()
        .map(|row| {
            Ok(TopologyLocalizationEntityRow {
                entity_id: serde_json::from_value(required_payload_value(
                    &row.external_row(),
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    MilestoneOneCertificationError::Query(format!(
                        "query certification entity lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.external_row(), "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, MilestoneOneCertificationError>>()?;
    let topology_relations = relation_rows
        .iter()
        .map(|row| {
            Ok(TopologyLocalizationRelationRow {
                relation_id: serde_json::from_value(required_payload_value(
                    &row.external_row(),
                    "lineage.provenance",
                )?)
                .map_err(|error| {
                    MilestoneOneCertificationError::Query(format!(
                        "query certification relation lineage provenance failed to decode: {error}"
                    ))
                })?,
                kind_name: required_payload_text(&row.external_row(), "topology.kind")?.to_string(),
            })
        })
        .collect::<Result<Vec<_>, MilestoneOneCertificationError>>()?;

    Ok(TopologyLocalizationReport {
        topology_entities,
        topology_relations,
    })
}

fn required_payload_value(
    payload: &serde_json::Value,
    dotted_path: &str,
) -> Result<serde_json::Value, MilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            MilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    Ok(current.clone())
}

fn required_payload_text<'a>(
    payload: &'a serde_json::Value,
    dotted_path: &str,
) -> Result<&'a str, MilestoneOneCertificationError> {
    let mut current = payload;
    for segment in dotted_path.split('.') {
        current = current.get(segment).ok_or_else(|| {
            MilestoneOneCertificationError::Query(format!(
                "query certification row is missing required field `{dotted_path}`"
            ))
        })?;
    }
    current.as_str().ok_or_else(|| {
        MilestoneOneCertificationError::Query(format!(
            "query certification field `{dotted_path}` must decode as text"
        ))
    })
}

pub(crate) fn build_primitive_family_coverage_matrix(
    interpretations: &schema::facade::platform::authority::TopologyInterpretationRecordSet,
) -> PrimitiveFamilyCoverageMatrix {
    let wire_open = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::OpenChain)
        .count();
    let wire_closed = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::ClosedCycle)
        .count();
    let wire_branch = interpretations
        .wires
        .iter()
        .filter(|record| record.class == WireInterpretationClass::ConnectedBranch)
        .count();
    let sheet_patch = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == ShellInterpretationClass::OpenSheet && record.face_count > 1
        })
        .count();
    let sheet_disk = interpretations
        .shells
        .iter()
        .filter(|record| {
            record.class == ShellInterpretationClass::OpenSheet
                && record.face_count == 1
                && record.boundary_component_count == 1
        })
        .count();
    let solid_shell = interpretations
        .shells
        .iter()
        .filter(|record| record.class == ShellInterpretationClass::ClosedSolid)
        .count();
    let nmt_edge_fan = interpretations
        .shells
        .iter()
        .filter(|record| {
            matches!(
                record.class,
                ShellInterpretationClass::OpenNonManifold
                    | ShellInterpretationClass::ClosedNonManifold
            )
        })
        .count();

    PrimitiveFamilyCoverageMatrix {
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

pub(crate) fn build_counter_report(
    authority_mutations: Option<&[TopologyMutation]>,
    topology_validation_report: &crate::validation::TopologyValidationReport,
    naming_attachment_report: &NamingAttachmentReport,
    primitive_family_coverage_matrix: &PrimitiveFamilyCoverageMatrix,
    read_basis: &DerivedTopologyReadBasis,
    replay_history_length: usize,
) -> MilestoneOneCounters {
    let (
        topology_entity_upsert_count,
        topology_relation_upsert_count,
        topology_relation_remove_count,
    ) = authority_mutations
        .map(count_topology_mutations)
        .unwrap_or((0, 0, 0));
    let derived_topology_full_fallback_count = read_basis
        .precision_fallbacks
        .iter()
        .filter(|record| record.disposition != FallbackDisposition::NoneRequired)
        .count()
        + read_basis.precision_budget_fallbacks.len();
    let touched_topology_aspect_count = read_basis
        .touched_aspects()
        .iter()
        .filter(|aspect| matches!(aspect, Aspect::Topology(_)))
        .count();

    MilestoneOneCounters {
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
