use std::collections::{BTreeMap, BTreeSet};

use worth_primitives::{truth_digest_parts, TruthDigestScope};

use super::denial::{
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenial,
    PlanarBooleanOverlapRegionSummumBonumCloseoutDenialKind as Kind,
};
use super::subcases::PlanarBooleanOverlapRegionSummumBonumSubcaseKind as SubcaseKind;
use super::summum_bonum::{
    PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    PlanarBooleanOverlapRegionOrderingParityWitness,
    PlanarBooleanOverlapRegionSharedAreaOutcomeWitness,
    PlanarBooleanOverlapRegionSummumBonumCloseoutCounters,
};
use crate::workload_platform::planar_boolean_overlap_region_extraction::{
    PlanarBooleanOverlapRegionCanonicalWindingSourceKind,
    PlanarBooleanOverlapRegionLedgerAssemblyBundle, PlanarBooleanPostAdmissionNormalizationBundle,
    PlanarBooleanSharedAreaAdmissionBundle,
};

pub(super) struct WitnessMaterial {
    pub(super) boundary_only_outcome: PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness,
    pub(super) shared_area_outcome: PlanarBooleanOverlapRegionSharedAreaOutcomeWitness,
    pub(super) canonical_winding_outcome: PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness,
    pub(super) nested_identity_outcome: PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness,
    pub(super) mixed_boundary_area_outcome: PlanarBooleanOverlapRegionMixedBoundaryAreaWitness,
    pub(super) ordering_parity: PlanarBooleanOverlapRegionOrderingParityWitness,
}

pub(super) fn witness_material(
    shared_area_bundle: &PlanarBooleanSharedAreaAdmissionBundle,
    canonical_winding_bundle: &PlanarBooleanPostAdmissionNormalizationBundle,
) -> Result<WitnessMaterial, PlanarBooleanOverlapRegionSummumBonumCloseoutDenial> {
    let boundary_only_rows = canonical_winding_bundle
        .source_region_candidate_boundary()
        .boundary_only_overlap_outcomes()
        .rows();
    let canonical_rows = canonical_winding_bundle
        .overlap_region_canonical_winding()
        .rows();
    let boundary_only_canonical_rows: Vec<_> = canonical_rows
        .iter()
        .filter(|row| {
            row.source_kind()
                == PlanarBooleanOverlapRegionCanonicalWindingSourceKind::BoundaryOnlyOutcome
        })
        .collect();
    let boundary_only_offender = boundary_only_canonical_rows.iter().find(|row| {
        row.area_overlap_component_identity().is_some()
            || row.canonical_operand_side().is_some()
            || row.canonical_winding_sign().is_some()
    });
    if boundary_only_rows.is_empty()
        || boundary_only_canonical_rows.is_empty()
        || boundary_only_offender.is_some()
    {
        let detail = if let Some(row) = boundary_only_offender {
            format!(
                "phase-16 closeout requires carried boundary-only canonical witnesses without area or winding admission; source={} area={:?} operand={:?} sign={:?} boundary_rows={} canonical_rows={}",
                row.source_identity(),
                row.area_overlap_component_identity(),
                row.canonical_operand_side(),
                row.canonical_winding_sign(),
                boundary_only_rows.len(),
                boundary_only_canonical_rows.len(),
            )
        } else {
            format!(
                "phase-16 closeout requires carried boundary-only canonical witnesses; boundary_rows={} canonical_rows={}",
                boundary_only_rows.len(),
                boundary_only_canonical_rows.len(),
            )
        };
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::BoundaryOnlyAreaAdmission,
            SubcaseKind::BoundaryOnlyCoincidentEdgesDoNotAdmitArea.spec_name(),
            detail,
        ));
    }

    let shared_area_rows = shared_area_bundle.shared_area_admission_outcomes().rows();
    let mixed_contact_rows = shared_area_bundle.mixed_boundary_area_outcomes().rows();
    let admitted_canonical_rows: Vec<_> = canonical_rows
        .iter()
        .filter(|row| {
            row.source_kind()
                == PlanarBooleanOverlapRegionCanonicalWindingSourceKind::AdmittedRegion
        })
        .collect();
    if shared_area_rows.is_empty()
        || admitted_canonical_rows.is_empty()
        || admitted_canonical_rows.iter().any(|row| {
            row.area_overlap_component_identity().is_none()
                || row.canonical_operand_side().is_none()
                || row.canonical_winding_sign().is_none()
        })
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::OppositeSenseWindingInstability,
            SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding.spec_name(),
            "phase-16 closeout requires admitted shared-area outcomes with carried canonical winding authority",
        ));
    }

    let canonical_pairs = canonical_rows.iter().fold(BTreeMap::new(), |mut map, row| {
        map.entry(row.source_identity())
            .or_insert(row.canonical_winding_identity());
        map
    });
    for row in canonical_rows {
        if canonical_pairs.get(row.source_identity()) != Some(&row.canonical_winding_identity()) {
            return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
                Kind::OppositeSenseWindingInstability,
                SubcaseKind::OppositeSenseSameAreaOverlapHasStableWinding.spec_name(),
                "phase-16 closeout requires one stable canonical winding identity per carried overlap outcome",
            ));
        }
    }

    let nested_rows: Vec<_> = canonical_rows
        .iter()
        .filter(|row| {
            row.lineage_identities().len() > 1 || row.canonical_source_loop_identities().len() > 1
        })
        .collect();
    let admitted_area_rows_by_island =
        admitted_canonical_rows
            .iter()
            .fold(BTreeMap::<&str, usize>::new(), |mut grouped, row| {
                *grouped.entry(row.island_identity()).or_default() += 1;
                grouped
            });
    let has_nested_multi_region_island = admitted_area_rows_by_island
        .values()
        .any(|row_count| *row_count > 1);
    if (nested_rows.is_empty() && !has_nested_multi_region_island)
        || admitted_canonical_rows
            .iter()
            .any(|row| row.lineage_identities().is_empty())
    {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::NestedIdentityInstability,
            SubcaseKind::NestedOverlapIslandsPreserveRegionIdentity.spec_name(),
            "phase-16 closeout requires carried overlap rows whose lineage preserves nested region identity",
        ));
    }

    if mixed_contact_rows.is_empty() {
        return Err(PlanarBooleanOverlapRegionSummumBonumCloseoutDenial::new(
            Kind::MixedBoundaryAreaCollapse,
            SubcaseKind::MixedBoundaryAndAreaContactDoesNotCollapse.spec_name(),
            "phase-16 closeout requires carried mixed boundary-area outcomes instead of reconstructing them from ledger coincidence",
        ));
    }

    let boundary_only_digest = digest_for(
        "boundary-only-outcome",
        boundary_only_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}",
                    row.outcome_identity(),
                    row.pure_boundary_only_outcome_identity()
                )
            })
            .collect(),
    );
    let shared_area_digest = digest_for(
        "shared-area-outcome",
        shared_area_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}",
                    row.outcome_identity(),
                    row.area_overlap_component_identity()
                )
            })
            .collect(),
    );
    let winding_digest = digest_for(
        "canonical-winding-outcome",
        canonical_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{:?}:{:?}",
                    row.source_identity(),
                    row.canonical_winding_identity(),
                    row.canonical_operand_side(),
                    row.canonical_winding_sign()
                )
            })
            .collect(),
    );
    let nested_digest = digest_for(
        "nested-identity-outcome",
        nested_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}",
                    row.source_identity(),
                    row.lineage_identities().join("|")
                )
            })
            .collect(),
    );
    let mixed_digest = digest_for(
        "mixed-boundary-area-outcome",
        mixed_contact_rows
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.outcome_identity(),
                    row.area_overlap_component_identities().join("|"),
                    row.boundary_contact_component_identities().join("|")
                )
            })
            .collect(),
    );
    let canonical_order_digest = digest_for(
        "ordering-canonical",
        canonical_rows.iter().map(ordering_projection).collect(),
    );
    let mut sorted = canonical_rows
        .iter()
        .map(ordering_projection)
        .collect::<Vec<_>>();
    sorted.sort();
    let order_invariant_digest = digest_for("ordering-canonical", sorted);

    Ok(WitnessMaterial {
        boundary_only_outcome: PlanarBooleanOverlapRegionBoundaryOnlyOutcomeWitness {
            digest: boundary_only_digest,
            region_count: boundary_only_rows
                .iter()
                .map(|row| row.outcome_identity())
                .collect::<BTreeSet<_>>()
                .len(),
            row_count: boundary_only_rows.len(),
        },
        shared_area_outcome: PlanarBooleanOverlapRegionSharedAreaOutcomeWitness {
            digest: shared_area_digest,
            component_count: shared_area_rows
                .iter()
                .map(|row| row.area_overlap_component_identity())
                .collect::<BTreeSet<_>>()
                .len(),
            row_count: shared_area_rows.len(),
        },
        canonical_winding_outcome: PlanarBooleanOverlapRegionCanonicalWindingOutcomeWitness {
            digest: winding_digest,
            stable_region_count: canonical_pairs.len(),
        },
        nested_identity_outcome: PlanarBooleanOverlapRegionNestedIdentityOutcomeWitness {
            digest: nested_digest,
            nested_region_count: nested_rows
                .iter()
                .map(|row| row.source_identity())
                .collect::<BTreeSet<_>>()
                .len(),
        },
        mixed_boundary_area_outcome: PlanarBooleanOverlapRegionMixedBoundaryAreaWitness {
            digest: mixed_digest,
            boundary_only_rows: mixed_contact_rows.len(),
            area_rows: shared_area_rows.len(),
        },
        ordering_parity: PlanarBooleanOverlapRegionOrderingParityWitness {
            canonical_digest: canonical_order_digest,
            order_invariant_digest,
        },
    })
}

pub(super) fn build_counters(
    bundle: &PlanarBooleanOverlapRegionLedgerAssemblyBundle,
    replay_rows_verified: usize,
    material: &WitnessMaterial,
) -> PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
    let bundle_counters = bundle.counters();
    PlanarBooleanOverlapRegionSummumBonumCloseoutCounters {
        readiness_inputs_consumed: 1,
        overlap_ledger_receipts_consumed: 1,
        replay_rows_verified,
        decision_rows_verified: bundle_counters.decision_rows_admitted(),
        ledger_rows_verified: bundle_counters.ledger_rows_admitted(),
        boundary_only_rows_verified: material.boundary_only_outcome.row_count(),
        area_rows_verified: material.shared_area_outcome.row_count(),
        mixed_boundary_area_rows_verified: usize::from(
            material.mixed_boundary_area_outcome.boundary_only_rows() > 0
                && material.mixed_boundary_area_outcome.area_rows() > 0,
        ),
        pairwise_rediscovery_attempts: 0,
    }
}

fn ordering_projection(
    row: &crate::workload_platform::planar_boolean_overlap_region_extraction::PlanarBooleanOverlapRegionCanonicalWindingRow,
) -> String {
    format!(
        "{:?}:{}:{}:{}:{:?}:{:?}",
        row.source_kind(),
        row.source_identity(),
        row.canonical_winding_identity(),
        row.area_overlap_component_identity().unwrap_or_default(),
        row.canonical_operand_side(),
        row.canonical_winding_sign()
    )
}

fn digest_for(label: &str, mut values: Vec<String>) -> String {
    values.sort();
    truth_digest_parts(
        TruthDigestScope::ArtifactIdentity,
        &[vec![label.to_string()], values].concat(),
    )
}
