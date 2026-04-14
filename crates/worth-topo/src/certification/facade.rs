use std::collections::{BTreeMap, BTreeSet};

use forge_relational::facade::identity::{EntityId, KindId};
use forge_relational::facade::runtime::RelationalReadView;
use worth_schema::facade::{
    DerivedTopologyReadBasis, WorthEntityKind, WorthMutationOrigin, WorthNamingEntityKind,
    WorthNamingRelationKind, WorthRelationKind, WorthShellInterpretationClass,
    WorthTopologyEntityKind, WorthTopologyRelationKind, WorthWireInterpretationClass,
};

use crate::certification::error::WorthMilestoneOneCertificationError;
use crate::certification::report::{
    WorthBranchLocalTopologyReport, WorthDeterministicDigest,
    WorthMilestoneOneCertificationReport, WorthNamingAttachmentReport, WorthNamingAttachmentRow,
    WorthPrimitiveFamilyCoverageEntry, WorthPrimitiveFamilyCoverageMatrix, WorthReplayParityReport,
    WorthTopologyLocalizationEntityRow, WorthTopologyLocalizationRelationRow,
    WorthTopologyLocalizationReport,
};
use crate::facade::{
    build_topology_read_artifact, certify_topology_view, validate_named_topology_truth,
    validate_topology_view, WorthTopologyMaterializer,
};

#[derive(Debug, Default, Clone, Copy)]
pub struct WorthMilestoneOneCertificationHarness;

impl WorthMilestoneOneCertificationHarness {
    pub fn certify_read_view(
        read_view: &RelationalReadView,
        read_basis: DerivedTopologyReadBasis,
    ) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
        validate_named_topology_truth(read_view)?;
        let topology = WorthTopologyMaterializer::materialize_from_truth(read_view)?;
        validate_topology_view(&topology)?;
        let read_artifact = build_topology_read_artifact(&read_basis, &topology);
        let certified_interpretation = certify_topology_view(read_basis.clone(), &topology);

        let topology_localization_report = build_topology_localization_report(read_view);
        let naming_attachment_report = build_naming_attachment_report(read_view);
        let primitive_family_coverage_matrix =
            build_primitive_family_coverage_matrix(&read_artifact.interpretations);
        let branch_local_topology_report = WorthBranchLocalTopologyReport {
            mutation_origin: read_basis.mutation_origin,
            branch_local: matches!(read_basis.mutation_origin, WorthMutationOrigin::BranchLocalApplication),
            snapshot_id: read_basis.snapshot.snapshot_id.0,
            touched_aspect_count: read_basis.touched_aspects.len(),
        };
        let milestone_1_replay_parity_report = WorthReplayParityReport {
            mutation_origin: read_basis.mutation_origin,
            replay_origin: matches!(read_basis.mutation_origin, WorthMutationOrigin::Replay),
            parity_status: if matches!(read_basis.mutation_origin, WorthMutationOrigin::Replay) {
                "replay-origin".to_string()
            } else {
                "direct-origin".to_string()
            },
        };

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
            primitive_family_coverage_matrix.entries.iter().map(|row| {
                format!(
                    "family:{}:{}:{}",
                    row.family, row.observed, row.observed_member_count
                )
            }),
        );

        Ok(WorthMilestoneOneCertificationReport {
            named_truth_validated: true,
            topology_validated: true,
            topology_truth_digest,
            naming_truth_digest,
            topology_validation_digest,
            topology_localization_report,
            naming_attachment_report,
            primitive_family_coverage_matrix,
            branch_local_topology_report,
            milestone_1_replay_parity_report,
            read_artifact,
            certified_interpretation,
        })
    }
}

pub fn certify_milestone_one_read_view(
    read_view: &RelationalReadView,
    read_basis: DerivedTopologyReadBasis,
) -> Result<WorthMilestoneOneCertificationReport, WorthMilestoneOneCertificationError> {
    WorthMilestoneOneCertificationHarness::certify_read_view(read_view, read_basis)
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
        .filter(|record| record.class == WorthShellInterpretationClass::OpenSheet)
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
            coverage_entry("SheetDisk(n)", 0),
            coverage_entry("SheetPatch(f)", sheet_patch),
            coverage_entry("SolidShell(f)", solid_shell),
            coverage_entry("NmtEdgeFan(k)", nmt_edge_fan),
        ],
    }
}

fn coverage_entry(family: &str, observed_member_count: usize) -> WorthPrimitiveFamilyCoverageEntry {
    WorthPrimitiveFamilyCoverageEntry {
        family: family.to_string(),
        observed: observed_member_count > 0,
        observed_member_count,
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
