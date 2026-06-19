use forge_query::facade::consumer_kit::{
    ForgeQueryGraphObligationConsumerKitError, ForgeQueryGraphObligationResidueManifest,
    ForgeQueryGraphObligationResidueRow,
};

use crate::topology_operators::adoption::catalog::topology_operator_graph_obligation_catalog;

use super::local_guard_residue::topology_operator_local_guard_residue_total;

pub fn topology_operator_graph_obligation_residue_manifest(
) -> Result<ForgeQueryGraphObligationResidueManifest, ForgeQueryGraphObligationConsumerKitError> {
    ForgeQueryGraphObligationResidueManifest::capped(
        topology_operator_graph_obligation_catalog()
            .residue_rows()
            .filter_map(|row| {
                Some(ForgeQueryGraphObligationResidueRow::explicit(
                    row.residue_class()?,
                    "worth-topo topology operator catalog",
                    "forge-query-9.9-phase-17",
                    residue_current_count_for_class(row.residue_class()?),
                    residue_cap_for_class(row.residue_class()?),
                    residue_blocker_for_class(row.residue_class()?),
                    residue_removal_trigger_for_class(row.residue_class()?),
                    "kept as explicit residue until the operator emits inspectable Query graph-obligation evidence",
                ))
            })
            .collect::<Result<Vec<_>, _>>()?,
    )
}

fn residue_blocker_for_class(class: &str) -> &'static str {
    match class {
        "wire-rehome-command-batch-operator" => {
            "wire rehome command-batch path has not yet been lowered through a covered operator graph obligation"
        }
        "shell-membership-command-batch-operator" => {
            "shell membership command-batch path has not yet been lowered through a covered operator graph obligation"
        }
        "face-inner-loop-command-batch-operator" => {
            "face inner-loop command-batch path has not yet been lowered through a covered operator graph obligation"
        }
        "scalar-topology-mutation-fronts" => {
            "scalar topology fronts still need per-touch selector registration and envelope proof"
        }
        "milestone-one-reference-integrity-pack" => {
            "relational custom invariant registrations are still the authoritative reference-integrity backstop"
        }
        "existing-entity-incoming-relation-count-mismatch-guards" => {
            "incoming relation-count local guards still protect non-covered local rewrite paths"
        }
        _ => "unclassified topology operator graph-obligation residue",
    }
}

fn residue_removal_trigger_for_class(class: &str) -> &'static str {
    match class {
        "milestone-one-reference-integrity-pack" => {
            "all milestone-one invariant registrations are represented as Query graph-obligation registrations with runtime envelope tests"
        }
        "existing-entity-incoming-relation-count-mismatch-guards" => {
            "every incoming relation-count local guard is deleted behind a covered obligation envelope or moved to a narrower residue row with a lower cap"
        }
        _ => {
            "the operator family has a covered catalog row, selector coverage, support pin, local ceremony audit, and runtime envelope test"
        }
    }
}

fn residue_current_count_for_class(class: &str) -> usize {
    match class {
        "existing-entity-incoming-relation-count-mismatch-guards" => {
            topology_operator_local_guard_residue_total()
        }
        _ => 1,
    }
}

fn residue_cap_for_class(class: &str) -> usize {
    match class {
        "existing-entity-incoming-relation-count-mismatch-guards" => {
            topology_operator_local_guard_residue_total()
        }
        _ => 1,
    }
}
