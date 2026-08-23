use std::collections::{BTreeSet, BinaryHeap};

use worth_ui_host_contract::UiMeasurementEvidenceFamily;

use super::{WorthUiMountedAllocationEstablishmentDenial, WorthUiMountedAllocationRuntimeStage};

type AllocationEntry = (
    crate::evidence::UiMeasurementBasis,
    crate::obligations::selection::UiSelectedObligationSet,
);

struct PartitionCandidate {
    entry: AllocationEntry,
    covered: BTreeSet<crate::graph::UiGraphNodeIdentity>,
}

pub(super) fn required_host_families(
    policy: &crate::declaration::UiDeclaredMeasurementPolicyPosture,
) -> BTreeSet<UiMeasurementEvidenceFamily> {
    let mut families = BTreeSet::new();
    if policy.requires_viewport_extent_observation() {
        families.insert(UiMeasurementEvidenceFamily::ViewportExtent);
    }
    if policy.requires_portal_anchor_observation() {
        families.insert(UiMeasurementEvidenceFamily::PortalAnchorRect);
    }
    for requirement in policy.evidence_requirements() {
        families.insert(match requirement {
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => {
                UiMeasurementEvidenceFamily::FontMetrics
            }
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
                UiMeasurementEvidenceFamily::ScrollContainerViewport
            }
            crate::declaration::UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => {
                UiMeasurementEvidenceFamily::PortalAnchorRect
            }
        });
    }
    families
}

pub(super) fn disjoint_partition(
    graph: &crate::graph::UiGraphSnapshot,
    entries: Vec<AllocationEntry>,
) -> Result<Vec<AllocationEntry>, WorthUiMountedAllocationEstablishmentDenial> {
    let mut uncovered = graph
        .allocation_planning_node_identities()
        .collect::<BTreeSet<_>>();
    let mut ranked = BinaryHeap::new();
    let mut candidates = entries
        .into_iter()
        .enumerate()
        .map(|(index, entry)| {
            let neighborhood = entry
                .0
                .admit_allocation_neighborhood(graph, &entry.1)
                .ok()?;
            let covered = neighborhood
                .members()
                .iter()
                .map(|member| member.graph_node_identity())
                .collect::<BTreeSet<_>>();
            (!covered.is_empty()).then(|| {
                ranked.push((covered.len(), index));
                PartitionCandidate { entry, covered }
            })
        })
        .collect::<Vec<_>>();
    let mut partition = Vec::new();
    while !uncovered.is_empty() {
        let chosen = next_disjoint(&mut ranked, &mut candidates, &uncovered).ok_or(
            WorthUiMountedAllocationEstablishmentDenial::Runtime(
                WorthUiMountedAllocationRuntimeStage::CatalogPreparation,
            ),
        )?;
        for identity in chosen.covered {
            uncovered.remove(&identity);
        }
        partition.push(chosen.entry);
    }
    Ok(partition)
}

fn next_disjoint(
    ranked: &mut BinaryHeap<(usize, usize)>,
    candidates: &mut [Option<PartitionCandidate>],
    uncovered: &BTreeSet<crate::graph::UiGraphNodeIdentity>,
) -> Option<PartitionCandidate> {
    while let Some((_, index)) = ranked.pop() {
        let candidate = candidates.get_mut(index)?.take()?;
        if candidate
            .covered
            .iter()
            .all(|identity| uncovered.contains(identity))
        {
            return Some(candidate);
        }
    }
    None
}
