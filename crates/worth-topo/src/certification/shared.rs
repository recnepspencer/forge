use schema::facade::topology_authoring::MilestoneOnePrimitiveCase;
use schema::facade::{TopologyMutation, TopologyMutationBatch};

use crate::certification::report::{
    DeterministicDigest, PrimitiveCorpusCaseReport, PrimitiveCorpusCoverageEntry,
    PrimitiveCorpusParityEntry, PrimitiveFamilyCoverageEntry,
};

pub(crate) fn coverage_entry(
    family: &str,
    observed_member_count: usize,
) -> PrimitiveFamilyCoverageEntry {
    PrimitiveFamilyCoverageEntry {
        family: family.to_string(),
        observed: observed_member_count > 0,
        observed_member_count,
    }
}

pub(crate) fn primitive_family_name(primitive: &MilestoneOnePrimitiveCase) -> &'static str {
    match primitive {
        MilestoneOnePrimitiveCase::WireOpen { .. } => "WireOpen(n)",
        MilestoneOnePrimitiveCase::WireClosed { .. } => "WireClosed(n)",
        MilestoneOnePrimitiveCase::WireBranch { .. } => "WireBranch(k)",
        MilestoneOnePrimitiveCase::SheetDisk { .. } => "SheetDisk(n)",
        MilestoneOnePrimitiveCase::SheetPatch { .. } => "SheetPatch(f)",
        MilestoneOnePrimitiveCase::SolidShell { .. } => "SolidShell(f)",
        MilestoneOnePrimitiveCase::NmtEdgeFan { .. } => "NmtEdgeFan(k)",
    }
}

pub(crate) fn canonical_milestone_one_primitive_families() -> [&'static str; 7] {
    [
        "WireOpen(n)",
        "WireClosed(n)",
        "WireBranch(k)",
        "SheetDisk(n)",
        "SheetPatch(f)",
        "SolidShell(f)",
        "NmtEdgeFan(k)",
    ]
}

pub(crate) fn admitted_range_expected_mainline_count(family: &str) -> usize {
    match family {
        "WireOpen(n)" => 12,
        "WireClosed(n)" => 10,
        "WireBranch(k)" => 10,
        "SheetDisk(n)" => 10,
        "SheetPatch(f)" => 9,
        "SolidShell(f)" => 7,
        "NmtEdgeFan(k)" => 8,
        _ => 0,
    }
}

pub(crate) fn admitted_range_expected_branch_local_count(family: &str) -> usize {
    match family {
        "WireBranch(k)" => 10,
        "SheetPatch(f)" => 9,
        "SolidShell(f)" => 7,
        "NmtEdgeFan(k)" => 8,
        _ => 0,
    }
}

pub(crate) fn validator_expectations_for_family(family: &str) -> &'static [&'static str] {
    match family {
        "WireOpen(n)" => &["ownership", "loop_wiring", "naming"],
        "WireClosed(n)" => &["ownership", "loop_wiring", "naming"],
        "WireBranch(k)" => &["ownership", "loop_wiring", "vertex_branching", "naming"],
        "SheetDisk(n)" => &["ownership", "loop_wiring", "shell_closure", "naming"],
        "SheetPatch(f)" => &[
            "ownership",
            "loop_wiring",
            "shell_closure",
            "radial",
            "naming",
        ],
        "SolidShell(f)" => &[
            "ownership",
            "loop_wiring",
            "shell_closure",
            "radial",
            "naming",
        ],
        "NmtEdgeFan(k)" => &["ownership", "loop_wiring", "radial", "naming"],
        _ => &[],
    }
}

pub(crate) fn derived_validator_expectations_for_family(family: &str) -> &'static [&'static str] {
    match family {
        "WireOpen(n)" => &["ownership", "loop_wiring"],
        "WireClosed(n)" => &["ownership", "loop_wiring"],
        "WireBranch(k)" => &["ownership", "loop_wiring", "vertex_branching"],
        "SheetDisk(n)" => &["ownership", "loop_wiring", "shell_closure"],
        "SheetPatch(f)" => &["ownership", "loop_wiring", "shell_closure", "radial"],
        "SolidShell(f)" => &["ownership", "loop_wiring", "shell_closure", "radial"],
        "NmtEdgeFan(k)" => &["ownership", "loop_wiring", "radial"],
        _ => &[],
    }
}

pub(crate) fn empty_corpus_coverage_entry(family: &str) -> PrimitiveCorpusCoverageEntry {
    PrimitiveCorpusCoverageEntry {
        family: family.to_string(),
        admitted_smallest_count: 0,
        admitted_generic_count: 0,
        admitted_hostile_count: 0,
        rejected_out_of_class_count: 0,
        role_closure_complete: false,
    }
}

pub(crate) fn empty_corpus_parity_entry(family: &str) -> PrimitiveCorpusParityEntry {
    PrimitiveCorpusParityEntry {
        family: family.to_string(),
        mainline_case_count: 0,
        branch_local_case_count: 0,
        branch_ids: Vec::new(),
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

pub(crate) fn parity_case_key(case: &PrimitiveCorpusCaseReport) -> String {
    format!("{}:{:?}:{:?}", case.family, case.role, case.primitive)
}

pub(crate) fn digest_rows(rows: impl Iterator<Item = String>) -> DeterministicDigest {
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

    DeterministicDigest {
        algorithm: "fnv1a64".to_string(),
        digest_hex: format!("{hash:016x}"),
        row_count: count,
    }
}

pub(crate) fn count_batch_mutations(batch: &TopologyMutationBatch) -> (usize, usize, usize) {
    let mut entity_upserts = 0usize;
    let mut relation_upserts = 0usize;
    let mut relation_removes = 0usize;

    for mutation in &batch.mutations {
        match mutation {
            TopologyMutation::CreateEntity { kind, .. }
            | TopologyMutation::UpsertEntity { kind, .. }
                if matches!(kind, schema::facade::EntityKind::Topology(_)) =>
            {
                entity_upserts += 1;
            }
            TopologyMutation::CreateRelation { kind, .. }
            | TopologyMutation::UpsertRelation { kind, .. }
                if matches!(kind, schema::facade::RelationKind::Topology(_)) =>
            {
                relation_upserts += 1;
            }
            TopologyMutation::RemoveRelation { .. } => {
                relation_removes += 1;
            }
            _ => {}
        }
    }

    (entity_upserts, relation_upserts, relation_removes)
}
